use argon2::{self, Config};
use std::{path::PathBuf, fs};
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use serde::{Serialize, Deserialize};
use serde_json;
// use rpassword;
use chrono::prelude::*;

use crate::shell::OsResult;

#[derive(Debug, PartialEq)]
pub enum UserType {
	None,
	Admin(User),
	User(User),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct User {
	username: String,
	password: String,
	created_at: String,
	// Last in the vector will be the latest
	username_modified: Vec<String>,
	password_modified: Vec<String>,
}

impl User {
	pub fn new(username: String, password: String) -> User {
		let now = get_time_now();
		let password = password.as_bytes();

		let mut nonce = vec![0; 32];
		let mut rng = ChaCha20Rng::from_entropy();
		rng.fill_bytes(&mut nonce);

		let password = argon2::hash_encoded(&password, &nonce, &Config::default()).unwrap();

		User {
			username,
			password,
			created_at: now.clone(),
			username_modified: vec![now.clone()],
			password_modified: vec![now.clone()],
		}
	}
}

#[derive(Serialize, Deserialize, Debug)]
struct UserItem {
	username: String,
	user: User,
}

impl UserItem {
	fn new(user: User) -> UserItem {
		UserItem {
			username: user.username.clone(),
			user: user,
		}
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SandFile<'a> {
	users: Vec<UserItem>,
	file: &'a str,
}

impl<'a> SandFile<'a> {
	pub fn new(file: &'a str) -> SandFile<'a> {
		SandFile {
			users: Vec::new(),
			file,
		}
	}

	pub fn load(file: &'a str) -> SandFile<'a> {
		let sand_file = PathBuf::from(file);
		let mut users = Vec::new();

		if sand_file.is_file() {
			// Load it, users = 
			println!("Loading user database...");
			let sand_contents = fs::read_to_string(file).unwrap();
			users = serde_json::from_str(&sand_contents).unwrap();
		}

		SandFile {
			users,
			file,
		}
	}

	pub fn add_user(&mut self, user: User) {
		match self.users.iter().find(|u| u.username == user.username) {
			Some(u) => {
				println!("error: User '{}' already exists", u.username);
			},
			None => {
				self.users.push(UserItem::new(user));
				self.save();
			}
		}
	}

	fn save(&self) {
		fs::write(self.file, serde_json::to_string(&self.users).unwrap()).expect("Writing to SAND file failed");
	}

	pub fn authenticate(&self, username: String, password: String) -> OsResult {
		let username = username;
		if let Some(i) = self.users.iter().find(|x| x.username == username) {
			match argon2::verify_encoded(&i.user.password, password.as_bytes()) {
				Ok(_) => {
					OsResult::Success
				},
				Err(error) => OsResult::Error(error.to_string()),
			}
		} else {
			OsResult::Error("Failed to authenticate".to_string())
		}
	}
}

fn get_time_now() -> String { Local::now().to_string() }