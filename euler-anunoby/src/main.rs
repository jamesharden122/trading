#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_logger::tracing;
use euler_anunoby::charts::PlottersChart;
use euler_anunoby::ops::*;
use euler_anunoby::surr_queries::*;
use euler_anunoby::tables::SalesTable;
use euler_anunoby::tables::TradeDisplay;
use nalgebra::DMatrix;
use plotters::prelude::*;
use std::convert::TryInto;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    #[route("/")]
    Home {},
}

fn main() {
    // Init logger
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    let mut response = use_signal(|| MyMatrix::new(10, 10));
    let log_in = move |_| {
        spawn(async move {
            let mut resp = query_surr_trademsg_db(
                String::from("root"),
                String::from("root"),
                String::from("equities"),
                String::from("historical"),
                String::from("trade"),
            )
            .await
            .unwrap();
            response.set(resp)
        });
    };
    rsx! {
        style {{include_str!("./../src/css_files/home_style.css")}}
        div {
            h1 {
                "Quant Streaming Demo"
            }
        }
        div {
            button { onclick: log_in, "Get Data" }
        }
        section { class: "grid-section",
            div { class: "grid-item",
                PlottersChart {data: response.read().clone().data, descrips: response.read().snapshot().unwrap() }
            }
            div { class: "grid-item",
                SalesTable {data: response.read().clone().data, descrips: response.read().snapshot().unwrap() }
            }
            div{ class: "grid-item",
                TradeDisplay {data: response.read().clone().data, descrips: response.read().snapshot().unwrap() }
            }
        }
    }
}

#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    tracing::info!("Server received: {}", data);
    Ok(())
}

#[server(GetServerData)]
async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("Hello from the server!".to_string())
}
