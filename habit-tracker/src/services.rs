use gloo_storage::{LocalStorage, Storage};

use crate::types::{AuthLoginResponse, AuthRegResponse, AuthRequest, User};
use dioxus_logger::tracing::info;

const BASE_URL: &str = "http://127.0.0.1:3000";

pub async fn signup(payload: AuthRequest) -> anyhow::Result<AuthRegResponse> {
  let url = format!("{}/auth/signup", BASE_URL);
  let response = reqwest::Client::new()
    .post(&url)
    .json(&payload)
    .send()
    .await?;
  let signup = response.json::<AuthRegResponse>().await?;
  Ok(signup)
}

pub async fn login(payload: AuthRequest) -> anyhow::Result<AuthLoginResponse> {
  let url = format!("{}/auth/login", BASE_URL);
  let response = reqwest::Client::new()
    .post(&url)
    .json(&payload)
    .send()
    .await?;
  let login_response = response.json::<AuthLoginResponse>().await?;
  Ok(login_response)
}

pub fn is_logged_in() -> bool {
  let user = LocalStorage::get("user")
    .ok()
    .unwrap_or_else(|| "".to_string());
  info!("user-log: {:?}", user);
  if user == "".to_string() {
    return false;
  }
  true
}

pub fn get_user() -> Option<User> {
  let user: Option<String> = LocalStorage::get("user").ok();
  if let Some(user) = user {
    let user: User = serde_json::from_str(&user).unwrap();
    return Some(user);
  }
  None
}
