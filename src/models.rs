#[derive(Clone)]
pub struct Ticket {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub status: TicketStatus,
    pub priority: TicketPriority,
}

#[derive(Clone)]
pub enum TicketStatus {
    Todo,
    InProgress,
    Done,
}

#[derive(Clone)]
pub enum TicketPriority {
    Low,
    Medium,
    High,
}

pub fn mock_tickets() -> Vec<Ticket> {
    vec![
        Ticket {
            id: 1,
            title: "Set up project".to_string(),
            description: "Create a minimal Dioxus project structure.".to_string(),
            status: TicketStatus::Done,
            priority: TicketPriority::High,
        },
        Ticket {
            id: 2,
            title: "Create homepage layout".to_string(),
            description: "Add a small header and main tickets section.".to_string(),
            status: TicketStatus::InProgress,
            priority: TicketPriority::High,
        },
        Ticket {
            id: 3,
            title: "Add ticket cards".to_string(),
            description: "Display each ticket title, description, status, and priority."
                .to_string(),
            status: TicketStatus::Todo,
            priority: TicketPriority::Medium,
        },
        Ticket {
            id: 4,
            title: "Style the page".to_string(),
            description: "Keep styling clean and minimal for Day 1.".to_string(),
            status: TicketStatus::Todo,
            priority: TicketPriority::Low,
        },
        Ticket {
            id: 5,
            title: "Prepare for Day 2".to_string(),
            description: "Leave the code simple so it is easy to extend later.".to_string(),
            status: TicketStatus::Todo,
            priority: TicketPriority::Medium,
        },
    ]
}
