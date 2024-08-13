use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct AuthRequest {
  pub username: String,
  pub password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RegResponse {
  id: String,
  username: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AuthError {
  pub message: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum AuthRegResponse {
  Ok(RegResponse),
  Err(AuthError),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginResponse {
  message: String,
  pub user: User,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
  pub token: String,
  pub username: String,
  pub id: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum AuthLoginResponse {
  Ok(LoginResponse),
  Err(AuthError),
}
