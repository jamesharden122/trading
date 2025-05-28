use crate::ops::MyMatrix;
use chrono::{NaiveDateTime, TimeZone, Utc};
use dioxus::prelude::*;
use tracing::info;

#[component]
pub fn PlottersChart(props: MyMatrix) -> Element {
    let width = 700.0;
    let height = 400.0;
    let y_padding = 100.0;
    let x_padding = y_padding / 1.5;

    // Limit to 100 points
    let max_points = 300;
    let mut combined: Vec<(f64, f64)> = props
        .data
        .column(3)
        .iter()
        .copied()
        .zip(props.data.column(0).iter().copied())
        .take(max_points)
        .collect();

    combined.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

    let x_values: Vec<f64> = combined.iter().map(|(x, _)| *x).collect();
    let y_values: Vec<f64> = combined.iter().map(|(_, y)| *y).collect();

    for (i, ts) in x_values.iter().enumerate() {
        info!("Row {}: ts_recv = {}", i, ts);
    }

    if x_values.is_empty() || y_values.is_empty() {
        return rsx!(div { "No data available" });
    }

    // Get bounds
    let x_min = *x_values
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let x_max = *x_values
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let y_min = *y_values
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let y_max = *y_values
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    // Scale function
    let scale_x = |x: f64| ((x - x_min) / (x_max - x_min) * (width - 2.0 * x_padding)) + x_padding;
    let scale_y =
        |y: f64| height - ((y - y_min) / (y_max - y_min) * (height - 2.0 * y_padding)) - y_padding;
    let x_ticks = 6;
    let y_ticks = 6;

    let x_tick_values: Vec<f64> = (0..x_ticks)
        .map(|i| x_min + (i as f64 / (x_ticks - 1) as f64) * (x_max - x_min))
        .collect();

    let y_tick_values: Vec<f64> = (0..y_ticks)
        .map(|i| y_min + (i as f64 / (y_ticks - 1) as f64) * (y_max - y_min))
        .collect();

    let points: Vec<String> = x_values
        .iter()
        .zip(y_values.iter())
        .map(|(&x, &y)| format!("{},{}", scale_x(x), scale_y(y)))
        .collect();

    let polyline_points = points.join(" ");

    let x_tick_elements: Vec<_> = x_tick_values
        .iter()
        .map(|&xv| {
            let x_pos = scale_x(xv);

            let timestamp = chrono::Utc.timestamp_nanos(xv as i64);
            let formatted = timestamp.format("%Y-%m-%d %H:%M:%S").to_string();

            rsx!(
                line {
                    x1: "{x_pos}",
                    y1: "{height - y_padding}",
                    x2: "{x_pos}",
                    y2: "{height - y_padding + 5.0}",
                    stroke: "black",
                    stroke_width: "1"
                },
                text {
                    x: "{x_pos}",
                    y: "{height - y_padding + 20.0}",
                    font_size: "11",
                    font_family: "Georgia",
                    font_weight: "700",
                    text_anchor: "start",
                    fill: "black",
                    transform: "rotate(40, {x_pos}, {height - y_padding})",
                    "{formatted}"
                }
            )
        })
        .collect();

    let y_tick_elements: Vec<_> = y_tick_values
        .iter()
        .map(|&yv| {
            let y_pos = scale_y(yv);
            let label = format!("{:.2}", yv);
            rsx! {
                        // Tick line extending out from the Y-axis at x = x_padding
            line {
                x1: "{x_padding}",
                y1: "{y_pos}",
                x2: "{width - x_padding}",
                y2: "{y_pos}",
                stroke: "#ccc",
                stroke_width: "0.5",
                stroke_dasharray: "4 2",
                stroke_opacity: "0.6"
            }
            line {
                x1: "{x_padding - 5.0}",
                y1: "{y_pos}",
                x2: "{x_padding}",
                y2: "{y_pos}",
                stroke: "black",
                stroke_width: "1"
            }
            // Label just to the left of the tick
            text {
                x: "{x_padding - 8.0}",
                y: "{y_pos + 4.0}",
                font_size: "11",
                font_family: "Georgia",
                font_weight: "700",
                text_anchor: "end",
                fill: "black",
                "{label}"
            }
                    }
        })
        .collect();

    rsx! {
        svg {
            width: "{width}",
            height: "{height}",
            style: "border: background-color: #0a0f0a;",
            rect {
                x: "{x_padding}",
                y: "{y_padding / 4.0}",
                width: "{width - x_padding * 2.0}",
                height: "{height - y_padding - y_padding / 4.0}",
                fill: "rgba(0, 128, 0, 0.05)", // translucent green
            }
            // Line chart
            polyline {
                points: "{polyline_points}",
                fill: "none",
                stroke: "#66bb6a", // light green
                stroke_width: "2"
            }
            // Y-axis
            line {
                x1: "{x_padding}",
                y1: "{y_padding/4.0}",
                x2: "{x_padding}",
                y2: "{height - y_padding}",
                stroke: "#81c784", // soft green for axis
                stroke_width: "1"
            },

            // X-axis
            line {
                x1: "{x_padding}",
                y1: "{height - y_padding}",
                x2: "{width - x_padding}",
                y2: "{height - y_padding}",
                stroke: "#81c784", // soft green for axis
                stroke_width: "1"
            },

            // X and Y ticks and labels
            { x_tick_elements.into_iter() },
            { y_tick_elements.into_iter() },

            // Y-axis label
            text {
                x: "20.0",
                y: "{height / 2.0}",
                transform: "rotate(-90, 20.0, {height / 2.0})",
                font_size: "14",
                font_family: "Georgia",
                font_weight: "700",
                text_anchor: "middle",
                fill: "black",
                "Price"
            },

            // X-axis label
            text {
                x: "{width / 2.0}",
                y: "{height - 5.0}",
                font_size: "14",
                font_family: "Georgia",
                font_weight: "700",
                text_anchor: "middle",
                fill: "black",
                "Timestamp"
            }
        }
    }
}
