use dioxus::{logger::tracing, prelude::*};
mod common;
use common::dom::{get_local_storage_value, get_media_theme, set_element_class};
use components::theme_toggle::IsDark;

use components::Navbar;
use views::Home;

mod components;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},

}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const GLOBAL_CSS: Asset = asset!("/assets/styling/global.css");
const TAILWIND_CSS: Asset = asset!("/assets/styling/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(IsDark("light".to_string())));
    let mut is_dark_context = use_context::<Signal<IsDark>>();

    use_effect(move || {
        let theme = {
            let storage_theme = get_local_storage_value("theme");
            match storage_theme {
                Ok(s) => s,
                Err(_) => {
                    let device_theme = get_media_theme();
                    match device_theme {
                        Ok(s) => s,
                        Err(_) => "light".to_string(),
                    }
                }
            }
        };
        is_dark_context.set(IsDark(theme.clone()));
        let _ = set_element_class("html", &theme);
    });

    rsx! {
        div {
            document::Link { rel: "icon", href: FAVICON }
            document::Stylesheet{  href: GLOBAL_CSS }
            document::Stylesheet { href: TAILWIND_CSS }
            Router::<Route> {}
        }
    }
}
