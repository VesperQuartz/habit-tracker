use chrono::Utc;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;

use crate::{
  services::{delete_habit, mark_habit_done_weekly},
  types::{HabitSmall, LogMessage, Logs},
};

#[derive(PartialEq, Clone, Props, Debug)]
pub struct AccordionProps {
  text: String,
  id: String,
}

pub fn Accordion(props: AccordionProps) -> Element {
  let mut show = use_signal(|| false);
  let id = props.id.clone();
  let mut future = use_context::<Resource<anyhow::Result<HabitSmall>>>();
  rsx! {
    div { class: "p-3",
      div { class: "flex justify-between items-center p-2 drop-shadow-md bg-[#F9F9F9] h-[3.37rem]",
        p { class: "text-[#333333] font-bold text-xl", "{props.text}" }
        if show() {
          img {
            onclick: move |_| {
                show.set(false);
            },
            src: "data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyNCIgaGVpZ2h0PSIyNCIgdmlld0JveD0iMCAwIDI0IDI0IiBmaWxsPSJub25lIiBzdHJva2U9IiMxQTM2MzYiIHN0cm9rZS13aWR0aD0iMiIgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIiBzdHJva2UtbGluZWpvaW49InJvdW5kIiBjbGFzcz0ibHVjaWRlIGx1Y2lkZS1taW51cyI+PHBhdGggZD0iTTUgMTJoMTQiLz48L3N2Zz4=",
          }
        } else {
          img {
            onclick: move |_| {
                show.set(true);
            },
            src: "data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyNCIgaGVpZ2h0PSIyNCIgdmlld0JveD0iMCAwIDI0IDI0IiBmaWxsPSJub25lIiBzdHJva2U9IiMxQTM2MzYiIHN0cm9rZS13aWR0aD0iMiIgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIiBzdHJva2UtbGluZWpvaW49InJvdW5kIiBjbGFzcz0ibHVjaWRlIGx1Y2lkZS1wbHVzIj48cGF0aCBkPSJNNSAxMmgxNCIvPjxwYXRoIGQ9Ik0xMiA1djE0Ii8+PC9zdmc+",
          }
        }
      }
      div {
        div { class: if show() { "flex items-center justify-between" } else { "hidden" },
          div { class: "p-2 flex items-center gap-2",
            input {
              r#type: "checkbox",
              class: "mx-2 w-5 h-5",
              onchange: move |e| {
                  e.stop_propagation();
                  if e.value() == "true" {
                      let value = id.clone();
                      spawn(async move {
                          let now = Utc::now();
                          let formatted_date = now.format("%Y-%m-%dT%H:%M:%SZ").to_string();
                          let habit = Logs {
                              habit_id: value.clone(),
                              status: "Done".to_string(),
                              date: formatted_date,
                          };
                          let response = mark_habit_done_weekly(habit).await;
                          match response {
                              Ok(data) => {
                                  match data {
                                      LogMessage::Yes(data) => {
                                          info!("Data: {:?}", data);
                                          eval(
                                              r#"showDXToast("Logs", "Marked as complete", "info", 5000);"#,
                                          );
                                      }
                                      LogMessage::No(_e) => {
                                          eval(
                                              r#"showDXToast("Logs", "Log has already been marked as done!", "error", 5000);"#,
                                          );
                                      }
                                  }
                              }
                              Err(e) => {
                                  info!("Error: {:?}", e);
                                  eval(
                                      r#"showDXToast("Logs", "Something went wrong", "error", 5000);"#,
                                  );
                              }
                          }
                      });
                  }
              },
            }
            label { class: "text-xl", "Mark as complete for the week" }
          }
          div {
            button {
              onclick: move |_| {
                  let value = props.id.clone();
                  spawn(async move {
                      let response = delete_habit(value.clone()).await;
                      match response {
                          Ok(_data) => {
                              eval(
                                  r#"showDXToast("Logs", "Habit deleted successfully", "info", 5000);"#,
                              );
                              future.restart();
                          }
                          Err(e) => {
                              info!("Error: {:?}", e);
                              eval(
                                  r#"showDXToast("Logs", "Something went wrong", "error", 5000);"#,
                              );
                          }
                      }
                  });
              },
              img {
                class: "cursor-pointer",
                title: "delete",
                src: "data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyNCIgaGVpZ2h0PSIyNCIgdmlld0JveD0iMCAwIDI0IDI0IiBmaWxsPSJub25lIiBzdHJva2U9IiNjMDFjMjgiIHN0cm9rZS13aWR0aD0iMiIgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIiBzdHJva2UtbGluZWpvaW49InJvdW5kIiBjbGFzcz0ibHVjaWRlIGx1Y2lkZS10cmFzaC0yIj48cGF0aCBkPSJNMyA2aDE4Ii8+PHBhdGggZD0iTTE5IDZ2MTRjMCAxLTEgMi0yIDJIN2MtMSAwLTItMS0yLTJWNiIvPjxwYXRoIGQ9Ik04IDZWNGMwLTEgMS0yIDItMmg0YzEgMCAyIDEgMiAydjIiLz48bGluZSB4MT0iMTAiIHgyPSIxMCIgeTE9IjExIiB5Mj0iMTciLz48bGluZSB4MT0iMTQiIHgyPSIxNCIgeTE9IjExIiB5Mj0iMTciLz48L3N2Zz4=",
              }
            }
          }
        }
      }
    }
  }
}
