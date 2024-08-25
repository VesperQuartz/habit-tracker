use dioxus::prelude::*;

use crate::asset::S1;
use crate::components::accordion_weekly::Accordion;
use crate::components::loader::Loader;
use crate::services::{cookie_parser, get_weekly_habit};
pub fn WeeklyHabit() -> Element {
  let w_habit =
    use_resource(move || async move { get_weekly_habit(cookie_parser().unwrap().id).await });
  use_context_provider(move || w_habit);
  rsx! {
    main {
      div {
        p { class: "text-2xl p-3 font-bold", "Weekly Habit" }
      }
      div { class: "flex justify-between",
        match &*w_habit.read_unchecked() {
            Some(Ok(habits)) => {
                rsx! {
                  div { class: "grow flex-1",
                    for habit in habits.habits.iter() {
                      Accordion { text: habit.name.clone(), id: habit.id.clone() }
                    }
                  }
                }
            }
            Some(Err(err)) => {
                rsx! {
                  div {
                    p { "An error occurred while fetching habits {err}" }
                  }
                }
            }
            None => {
                rsx! {
                  Loader {}
                }
            }
        }
        div {
          img { src: "{S1}" }
        }
      }
    }
  }
}
