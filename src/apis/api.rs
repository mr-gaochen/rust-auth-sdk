use serde_json::Value;

use super::configuration::Configuration;
use crate::model::UserInfo;
use anyhow::{Context, Result};

/// 根据token 获取当前用户信息
pub async fn get_user_info(config: &Configuration) -> Result<UserInfo> {
    let uri_str = format!("{}/v1/user/current", config.base_path);
    let client = &config.client;
    let request_builder = if let Some(token) = &config.bearer_access_token {
        client.get(uri_str).header("Token", token)
    } else {
        client.get(uri_str)
    };
    let response = request_builder
        .send()
        .await
        .context("Auth check - Failed to send request")?;
    if !response.status().is_success() {
        return Err(anyhow::anyhow!("Error status: {}", response.status()));
    }

    let body = response
        .text()
        .await
        .context("Failed to read response body")?;

    let data: Value = serde_json::from_str(&body).context("Failed to parse JSON response")?;

    if data["code"] != "200" {
        return Err(anyhow::anyhow!("Error code: {}", data["code"]));
    }
    let user_info: UserInfo =
        serde_json::from_value(data["data"].clone()).context("Failed to parse UserInfo")?;

    Ok(user_info) // 返回 UserInfo
}

#[cfg(test)]
mod tests {
    use crate::apis::configuration::Configuration;

    use super::get_user_info;

    #[tokio::test]
    async fn test_get_user_info() {
        let config = Configuration::new();
        let user = get_user_info(&config).await.unwrap();
        println!("user:{:?}", user);
    }
}
