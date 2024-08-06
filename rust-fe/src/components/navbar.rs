use dioxus::prelude::*;

use crate::Route;

pub fn NavBar() -> Element {
  rsx! {
    nav { "I am the nav bar" }
    Outlet::<Route> {}
  }
}
