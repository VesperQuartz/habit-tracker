use dioxus::prelude::*;
use dioxus_logger::tracing::info;

#[derive(PartialEq, Clone, Props, Debug)]
pub struct CardProps {
  title: String,
  count: i32,
  taskCount: usize,
  icon: Element,
  onclick: Option<EventHandler<MouseEvent>>,
}

pub fn HabitCard(props: CardProps) -> Element {
  rsx! {
    div { class: "flex gap-2 p-3 justify-between items-center sm:w-[25rem] h-[12.5rem] bg-[#F9F9F9] drop-shadow-md rounded-md",
      div { class: "flex flex-col gap-2",
        p { class: "text-[1.5rem]", "{props.title}" }
        p { class: "text-[#333333] font-bold text-[1.25rem]",
          span { class: "text-[#D6BD98]", "{props.taskCount}/{props.count}" }
          " Done"
        }
        p { class: "text-[#333333]", "You have {props.taskCount} habit marked done" }
        button {
          class: "bg-[#1A3636] text-white rounded drop-shadow-md p-1 flex items-center gap-2 w-fit",
          onclick: move |event| props.onclick.unwrap().call(event),
          "All Habits"
          img { src: "data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxNiIgaGVpZ2h0PSIxNiIgdmlld0JveD0iMCAwIDI0IDI0IiBmaWxsPSJub25lIiBzdHJva2U9IiNmZmZmZmYiIHN0cm9rZS13aWR0aD0iMiIgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIiBzdHJva2UtbGluZWpvaW49InJvdW5kIiBjbGFzcz0ibHVjaWRlIGx1Y2lkZS1hcnJvdy1yaWdodCI+PHBhdGggZD0iTTUgMTJoMTQiLz48cGF0aCBkPSJtMTIgNSA3IDctNyA3Ii8+PC9zdmc+" }
        }
      }
      div { {props.icon} }
    }
  }
}
