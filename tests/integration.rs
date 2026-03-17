#[cfg(feature = "online-tests")]
mod online {
    use std::process::Command;

    fn run_pfx(args: &[&str]) -> (String, bool) {
        let output = Command::new("cargo")
            .args(["run", "--quiet", "--"])
            .args(args)
            .output()
            .expect("Failed to execute pixi-pfx");
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        (stdout, output.status.success())
    }

    fn parse_output(args: &[&str]) -> serde_json::Value {
        let (stdout, _) = run_pfx(args);
        serde_json::from_str(&stdout).expect("Failed to parse JSON output")
    }

    #[test]
    fn test_channel_get_conda_forge() {
        let result = parse_output(&["channel", "get", "conda-forge"]);
        assert_eq!(result["ok"], true);
        assert_eq!(result["data"]["name"], "conda-forge");
        assert_eq!(result["data"]["is_public"], true);
    }

    #[test]
    fn test_nonexistent_channel_returns_null() {
        let result = parse_output(&["channel", "get", "nonexistent-channel-99999"]);
        assert_eq!(result["ok"], true);
        assert!(result["data"].is_null());
    }

    #[test]
    fn test_channel_list() {
        let result = parse_output(&["channel", "list", "--limit", "3", "--public"]);
        assert_eq!(result["ok"], true);
        let page = &result["data"]["page"];
        assert!(page.is_array());
        assert!(page.as_array().unwrap().len() <= 3);
    }

    #[test]
    fn test_package_search_numpy() {
        let result = parse_output(&["package", "search", "numpy", "--limit", "5"]);
        assert_eq!(result["ok"], true);
        let page = &result["data"]["page"];
        assert!(page.is_array());
        assert!(!page.as_array().unwrap().is_empty());
    }

    #[test]
    fn test_package_get_conda_forge_numpy() {
        let result = parse_output(&[
            "package",
            "get",
            "conda-forge",
            "numpy",
            "--variants-limit",
            "2",
        ]);
        assert_eq!(result["ok"], true);
        assert_eq!(result["data"]["name"], "numpy");
        assert_eq!(result["data"]["channel"]["name"], "conda-forge");
    }

    #[test]
    fn test_package_versions() {
        let result = parse_output(&[
            "package",
            "versions",
            "conda-forge",
            "numpy",
            "--limit",
            "3",
        ]);
        assert_eq!(result["ok"], true);
        assert_eq!(result["data"]["name"], "numpy");
        let versions = &result["data"]["versions"]["page"];
        assert!(versions.is_array());
    }

    #[test]
    fn test_package_matchspec() {
        let result = parse_output(&[
            "package",
            "matchspec",
            "numpy",
            "--channel",
            "conda-forge",
        ]);
        assert_eq!(result["ok"], true);
        assert_eq!(result["data"]["name"], "numpy");
    }

    #[test]
    fn test_whoami_unauthenticated() {
        let result = parse_output(&["auth", "whoami"]);
        assert_eq!(result["ok"], true);
        assert!(result["data"].is_null());
    }

    #[test]
    fn test_describe() {
        let result = parse_output(&["describe"]);
        assert_eq!(result["ok"], true);
        assert!(result["data"]["commands"]["channel"].is_object());
        assert!(result["data"]["commands"]["package"].is_object());
        assert!(result["data"]["commands"]["auth"].is_object());
    }

    #[test]
    fn test_describe_specific_command() {
        let result = parse_output(&["describe", "channel", "get"]);
        assert_eq!(result["ok"], true);
        assert!(result["data"]["description"].is_string());
        assert!(result["data"]["args"].is_array());
    }

    #[test]
    fn test_channel_list_with_search() {
        let result = parse_output(&[
            "channel",
            "list",
            "--search",
            "conda",
            "--limit",
            "5",
        ]);
        assert_eq!(result["ok"], true);
        let page = &result["data"]["page"];
        assert!(page.is_array());
    }
}
