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
    window, CanvasRenderingContext2d, HtmlCanvasElement, HtmlDivElement, HtmlVideoElement,
};

use super::body_foi::FoiMem;


pub async fn plots_frame(
    canvas: &HtmlCanvasElement,
    memory: &mut FoiMem,
) {
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

    ctx.clear_rect(
        0 as f64,
        0 as f64,
        canvas_size.width as f64,
        canvas_size.height as f64,
    );

    ctx.set_fill_style(&"#000c".into());

    ctx.begin_path();
    ctx.set_stroke_style(&"blue".into());
    let mut counter = 0;
    ctx.move_to(counter as f64, 0.);

    // in the future: a low-cut value. for now, just this
    graph_offset = memory.left_wrist.x[0] - (canvas_size.height as f64 / 2.);

    for value in memory.left_wrist.x {
        ctx.line_to(counter as f64, value - graph_offset);
        // IDK how to get iterator, I got no internet now lol.
        counter += 1;
    }
    ctx.stroke();

    counter = 0;
    graph_offset = memory.left_wrist.y[0] - (canvas_size.height as f64 / 2.);

    ctx.begin_path();
    ctx.set_stroke_style(&"red".into());
    ctx.move_to(counter as f64, 0.);
    let mut counter = 0;
    for value in memory.left_wrist.y {
        ctx.line_to(counter as f64, value - graph_offset);
        // IDK how to get iterator, I got no internet now lol.
        counter += 1;
    }
    ctx.stroke();
}
