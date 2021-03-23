pub struct ApiResponse<T> {
    pub data: T,
}

impl<T> ApiResponse<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

pub struct TokenResponse {
    pub token: String,
}

impl TokenResponse {
    pub fn new(token: &str) -> Self {
        Self {
            token: token.to_string(),
        }
    }
}
