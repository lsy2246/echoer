use super::Toggle;
use crate::utils::dom::{add_element_class, get_document, remove_element_class};
use crate::Route;
use dioxus::{logger::tracing, prelude::*};
use wasm_bindgen::{prelude::Closure, JsCast};

#[component]
pub fn Navbar() -> Element {
    let mut progress_signal = use_signal(|| 0);
    let mut progress_hover = use_signal(|| false);

    use_effect(move || {
        let window = web_sys::window().unwrap();
        let document = get_document().unwrap();
        let document_element = document.document_element().unwrap();
        let closure: Closure<dyn FnMut(_)> = Closure::wrap(Box::new(move |_: web_sys::Event| {
            if progress_hover() {
                return;
            }
            let screen_height = document_element.scroll_height() as f64;
            let inner_height = window.inner_height().unwrap().as_f64().unwrap();
            let scrool_height = window.scroll_y().unwrap();
            let max_scroll_height = screen_height - inner_height;

            let percent = if max_scroll_height > 0.0 {
                scrool_height / max_scroll_height
            } else {
                0.0
            };
            let percent = (percent.clamp(0.0, 1.0) * 100.0) as i32;
            if percent > 0 {
                let _ = remove_element_class("nav .progress", "hidden");
            } else {
                let _ = add_element_class("nav .progress", "hidden");
            }
            progress_signal.set(percent);
        }) as Box<dyn FnMut(_)>);
        document
            .add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    });
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
                    class:"flex gap-10 justify-center",
                    Link {
                        to: Route::Home {},
                        "首页"
                    }
                    Link {
                        to: Route::Home {},
                        "首页"
                    }
                    Link {
                        to: Route::Home {},
                        "首页"
                    }
                }

                div {
                    class:"justify-self-end flex items-center gap-1",
                    Toggle {
                        height: 30,
                        width: 30
                    }
                    div {
                        class:"progress bg-accent h-7 p-2 rounded-full text-xs flex items-center justify-center min-w-7 transition-all duration-300 hidden",
                        onmouseenter:move|_|{
                            progress_hover.set(true);
                        },
                        onmouseleave:move|_|{
                            progress_hover.set(false);
                        },
                        onclick:move|_|{
                            progress_hover.set(false);
                            let window = web_sys::window().unwrap();
                            window.scroll_to_with_x_and_y(0.0, 0.0);

                        },
                        {
                            if progress_hover(){
                                "↑".to_string()
                            }
                            else{
                                if progress_signal()==100{
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
        div {
            class:"w-[80%] mx-auto mt-14",
            Outlet::<Route> {}
        }
    }
}
