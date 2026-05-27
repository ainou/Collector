use semver::Version;

pub async fn check_for_update() -> Option<String> {
    let response = reqwest::Client::new()
        .get("https://api.github.com/repos/juliandeans/Collector/releases/latest")
        .header(reqwest::header::USER_AGENT, "Collector-App")
        .send()
        .await
        .ok()?
        .error_for_status()
        .ok()?;
    let payload: serde_json::Value = response.json().await.ok()?;
    let tag_name = payload.get("tag_name")?.as_str()?;
    let latest = tag_name.trim_start_matches('v');
    let current_version = Version::parse(env!("CARGO_PKG_VERSION")).ok()?;
    let latest_version = Version::parse(latest).ok()?;

    if latest_version > current_version {
        Some(latest.to_string())
    } else {
        None
    }
}
