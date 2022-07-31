#[derive(Clone, Debug, Default)]
pub struct Store {
    api_token: String,
}

impl Store {
    pub fn new(api_token: &str) -> Self {
        Self {
            api_token: api_token.to_string(),
        }
    }

    pub fn api_token_check(&self, auth_header: &str) -> bool {
        auth_header == format!("Bearer {}", self.api_token)
    }
}
