#[derive(Debug, Clone)]
pub struct Configuration {
    pub base_path: String,
    pub user_agent: Option<String>,
    pub client: &'static Lazy<reqwest::Client>,
    pub bearer_access_token: Option<String>,
}

impl Configuration {
    pub fn new() -> Configuration {
        Configuration::default()
    }
}

// 静态的 Lazy 初始化，确保 Client 只被创建一次
pub static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| Client::new());

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            base_path: "http://localhost".to_owned(),
            user_agent: Some("OpenAPI-Generator/1.0.0/rust".to_owned()),
            client: &HTTP_CLIENT,
            bearer_access_token: Some("".to_string()),
        }
    }
}
