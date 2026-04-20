use dioxus::prelude::*;

use crate::models::{Ticket, TicketPriority, TicketStatus};

#[cfg(not(target_arch = "wasm32"))]
use rusqlite::Connection;

#[cfg(not(target_arch = "wasm32"))]
pub fn init_db() -> Result<(), rusqlite::Error> {
    let connection = Connection::open("tickets.db")?;

    connection.execute(
        "CREATE TABLE IF NOT EXISTS tickets (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT NOT NULL,
            status TEXT NOT NULL,
            priority TEXT NOT NULL
        )",
        [],
    )?;

    Ok(())
}

#[server]
pub async fn list_tickets() -> ServerFnResult<Vec<Ticket>> {
    #[cfg(target_arch = "wasm32")]
    {
        return Err(ServerFnError::new(
            "list_tickets should run on the server runtime",
        ));
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let connection = Connection::open("tickets.db").map_err(ServerFnError::new)?;

        let mut statement = connection
            .prepare("SELECT id, title, description, status, priority FROM tickets ORDER BY id ASC")
            .map_err(ServerFnError::new)?;

        let rows = statement
            .query_map([], |row| {
                let status_text: String = row.get(3)?;
                let priority_text: String = row.get(4)?;

                Ok(Ticket {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    description: row.get(2)?,
                    status: parse_status(&status_text),
                    priority: parse_priority(&priority_text),
                })
            })
            .map_err(ServerFnError::new)?;

        let mut tickets = Vec::new();
        for row in rows {
            tickets.push(row.map_err(ServerFnError::new)?);
        }

        Ok(tickets)
    }
}

#[server]
pub async fn create_ticket(
    title: String,
    description: String,
    priority: TicketPriority,
) -> ServerFnResult<()> {
    #[cfg(target_arch = "wasm32")]
    {
        let _ = (title, description, priority);
        return Err(ServerFnError::new(
            "create_ticket should run on the server runtime",
        ));
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let connection = Connection::open("tickets.db").map_err(ServerFnError::new)?;

        connection
            .execute(
                "INSERT INTO tickets (title, description, status, priority) VALUES (?1, ?2, ?3, ?4)",
                (
                    title,
                    description,
                    "Todo",
                    format_priority(priority),
                ),
            )
            .map_err(ServerFnError::new)?;

        Ok(())
    }
}

#[server]
pub async fn update_ticket_status(id: u32, status: TicketStatus) -> ServerFnResult<()> {
    #[cfg(target_arch = "wasm32")]
    {
        let _ = (id, status);
        return Err(ServerFnError::new(
            "update_ticket_status should run on the server runtime",
        ));
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let connection = Connection::open("tickets.db").map_err(ServerFnError::new)?;

        connection
            .execute(
                "UPDATE tickets SET status = ?1 WHERE id = ?2",
                (format_status(status), id),
            )
            .map_err(ServerFnError::new)?;

        Ok(())
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn parse_status(value: &str) -> TicketStatus {
    match value {
        "In Progress" => TicketStatus::InProgress,
        "Done" => TicketStatus::Done,
        _ => TicketStatus::Todo,
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn parse_priority(value: &str) -> TicketPriority {
    match value {
        "Low" => TicketPriority::Low,
        "High" => TicketPriority::High,
        _ => TicketPriority::Medium,
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn format_priority(priority: TicketPriority) -> &'static str {
    match priority {
        TicketPriority::Low => "Low",
        TicketPriority::Medium => "Medium",
        TicketPriority::High => "High",
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn format_status(status: TicketStatus) -> &'static str {
    match status {
        TicketStatus::Todo => "Todo",
        TicketStatus::InProgress => "In Progress",
        TicketStatus::Done => "Done",
    }
}
