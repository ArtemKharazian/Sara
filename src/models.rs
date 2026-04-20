use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Ticket {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub status: TicketStatus,
    pub priority: TicketPriority,
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TicketStatus {
    Todo,
    InProgress,
    Done,
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TicketPriority {
    Low,
    Medium,
    High,
}
