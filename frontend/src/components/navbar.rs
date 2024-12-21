use super::Toggle;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    rsx! {

        div {
            id: "navbar",
            Link {
                to: Route::Home {},
                "home"
            }

            Toggle{}

        }

        Outlet::<Route> {}
    }
}
