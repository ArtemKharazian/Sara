use dioxus::prelude::*;

mod home;
mod models;

use home::Home;

fn main() {
    dioxus::launch(App);
}

#[derive(Routable, Clone, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
}

#[component]
fn App() -> Element {
    rsx! {
        style {
            {r#"
                * {
                    box-sizing: border-box;
                }

                body {
                    margin: 0;
                    font-family: Arial, sans-serif;
                    background: #f7f7f8;
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
                }

                .header p {
                    margin: 6px 0 0;
                    color: #666;
                }

                .section-title {
                    margin: 0 0 12px;
                    font-size: 20px;
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
                }

                .ticket-card h3 {
                    margin: 0 0 8px;
                    font-size: 18px;
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
            "#}
        }

        Router::<Route> {}
    }
}
