use dioxus::prelude::*;

mod create_ticket;
#[cfg(all(feature = "server", not(target_arch = "wasm32")))]
mod db;
mod home;
mod models;
mod ticket_details;

use create_ticket::CreateTicket;
use home::Home;
use models::mock_tickets;
use ticket_details::TicketDetails;

fn main() {
    #[cfg(all(feature = "server", not(target_arch = "wasm32")))]
    db::init_db().expect("failed to initialize SQLite database");

    dioxus::launch(App);
}

#[derive(Routable, Clone, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/create")]
    CreateTicket {},
    #[route("/ticket/:id")]
    TicketDetails { id: u32 },
}

#[component]
fn App() -> Element {
    let tickets = use_signal(mock_tickets);
    use_context_provider(|| tickets);

    rsx! {
        style {
            {r#"
                * {
                    box-sizing: border-box;
                }

                body {
                    margin: 0;
                    font-family: Arial, sans-serif;
                    background: #c4dfe6;
                    color: #222;
                }

                .container {
                    max-width: 760px;
                    margin: 40px auto;
                    padding: 0 16px 24px;
                }

                .header {
                    margin-bottom: 24px;
                }

                .header h1 {
                    margin: 0;
                    font-size: 28px;
                    color: #003B46;
                }

                .header p {
                    margin: 6px 0 0;
                    color: #003B46;
                }

                .actions {
                    margin-bottom: 12px;
                }

                .button,
                .link-button {
                    display: inline-block;
                    padding: 8px 12px;
                    border: 1px solid #d0d0d6;
                    border-radius: 6px;
                    background: #fff;
                    color: #003B46;
                    text-decoration: none;
                    cursor: pointer;
                    font-size: 14px;
                }
                
                .button.active {
                    background: #66A5AD;
                    color: #ffffff;
                }

                .button:hover,
                .link-button:hover {
                    background: #66A5AD;
                    color: #ffffff;
                }

                .filter-row {
                    display: flex;
                    gap: 8px;
                    margin-bottom: 12px;
                    flex-wrap: wrap;
                }

                .section-title {
                    margin: 0 0 12px;
                    font-size: 20px;
                    color: #003B46;
                }

                .ticket-list {
                    display: grid;
                    gap: 12px;
                }

                .ticket-card {
                    background: #fff;
                    border: 1px solid #e3e3e8;
                    border-radius: 8px;
                    padding: 12px;
                    position: relative;
                    padding-bottom: 44px;
                }

                .ticket-status-strip {
                    height: 10px;
                    border-radius: 4px;
                    margin: -12px -12px 10px;
                }

                .status-todo {
                    background:rgb(252, 138, 125);
                }

                .status-in-progress {
                    background: #3498db;
                }

                .status-done {
                    background: #2ecc71;
                }

                .ticket-link {
                    text-decoration: none;
                    color: inherit;
                }

                .ticket-card h3 {
                    margin: 0 0 8px;
                    font-size: 18px;
                    color: #003B46;
                }

                .ticket-card p {
                    margin: 0 0 10px;
                    color: #444;
                }

                .meta {
                    font-size: 14px;
                    color: #555;
                    display: flex;
                    gap: 14px;
                }

                .delete-button {
                    position: absolute;
                    right: 12px;
                    bottom: 12px;
                }

                .form-card {
                    background: #fff;
                    border: 1px solid #e3e3e8;
                    border-radius: 8px;
                    padding: 14px;
                }

                .form-group {
                    margin-bottom: 12px;
                }

                .form-group label {
                    display: block;
                    margin-bottom: 6px;
                    font-size: 14px;
                }

                .input,
                .textarea,
                .select {
                    width: 100%;
                    padding: 8px;
                    border: 1px solid #d0d0d6;
                    border-radius: 6px;
                    font-size: 14px;
                    font-family: inherit;
                    background: #fff;
                }

                .textarea {
                    min-height: 90px;
                    resize: vertical;
                }

                .form-actions {
                    display: flex;
                    gap: 8px;
                }
            "#}
        }

        Router::<Route> {}
    }
}
