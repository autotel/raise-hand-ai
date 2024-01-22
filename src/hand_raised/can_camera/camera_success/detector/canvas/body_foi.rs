/**   body features of interest */
use aspect_fit::size::Size;
use js_sys::{Float64Array, Array};
use std::f64::consts::PI;
use web_sys::{CanvasRenderingContext2d, console::log_1, console::log };
use wasm_tensorflow_models_pose_detection::pose::Pose;

pub struct FoiMem {
    pub past_values: [f64;32],
}

pub fn draw(
    ctx: &CanvasRenderingContext2d,
    pose: &Pose,
    view_size: Size<u32>,
    memory:  &mut FoiMem,
) {

    ctx.clear_rect(0 as f64, 0 as f64, view_size.width as f64, view_size.height as f64);    
    let draw_point = pose.keypoints.iter().find(|keypoint| {
        keypoint.name == Some("left_wrist".into())
            // || keypoint.name == Some("right_wrist".into())
    }).unwrap();

    ctx.set_fill_style(&"blue".into());
    ctx.begin_path();

    ctx.arc(
        draw_point.x,
        draw_point.y,
        // (1. - point_z) * 4.,
        3.,
        0 as f64,
        (2 as f64) * PI,
    ).unwrap();
    ctx.fill();
    ctx.close_path();

    memory.past_values.rotate_right(1);
    memory.past_values[0] = draw_point.x;
    // how to console.log:
    // let str = format!("{:?}",memory.past_values);
    // log_1(&str.into());}
    ctx.begin_path();
    ctx.set_stroke_style(&"blue".into());
    ctx.move_to(draw_point.x, draw_point.y);
    let mut counter = 0;
    for  value in memory.past_values {
        ctx.line_to(value, (counter as f64) + draw_point.y);
        // IDK how to get iterator, I got no internet now lol.
        counter += 1;
    }
    ctx.stroke();

}
