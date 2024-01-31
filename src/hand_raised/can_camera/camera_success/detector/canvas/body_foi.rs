/**   body features of interest */
use aspect_fit::size::Size;
use geo::{coord, Coord};
use js_sys::{Array, Float64Array, Math::sqrt};
use std::{collections::HashMap, f64::consts::PI, vec};
use wasm_tensorflow_models_pose_detection::pose::Pose;
use web_sys::{console::log, console::log_1, CanvasRenderingContext2d};

pub const POINT_HISTORY_LENGTH: usize = 128;

/** todo: maybe use linked lists instead of arrays? because of all the rotating*/
pub struct KeypointHistory {
    pub name: String,
    pub x: [f64; POINT_HISTORY_LENGTH],
    pub y: [f64; POINT_HISTORY_LENGTH],
    pub deltaxy: [f64; POINT_HISTORY_LENGTH],
    //** center of movement, for short time period */
    pub lowpassed_x: [f64; POINT_HISTORY_LENGTH],
    pub lowpassed_y: [f64; POINT_HISTORY_LENGTH],
    //** detected beats */
    pub movement_size: [f64; POINT_HISTORY_LENGTH],
    // other ideas:
    // transient 1st derivate: how fast movement change
    // transient 2nd derivate: how fast acceleration changed (detect air drums)
    pub beats_per_frame: f64,
    pub beat_sawtooth: [f64; POINT_HISTORY_LENGTH],
    pub last_beat_time_ago: u64,
}
pub struct FoiMem {
    // pub left_wrist: KeypointHistory,
    pub history: HashMap<String, KeypointHistory>,
}

#[derive(Debug)]
pub struct RangeAndAverage {
    pub range: f64,
    pub average: f64,
}

pub fn range_and_average(arr: &[f64]) -> RangeAndAverage {
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
    RangeAndAverage {
        range: max - min,
        average: sum / arr.len() as f64,
    }
}
pub fn range(arr: &[f64]) -> f64 {
    let mut min = arr[0];
    let mut max = arr[0];
    for value in arr {
        if *value < min {
            min = *value;
        }
        if *value > max {
            max = *value;
        }
    }
    max - min
}

pub fn average(arr: &[f64]) -> f64 {
    arr.iter().sum::<f64>() / arr.len() as f64
}
pub fn boxcar(now: &f64, prev: &f64, weight_new: f64) -> f64 {
    now * weight_new + prev * (1. - weight_new)
}

fn draw_circle(ctx: &CanvasRenderingContext2d, x: f64, y: f64, radius: f64, color: &str) {
    ctx.begin_path();
    ctx.set_fill_style(&color.into());
    ctx.arc(x, y, radius, 0 as f64, (2 as f64) * PI)
        .expect("foi drawing point");
    ctx.fill();
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
                    deltaxy: [0.; POINT_HISTORY_LENGTH],
                    lowpassed_x: [0.; POINT_HISTORY_LENGTH],
                    lowpassed_y: [0.; POINT_HISTORY_LENGTH],
                    movement_size: [0.; POINT_HISTORY_LENGTH],
                    beats_per_frame: 0.,
                    beat_sawtooth: [0.; POINT_HISTORY_LENGTH],
                    last_beat_time_ago: 0,
                },
            );
        } else {
            let mut history = memory
                .history
                .get_mut(&keypoint.name.clone().unwrap())
                .unwrap();

            history.x.rotate_right(1);
            history.y.rotate_right(1);
            history.lowpassed_x.rotate_right(1);
            history.lowpassed_y.rotate_right(1);
            history.movement_size.rotate_right(1);
            history.beat_sawtooth.rotate_right(1);
            
            // todo: ignore aswell if the keypoint is outside of camera
            match keypoint.score {
                Some(score) => if score < 0.7 { 
                    history.x[0] = 0.;
                    history.y[0] = 0.;
                    history.lowpassed_x[0] = 0.;
                    history.lowpassed_y[0] = 0.;
                    history.movement_size[0] = 0.;
                    history.beat_sawtooth[0] = 0.;
                    continue;
                 } ,
                _ => continue,
            }


            let range = coord! {
                x: range(&history.x),
                y: range(&history.y),
            };

            let avg = coord! {
                x: average(&history.x),
                y: average(&history.y),
            };

            history.x[0] = keypoint.x;
            history.y[0] = keypoint.y;

            history.lowpassed_x[0] = boxcar(&keypoint.x, &history.lowpassed_x[1], 0.1);
            history.lowpassed_y[0] = boxcar(&keypoint.y, &history.lowpassed_y[1], 0.1);

            let delta = coord! {
                x: keypoint.x - history.lowpassed_x[0],
                y: keypoint.y - history.lowpassed_y[0],
            };

            history.movement_size[0] = sqrt(delta.x * delta.x + delta.y * delta.y);

            let hysteresis: f64 = 5.;

            history.beat_sawtooth[0] = if delta.x > hysteresis {
                history.last_beat_time_ago = 0;
                1.
            } else {
                history.last_beat_time_ago += 1;
                (history.beat_sawtooth[1] - 0.1).max(-1.)
            };
            // but lowpass and dont take the last beat into account
            history.beats_per_frame = 1. / history.last_beat_time_ago as f64;

            ctx.begin_path();

            draw_circle(ctx, keypoint.x, keypoint.y, 3., "blue");
            draw_circle(
                ctx,
                history.lowpassed_x[0],
                history.lowpassed_y[0],
                3.,
                "red",
            );
            draw_circle(ctx, avg.x, avg.y, 3., "green");
            draw_circle(
                ctx,
                delta.x + keypoint.x,
                delta.y + keypoint.y,
                3.,
                "orange",
            );

            ctx.fill();

            // draw y moving along x
            ctx.set_stroke_style(&"#cde6".into());
            ctx.begin_path();
            // plot history x,y points
            for (x, y) in history.x.iter().zip(history.y.iter()) {
                if *x == 0. && *y == 0. {
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
