use std::format;

use crate::utils::dom::add_element_class;
use common::helps::generate_random_string;
use crate::Route;
use dioxus::{logger::tracing, prelude::*};
use dioxus_free_icons::icons::bs_icons::{BsCheckCircle, BsInfoCircle, BsXCircle, BsXLg};
use dioxus_free_icons::Icon;
use web_sys::window;

#[derive(PartialEq, Clone)]
pub struct NotificationProvider {
    notifications: Vec<NotificationProps>,
}

impl Default for NotificationProvider {
    fn default() -> Self {
        Self {
            notifications: Vec::new(),
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum NotificationType {
    Info,
    Error,
    Success,
}

#[derive(PartialEq, Props, Clone)]
pub struct NotificationProps {
    id: String,
    title: String,
    content: String,
    time: u64,
    r#type: NotificationType,
}

impl Default for NotificationProps {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            title: "".to_string(),
            content: "".to_string(),
            time: 5,
            r#type: NotificationType::Info,
        }
    }
}

#[derive(PartialEq, Clone)]
struct NoticationColorMatching {
    icon: Element,
    bg_color: &'static str,
    border_color: &'static str,
    text_color: &'static str,
    progress_color: &'static str,
    hover_border_color: &'static str,
    hover_text_color: &'static str,
}

fn get_color_matching(notification_type: &NotificationType) -> NoticationColorMatching {
    match notification_type {
        NotificationType::Info => NoticationColorMatching {
            icon: rsx! {
                Icon {
                    icon: BsInfoCircle,
                    width:18,
                    height:18,
                }
            },
            bg_color: "bg-blue-50",
            border_color: "border-blue-200",
            text_color: "text-blue-600",
            progress_color: "bg-blue-400",
            hover_border_color: "hover:border-blue-400",
            hover_text_color: "hover:text-blue-400",
        },
        NotificationType::Error => NoticationColorMatching {
            icon: rsx! {
                Icon {
                    icon: BsXCircle,
                    width:18,
                    height:18,
                }
            },
            bg_color: "bg-red-50",
            border_color: "border-red-200",
            text_color: "text-red-600",
            progress_color: "bg-red-400",
            hover_border_color: "hover:border-red-400",
            hover_text_color: "hover:text-red-400",
        },
        NotificationType::Success => NoticationColorMatching {
            icon: rsx! {
                Icon {
                    icon: BsCheckCircle,
                    width:18,
                    height:18,
                }
            },
            bg_color: "bg-green-50",
            border_color: "border-green-200",
            text_color: "text-green-600",
            progress_color: "bg-green-400",
            hover_border_color: "hover:border-green-400",
            hover_text_color: "hover:text-green-400",
        },
    }
}

pub fn remove_notification(id: String) {
    let mut notifications_context = use_context::<Signal<NotificationProvider>>();
    notifications_context.with_mut(|state| {
        state
            .notifications
            .retain(|notification| notification.id != id);
    });
}

#[component]
pub fn NotificationCard(props: NotificationProps) -> Element {
    let color_matching = get_color_matching(&props.r#type);
    let mut progress = use_signal(|| 0);
    let mut is_hover = use_signal(|| false);
    let id_for_future = props.id.clone();

    use_future(move || {
        let id = id_for_future.clone();
        let progress_time = (props.time * 10) as i32;
        async move {
            loop {
                if is_hover() {
                    progress.set(0);
                } else {
                    progress.set(progress() + 1);
                }
                let current_progress = progress();

                let promise = js_sys::Promise::new(&mut |resolve, _| {
                    window()
                        .unwrap()
                        .set_timeout_with_callback_and_timeout_and_arguments_0(
                            &resolve,
                            progress_time,
                        )
                        .unwrap();
                });
                wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();

                if current_progress >= 100 && !is_hover() {
                    let _ = add_element_class(&format!("#{}", id), "animate-slide-out");
                    let promise = js_sys::Promise::new(&mut |resolve, _| {
                        window()
                            .unwrap()
                            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 300)
                            .unwrap();
                    });
                    wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
                    remove_notification(id.clone());
                    break;
                }
            }
        }
    });

    rsx! {
        div {
            key: "{props.id}",
            id: props.id.clone(),
            class: "rounded px-3 py-3 m-2 border-2 border-transparent text-gray-800 {color_matching.bg_color} text-base animate-slide-in dark:text-gray-200 {color_matching.hover_border_color} transition-all duration-200 ease-in-out hover:scale-[1.02]",
            onmouseenter:move |_|{
                is_hover.set(true)
            },
            onmouseleave: move |_| {
                is_hover.set(false)
            },
            div {
                div {
                    class:"flex items-center justify-between {color_matching.text_color}",
                    div {
                        class:"mr-2 relative top-[0.06rem]",
                        {color_matching.icon},
                    }
                    div {
                        class:"truncate min-w-0",
                        {props.title.clone()}
                    }
                    div {
                        onclick:move |e|{
                            e.prevent_default();
                            remove_notification(props.id.clone());
                        },
                        class:"ml-1 flex-shrink-0 opacity-60 hover:opacity-100 transition-opacity duration-200 {color_matching.text_color}",
                        Icon{
                            icon:BsXLg,
                            height:18,
                            width:18
                        }
                    }
                }
                div {
                    class:"text-sm {color_matching.text_color}",
                    {props.content.clone()}
                }
                div {
                    class:"mt-2 border border-solid rounded-md {color_matching.border_color}",
                    div {
                        class:"w-full h-1 transition-all ease-linear {color_matching.progress_color}",
                        style:"width:{progress()}%",
                    }
                }
            }
        }
    }
}

#[component]
pub fn Notification() -> Element {
    let notifications_context = use_context::<Signal<NotificationProvider>>();
    let notifications = notifications_context().clone();

    return rsx! {
        div {
            class: "w-[20rem] absolute right-8 top-5 max-sm:w-[12rem] ",
            {notifications.notifications.iter().map(move |item| {
                rsx! {
                    NotificationCard {
                        key: "{item.id}",
                        ..item.clone()
                    }
                }
            })}
        }
        Outlet::<Route> {}
    };
}

pub struct Toast();

impl Toast {
    pub fn show(props: NotificationProps) {
        let mut notification_context = use_context::<Signal<NotificationProvider>>();
        let mut new_provider = notification_context().clone();
        let mut new_props = props;
        new_props.id = format!("notification-{}", generate_random_string(10));
        new_provider.notifications.push(new_props);
        notification_context.set(new_provider);
    }
    pub fn info(title: String, content: String) {
        Self::show(NotificationProps {
            title,
            content,
            r#type: NotificationType::Info,
            ..NotificationProps::default()
        });
    }
    pub fn error(title: String, content: String) {
        Self::show(NotificationProps {
            title,
            content,
            r#type: NotificationType::Error,
            ..NotificationProps::default()
        });
    }
    pub fn success(title: String, content: String) {
        Self::show(NotificationProps {
            title,
            content,
            r#type: NotificationType::Success,
            ..NotificationProps::default()
        });
    }
}
