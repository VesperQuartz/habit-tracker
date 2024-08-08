#![allow(non_snake_case)]
use crate::components::loader::Loader;
use crate::services::signup;
use crate::types::{AuthRegResponse, AuthRequest};
use crate::Route;
use dioxus::prelude::*;

pub fn Register() -> Element {
  let router = use_navigator();
  let mut username = use_signal(|| "".to_string());
  let mut errors = use_signal(|| "".to_string());
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
                    let payload = AuthRequest {
                        username: username(),
                        password: password(),
                    };
                    let response = signup(payload).await;
                    match response {
                        Ok(data) => {
                          match data {
                            AuthRegResponse::Ok(_) => {
                              loading.set(false);
                              router.push(Route::Login {});
                            }
                            AuthRegResponse::Err(err) => {
                              let error = serde_json::to_string(&err.message);
                              if let Ok(error) = error {
                                 errors.set(error);
                                 loading.set(false);
                              }
                              println!("Login response {:?}", err);
                            }
                          }
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
          if errors() != "".to_string() {
            p { class: "text-red-500  flex justify-center items-center my-1", "{errors}" }
          }
          p { class: "text-center text-sm",
            "Already have an account?"
            Link {class: "text-blue-400 mx-1", to: Route::Login {}, "Login" }
          }
        }
      }
    }
  }
}
