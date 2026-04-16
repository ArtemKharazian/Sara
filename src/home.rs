use dioxus::prelude::*;

use crate::{
    models::{Ticket, TicketPriority, TicketStatus},
    Route,
};

#[derive(Clone, Copy, PartialEq)]
enum StatusFilter {
    All,
    Todo,
    InProgress,
    Done,
}

#[component]
pub fn Home() -> Element {
    let mut tickets = use_context::<Signal<Vec<Ticket>>>();
    let mut selected_filter = use_signal(|| StatusFilter::All);

    let filtered_tickets: Vec<Ticket> = tickets
        .read()
        .iter()
        .filter(|ticket| match *selected_filter.read() {
            StatusFilter::All => true,
            StatusFilter::Todo => ticket.status == TicketStatus::Todo,
            StatusFilter::InProgress => ticket.status == TicketStatus::InProgress,
            StatusFilter::Done => ticket.status == TicketStatus::Done,
        })
        .cloned()
        .collect();

    rsx! {
        main {
            class: "container",

            header {
                class: "header",
                h1 { "Sara" }
            }

            section {
                h2 {
                    class: "section-title",
                    "Tickets"
                }

                div {
                    class: "actions",
                    Link {
                        class: "link-button",
                        to: Route::CreateTicket {},
                        "Create Ticket"
                    }
                }

                div {
                    class: "filter-row",
                    button {
                        class: if *selected_filter.read() == StatusFilter::All { "button active" } else { "button" },
                        onclick: move |_| selected_filter.set(StatusFilter::All),
                        "All"
                    }
                    button {
                        class: if *selected_filter.read() == StatusFilter::Todo { "button active" } else { "button" },
                        onclick: move |_| selected_filter.set(StatusFilter::Todo),
                        "Todo"
                    }
                    button {
                        class: if *selected_filter.read() == StatusFilter::InProgress { "button active" } else { "button" },
                        onclick: move |_| selected_filter.set(StatusFilter::InProgress),
                        "In Progress"
                    }
                    button {
                        class: if *selected_filter.read() == StatusFilter::Done { "button active" } else { "button" },
                        onclick: move |_| selected_filter.set(StatusFilter::Done),
                        "Done"
                    }
                }

                div {
                    class: "ticket-list",

                    for ticket in filtered_tickets {
                        Link {
                            key: "{ticket.id}",
                            class: "ticket-link",
                            to: Route::TicketDetails { id: ticket.id },
                            article {
                                class: "ticket-card",
                                div {
                                    class: "ticket-status-strip {status_strip_class(ticket.status)}"
                                }
                                h3 { "{ticket.title}" }
                                p { "{ticket.description}" }
                                div {
                                    class: "meta",
                                    span {
                                        "Status: "
                                        select {
                                            value: "{format_status(&ticket.status)}",
                                            onclick: move |event| event.stop_propagation(),
                                            onchange: {
                                                let ticket_id = ticket.id;
                                                move |event| {
                                                    let next_status = parse_status(&event.value());
                                                    if let Some(current_ticket) = tickets.write().iter_mut().find(|item| item.id == ticket_id) {
                                                        current_ticket.status = next_status;
                                                    }
                                                }
                                            },
                                            option { value: "Todo", "Todo" }
                                            option { value: "In Progress", "In Progress" }
                                            option { value: "Done", "Done" }
                                        }
                                    }
                                    span { "Priority: {format_priority(&ticket.priority)}" }
                                    button {
                                        class: "button",
                                        onclick: {
                                            let ticket_id = ticket.id;
                                            move |event| {
                                                event.stop_propagation();
                                                event.prevent_default();
                                                tickets.write().retain(|item| item.id != ticket_id);
                                            }
                                        },
                                        "Delete"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn format_status(status: &TicketStatus) -> &'static str {
    match status {
        TicketStatus::Todo => "Todo",
        TicketStatus::InProgress => "In Progress",
        TicketStatus::Done => "Done",
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

fn status_strip_class(status: TicketStatus) -> &'static str {
    match status {
        TicketStatus::Todo => "status-todo",
        TicketStatus::InProgress => "status-in-progress",
        TicketStatus::Done => "status-done",
    }
}

fn format_priority(priority: &TicketPriority) -> &'static str {
    match priority {
        TicketPriority::Low => "Low",
        TicketPriority::Medium => "Medium",
        TicketPriority::High => "High",
    }
}
