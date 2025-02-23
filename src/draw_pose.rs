use std::f64::consts::PI;

use average::Mean;
use coloriz::{Gradient, RGB};
use js_sys::Array;
use wasm_bindgen::JsValue;
use wasm_tensorflow_models_pose_detection::{model::Model, pose::Pose, util::get_adjacent_pairs};
use web_sys::CanvasRenderingContext2d;

fn max(a: f64, b: f64) -> f64 {
    if a > b {
        a
    } else {
        b
    }
}

fn rgb_to_js_value(rgb: &RGB) -> JsValue {
    format!("rgb({}, {}, {})", rgb.r, rgb.g, rgb.b).into()
}

pub fn draw_pose(
    scale: &f64,
    ctx: &CanvasRenderingContext2d,
    min_point_score: f64,
    pose: &Pose,
    model: &Model,
) {
    let worst_score_color = RGB::new(255, 0, 0);
    let best_score_color = RGB::new(200, 210, 255);
    let gradient = Gradient::new(worst_score_color, best_score_color);

    let canvas = ctx.canvas().unwrap();

    ctx.clear_rect(
        0 as f64,
        0 as f64,
        canvas.width() as f64,
        canvas.height() as f64,
    );
    let scale = scale.clone();
    ctx.scale(scale, scale)
        .expect("poses scaling to client size");

    for point in &pose.keypoints {
        if point.score.map_or(true, |score| score >= min_point_score) {
            ctx.set_fill_style(&rgb_to_js_value(
                &gradient.at(point.score.as_ref().unwrap().clone() as f32),
            ));
            ctx.begin_path();
            let mut point_z: f64 = (point.z.unwrap_or(5 as f64));
            ctx.arc(
                point.x,
                point.y,
                max(0 as f64, (1. - point_z) * 4.),
                // 6.,
                0 as f64,
                (2 as f64) * PI,
            )
            .unwrap();
            ctx.fill();
        }
    }

    for (a, b) in get_adjacent_pairs(model.clone()) {
        let point_a = &pose.keypoints[a as usize];
        let point_b = &pose.keypoints[b as usize];
        if point_a.score.map_or(true, |score| score > min_point_score)
            && point_b.score.map_or(true, |score| score > min_point_score)
        {
            ctx.set_line_dash(&Array::new()).unwrap();
            ctx.set_line_width(2 as f64);
            let score_for_color = vec![
                point_a.score.unwrap_or(1 as f64),
                point_b.score.unwrap_or(1 as f64),
            ]
            .into_iter()
            .collect::<Mean>()
            .mean();

            ctx.set_stroke_style(&rgb_to_js_value(&gradient.at(score_for_color as f32)));
            ctx.begin_path();
            ctx.move_to(point_a.x, point_a.y);
            ctx.line_to(point_b.x, point_b.y);
            ctx.stroke();
        }
    }
    ctx.reset_transform().expect("poses reset transform")
}
