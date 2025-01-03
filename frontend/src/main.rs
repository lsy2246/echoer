use dioxus::{logger::tracing, prelude::*};
use wasm_bindgen::prelude::*;
mod common;
use common::dom::{add_element_class, get_media_theme, remove_element_class};
use components::notification::{Notification, NotificationProvider};
use components::theme_toggle::{get_theme, ThemeProvider};
use components::Navbar;
use views::Home;

mod components;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[layout(Notification)]
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
    let theme = get_theme();
    use_context_provider(|| Signal::new(ThemeProvider(theme.clone())));
    use_context_provider(|| Signal::new(NotificationProvider::default()));
    let mut is_dark_context = use_context::<Signal<ThemeProvider>>();

    use_effect(move || {
        let _ = add_element_class("html", &theme);
        let window = web_sys::window().unwrap();
        let media_query = window
            .match_media("(prefers-color-scheme: dark)")
            .unwrap()
            .unwrap();
        let closure = Closure::wrap(Box::new(move |_: web_sys::MediaQueryListEvent| {
            let theme = get_media_theme().unwrap_or_else(|_| "light".to_string());
            let _ = remove_element_class("html", &is_dark_context().0);
            let _ = add_element_class("html", &theme);
            is_dark_context.set(ThemeProvider(theme));
        }) as Box<dyn FnMut(_)>);
        media_query
            .add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
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
