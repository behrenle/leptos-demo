pub enum AppAction {
    Register { username: String, password: String },
    Login { username: String, password: String },
    Logout,
}
