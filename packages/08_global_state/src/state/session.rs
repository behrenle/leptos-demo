#[derive(Debug, Clone)]
pub struct Session {
    pub username: String,
}

impl Session {
    pub fn new(username: impl ToString) -> Self {
        Self {
            username: username.to_string(),
        }
    }
}
