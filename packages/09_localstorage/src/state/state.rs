use super::{action::AppAction, session::Session, user_database::UserDatabase};

#[derive(Debug, Clone)]
pub struct State {
    pub session: Option<Session>,
    users: UserDatabase,
}

impl Default for State {
    fn default() -> Self {
        Self {
            session: None,
            users: UserDatabase::new(),
        }
    }
}

impl State {
    pub fn apply(&mut self, action: AppAction) -> Result<(), &'static str> {
        match action {
            AppAction::Register { username, password } => {
                self.users.insert(username, password)
            }
            AppAction::Login { username, password } => {
                self.users.login(&username, &password)?;
                self.session = Some(Session::new(username));
                Ok(())
            }
            AppAction::Logout => {
                self.session = None;
                Ok(())
            }
        }
    }
}
