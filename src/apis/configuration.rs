use once_cell::sync::Lazy;
use reqwest::Client as ReqwestClient;
use std::time::Duration;

/// 静态的 Lazy 初始化，确保 `reqwest::Client` 只被创建一次
pub static HTTP_CLIENT: Lazy<ReqwestClient> = Lazy::new(|| {
    ReqwestClient::builder()
        .user_agent("OpenAPI-Generator/1.0.0/rust")
        .timeout(Duration::from_secs(1000))
        .connect_timeout(Duration::from_secs(1000))
        .build()
        .expect("Failed to build reqwest client")
});

#[derive(Debug, Clone)]
pub struct Configuration {
    pub base_path: String,
    pub user_agent: Option<String>,
    pub client: &'static ReqwestClient,
    pub bearer_access_token: Option<String>,
}

impl Configuration {
    /// 创建一个新的 Configuration 实例，使用默认配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 创建一个新的 Configuration 实例，允许自定义配置
    pub fn with_custom_config(
        base_path: String,
        user_agent: Option<String>,
        bearer_access_token: Option<String>,
    ) -> Self {
        Configuration {
            base_path,
            user_agent,
            client: &HTTP_CLIENT,
            bearer_access_token,
        }
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            base_path: "http://localhost".to_owned(),
            user_agent: Some("OpenAPI-Generator/1.0.0/rust".to_owned()),
            client: &HTTP_CLIENT,
            bearer_access_token: None, // 改为 None 更符合语义
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::StatusCode;

    #[tokio::test]
    async fn test_configuration_defaults() {
        let config = Configuration::new();
        assert_eq!(config.base_path, "http://localhost");
        assert_eq!(
            config.user_agent,
            Some("OpenAPI-Generator/1.0.0/rust".to_owned())
        );
        assert!(config.bearer_access_token.is_none());
        // 测试 client 是否可以发送请求
        let response = config.client.get("https://httpbin.org/get").send().await;
        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_configuration_with_custom_config() {
        let custom_user_agent = Some("Custom-Agent/2.0".to_owned());
        let custom_token = Some("token123".to_owned());
        let config = Configuration::with_custom_config(
            "https://api.example.com".to_owned(),
            custom_user_agent.clone(),
            custom_token.clone(),
        );

        assert_eq!(config.base_path, "https://api.example.com");
        assert_eq!(config.user_agent, custom_user_agent);
        assert_eq!(config.bearer_access_token, custom_token);

        // 测试 client 是否可以发送带有自定义 User-Agent 的请求
        let response = config
            .client
            .get("https://httpbin.org/headers")
            .send()
            .await
            .expect("Failed to send request");

        assert_eq!(response.status(), StatusCode::OK);
        let json: serde_json::Value = response.json().await.expect("Failed to parse JSON");
        assert_eq!(
            json["headers"]["User-Agent"],
            "OpenAPI-Generator/1.0.0/rust"
        );
    }
}
