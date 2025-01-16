use dioxus::{logger::tracing, prelude::*};
use wasm_bindgen::{prelude::Closure, JsCast};
mod components;
mod utils;
mod views;
use components::notification::{Notification, NotificationProvider};
use components::theme_toggle::{get_theme, ThemeProvider};
use components::Navbar;
use utils::dom::{get_media_theme, set_element_dataset};
use views::Home;
use web_sys::Event;

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

#[derive(Debug, Clone, PartialEq)]
enum DeviceType {
    DESKTOP,
    MOBILE,
}

impl std::fmt::Display for DeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            DeviceType::DESKTOP => "desktop",
            DeviceType::MOBILE => "mobile",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, PartialEq)]
struct DeviceProvider(pub DeviceType);

impl Default for DeviceProvider {
    fn default() -> Self {
        DeviceProvider(DeviceType::DESKTOP)
    }
}

impl From<f64> for DeviceProvider {
    fn from(value: f64) -> Self {
        let derive_type = if value > 1024.0 {
            DeviceType::DESKTOP
        } else {
            DeviceType::MOBILE
        };
        DeviceProvider(derive_type)
    }
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let theme = get_theme();
    use_context_provider(|| Signal::new(ThemeProvider(theme.clone())));
    use_context_provider(|| Signal::new(NotificationProvider::default()));
    use_context_provider(|| Signal::new(DeviceProvider::default()));
    let mut is_dark_context = use_context::<Signal<ThemeProvider>>();
    let mut dervice_context = use_context::<Signal<DeviceProvider>>();

    use_effect(move || {
        let window = web_sys::window().unwrap();
        let width = window.inner_width().unwrap().as_f64().unwrap();
        dervice_context.set(width.into());
        let closure = Closure::wrap(Box::new(move |_: Event| {
            let width = window.inner_width().unwrap().as_f64().unwrap();
            dervice_context.set(width.into());
        }) as Box<dyn FnMut(Event)>);
        let window = web_sys::window().unwrap();
        window
            .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    });

    use_effect(move || {
        let _ = set_element_dataset("html", "theme", &theme);
        let window = web_sys::window().unwrap();
        let media_query = window
            .match_media("(prefers-color-scheme: dark)")
            .unwrap()
            .unwrap();
        let closure = Closure::wrap(Box::new(move |_: web_sys::MediaQueryListEvent| {
            let theme = get_media_theme().unwrap_or_else(|_| "light".to_string());
            let _ = set_element_dataset("html", "theme", &theme);
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
