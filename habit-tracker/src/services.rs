use gloo_storage::{LocalStorage, Storage};

use crate::types::{
  AuthLoginResponse, AuthRegResponse, AuthRequest, Count, Habit, HabitResponse, HabitSmall,
  HabitTask, HabitTiny, LogMessage, Logs, Message, Title, User, UserLoginRes,
};

const BASE_URL: &str = "https://api.habittracker.ignorelist.com";
// const BASE_URL: &str = "http://localhost:4000";

/// The `signup` function in Rust sends a POST request to the signup endpoint with a payload and returns
/// the response as an `AuthRegResponse`.
/// 
/// Arguments:
/// 
/// * `payload`: The `payload` parameter in the `signup` function is of type `AuthRequest`. It contains
/// the data needed for the signup process, such as the user's email, password, and any other required
/// information for registration.
/// 
/// Returns:
/// 
/// The `signup` function returns a `Result` with the success case containing an `AuthRegResponse` if
/// the signup process is successful, or an error wrapped in `anyhow::Error` if there is an issue during
/// the signup process.
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

/// The `login` function in Rust sends a POST request to a login endpoint with authentication payload
/// and returns the login response asynchronously.
/// 
/// Arguments:
/// 
/// * `payload`: The `payload` parameter in the `login` function is of type `AuthRequest`. It contains
/// the data necessary for authentication, such as username and password.
/// 
/// Returns:
/// 
/// The `login` function returns a `Result` with the success case containing an `AuthLoginResponse` and
/// the error case containing an `anyhow::Error`.
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

/// The function `is_logged_in` checks if a user is logged in by retrieving the user data from local
/// storage in Rust.
/// 
/// Returns:
/// 
/// The function `is_logged_in` returns a boolean value indicating whether a user is logged in or not.
/// If the user retrieved from the local storage is an empty string, the function returns `false`,
/// indicating that the user is not logged in. Otherwise, it returns `true`, indicating that the user is
/// logged in.
pub fn is_logged_in() -> bool {
  let user = LocalStorage::get("user")
    .ok()
    .unwrap_or_else(|| "".to_string());
  if user == "".to_string() {
    return false;
  }
  true
}

/// The function `cookie_parser` retrieves user login information from local storage in Rust.
/// 
/// Returns:
/// 
/// The `cookie_parser` function returns an `Option` containing a `UserLoginRes` struct. If the
/// `LocalStorage` contains a user value, it is deserialized into a `UserLoginRes` struct and returned
/// as `Some(user)`. If the user value is not found in the `LocalStorage`, it returns `None`.
pub fn cookie_parser() -> Option<UserLoginRes> {
  let user: Option<String> = LocalStorage::get("user").ok();
  if let Some(user) = user {
    let user: UserLoginRes = serde_json::from_str(&user).unwrap();
    return Some(user);
  }
  None
}

/// The function `add_habit` sends a POST request to a specified URL with a payload, receives a
/// response, and returns a parsed `HabitTiny` object.
/// 
/// Arguments:
/// 
/// * `payload`: The `add_habit` function you provided is an asynchronous function that sends a POST
/// request to a specified URL with a payload of type `Habit`. It then expects a response in the form of
/// a `HabitTiny` object.
/// 
/// Returns:
/// 
/// The `add_habit` function returns a `Result` containing a `HabitTiny` object or an error wrapped in
/// the `anyhow` crate's `Result` type.
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

/// The function `mark_habit_done` sends a POST request to a specified URL with a payload to log a daily
/// habit and returns the response as a `LogMessage`.
/// 
/// Arguments:
/// 
/// * `payload`: The `payload` parameter in the `mark_habit_done` function likely represents the data
/// that needs to be sent to the server when marking a habit as done. It could include information such
/// as the habit ID, the date it was completed, and any additional details related to the habit
/// completion. This
/// 
/// Returns:
/// 
/// The `mark_habit_done` function returns a `Result` containing a `LogMessage` or an error wrapped in
/// the `anyhow` crate's `Result` type.
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

/// This Rust function marks a habit as done weekly by sending a POST request to a specified URL with
/// the provided payload.
/// 
/// Arguments:
/// 
/// * `payload`: The `payload` parameter in the `mark_habit_done_weekly` function likely represents data
/// related to logging a habit as done on a weekly basis. This data could include information such as
/// the habit ID, the user ID, the date of completion, and any additional details related to the habit
/// being
/// 
/// Returns:
/// 
/// The function `mark_habit_done_weekly` returns a `Result` with the type `LogMessage` wrapped in an
/// `anyhow::Result`.
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

/// The function `get_habit` asynchronously fetches a habit from a specified URL and returns a
/// `HabitResponse` if successful.
/// 
/// Arguments:
/// 
/// * `id`: The `id` parameter in the `get_habit` function is a `String` type representing the unique
/// identifier of a habit that you want to retrieve from the server.
/// 
/// Returns:
/// 
/// The function `get_habit` returns a `Result` containing a `HabitResponse` if the request is
/// successful and the status code is OK. If the status code is not OK, it will panic with the message
/// "something went wrong!".
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

/// The function `edit_name` in Rust asynchronously updates the title of a habit using a PATCH request
/// and returns the updated message.
/// 
/// Arguments:
/// 
/// * `id`: The `id` parameter in the `edit_name` function is a `String` type, which represents the
/// unique identifier of a habit that you want to edit.
/// * `title`: The `edit_name` function you provided is an asynchronous function that takes in a
/// `String` `id` and a `Title` struct as parameters. The function sends a PATCH request to a specific
/// URL constructed using the `BASE_URL` and the `id`, with the `title` data as
/// 
/// Returns:
/// 
/// This function is returning a `Result` type with the `Message` struct as the success variant. The
/// `Message` struct represents the response from the API call made to edit the name of a habit.
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

