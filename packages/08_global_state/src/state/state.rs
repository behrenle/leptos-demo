use super::{action::AppAction, session::Session};

#[derive(Debug, Clone)]
pub struct State {
    pub session: Option<Session>,
}

impl Default for State {
    fn default() -> Self {
        Self { session: None }
    }
}

impl State {
    pub fn apply(&mut self, action: AppAction) -> Result<(), &'static str> {
        match action {
            AppAction::Login { username, password } => {
                if username == "username" && password == "password" {
                    self.session = Some(Session::new(username));
                } else {
                    return Err("Invalid username or password");
                }
            }
            AppAction::Logout => {
                self.session = None;
            }
        }
        Ok(())
    }
}
