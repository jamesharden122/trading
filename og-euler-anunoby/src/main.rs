#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*; // Make sure this is imported

use dioxus_logger::tracing;
use og_euler_anunoby::surr_queries::query_surr_trademsg_db;
use og_euler_anunoby::{
    charts::{single_asset_lc::PlottersChart,clustering::ScatterPlot},
    ops::MyMatrix,
    tables::{SalesTable, TradeDisplay},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Routable, Debug, PartialEq, Serialize, Deserialize)]
enum Route {
    #[route("/")]
    Home,
}

fn main() {
    #[cfg(feature = "server")]
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(launch_server());
    #[cfg(not(feature = "server"))]
    dioxus::launch(App);
}

#[cfg(feature = "server")]
async fn launch_server() {
    // Connect to dioxus' logging infrastructure
    dioxus::logger::initialize_default();

    // Connect to the IP and PORT env vars passed by the Dioxus CLI (or your dockerfile)
    let socket_addr = dioxus_cli_config::fullstack_address_or_localhost();

    // Build a custom axum router
    let router = axum::Router::new()
        .serve_dioxus_application(ServeConfigBuilder::new(), App)
        .into_make_service();

    // And launch it!
    let listener = tokio::net::TcpListener::bind(socket_addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    let response = use_signal(MyMatrix::new10x);

    let get_data_button = {
        let mut response = response.clone();
        let log_in = move |_| {
            let mut response = response.clone();
            spawn(async move {
                if let Ok(resp) = query_surr_trademsg_db(
                    "root".into(),
                    "root".into(),
                    "equities".into(),
                    "historical".into(),
                    "trade".into(),
                )
                .await
                {
                    response.set(resp);
                }
            });
        };
        Some(rsx!(button { onclick: log_in, "Get Data" }))
    };
    let data = response.read();
    let descrips = data.snapshot().unwrap_or_default();

    rsx! {
        style { { include_str!("./../src/css_files/home_style.css") } }

        div {
            h1 { "Quant Streaming Demo" }
        }

        div {
            { get_data_button }
        }
        section { class: "grid-wrapper",
            section { class: "grid-section",
                div { class: "grid-item",
                    PlottersChart { data: data.data.clone(), descrips: descrips }
                }
                div { class: "grid-item",
                    SalesTable { data: data.data.clone(), descrips: descrips }
                }
                div { class: "grid-item",
                    TradeDisplay { data: data.data.clone(), descrips: descrips }
                }
                div { class: "grid-item",
                    ScatterPlot { data: data.data.clone(), descrips: descrips }
                }
            }
            section { class: "grid-section",
                div { class: "grid-item",
                    h3 { "hello" }
                }
            }
        }
    }
}
