use chrono::{prelude::*, Duration};
use dioxus::prelude::*;
use dioxus_logger::tracing::info;

pub fn ToDate() -> Element {
  let today = Utc::now().date_naive();
  let current_date = Utc::now();
  let formatted_date = format!("{} {}, {}", today.format("%B"), today.day(), today.year());
  let days_to_show = Vec::from([
    current_date - Duration::days(4),
    current_date - Duration::days(3),
    current_date - Duration::days(2),
    current_date - Duration::days(1),
    current_date,
    current_date + Duration::days(1),
  ]);
  let weekday = current_date.format("%a").to_string();
  info!("formatted_date: {:?}", current_date.format("%a"));
  rsx! {
    div { class: "flex flex-col gap-4 items-center justify-center w-full",
      p { class: "text-[#1A3636] text-[1.5rem] font-bold", "{formatted_date}" }
      div { class: "flex space-x-[2rem] text-[1.5rem]",
        for date in days_to_show.iter() {
          if date == &current_date {
            div { class: "flex flex-col items-center gap-1 justify-center bg-[#1A3636] rounded-md drop-shadow-md p-1 text-white",
              p { "{date.day()}" }
              p { "{weekday}" }
            }
          } else {
            p { "{date.day()}" }
          }
        }
      }
    }
  }
}
