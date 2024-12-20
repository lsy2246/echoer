use dioxus::{logger::tracing, prelude::*};
use web_sys::{window, MediaQueryList, Storage};
mod common;
use common::error::{CustomErrorInto, CustomResult};

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
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[derive(Debug, Clone, PartialEq)]
pub struct IsDark(bool);

fn get_storage() -> CustomResult<Storage> {
    window()
        .ok_or("浏览器window对象不存在".into_custom_error())?
        .local_storage()
        .map_err(|_| "无法访问localStorage API".into_custom_error())?
        .ok_or("浏览器不支持localStorage".into_custom_error())
}

pub fn get_local_storage_value(key: &str) -> CustomResult<String> {
    get_storage()?
        .get_item(key)
        .map_err(|_| "读取localStorage时发生错误".into_custom_error())?
        .ok_or(format!("localStorage中不存在键'{}'", key).into_custom_error())
}

pub fn set_local_storage_value(key: &str, value: &str) -> CustomResult<()> {
    get_storage()?
        .set_item(key, value)
        .map_err(|_| format!("无法将值写入localStorage的'{}'键", key).into_custom_error())
}

pub fn get_media_theme() -> CustomResult<bool> {
    let media_query = window()
        .ok_or("浏览器window对象不存在".into_custom_error())?
        .match_media("(prefers-color-scheme: dark)")
        .map_err(|_| format!("读取媒体查询时发生错误").into_custom_error())?
        .ok_or("查询media时发生错误".into_custom_error())?
        .matches();
    Ok(media_query)
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(IsDark(false)));
    let mut is_dark_context = use_context::<Signal<IsDark>>();

    #[cfg(target_arch = "wasm32")]
    {
        let _ = use_memo(move || {
            let storage_theme = get_local_storage_value("theme");
            match storage_theme {
                Ok(b) => is_dark_context.set(IsDark(b == "true")),
                Err(_) => {
                    let device_theme = get_media_theme();
                    match device_theme {
                        Ok(b) => is_dark_context.set(IsDark(b)),
                        Err(_) => is_dark_context.set(IsDark(false))
                    }
                }
            }
        });
    }

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet{  href: GLOBAL_CSS }
        document::Stylesheet { href: TAILWIND_CSS }

        Router::<Route> {}
    }
}
