use gloo_storage::{LocalStorage, Storage};

use crate::types::{
  AuthLoginResponse, AuthRegResponse, AuthRequest, Habit, HabitResponse, HabitTiny, Logs, Message,
  Title, User, UserLoginRes,
};
use dioxus_logger::tracing::info;

const BASE_URL: &str = "https://api.habittracker.ignorelist.com";

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

pub fn cookie_parser() -> Option<UserLoginRes> {
  let user: Option<String> = LocalStorage::get("user").ok();
  if let Some(user) = user {
    let user: UserLoginRes = serde_json::from_str(&user).unwrap();
    return Some(user);
  }
  None
}

pub async fn add_habit(payload: Habit) -> anyhow::Result<HabitTiny> {
  let url = format!("{}/habit", BASE_URL);
  let habit = reqwest::Client::new()
    .post(&url)
    .json(&payload)
    .send()
    .await?;
  let response = habit.json::<HabitTiny>().await?;
  Ok(response)
}

pub async fn get_habit(id: String) -> anyhow::Result<HabitResponse> {
  let url = format!("{}/habit/{}", BASE_URL, id);
  let habit = reqwest::Client::new().get(&url).send().await?;
  match habit.status() {
    reqwest::StatusCode::OK => {
      let response = habit.json::<HabitResponse>().await?;
      Ok(response)
    }
    _ => {
      panic!("something went wrong!");
    }
  }
}

pub async fn edit_name(id: String, title: Title) -> anyhow::Result<Message> {
  let url = format!("{}/habit/{}", BASE_URL, id);
  let habit = reqwest::Client::new()
    .patch(&url)
    .json(&title)
    .send()
    .await?;
  let response = habit.json::<Message>().await?;
  Ok(response)
}

pub async fn delete_habit(id: String) -> anyhow::Result<Message> {
  let url = format!("{}/habit/{}", BASE_URL, id);
  let habit = reqwest::Client::new().delete(&url).send().await?;
  let response = habit.json::<Message>().await?;
  Ok(response)
}

pub async fn get_user(id: String) -> anyhow::Result<User> {
  let url = format!("{}/users/{}", BASE_URL, id);
  let users = reqwest::Client::new().get(&url).send().await?;
  match users.status() {
    reqwest::StatusCode::OK => {
      let response = users.json::<User>().await?;
      Ok(response)
    }
    _ => {
      panic!("something went wrong!");
    }
  }
}

pub async fn set_log_status(logs: Logs) -> anyhow::Result<Message> {
  let url = format!("{}/habit/log", BASE_URL);
  let logs = reqwest::Client::new().post(&url).json(&logs).send().await?;
  let response = logs.json::<Message>().await?;
  Ok(response)
}
