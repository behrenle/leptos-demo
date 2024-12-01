use std::collections::HashMap;

use argon2::{
    password_hash::{
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Argon2,
};
use leptos::prelude::window;
use rand_core::OsRng;
use serde::{Deserialize, Serialize};

const STORAGE_KEY: &str = "users";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDatabase {
    password_hashes: HashMap<String, String>,
}

impl UserDatabase {
    pub fn new() -> Self {
        let storage = window()
            .local_storage()
            .ok()
            .flatten()
            .expect("localstorage");

        if let Some(data) = storage.get(STORAGE_KEY).ok().flatten() {
            let user_database: Option<UserDatabase> =
                serde_json::from_str(&data).ok();
            if let Some(user_database) = user_database {
                return user_database;
            } else {
                storage
                    .remove_item(STORAGE_KEY)
                    .expect("clear user database");
            }
        }

        Self {
            password_hashes: HashMap::new(),
        }
    }

    pub fn insert(
        &mut self,
        username: String,
        password: String,
    ) -> Result<(), &'static str> {
        if self.password_hashes.contains_key(&username) {
            Err("User already exists")
        } else {
            let salt = SaltString::generate(&mut OsRng);
            let argon2 = Argon2::default();
            let password_hash = argon2
                .hash_password(password.as_bytes(), &salt)
                .map_err(|_| "Failed to hash password")?
                .to_string();

            self.password_hashes.insert(username, password_hash);

            let storage = window()
                .local_storage()
                .ok()
                .flatten()
                .expect("localstorage");

            let data = serde_json::to_string(&self)
                .map_err(|_| "Failed to serialize user database")?;

            storage
                .set(STORAGE_KEY, &data)
                .map_err(|_| "Failed to save user database")?;

            Ok(())
        }
    }

    pub fn login(
        &self,
        username: &str,
        password: &str,
    ) -> Result<(), &'static str> {
        let password_hash =
            self.password_hashes.get(username).ok_or("User not found")?;
        if Argon2::default()
            .verify_password(
                password.as_bytes(),
                &PasswordHash::new(password_hash)
                    .map_err(|_| "Failed to hash password")?,
            )
            .is_ok()
        {
            Ok(())
        } else {
            Err("Invalid password")
        }
    }
}
