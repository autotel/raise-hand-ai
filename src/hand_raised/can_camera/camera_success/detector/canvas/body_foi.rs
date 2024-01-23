/**   body features of interest */
use aspect_fit::size::Size;
use js_sys::{Array, Float64Array};
use std::f64::consts::PI;
use wasm_tensorflow_models_pose_detection::pose::Pose;
use web_sys::{console::log, console::log_1, CanvasRenderingContext2d};

pub const POINT_HISTORY_LENGTH: usize = 128;

pub struct KeypointHistory {
    pub x: [f64; POINT_HISTORY_LENGTH],
    pub y: [f64; POINT_HISTORY_LENGTH],
    /** angle deltas */
    pub ad: [f64; POINT_HISTORY_LENGTH],
}
pub struct FoiMem {
    pub left_wrist: KeypointHistory,
}

pub fn draw(
    ctx: &CanvasRenderingContext2d,
    pose: &Pose,
    view_size: Size<u32>,
    memory: &mut FoiMem,
) {
    ctx.clear_rect(
        0 as f64,
        0 as f64,
        view_size.width as f64,
        view_size.height as f64,
    );


    ctx.set_fill_style(&"#000c".into());
    // ctx.fill_rect(
    //     0 as f64,
    //     0 as f64,
    //     view_size.width as f64,
    //     view_size.height as f64,
    // );
    

    let draw_point = pose
        .keypoints
        .iter()
        .find(|keypoint| {
            keypoint.name == Some("left_wrist".into())
            // || keypoint.name == Some("right_wrist".into())
        })
        .unwrap();

    ctx.set_fill_style(&"blue".into());
    ctx.begin_path();

    ctx.arc(
        draw_point.x,
        draw_point.y,
        // (1. - point_z) * 4.,
        3.,
        0 as f64,
        (2 as f64) * PI,
    )
    .unwrap();
    ctx.fill();
    ctx.close_path();

    memory.left_wrist.x.rotate_right(1);
    memory.left_wrist.x[0] = draw_point.x;
    ctx.begin_path();
    ctx.set_stroke_style(&"blue".into());
    ctx.move_to(draw_point.x, draw_point.y);
    let mut counter = 0;
    for value in memory.left_wrist.x {
        ctx.line_to(value, (counter as f64) + draw_point.y);
        // IDK how to get iterator, I got no internet now lol.
        counter += 1;
    }
    ctx.stroke();

    memory.left_wrist.y.rotate_right(1);
    memory.left_wrist.y[0] = draw_point.y;
    ctx.begin_path();
    ctx.set_stroke_style(&"red".into());
    ctx.move_to(draw_point.x, draw_point.y);
    let mut counter = 0;
    for value in memory.left_wrist.y {
        ctx.line_to((counter as f64) + draw_point.x, value);
        // IDK how to get iterator, I got no internet now lol.
        counter += 1;
    }
    ctx.stroke();
}

// how to console.log:
// let str = format!("{:?}",memory.left_wrist.x);
// log_1(&str.into());}
