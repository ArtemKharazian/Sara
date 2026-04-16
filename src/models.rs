#[derive(Clone, PartialEq)]
pub struct Ticket {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub status: TicketStatus,
    pub priority: TicketPriority,
}

#[derive(Clone, Copy, PartialEq)]
pub enum TicketStatus {
    Todo,
    InProgress,
    Done,
}

#[derive(Clone, Copy, PartialEq)]
pub enum TicketPriority {
    Low,
    Medium,
    High,
}

pub fn mock_tickets() -> Vec<Ticket> {
    vec![
        Ticket {
            id: 1,
            title: "test 1".to_string(),
            description: "hjvbjhvjhvjc".to_string(),
            status: TicketStatus::Done,
            priority: TicketPriority::High,
        },
        Ticket {
            id: 2,
            title: "test 2".to_string(),
            description: "jkhköjiohlh".to_string(),
            status: TicketStatus::InProgress,
            priority: TicketPriority::High,
        },
        Ticket {
            id: 3,
            title: "something 3".to_string(),
            description: "hjjhgfhghjfg".to_string(),
            status: TicketStatus::Todo,
            priority: TicketPriority::Medium,
        },
        Ticket {
            id: 4,
            title: "ticket 4".to_string(),
            description: "jkjhlk".to_string(),
            status: TicketStatus::Todo,
            priority: TicketPriority::Low,
        },
        Ticket {
            id: 5,
            title: "problem 5".to_string(),
            description: "jhhcfgxfhx".to_string(),
            status: TicketStatus::Todo,
            priority: TicketPriority::Medium,
        },
    ]
}
