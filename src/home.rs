use dioxus::prelude::*;

use crate::models::{mock_tickets, TicketPriority, TicketStatus};

#[component]
pub fn Home() -> Element {
    // Day 1: ticket data is hardcoded in memory.
    let tickets = mock_tickets();

    rsx! {
        main {
            class: "container",

            header {
                class: "header",
                h1 { "Simple Ticket System" }
                p { "Day 1 MVP" }
            }

            section {
                h2 {
                    class: "section-title",
                    "Tickets"
                }

                div {
                    class: "ticket-list",

                    for ticket in tickets {
                        article {
                            key: "{ticket.id}",
                            class: "ticket-card",
                            h3 { "{ticket.title}" }
                            p { "{ticket.description}" }
                            div {
                                class: "meta",
                                span { "Status: {format_status(&ticket.status)}" }
                                span { "Priority: {format_priority(&ticket.priority)}" }
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

fn format_priority(priority: &TicketPriority) -> &'static str {
    match priority {
        TicketPriority::Low => "Low",
        TicketPriority::Medium => "Medium",
        TicketPriority::High => "High",
    }
}
