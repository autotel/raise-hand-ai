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

use super::body_foi::{self, FoiMem};

struct Config {
    pub show_threshold_line: bool,
    pub show_key_points: bool,
    pub show_reach_circle: bool,
    pub show_reach_box: bool,
    pub show_pointer_on_screen: bool,
    pub threshold: f64,
}

static CONFIG: Config = Config {
    show_threshold_line: false,
    show_key_points: true,
    show_reach_box: true,
    show_reach_circle: false,
    show_pointer_on_screen: false,
    threshold: 0.75,
};

pub async fn detector_frame(
    video: &HtmlVideoElement,
    canvas: &HtmlCanvasElement,
    container: &HtmlDivElement,
    pointer_canvas: &HtmlCanvasElement,
    detector: &PoseDetector,
    model: &Model,
    memory: &mut FoiMem,
) {
    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();
    let pointer_ctx = pointer_canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    let video_to_canvas_scale = canvas.width() as f64 / video.video_width() as f64 ;

    // VERY IMPORTANT: estimating poses before the video plays results in the error
    // RuntimeError: Aborted(native code called abort(). To avoid this error, just await video.play().
    JsFuture::from(video.play().unwrap()).await.unwrap();
    let poses = {
        let mut poses = detector
            .estimate_poses(
                &video.dyn_ref().unwrap(),
                EstimationConfig::BlazePoseOrMoveNet(CommonEstimationConfig {
                    flip_horizontal: Some(false),
                    max_poses: None,
                }),
                None,
            )
            .await
            .unwrap();
        // flip_horizontal(&mut poses, video.video_width() as f64);
        poses
    };

    if CONFIG.show_key_points {
        draw_poses(&video_to_canvas_scale, &ctx, 0.3, 0.3, &poses, model);
    }

    for pose in poses {
        body_foi::draw(
            &video_to_canvas_scale,
            &pointer_ctx,
            &pose,
            Size {
                width: video.video_width(),
                height: video.video_height(),
            },
            memory,
        );
    }
}
