use crate::ops::MyMatrix;
use crate::surr_queries::query_surr_trademsg_db;
use dioxus::prelude::*;
use plotters::prelude::*;

#[component]
pub fn PlottersChart(props: MyMatrix) -> Element {
    let svg = use_signal(String::new);
    let mut new_matrix = use_signal(|| MyMatrix::new(10, 10));
    use_effect(move || {
        to_owned![svg];
        spawn(async move {
            new_matrix.set(
                query_surr_trademsg_db(
                    String::from("root"),
                    String::from("root"),
                    String::from("equities"),
                    String::from("historical"),
                    String::from("trade"),
                )
                .await
                .unwrap(),
            );
            let binding = new_matrix.read();
            let first_column = binding.data.column(0);
            let third_column: Vec<f64> = (1..=first_column.len()).map(|v| v as f64).collect(); //let third_column = matrix.column(2);
            let points: Vec<(f64, f64)> = third_column
                .iter()
                .zip(first_column.iter())
                .map(|(&x, &y)| (x, y))
                .collect();
            // Extract the first and third columns
            // Find the min and max of the first column
            let mut buffer = String::new();
            {
                let root = SVGBackend::with_string(&mut buffer, (600, 400)).into_drawing_area();
                root.fill(&WHITE).unwrap();
                let mut chart = ChartBuilder::on(&root)
                    .margin(5)
                    .x_label_area_size(50)
                    .y_label_area_size(50)
                    .build_cartesian_2d(0.0f64..100.0f64, 216.0f64..220.0f64)
                    .unwrap();

                chart
                    .configure_mesh()
                    .light_line_style(WHITE)
                    .x_desc("Time")
                    .y_desc("Price")
                    .axis_desc_style(("sans-serif", 15))
                    .label_style(("sans-serif", 12).into_font())
                    .draw()
                    .unwrap();

                // Draw the series using the points

                chart.draw_series(LineSeries::new(points, &RED)).unwrap();
            }
            svg.set(buffer);
        });
    });
    rsx! {
        div {class: "main_chart",
            dangerous_inner_html: "{svg}"
        }
    }
}
