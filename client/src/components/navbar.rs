use super::Toggle;
use crate::DeviceProvider;
use crate::Route;
use dioxus::{logger::tracing, prelude::*};
use global_attributes::dangerous_inner_html;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::Event;

#[component]
pub fn Navbar() -> Element {
    let mut progress_signal = use_signal(|| 0);
    let mut progress_hover = use_signal(|| false);
    let mut is_more_single = use_signal(|| false);
    let mut device_context = use_context::<Signal<DeviceProvider>>();

    use_effect(move || {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let document_element = document.document_element().unwrap();

        let closure = Closure::wrap(Box::new(move |_: Event| {
            let screen_height = document_element.scroll_height() as f64;
            let inner_height = window.inner_height().unwrap().as_f64().unwrap();
            let scroll_height = window.scroll_y().unwrap();
            let max_scroll_height = screen_height - inner_height;

            let percent = if max_scroll_height > 0.0 {
                scroll_height / max_scroll_height
            } else {
                0.0
            };
            let percent = (percent.clamp(0.0, 1.0) * 100.0) as i32;
            progress_signal.set(percent);
        }) as Box<dyn FnMut(Event)>);

        document
            .add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    });

    let tp = r#"<div>hello,world</div>"#;

    rsx! {
        nav {
            class: "border-red-500 bg-slate-400 text-xl fixed top-0 left-0 right-0",
            div {
                class: "grid grid-cols-3 items-center text-center w-[80%] mx-auto h-12",
                div {
                    class:"justify-self-start",
                    Link {
                        to: Route::Home {},
                        "echoer"
                    }
                }
                div {
                    class:"flex gap-5 justify-center",
                    Link {
                        to: Route::Home {},
                        "首页"
                    }
                    Link {
                        to: Route::Home {},
                        "首页"
                    }

                }
                div{
                    dangerous_inner_html:"{tp}"
                }

                div {
                    class:"justify-self-end flex items-center gap-1",
                    Toggle {
                        height: 30,
                        width: 30
                    }
                    div {
                        class: "progress bg-accent h-7 rounded-full text-xs flex items-center justify-center transition-all duration-300 overflow-hidden",
                        style: if progress_signal() > 0 {
                            format!("opacity: 1; transform: translateX(0); width: {}px;", if progress_signal()==100 { "70" } else { "28" })
                        } else {
                            "opacity: 0; transform: translateX(100%); width: 0px".to_string()
                        },
                        onmouseenter: move |_| {
                            progress_hover.set(true);
                        },
                        onmouseleave: move |_| {
                            progress_hover.set(false);
                        },
                        onclick: move |_| {
                            progress_hover.set(false);
                            let window = web_sys::window().unwrap();
                            window.scroll_to_with_x_and_y(0.0, 0.0);
                        },
                        div {
                            class: "px-2 whitespace-nowrap transition-opacity duration-150",
                            style: "opacity: 1",
                            {
                                if progress_hover() {
                                    "↑".to_string()
                                }
                                else{
                                    if progress_signal()==100 {
                                        "返回顶部".to_string()
                                    }
                                    else {
                                        progress_signal().to_string()
                                    }
                                }
                            }
                        }
                     }
                }
            }
        }
        div {
            class:"w-[80%] mx-auto mt-14",
            Outlet::<Route> {}
        }
    }
}
