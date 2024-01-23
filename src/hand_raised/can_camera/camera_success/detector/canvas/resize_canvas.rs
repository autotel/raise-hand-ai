use aspect_fit::{aspect_fit::aspect_fit, scale_size::scale_size, size::Size};
use wasm_bindgen::UnwrapThrowExt;
use web_sys::console::{log, log_1};

use super::resize_canvas_input::ResizeCanvasInput;

pub fn resize_canvas(input: &ResizeCanvasInput) {
    let video = input.video_ref.current().unwrap();
    let container = input.container_ref.current().unwrap();

    let video_source_size = Size {
        width: video.video_width(),
        height: video.video_height(),
    };

    let target_width = container.client_width() as u32;
    let target_height = container.client_height() as u32;

    // let str: String = format!("{:?}, {:?}", video_source_size.width, video_source_size.height);
    // log_1(&str.into());

    if video_source_size.width > 0 && video_source_size.height > 0 {
        for canvas in input.resize_targets.iter() {
            let canvas = canvas.current().unwrap();

            let attribute_proportion = canvas
                .get_attribute("data-proportion")
                .unwrap_or("0".into());

            let parsed_proportion = attribute_proportion.parse::<f64>().unwrap_or(0.);
            if parsed_proportion == 0. {
                canvas.set_width(target_width);
                canvas.set_height(target_height);
            } else {
                let new_width = target_width;
                let new_height = (new_width as f64 * parsed_proportion) as u32;

                canvas.set_width(new_width);
                canvas.set_height(new_height);
            }
        }
    }
}
