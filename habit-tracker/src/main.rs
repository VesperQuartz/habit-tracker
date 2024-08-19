#![allow(non_snake_case)]
pub mod asset;
pub mod components;
pub mod routes;
pub mod services;
pub mod types;
use components::layout::RootLayout;
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use routes::signup::Register;
use routes::{dashboard::Dashboard, login::Login};
use services::is_logged_in;
const STYLE: &str = asset!("./assets/tailwind.css");

#[derive(Routable, PartialEq, Clone, Debug)]
#[rustfmt::skip]
enum Route {
	#[layout(RootLayout)]
		#[route("/")]
		Home {},
	#[end_layout]
  #[route("/login")]
  Login {},
  #[route("/signup")]
  Register {},
  #[route("/:..segments")]
  NotFound { segments: Vec<String> },
}

#[component]
fn Home() -> Element {
  let router = use_navigator();
  let mut logged = use_signal(is_logged_in);
  info!("logged in: {:?}", logged);
  use_effect(move || {
    logged.set(is_logged_in());
  });
  match logged() {
    true => {
      rsx! {
        Dashboard {}
      }
    }
    false => {
      router.push(Route::Login {});
      rsx! {
        p { "Redirecting to login" }
      }
    }
  }
}

#[component]
fn NotFound(segments: Vec<String>) -> Element {
  rsx!(div {})
}

fn main() {
  dioxus_logger::init(Level::INFO).expect("logger failed to init");
  launch(|| {
    rsx! {
      head::Link { rel: "stylesheet", href: STYLE }
      Router::<Route> {}
    }
  });
}