/// The function `delete_habit` asynchronously deletes a habit by sending a DELETE request to a
/// specified URL and returns the response as a `Message`.
/// 
/// Arguments:
/// 
/// * `id`: The `delete_habit` function you provided is an asynchronous function that sends a DELETE
/// request to a specific URL formed using the `BASE_URL` and the `id` parameter. It then expects a JSON
/// response that can be deserialized into a `Message` struct.
/// 
/// Returns:
/// 
/// The `delete_habit` function returns a `Result` containing a `Message`.
pub async fn delete_habit(id: String) -> anyhow::Result<Message> {
  let url = format!("{}/habit/{}", BASE_URL, id);
  let habit = reqwest::Client::new().delete(&url).send().await?;
  let response = habit.json::<Message>().await?;
  Ok(response)
}

/// The function `get_user` in Rust asynchronously fetches user data from a specified URL and returns a
/// `User` object if the request is successful.
/// 
/// Arguments:
/// 
/// * `id`: The `id` parameter in the `get_user` function represents the unique identifier of the user
/// you want to retrieve from the API.
/// 
/// Returns:
/// 
/// The function `get_user` returns a `Result` containing a `User` object if the request is successful
/// and the response status is OK. If the response status is not OK, it will panic with the message
/// "something went wrong!".
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

/// The function `get_daily_count` makes an asynchronous HTTP request to retrieve daily count data for a
/// habit user and returns the count if successful.
/// 
/// Arguments:
/// 
/// * `id`: The `id` parameter in the `get_daily_count` function is a `String` representing the user's
/// ID. This ID is used to construct a URL to fetch the daily count of a habit for a specific user.
/// 
/// Returns:
/// 
/// The function `get_daily_count` returns a `Result` containing a `Count` struct if the request is
/// successful and the response can be deserialized into a `Count` struct. If the request fails or the
/// response status code is not OK, it will panic with the message "something went wrong!".
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

/// The function `get_weekly_count` in Rust makes an asynchronous HTTP request to retrieve weekly count
/// data for a user's habit.
/// 
/// Arguments:
/// 
/// * `id`: The `id` parameter in the `get_weekly_count` function is a `String` type representing the
/// identifier of a user's habit for which you want to retrieve the weekly count.
/// 
/// Returns:
/// 
/// The function `get_weekly_count` returns a `Result` containing a `Count` struct if the request is
/// successful and the response can be deserialized into a `Count` struct. If the request fails or the
/// response status code is not OK, it will panic with the message "something went wrong!".
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

/// The function `get_daily_task` retrieves daily habit tasks for a specific ID using an async Rust
/// implementation.
/// 
/// Arguments:
/// 
/// * `id`: The `id` parameter in the `get_daily_task` function represents the identifier of a habit or
/// task for which you want to retrieve the daily log. This identifier is used to construct the URL for
/// the API request to fetch the daily tasks associated with that specific habit or task.
/// 
/// Returns:
/// 
/// The function `get_daily_task` returns a `Result` containing a vector of `HabitTask` objects wrapped
/// in an `anyhow::Result`.
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

/// This Rust function retrieves a weekly task for a given ID from a specified base URL using reqwest
/// and returns it as a vector of HabitTask objects.
/// 
/// Arguments:
/// 
/// * `id`: The `id` parameter in the `get_weekly_task` function represents the identifier of a habit or
/// task for which you want to retrieve the weekly log. This identifier is used to construct the URL for
/// the API endpoint that provides the weekly log data for the specified habit or task.
/// 
/// Returns:
/// 
/// The function `get_weekly_task` returns a `Result` containing a vector of `HabitTask` objects wrapped
/// in an `anyhow::Result`.
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

/// This Rust function sends a POST request to a specified URL with log data and returns a message
/// response asynchronously.
/// 
/// Arguments:
/// 
/// * `logs`: Logs is a struct or data type that contains information related to logging activities. It
/// could include details such as log messages, timestamps, log levels, or any other relevant
/// information needed for logging purposes.
/// 
/// Returns:
/// 
/// The function `set_log_status` returns a `Result` containing a `Message` or an error wrapped in an
/// `anyhow::Result`.
pub async fn set_log_status(logs: Logs) -> anyhow::Result<Message> {
  let url = format!("{}/habit/log", BASE_URL);
  let logs = reqwest::Client::new().post(&url).json(&logs).send().await?;
  let response = logs.json::<Message>().await?;
  Ok(response)
}

/// This Rust function asynchronously fetches a daily habit for a user based on the provided ID.
/// 
/// Arguments:
/// 
/// * `id`: The `id` parameter in the `get_daily_habit` function represents the unique identifier of the
/// daily habit that you want to retrieve. This identifier is used to construct the URL for the API
/// request to fetch the daily habit information.
/// 
/// Returns:
/// 
/// The function `get_daily_habit` returns a `Result` containing a `HabitSmall` object if the request is
/// successful and the response status is OK. If the response status is not OK, it will panic with the
/// message "something went wrong!".
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

/// This Rust function asynchronously fetches a user's weekly habit data from a specified URL and
/// returns it as a `HabitSmall` struct.
/// 
/// Arguments:
/// 
/// * `id`: The `id` parameter in the `get_weekly_habit` function represents the unique identifier of
/// the user for whom you want to retrieve the weekly habit information.
/// 
/// Returns:
/// 
/// The function `get_weekly_habit` returns a `Result` containing a `HabitSmall` object if the request
/// is successful and the response status code is OK. If there is an error or the status code is not OK,
/// it will panic with the message "something went wrong!".
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
