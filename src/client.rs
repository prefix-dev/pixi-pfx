use cynic::GraphQlResponse;
use rattler_networking::{Authentication, AuthenticationStorage};
use serde::Serialize;

use crate::error::PfxError;

#[allow(dead_code)]
pub const DEFAULT_ENDPOINT: &str = "https://prefix.dev/api/graphql";

pub struct PrefixClient {
    http: reqwest::Client,
    endpoint: String,
    token: Option<String>,
    auth_storage: Result<AuthenticationStorage, String>,
}

impl PrefixClient {
    pub fn new(endpoint: String, token: Option<String>) -> Self {
        Self {
            http: reqwest::Client::new(),
            endpoint,
            token,
            auth_storage: AuthenticationStorage::from_env_and_defaults()
                .map_err(|error| error.to_string()),
        }
    }

    pub async fn authentication_status(&self) -> Result<serde_json::Value, PfxError> {
        if self.token.is_some() {
            return Ok(serde_json::json!({
                "endpoint": self.endpoint,
                "source": "explicit-token",
                "method": "BearerToken",
            }));
        }

        let storage = self
            .auth_storage
            .as_ref()
            .map_err(|error| PfxError::AuthStorage(error.clone()))?;
        let (_, auth) = storage
            .get_by_url_refreshed(&self.endpoint)
            .await
            .map_err(|error| PfxError::AuthStorage(error.to_string()))?;
        let status = match auth {
            Some(Authentication::OAuth {
                expires_at,
                refresh_token,
                token_endpoint,
                client_id,
                ..
            }) => serde_json::json!({
                "endpoint": self.endpoint,
                "source": "rattler-storage",
                "method": "OAuth",
                "expires_at": expires_at,
                "has_refresh_token": refresh_token.is_some(),
                "token_endpoint": token_endpoint,
                "client_id": client_id,
            }),
            Some(auth) => serde_json::json!({
                "endpoint": self.endpoint,
                "source": "rattler-storage",
                "method": auth.method(),
            }),
            None => serde_json::json!({
                "endpoint": self.endpoint,
                "source": null,
                "method": null,
            }),
        };
        Ok(status)
    }

    pub async fn execute<ResponseData, Vars>(
        &self,
        operation: cynic::Operation<ResponseData, Vars>,
    ) -> Result<ResponseData, PfxError>
    where
        ResponseData: serde::de::DeserializeOwned,
        Vars: Serialize,
    {
        let mut endpoint = self.endpoint.clone();
        let stored_auth = if self.token.is_none() {
            let storage = self
                .auth_storage
                .as_ref()
                .map_err(|error| PfxError::AuthStorage(error.clone()))?;
            let (resolved_url, auth) = storage
                .get_by_url_refreshed(&endpoint)
                .await
                .map_err(|error| PfxError::AuthStorage(error.to_string()))?;
            endpoint = resolved_url.to_string();
            auth
        } else {
            None
        };

        let mut req = self.http.post(endpoint).json(&operation);
        if let Some(token) = &self.token {
            req = req.bearer_auth(token);
        } else if let Some(auth) = stored_auth {
            req = match auth {
                Authentication::BearerToken(token)
                | Authentication::OAuth {
                    access_token: token,
                    ..
                } => req.bearer_auth(token),
                Authentication::BasicHTTP { username, password } => {
                    req.basic_auth(username, Some(password))
                }
                // Conda tokens are already inserted into the resolved URL by
                // AuthenticationStorage. Other credential types do not apply
                // to the HTTP GraphQL endpoint.
                Authentication::CondaToken(_) | Authentication::S3Credentials { .. } => req,
            };
        }

        let resp = req.send().await?;
        let text = resp.text().await?;
        let gql_resp: GraphQlResponse<ResponseData> =
            serde_json::from_str(&text).map_err(|e| PfxError::Graphql {
                message: format!("Failed to decode response: {e}"),
                details: Some(serde_json::json!({ "body_preview": &text[..text.len().min(500)] })),
            })?;

        if let Some(errors) = gql_resp.errors
            && !errors.is_empty()
        {
            let message = errors
                .iter()
                .map(|e| e.message.clone())
                .collect::<Vec<_>>()
                .join("; ");
            let details: Option<serde_json::Value> = Some(serde_json::Value::Array(
                errors
                    .iter()
                    .map(|e| {
                        serde_json::json!({
                            "message": e.message,
                            "path": format!("{:?}", e.path),
                        })
                    })
                    .collect(),
            ));
            return Err(PfxError::Graphql { message, details });
        }

        gql_resp.data.ok_or_else(|| PfxError::Graphql {
            message: "No data in response".to_string(),
            details: None,
        })
    }
}
