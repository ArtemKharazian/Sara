use dioxus::prelude::*;

use crate::{
    models::{Ticket, TicketPriority, TicketStatus},
    Route,
};

#[component]
pub fn CreateTicket() -> Element {
    let mut title = use_signal(String::new);
    let mut description = use_signal(String::new);
    let mut priority = use_signal(|| TicketPriority::Medium);
    let mut tickets = use_context::<Signal<Vec<Ticket>>>();
    let nav = navigator();

    rsx! {
        main {
            class: "container",

            header {
                class: "header",
                h1 { "Create Ticket" }
                p { "Add a new ticket for Day 2." }
            }

            section {
                class: "form-card",

                div {
                    class: "form-group",
                    label { "Title" }
                    input {
                        class: "input",
                        value: "{title}",
                        oninput: move |event| title.set(event.value()),
                    }
                }

                div {
                    class: "form-group",
                    label { "Description" }
                    textarea {
                        class: "textarea",
                        value: "{description}",
                        oninput: move |event| description.set(event.value()),
                    }
                }

                div {
                    class: "form-group",
                    label { "Priority" }
                    select {
                        class: "select",
                        value: "{priority_value(*priority.read())}",
                        onchange: move |event| priority.set(parse_priority(&event.value())),
                        option { value: "Low", "Low" }
                        option { value: "Medium", "Medium" }
                        option { value: "High", "High" }
                    }
                }

                div {
                    class: "form-actions",
                    button {
                        class: "button",
                        onclick: move |_| {
                            if title.read().trim().is_empty() {
                                return;
                            }

                            let next_id = tickets
                                .read()
                                .iter()
                                .map(|ticket| ticket.id)
                                .max()
                                .unwrap_or(0)
                                + 1;

                            tickets.write().push(Ticket {
                                id: next_id,
                                title: title.read().trim().to_string(),
                                description: description.read().trim().to_string(),
                                status: TicketStatus::Todo,
                                priority: *priority.read(),
                            });

                            nav.push(Route::Home {});
                        },
                        "Save"
                    }

                    Link {
                        class: "link-button",
                        to: Route::Home {},
                        "Cancel"
                    }
                }
            }
        }
    }
}

fn parse_priority(value: &str) -> TicketPriority {
    match value {
        "Low" => TicketPriority::Low,
        "High" => TicketPriority::High,
        _ => TicketPriority::Medium,
    }
}

fn priority_value(priority: TicketPriority) -> &'static str {
    match priority {
        TicketPriority::Low => "Low",
        TicketPriority::Medium => "Medium",
        TicketPriority::High => "High",
    }
}
