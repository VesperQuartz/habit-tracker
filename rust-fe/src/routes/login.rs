#![allow(non_snake_case)]
use crate::components::loader::Loader;
use crate::services::login;
use crate::types::{AuthLoginResponse, AuthRequest};
use crate::Route;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use gloo::storage::LocalStorage;
use gloo_storage::Storage;

pub fn Login() -> Element {
  let router = use_navigator();
  let mut username = use_signal(|| "".to_string());
  let mut errors = use_signal(|| "".to_string());
  let mut password = use_signal(|| "".to_string());
  let mut loading = use_signal(|| false);
  rsx! {
    div { class: "flex justify-center items-center h-screen min-w-screen h-screen",
      div { class: "w-[300px] rounded-xl drop-shadow-lg p-4 bg-white p-4",
        div { class: "flex justify-center items-center gap-2 flex-col",
          p { class: "text-2xl font-bold", "Login" }
          p { class: "text-sm font-bold", "Welcome Back!" }
        }
        div { class: "p-2",
          form {
            onsubmit: move |_event| {
                spawn(async move {
                    loading.set(true);
                    let payload = AuthRequest {
                        username: username(),
                        password: password(),
                    };
                    let response = login(payload).await;
                    match response {
                        Ok(data) => {
                          info!("Data: {data:?}");
                          match data {
                            AuthLoginResponse::Ok(data) => {
                              let user = serde_json::to_string(&data.user);
                              if let Ok(user) = user {
                                info!(user);
                                LocalStorage::set("user", user).ok();
                                loading.set(false);
                                router.push(Route::Home {});
                              }
                            },
                            AuthLoginResponse::Err(data) => {
                              let error = serde_json::to_string(&data.message);
                              if let Ok(error) = error {
                                info!(error);
                                errors.set(error);
                                loading.set(false);
                              }
                            },
                          }
                        }
                        Err(err) => {
                          info!("Error: {err:?}");
                            loading.set(false);
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
                  "Login"
                }
              }
            }
          }
        }
        div {
          if errors() != "".to_string() {
            p { class: "text-red-500  flex justify-center items-center my-1", "{errors}" }
          }
          p { class: "text-center text-sm",
            "Don't have an account?"
            Link { class: "text-blue-400 mx-1", to: Route::Register {}, "Register" }
          }
        }
      }
    }
  }
}
