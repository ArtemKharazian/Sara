use dioxus::prelude::*;

use crate::{
    db::{create_ticket, list_tickets},
    models::{Ticket, TicketPriority},
    Route,
};

#[component]
pub fn CreateTicket() -> Element {
    let mut title = use_signal(String::new);
    let mut description = use_signal(String::new);
    let mut priority = use_signal(|| TicketPriority::Medium);
    let mut save_error = use_signal(|| None::<String>);
    let tickets = use_context::<Signal<Vec<Ticket>>>();
    let nav = navigator();

    rsx! {
        main {
            class: "container",

            header {
                class: "header",
                h1 { "Create Ticket" }
                p { "Add a new ticket to the system" }
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
                            let title_value = title.read().trim().to_string();
                            if title_value.is_empty() {
                                save_error.set(Some("Title is required.".to_string()));
                                return;
                            }

                            save_error.set(None);

                            let description_value = description.read().trim().to_string();
                            let priority_value = *priority.read();
                            let nav = nav.clone();
                            let mut tickets = tickets;
                            let mut save_error = save_error;

                            spawn(async move {
                                match create_ticket(title_value, description_value, priority_value)
                                    .await
                                {
                                    Ok(()) => {
                                        if let Ok(db_tickets) = list_tickets().await {
                                            tickets.set(db_tickets);
                                        }
                                        nav.push(Route::Home {});
                                    }
                                    Err(error) => {
                                        save_error.set(Some(format!(
                                            "Save failed: {error}"
                                        )));
                                    }
                                }
                            });
                        },
                        "Save"
                    }

                    Link {
                        class: "link-button",
                        to: Route::Home {},
                        "Cancel"
                    }
                }

                if let Some(message) = save_error.read().clone() {
                    p {
                        style: "margin-top: 10px; color: #b00020;",
                        "{message}"
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
