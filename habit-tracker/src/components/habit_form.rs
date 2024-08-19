use dioxus::prelude::*;

use crate::components::layout::FormDialog;

pub fn HabitForm() -> Element {
  let mut show_dialog = use_context::<Signal<FormDialog>>();
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
      form { class: "flex flex-col gap-3 text-white p-4",
        div { class: "flex flex-col",
          label { "Habit Name" }
          input {
            r#type: "text",
            class: "text-black rounded-md outline-none h-10 p-1",
          }
        }
        div { class: "flex flex-col",
          label { "Description" }
          textarea { class: "text-black rounded-md outline-none p-1" }
        }
        div { class: "flex flex-col",
          label { "Frequency" }
          select { class: "text-black rounded-md outline-none h-10 p-1",
            option { value: "Daily", "Daily" }
            option { value: "Weekly", "Weekly" }
          }
        }
        div { class: "flex flex-col",
          label { "Start Period" }
          input {
            r#type: "date",
            class: "text-black rounded-md outline-none h-10 p-1",
          }
        }
        div { class: "flex flex-col justify-center items-center",
          button { class: "bg-[#1A3636] w-full justify-center items-center text-white drop-shadow-md p-2 rounded-md flex",
            "submit"
          }
        }
      }
    }
  }
}
