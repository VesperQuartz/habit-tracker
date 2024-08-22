#![allow(non_snake_case)]
use crate::asset::{DAILY, WEEKLY};
use crate::components::{habit_card::HabitCard, today::ToDate};
use crate::services::{
  cookie_parser, get_daily_count, get_daily_task, get_weekly_count, get_weekly_task,
};
use crate::Route;
use dioxus::prelude::*;
use dioxus_logger::tracing::{error, info};

#[component]
pub fn Dashboard() -> Element {
  let router = use_navigator();
  let daily_count =
    use_resource(move || async move { get_daily_count(cookie_parser().unwrap().id).await });
  let weekly_count =
    use_resource(move || async move { get_weekly_count(cookie_parser().unwrap().id).await });
  let daily_task =
    use_resource(move || async move { get_daily_task(cookie_parser().unwrap().id).await });
  let weekly_task =
    use_resource(move || async move { get_weekly_task(cookie_parser().unwrap().id).await });
  rsx! {
    Title { "dashboard" }
    main { class: "flex grow flex-1 justify-center",
      div {
        ToDate {}
        div { class: "flex flex-col sm:flex-row justify-center items-center my-10 gap-5",
          HabitCard {
            title: "Daily",
            count: match &*daily_count.read_unchecked() {
                Some(Ok(count)) => count.count,
                _ => 0,
            },
            taskCount: match &*daily_task.read_unchecked() {
                Some(Ok(task)) => task.len(),
                _ => 0,
            },
            icon: rsx! {
              img { src: "{DAILY}" }
            },
            onclick: move |_| {
                router.push(Route::DailyHabit {});
            },
          }
          HabitCard {
            title: "Weekly",
            count: match &*weekly_count.read_unchecked() {
                Some(Ok(count)) => count.count,
                _ => 0,
            },
            taskCount: match &*weekly_task.read_unchecked() {
                Some(Ok(task)) => {
                    info!("task: {task:?}");
                    task.len()
                }
                Some(Err(e)) => {
                    error!("Failed to get task: {e:?}");
                    0
                }
                _ => 0,
            },
            icon: rsx! {
              img { src: "{WEEKLY}" }
            },
            onclick: move |_| {
                router.push(Route::WeeklyHabit {});
            },
          }
        }
      }
    }
  }
}
