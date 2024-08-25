use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Title {
  pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Count {
  pub count: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Message {
  pub message: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Success {
  pub success: String,
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

#[derive(Deserialize, Serialize, Debug, Clone)]
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
  pub user_id: String,
  pub name: String,
  #[serde(rename = "startDate")]
  pub start_date: String,
  pub frequency: String,
  pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HabitTiny {
  pub id: String,
  #[serde(rename = "userId")]
  pub user_id: String,
  pub name: String,
  pub description: Option<String>,
  #[serde(rename = "startDate")]
  pub start_date: DateTime<Utc>,
  pub frequency: String,
  #[serde(rename = "createdAt")]
  pub created_at: DateTime<Utc>,
  #[serde(rename = "updatedAt")]
  pub updated_at: DateTime<Utc>,
  pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HabitSmall {
  pub habits: Vec<HabitTiny>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct HabitTask {
  pub habit: HabitTiny,
  pub id: String,
  #[serde(rename = "habitId")]
  pub habit_id: String,
  pub date: DateTime<Utc>,
  pub status: String,
  #[serde(rename = "createdAt")]
  pub created_at: DateTime<Utc>,
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
  pub habit_id: String,
  pub date: String,
  pub status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogsError {
  pub message: String,
  pub error: String,
  #[serde(rename = "statusCode")]
  pub status_code: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum LogMessage {
  Yes(Success),
  No(LogsError),
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
