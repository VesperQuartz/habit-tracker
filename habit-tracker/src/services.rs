use gloo_storage::{LocalStorage, Storage};

use crate::types::{
  AuthLoginResponse, AuthRegResponse, AuthRequest, Count, Habit, HabitResponse, HabitSmall,
  HabitTask, HabitTiny, LogMessage, Logs, Message, Title, User, UserLoginRes,
};

const BASE_URL: &str = "https://api.habittracker.ignorelist.com";
// const BASE_URL: &str = "http://localhost:4000";

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

pub async fn mark_habit_done(payload: Logs) -> anyhow::Result<LogMessage> {
  let url = format!("{}/habit/log/daily", BASE_URL);
  let response = reqwest::Client::new()
    .post(&url)
    .json(&payload)
    .send()
    .await?;
  let done_response = response.json::<LogMessage>().await?;
  Ok(done_response)
}

pub async fn mark_habit_done_weekly(payload: Logs) -> anyhow::Result<LogMessage> {
  let url = format!("{}/habit/log/weekly", BASE_URL);
  let response = reqwest::Client::new()
    .post(&url)
    .json(&payload)
    .send()
    .await?;
  let done_response = response.json::<LogMessage>().await?;
  Ok(done_response)
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

pub async fn get_daily_count(id: String) -> anyhow::Result<Count> {
  let url = format!("{}/users/habit/daily/{}/count", BASE_URL, id);
  let count = reqwest::Client::new().get(&url).send().await?;
  match count.status() {
    reqwest::StatusCode::OK => {
      let response = count.json::<Count>().await?;
      Ok(response)
    }
    _ => {
      panic!("something went wrong!");
    }
  }
}

pub async fn get_weekly_count(id: String) -> anyhow::Result<Count> {
  let url = format!("{}/users/habit/weekly/{}/count", BASE_URL, id);
  let count = reqwest::Client::new().get(&url).send().await?;
  match count.status() {
    reqwest::StatusCode::OK => {
      let response = count.json::<Count>().await?;
      Ok(response)
    }
    _ => {
      panic!("something went wrong!");
    }
  }
}

pub async fn get_daily_task(id: String) -> anyhow::Result<Vec<HabitTask>> {
  let url = format!("{}/habit/log/{}/daily", BASE_URL, id);
  let task = reqwest::Client::new().get(&url).send().await?;
  match task.status() {
    reqwest::StatusCode::OK => {
      let response = task.json::<Vec<HabitTask>>().await?;
      Ok(response)
    }
    _ => {
      panic!("something went wrong!");
    }
  }
}

pub async fn get_weekly_task(id: String) -> anyhow::Result<Vec<HabitTask>> {
  let task = format!("{}/habit/log/{}/weekly", BASE_URL, id);
  let count = reqwest::Client::new().get(&task).send().await?;
  match count.status() {
    reqwest::StatusCode::OK => {
      let response = count.json::<Vec<HabitTask>>().await?;
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

pub async fn get_daily_habit(id: String) -> anyhow::Result<HabitSmall> {
  let url = format!("{}/users/habit/daily/{}", BASE_URL, id);
  let daily_habit = reqwest::Client::new().get(&url).send().await?;
  match daily_habit.status() {
    reqwest::StatusCode::OK => {
      let response = daily_habit.json::<HabitSmall>().await?;
      Ok(response)
    }
    _ => {
      panic!("something went wrong!");
    }
  }
}

pub async fn get_weekly_habit(id: String) -> anyhow::Result<HabitSmall> {
  let url = format!("{}/users/habit/weekly/{}", BASE_URL, id);
  let daily_habit = reqwest::Client::new().get(&url).send().await?;
  match daily_habit.status() {
    reqwest::StatusCode::OK => {
      let response = daily_habit.json::<HabitSmall>().await?;
      Ok(response)
    }
    _ => {
      panic!("something went wrong!");
    }
  }
}
