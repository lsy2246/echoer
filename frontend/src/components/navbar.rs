use crate::Route;
use dioxus::prelude::*;
const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");
use super::Toggle;


#[component]
pub fn Navbar() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }

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
