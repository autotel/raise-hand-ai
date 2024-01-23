/**   body features of interest */
use aspect_fit::size::Size;
use js_sys::{Array, Float64Array};
use std::{collections::HashMap, f64::consts::PI};
use wasm_tensorflow_models_pose_detection::pose::Pose;
use web_sys::{console::log, console::log_1, CanvasRenderingContext2d};

pub const POINT_HISTORY_LENGTH: usize = 128;

pub struct KeypointHistory {
    pub name: String,
    pub x: [f64; POINT_HISTORY_LENGTH],
    pub y: [f64; POINT_HISTORY_LENGTH],
    /** angle deltas */
    pub ad: [f64; POINT_HISTORY_LENGTH],
}
pub struct FoiMem {
    // pub left_wrist: KeypointHistory,
    pub history: HashMap<String, KeypointHistory>,
}

pub fn draw(
    scale: &f64,
    ctx: &CanvasRenderingContext2d,
    pose: &Pose,
    view_size: Size<u32>,
    memory: &mut FoiMem,
) {

    let scale = scale.clone();
    ctx.scale(scale, scale).expect("foi scaling to client size");

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

    for keypoint in pose.keypoints.iter() {
        let exists = memory.history.contains_key(&keypoint.name.clone().unwrap());
        if !exists {
            memory.history.insert(
                keypoint.name.clone().unwrap(),
                KeypointHistory {
                    name: keypoint.name.clone().unwrap(),
                    x: [0.; POINT_HISTORY_LENGTH],
                    y: [0.; POINT_HISTORY_LENGTH],
                    ad: [0.; POINT_HISTORY_LENGTH],
                },
            );
        } else {
            let mut history = memory
                .history
                .get_mut(&keypoint.name.clone().unwrap())
                .unwrap();
            history.x.rotate_right(1);
            history.x[0] = keypoint.x;
            history.y.rotate_right(1);
            history.y[0] = keypoint.y;
            history.ad[0] = 0.;
            // draw y moving along x
            ctx.begin_path();
            ctx.set_stroke_style(&"#cde6".into());
            ctx.begin_path();
            // ctx.move_to(keypoint.x, keypoint.y);
            // plot history x,y points
            for (x, y) in history.x.iter().zip(history.y.iter()) {
                if(*x == 0. && *y == 0.) {
                    continue;
                }
                ctx.line_to(*x, *y);
            }
            ctx.stroke();
        }
    }
    ctx.reset_transform().expect("foi caling back");
}

// how to console.log:
// let str = format!("{:?}",memory.left_wrist.x);
// log_1(&str.into());}
