use dioxus::prelude::*;

use crate::{
    models::{Ticket, TicketPriority, TicketStatus},
    Route,
};

#[component]
pub fn TicketDetails(id: u32) -> Element {
    let mut tickets = use_context::<Signal<Vec<Ticket>>>();

    let find_ticket = tickets.read().iter().find(|ticket| ticket.id == id).cloned();

    rsx! {
        main {
            class: "container",

            header {
                class: "header",
                h1 { "Ticket Details" }
            }

            if let Some(ticket) = find_ticket {
                section {
                    class: "form-card",

                    p { "ID: {ticket.id}" }

                    div {
                        class: "form-group",
                        label { "Title" }
                        input {
                            class: "input",
                            value: "{ticket.title}",
                            oninput: move |event| {
                                if let Some(current_ticket) = tickets.write().iter_mut().find(|item| item.id == id) {
                                    current_ticket.title = event.value();
                                }
                            },
                        }
                    }

                    div {
                        class: "form-group",
                        label { "Description" }
                        textarea {
                            class: "textarea",
                            value: "{ticket.description}",
                            oninput: move |event| {
                                if let Some(current_ticket) = tickets.write().iter_mut().find(|item| item.id == id) {
                                    current_ticket.description = event.value();
                                }
                            },
                        }
                    }

                    div {
                        class: "form-group",
                        label { "Status" }
                        select {
                            class: "select",
                            value: "{status_value(ticket.status)}",
                            onchange: move |event| {
                                if let Some(current_ticket) = tickets.write().iter_mut().find(|item| item.id == id) {
                                    current_ticket.status = parse_status(&event.value());
                                }
                            },
                            option { value: "Todo", "Todo" }
                            option { value: "In Progress", "In Progress" }
                            option { value: "Done", "Done" }
                        }
                    }

                    div {
                        class: "form-group",
                        label { "Priority" }
                        select {
                            class: "select",
                            value: "{priority_value(ticket.priority)}",
                            onchange: move |event| {
                                if let Some(current_ticket) = tickets.write().iter_mut().find(|item| item.id == id) {
                                    current_ticket.priority = parse_priority(&event.value());
                                }
                            },
                            option { value: "Low", "Low" }
                            option { value: "Medium", "Medium" }
                            option { value: "High", "High" }
                        }
                    }

                    div {
                        class: "form-actions",
                        Link {
                            class: "link-button",
                            to: Route::Home {},
                            "Back"
                        }
                    }
                }
            } else {
                section {
                    class: "form-card",
                    p { "Ticket not found" }
                    Link {
                        class: "link-button",
                        to: Route::Home {},
                        "Back"
                    }
                }
            }
        }
    }
}

fn parse_status(value: &str) -> TicketStatus {
    match value {
        "Todo" => TicketStatus::Todo,
        "In Progress" => TicketStatus::InProgress,
        "Done" => TicketStatus::Done,
        _ => TicketStatus::Todo,
    }
}

fn status_value(status: TicketStatus) -> &'static str {
    match status {
        TicketStatus::Todo => "Todo",
        TicketStatus::InProgress => "In Progress",
        TicketStatus::Done => "Done",
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
