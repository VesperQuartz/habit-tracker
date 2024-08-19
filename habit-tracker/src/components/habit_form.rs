use dioxus::prelude::*;
use dioxus_logger::tracing::info;

use crate::{
  components::{layout::FormDialog, loader::Loader},
  services::{add_habit, cookie_parser},
  types::Habit,
};

pub fn HabitForm() -> Element {
  let user = cookie_parser().unwrap();
  let mut name = use_signal(|| String::new());
  let mut description = use_signal(|| String::new());
  let mut frequency = use_signal(|| String::new());
  let mut start_period = use_signal(|| String::new());
  let mut show_dialog = use_context::<Signal<FormDialog>>();
  let mut loading = use_signal(|| false);

  rsx! {
    div { class: "bg-[#40534C] md:w-[28.7rem] drop-shadow-xl rounded-xl flex flex-col",
      div { class: "p-2 flex justify-end",
        button {
          onclick: move |_| {
              show_dialog.write().0 = false;
          },
          img { src: "data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyNCIgaGVpZ2h0PSIyNCIgdmlld0JveD0iMCAwIDI0IDI0IiBmaWxsPSJub25lIiBzdHJva2U9IiNmZmZmZmYiIHN0cm9rZS13aWR0aD0iMiIgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIiBzdHJva2UtbGluZWpvaW49InJvdW5kIiBjbGFzcz0ibHVjaWRlIGx1Y2lkZS1jaXJjbGUteCI+PGNpcmNsZSBjeD0iMTIiIGN5PSIxMiIgcj0iMTAiLz48cGF0aCBkPSJtMTUgOS02IDYiLz48cGF0aCBkPSJtOSA5IDYgNiIvPjwvc3ZnPg==" }
        }
      }
      form {
        onsubmit: move |_| {
            eval(r#"showDXToast("Habit", "Habit is Added Sucessfully", "info", 5000);"#);
            let id = user.id.clone();
            spawn(async move {
                loading.set(true);
                let payload = Habit {
                    user_id: id,
                    name: name(),
                    description: description(),
                    frequency: frequency(),
                    start_date: start_period(),
                };
                let response = add_habit(payload).await;
                match response {
                    Ok(data) => {
                        info!("Ok {:?}", data);
                        eval(
                            r#"showDXToast("Habit", "Habit is Added Sucessfully", "info", 5000);"#,
                        );
                        loading.set(false);
                    }
                    Err(err) => {
                        info!("err: {:?}", err);
                        eval(r#"showDXToast("Habit", "Cannot add habit", "error", 5000);"#);
                        loading.set(false);
                    }
                }
            });
        },
        class: "flex flex-col gap-3 text-white p-4",
        div { class: "flex flex-col",
          label { r#for: "habit", "Habit" }
          input {
            id: "habit",
            oninput: move |event| {
                name.set(event.value());
            },
            r#type: "text",
            class: "text-black rounded-md outline-none h-10 p-1",
          }
        }
        div { class: "flex flex-col",
          label { r#for: "description", "Description" }
          textarea {
            id: "description",
            oninput: move |event| {
                description.set(event.value());
            },
            class: "text-black rounded-md outline-none p-1",
          }
        }
        div { class: "flex flex-col",
          label { r#for: "frequency", "Frequency" }

          select {
            id: "frequency",
            class: "text-black rounded-md outline-none h-10 p-1",
            oninput: move |event| {
                frequency.set(event.value());
            },
            option { value: "", "---Select Frequency---" }
            option { value: "Daily", "Daily" }
            option { value: "Weekly", "Weekly" }
          }
        }
        div { class: "flex flex-col",
          label { r#for: "start", "Start Period" }
          input {
            id: "start",
            oninput: move |event| {
                start_period.set(event.value());
            },
            r#type: "date",
            class: "text-black rounded-md outline-none h-10 p-1",
          }
        }
        div { class: "flex flex-col justify-center items-center",
          button { class: "bg-[#1A3636] w-full justify-center items-center text-white drop-shadow-md p-2 rounded-md flex",
            if loading() {
              Loader {}
            } else {
              "Add Habit"
            }
          }
        }
      }
    }
  }
}
