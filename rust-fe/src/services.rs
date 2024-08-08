use crate::types::{AuthLoginResponse, AuthRegResponse, AuthRequest};

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
