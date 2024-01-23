use aspect_fit::{aspect_fit::aspect_fit, size::Size};
use js_sys::{Array, Reflect};
use real_float::Real;
use std::{cmp::Ordering, f64::consts::PI};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use wasm_tensorflow_models_pose_detection::{
    model::Model,
    pose::Keypoint,
    pose_detector::{CommonEstimationConfig, EstimationConfig, PoseDetector},
};
use web_sys::{
    console::log_1, window, CanvasRenderingContext2d, HtmlCanvasElement, HtmlDivElement,
    HtmlVideoElement,
};

use super::body_foi::{FoiMem, POINT_HISTORY_LENGTH};

fn range_and_average(arr: &[f64]) -> (f64, f64) {
    let mut min = arr[0];
    let mut max = arr[0];
    let mut sum = 0.;
    for value in arr {
        if *value < min {
            min = *value;
        }
        if *value > max {
            max = *value;
        }
        sum += *value;
    }
    (max - min, sum / arr.len() as f64)
}

pub async fn plots_frame(canvas: &HtmlCanvasElement, memory: &mut FoiMem) {
    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();
    let canvas_size = Size {
        width: canvas.offset_width() as u32,
        height: canvas.offset_height() as u32,
    };

    let mut graph_offset = 0.;

    let px_per_step = canvas_size.width as f64 / POINT_HISTORY_LENGTH as f64;
    let half_height = canvas_size.height as f64 / 2.;
    ctx.clear_rect(
        0 as f64,
        0 as f64,
        canvas_size.width as f64,
        canvas_size.height as f64,
    );

    ctx.set_fill_style(&"#000c".into());
    
    // iterate hash map of string, history
    
    for (name, history) in &memory.history {
    
        ctx.begin_path();
        ctx.set_stroke_style(&"blue".into());
        let mut counter = 0;
        ctx.move_to(counter as f64, 0.);
        let (rng, avg) = range_and_average(&history.x);
        let graph_range = half_height / rng;
        let graph_offset = half_height - (avg * graph_range);

        for value in history.x {
            let y = graph_range * value + graph_offset;
            ctx.line_to(counter as f64 * px_per_step, y);
            // IDK how to get iterator, I got no internet now lol.
            counter += 1;
        }
        ctx.stroke();

        counter = 0;

        let (rng, avg) = range_and_average(&history.y);
        let graph_range = half_height / rng;
        let graph_offset = half_height - (avg * graph_range);

        ctx.begin_path();
        ctx.set_stroke_style(&"red".into());
        ctx.move_to(counter as f64, 0.);
        let mut counter = 0;
        for value in history.y {
            let y = graph_range * value + graph_offset;
            ctx.line_to(counter as f64 * px_per_step, y);
            // IDK how to get iterator, I got no internet now lol.
            counter += 1;
        }
        ctx.stroke();

        // log value of px_per_step
        // log_1(&px_per_step.into());
    }
}
