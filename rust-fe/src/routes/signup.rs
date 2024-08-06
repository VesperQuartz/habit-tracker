#![allow(non_snake_case)]
use crate::components::loader::Loader;
use crate::Route;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct RegRequest {
  username: String,
  password: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct RegResponse {
  id: String,
  username: String,
}

async fn signup(payload: RegRequest) -> anyhow::Result<RegResponse> {
  let base_url = "http://127.0.0.1:3000";
  let url = format!("{}/auth/signup", base_url);
  let response = reqwest::Client::new()
    .post(&url)
    .json(&payload)
    .send()
    .await?;

  let login_response = response.json::<RegResponse>().await?;
  Ok(login_response)
}

pub fn Register() -> Element {
  let router = use_navigator();
  let mut username = use_signal(|| "".to_string());
  let mut password = use_signal(|| "".to_string());
  let mut loading = use_signal(|| false);
  rsx! {
    div { class: "flex justify-center items-center h-screen min-w-screen h-screen",
      div { class: "w-[300px] rounded-xl drop-shadow-lg p-4 bg-white p-4",
        div { class: "flex justify-center items-center gap-2 flex-col",
          p { class: "text-2xl font-bold", "Register" }
          // p { class: "text-sm font-bold", "Welcome Back!" }
        }
        div { class: "p-2",
          form {
            onsubmit: move |_event| {
                spawn(async move {
                    loading.set(true);
                    let payload = RegRequest {
                        username: username(),
                        password: password(),
                    };
                    let response = signup(payload).await;
                    match response {
                        Ok(data) => {
                            loading.set(false);
                            println!("Login response {:?}", data);
                            router.push(Route::Login {});
                        }
                        Err(err) => {
                            loading.set(false);
                            println!("Login response {:?}", err);
                        }
                    }
                });
            },
            class: "flex flex-col gap-4 p-4",
            div { class: "flex flex-col gap-2",
              label { "username" }
              input {
                oninput: move |event| { username.set(event.value()) },
                value: "{username}",
                class: "border-2 border-gray-300 p-2 rounded-md",
                r#type: "text"
              }
            }
            div { class: "flex flex-col gap-2",
              label { "password" }
              input {
                oninput: move |event| { password.set(event.value()) },
                value: "{password}",
                class: "border-2 border-gray-300 p-2 rounded-md",
                r#type: "password"
              }
            }
            div { class: "flex justify-center",
              button { class: "flex justify-center bg-blue-500 text-white p-2 rounded-md w-full",
                if loading() {
                  Loader {}
                } else {
                  "Register"
                }
              }
            }
          }
        }
        div {
          p { class: "text-center text-sm",
            "Already have an account?"
            Link {class: "text-blue-400 mx-1", to: Route::Login {}, "Login" }
          }
        }
      }
    }
  }
}
