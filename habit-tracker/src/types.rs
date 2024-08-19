use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Title {
  pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Message {
  pub message: String,
}

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
  pub user: UserLoginRes,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserLoginRes {
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

#[derive(Deserialize, Serialize, Debug)]
pub enum Frequency {
  Daily(String),
  Weekly(String),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Habit {
  #[serde(rename = "userId")]
  user_id: String,
  name: String,
  #[serde(rename = "startDate")]
  start_date: DateTime<Utc>,
  frequency: Frequency,
  description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HabitTiny {
  id: String,
  #[serde(rename = "userId")]
  user_id: String,
  name: String,
  description: String,
  #[serde(rename = "startDate")]
  start_date: DateTime<Utc>,
  frequency: String,
  #[serde(rename = "createdAt")]
  created_at: DateTime<Utc>,
  #[serde(rename = "updatedAt")]
  updated_at: DateTime<Utc>,
  message: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Log {
  id: String,
  #[serde(rename = "habitId")]
  habit_id: String,
  date: DateTime<Utc>,
  status: String,
  #[serde(rename = "createdAt")]
  created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Logs {
  #[serde(rename = "habitId")]
  habit_id: String,
  date: String,
  status: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Reminder {
  id: String,
  #[serde(rename = "habitId")]
  habit_id: String,
  #[serde(rename = "reminderTime")]
  reminder_time: DateTime<Utc>,
  method: String,
  #[serde(rename = "createdAt")]
  created_at: DateTime<Utc>,
  #[serde(rename = "updatedAt")]
  updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HabitResponse {
  habit: HabitTiny,
  logs: Vec<Log>,
  reminder: Vec<Reminder>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
  id: String,
  username: String,
  #[serde(rename = "createdAt")]
  created_at: DateTime<Utc>,
  #[serde(rename = "updatedAt")]
  updated_at: DateTime<Utc>,
  habits: Vec<HabitResponse>,
}
