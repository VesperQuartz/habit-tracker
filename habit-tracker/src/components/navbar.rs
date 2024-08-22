use dioxus::prelude::*;

use crate::{
  components::{
    habit_form::HabitForm,
    layout::{FormDialog, ShowSide},
  },
  services::cookie_parser,
  Route,
};

pub fn NavBar() -> Element {
  let user = cookie_parser();
  let mut show_dialog = use_context::<Signal<FormDialog>>();
  let mut show_side = use_context::<Signal<ShowSide>>();
  let router = use_navigator();

  rsx! {
    div {
      div {
        if !show_side().0 {
          button {
            class: "p-1 m-2",
            onclick: move |_| {
                show_side.write().0 = true;
            },
            img { src: "data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyNCIgaGVpZ2h0PSIyNCIgdmlld0JveD0iMCAwIDI0IDI0IiBmaWxsPSJub25lIiBzdHJva2U9IiMxQTM2MzYiIHN0cm9rZS13aWR0aD0iMiIgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIiBzdHJva2UtbGluZWpvaW49InJvdW5kIiBjbGFzcz0ibHVjaWRlIGx1Y2lkZS1jaGV2cm9uLXJpZ2h0Ij48cGF0aCBkPSJtOSAxOCA2LTYtNi02Ii8+PC9zdmc+" }
          }
        } else {
          div {}
        }
      }
      nav { class: "relative flex h-[5.7rem] p-3 items-center justify-between w-full",
        div {
          if let Some(user) = user {
            p {
              onclick: move |_| {
                  router.push(Route::Home {});
              },
              class: "text-[#333333] font-bold text-[2.8rem] cursor-pointer",
              "Hey, {user.username}!"
            }
          } else {
            p { class: "text-[#333333] font-bold text-[2.8rem]", "Welcome, Guest" }
          }
        }
        div {
          div { class: "flex gap-3 mr-8",
            button {
              onclick: move |_| {
                  show_dialog.write().0 = !show_dialog().0;
              },
              img { src: "data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyMCIgaGVpZ2h0PSIyMCIgdmlld0JveD0iMCAwIDI0IDI0IiBmaWxsPSJub25lIiBzdHJva2U9IiMxQTM2MzYiIHN0cm9rZS13aWR0aD0iMiIgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIiBzdHJva2UtbGluZWpvaW49InJvdW5kIiBjbGFzcz0ibHVjaWRlIGx1Y2lkZS1wbHVzIj48cGF0aCBkPSJNNSAxMmgxNCIvPjxwYXRoIGQ9Ik0xMiA1djE0Ii8+PC9zdmc+" }
            }
            if show_dialog().0 {
              div { class: "absolute top-16 z-50 right-10", HabitForm {} }
            }
            img { src: "data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyMCIgaGVpZ2h0PSIyMCIgdmlld0JveD0iMCAwIDI0IDI0IiBmaWxsPSJub25lIiBzdHJva2U9IiMxQTM2MzYiIHN0cm9rZS13aWR0aD0iMiIgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIiBzdHJva2UtbGluZWpvaW49InJvdW5kIiBjbGFzcz0ibHVjaWRlIGx1Y2lkZS1iZWxsIj48cGF0aCBkPSJNNiA4YTYgNiAwIDAgMSAxMiAwYzAgNyAzIDkgMyA5SDNzMy0yIDMtOSIvPjxwYXRoIGQ9Ik0xMC4zIDIxYTEuOTQgMS45NCAwIDAgMCAzLjQgMCIvPjwvc3ZnPg==" }
            img { src: "data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyMCIgaGVpZ2h0PSIyMCIgdmlld0JveD0iMCAwIDI0IDI0IiBmaWxsPSJub25lIiBzdHJva2U9IiMxQTM2MzYiIHN0cm9rZS13aWR0aD0iMiIgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIiBzdHJva2UtbGluZWpvaW49InJvdW5kIiBjbGFzcz0ibHVjaWRlIGx1Y2lkZS1oZWFydCI+PHBhdGggZD0iTTE5IDE0YzEuNDktMS40NiAzLTMuMjEgMy01LjVBNS41IDUuNSAwIDAgMCAxNi41IDNjLTEuNzYgMC0zIC41LTQuNSAyLTEuNS0xLjUtMi43NC0yLTQuNS0yQTUuNSA1LjUgMCAwIDAgMiA4LjVjMCAyLjMgMS41IDQuMDUgMyA1LjVsNyA3WiIvPjwvc3ZnPg==" }
          }
        }
      }
    }
  }
}
