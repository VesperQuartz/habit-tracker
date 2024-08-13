#![allow(non_snake_case)]
use crate::assets::{DAILY, WEEKLY};
use crate::components::{habit_card::HabitCard, today::ToDate};
use dioxus::prelude::*;

pub fn Dashboard() -> Element {
  rsx! {
    main {
      div {
        ToDate {}
        div { class: "flex flex-col sm:flex-row justify-center items-center my-10 gap-5",
          HabitCard {
            title: "Daily",
            count: 10,
            description: "You have 2 habit marked done!",
            icon: rsx! {
                img { src: "{DAILY}" }
            }
          }
          HabitCard {
            title: "Weekly",
            count: 7,
            description: "You have 2 habit marked done!",
            icon: rsx! {
                img { src: "{WEEKLY}" }
            }
          }
        }
      }
    }
  }
}
