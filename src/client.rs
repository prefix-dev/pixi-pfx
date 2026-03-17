use cynic::GraphQlResponse;
use serde::Serialize;

use crate::error::PfxError;

#[allow(dead_code)]
pub const DEFAULT_ENDPOINT: &str = "https://prefix.dev/api/graphql";

pub struct PrefixClient {
    http: reqwest::Client,
    endpoint: String,
    token: Option<String>,
}

impl PrefixClient {
    pub fn new(endpoint: String, token: Option<String>) -> Self {
        Self {
            http: reqwest::Client::new(),
            endpoint,
            token,
        }
    }

    pub async fn execute<ResponseData, Vars>(
        &self,
        operation: cynic::Operation<ResponseData, Vars>,
    ) -> Result<ResponseData, PfxError>
    where
        ResponseData: serde::de::DeserializeOwned,
        Vars: Serialize,
    {
        let mut req = self.http.post(&self.endpoint).json(&operation);

        if let Some(token) = &self.token {
            req = req.header("Authorization", format!("Bearer {token}"));
        }

        let resp = req.send().await?;
        let text = resp.text().await?;
        let gql_resp: GraphQlResponse<ResponseData> = serde_json::from_str(&text)
            .map_err(|e| PfxError::Graphql {
                message: format!("Failed to decode response: {e}"),
                details: Some(serde_json::json!({ "body_preview": &text[..text.len().min(500)] })),
            })?;

        if let Some(errors) = gql_resp.errors {
            if !errors.is_empty() {
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
        }

        gql_resp.data.ok_or_else(|| PfxError::Graphql {
            message: "No data in response".to_string(),
            details: None,
        })
    }
}
