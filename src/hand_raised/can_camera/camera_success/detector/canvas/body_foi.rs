/**   body features of interest */
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

use crate::{
    draw_poses::draw_poses,
    flip_horizontal::flip_horizontal,
    rect_in_sliced_circle::{rect_in_sliced_circle, Ratio, Slice},
    side_maps::SIDE_MAPS,
};

use wasm_tensorflow_models_pose_detection::pose::Pose;

pub struct ScreenSize {
    pub width: u32,
    pub height: u32,
}

pub fn draw(
    ctx: &CanvasRenderingContext2d,
    pose: &Pose,
    detector_size: Size<u32>,
    view_size: Size<u32>,
) {

    ctx.clear_rect(0 as f64, 0 as f64, view_size.width as f64, view_size.height as f64);    
    let draw_point = pose.keypoints.iter().find(|keypoint| {
        keypoint.name == Some("left_wrist".into())
            // || keypoint.name == Some("right_wrist".into())
    }).unwrap();

    // if(draw_point.is_none()) {
    //     return;
    // }
    let reset = ctx.reset_transform();
    ctx.set_fill_style(&"blue".into());
    ctx.begin_path();

    ctx.arc(
        draw_point.x,
        draw_point.y,
        // (1. - point_z) * 4.,
        6.,
        0 as f64,
        (2 as f64) * PI,
    )
    .unwrap();
    ctx.fill();
}
