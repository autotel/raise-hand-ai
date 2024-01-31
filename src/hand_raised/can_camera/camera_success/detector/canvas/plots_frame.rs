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

use super::body_foi::{average, range, range_and_average, FoiMem, POINT_HISTORY_LENGTH};


const line_colors: [&str;8] = [
    "#4e0250ff",
    "#801a86ff",
    "#723a86ff",
    "#645986ff",
    "#7a9e87ff",
    "#8fe388ff",
    "#74d085ff",
    "#58bc82ff",
];

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

    let px_per_step = canvas_size.width as f64 / POINT_HISTORY_LENGTH as f64;
    let half_height = canvas_size.height as f64 / 2.;
    let mut color_counter = 0 as usize;
    

    let plot_only: Vec<String> = vec![
        "left_wrist".into(),
        // "right_wrist".into(),
        // "left_elbow".into(),
        // "right_elbow".into(),
        // "left_shoulder".into(),
        // "right_shoulder".into(),
        // "left_ankle".into(),
        // "right_ankle".into(),
    ];

    ctx.clear_rect(
        0 as f64,
        0 as f64,
        canvas_size.width as f64,
        canvas_size.height as f64,
    );

    ctx.set_fill_style(&"#000c".into());

    let mut plotty = |name: &str, arr: &[f64]| {
        let color = line_colors[color_counter];
        color_counter += 1;
        ctx.begin_path();
        ctx.set_stroke_style(&color.into());
        let mut counter_x = 0;
        ctx.move_to(counter_x as f64, 0.);
        let (rng, avg) = (range(arr),average(arr));
        let clamped_rng = rng.max(1.).min(20.);
        let graph_range = half_height / -clamped_rng;
        let graph_offset = half_height - (avg * graph_range);
        let text_content = format!("{}: {} - {}", name, clamped_rng, avg);
        ctx.fill_text(&text_content, 10., 10. * color_counter as f64).expect("error drawing text");

        for value in arr {
            let y = graph_range * value + graph_offset;
            ctx.line_to(counter_x as f64 * px_per_step, y);
            // IDK how to get iterator, I got no internet now lol.
            counter_x += 1;
        }
        ctx.stroke();

    };

    let mut draw_value = |name: &str, value: f64| {
        let text_content = format!("{}= {}", name, value);
        ctx.fill_text(&text_content, 10., 50. as f64).expect("error drawing text");

    };

    for (name, history) in &memory.history {

        if (!plot_only.contains(name)) {
            continue;
        }

        // plotty(&name, &history.x);
        // plotty(&name, &history.y);
        // plotty(&name, &history.center_short);
        // plotty(&name, &history.movement_size);
        plotty(&name, &history.beat_sawtooth);
        draw_value(&"bpf", history.beats_per_frame);

        
        // log value of px_per_step
        // log_1(&px_per_step.into());
    }
}
