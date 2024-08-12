#![allow(non_snake_case)]
pub mod assets;
pub mod components;
pub mod routes;
pub mod services;
pub mod types;
use components::navbar::NavBar;
use dioxus::{launch, prelude::*};
use dioxus_logger::tracing::{info, Level};
use routes::login::Login;
use routes::signup::Register;
use services::is_logged_in;

#[derive(Routable, PartialEq, Clone, Debug)]
#[rustfmt::skip]
enum Route {
	#[layout(NavBar)]
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
  let mut logged = use_signal(|| is_logged_in());
  info!("logged in: {:?}", logged);
  use_effect(move || {
    logged.set(is_logged_in());
  });
  match logged() {
    true => {
      rsx! {
        main { "Logged in" }
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
      Router::<Route> {}
    }
  });
}
