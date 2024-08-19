#![allow(non_snake_case)]
use crate::asset::{L1, L2, L3, L4};
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
  let mut password = use_signal(|| "".to_string());
  let mut loading = use_signal(|| false);
  let mut errors = use_signal(|| "".to_string().replace("\"", ""));
  rsx! {
    Title { "login" }
    main { class: "min-h-screen flex justify-center items-center",
      div { class: "flex justify-evenly gap-28 items-center",
        div {
          div { class: "md:flex md:gap-1 hidden",
            div { class: "flex flex-col gap-4 items-center",
              img {
                class: "w-[27.6rem] h-[25rem]",
                src: "{L1}",
                alt: "l1",
              }
              img { class: "w-[12.5rem] h-[12rem]", src: "{L2}" }
            }
            div { class: "flex flex-col gap-[7rem] z-10",
              img {
                class: "w-[12.9rem] h-[13rem] z-10",
                src: "{L3}",
              }
              img {
                class: "w-[12.9rem] h-[13rem] z-10",
                src: "{L4}",
              }
            }
          }
        }
        div {
          div { class: "flex flex-col gap-8",
            div { class: "flex flex-col gap-1",
              p { class: "text-[#1A3636] font-bold text-[3rem]", "Hi, Welcome Back" }
              p { class: "text-[#333333] text-[2rem] font-light", "Enter Your details" }
            }
            div {
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
                                match data {
                                    AuthLoginResponse::Ok(data) => {
                                        let user = serde_json::to_string(&data.user);
                                        if let Ok(user) = user {
                                            info!(user);
                                            LocalStorage::set("user", user).ok();
                                            loading.set(false);
                                            router.push(Route::Home {});
                                        }
                                    }
                                    AuthLoginResponse::Err(data) => {
                                        let error = serde_json::to_string(&data.message);
                                        if let Ok(error) = error {
                                            info!(error);
                                            errors.set(error);
                                            loading.set(false);
                                        }
                                    }
                                }
                            }
                            Err(err) => {
                                info!("Error: {err:?}");
                                loading.set(false);
                            }
                        }
                    });
                },
                class: "flex flex-col gap-4",
                div { class: "flex flex-col",
                  label { class: "text-[#333333]", "Username" }
                  input {
                    name: "username",
                    id: "username",
                    placeholder: "John Doe",
                    class: "border-2 p-2 border-[#40534C] h-[3.3rem] w-[23rem] rounded-md",
                    r#type: "text",
                    oninput: move |event| { username.set(event.value()) },
                  }
                }
                div { class: "flex flex-col",
                  label { "Password" }
                  input {
                    name: "password",
                    id: "password",
                    placeholder: "********",
                    class: "border-2 p-2 border-[#40534C] h-[3.3rem] w-[23rem] rounded-md",
                    oninput: move |event| { password.set(event.value()) },
                    r#type: "password",
                  }
                  span { class: "text-[#40534C] text-[0.8rem]", "Forgot Password?" }
                }
                div { class: "flex flex-col justify-center mt-[2rem]",
                  button { class: "bg-[#1A3636] w-[23rem] flex items-center justify-center gap-2 font-bold rounded-md p-2 shadow drop-shadow-xl text-white",
                    if loading() {
                      Loader {}
                    } else {
                      "Login"
                      div { class: "icon-arrow-right" }
                    }
                  }
                  p {
                    "Dont have an account? "
                    Link {
                      class: "text-[#1A3636] text-xl mx-1",
                      to: Route::Register {},
                      "Sign up"
                    }
                  }
                  if !errors().is_empty() && errors().len() < 50 {
                    p { class: "text-red-500  flex justify-center items-center my-1",
                      img { src: "data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgaGVpZ2h0PSIxNiIgdmlld0JveD0iMCAwIDI0IDI0IiBmaWxsPSJub25lIiBzdHJva2U9IiNlMDFiMjQiIHN0cm9rZS13aWR0aD0iMiIgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIiBzdHJva2UtbGluZWpvaW49InJvdW5kIiBjbGFzcz0ibHVjaWRlIGx1Y2lkZS1jaXJjbGUtb2ZmIj48cGF0aCBkPSJtMiAyIDIwIDIwIi8+PHBhdGggZD0iTTguMzUgMi42OUExMCAxMCAwIDAgMSAyMS4zIDE1LjY1Ii8+PHBhdGggZD0iTTE5LjA4IDE5LjA4QTEwIDEwIDAgMSAxIDQuOTIgNC45MiIvPjwvc3ZnPg==" }
                      " {errors}"
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}
