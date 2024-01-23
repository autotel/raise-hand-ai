use std::{rc::Rc, collections::HashMap};

use fps_counter::FPSCounter;
use js_sys::Reflect;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use wasm_react::{
    create_element, h,
    hooks::{use_effect, use_js_ref, use_state, Deps},
    props::{Props, Style},
    Component, VNode,
};
use wasm_repeated_animation_frame::RafLoop;
use wasm_tensorflow_models_pose_detection::model::Model;
use web_sys::{
    console::log_1, HtmlCanvasElement, HtmlDivElement, HtmlVideoElement, MediaStreamTrack,
};

use crate::{
    get_set::GetSet,
    hand_raised::{camera_data::CameraData, can_camera::use_detector::DetectorData},
    use_future::FutureState,
};

use self::{
    body_foi::{FoiMem, KeypointHistory, POINT_HISTORY_LENGTH},
    detector_frame::detector_frame,
    resize_canvas_input::ResizeCanvasInput,
    use_play_promise_and_auto_resize_canvas::use_play_promise_and_auto_resize_canvas,
};

mod body_foi;
mod detector_frame;
mod plots_frame;
mod resize_canvas;
mod resize_canvas_input;
mod use_play_promise_and_auto_resize_canvas;

pub struct Canvas<G0: GetSet<Option<String>>, G1: GetSet<Model>> {
    pub camera_data: Rc<CameraData<G0>>,
    pub detector: Rc<DetectorData<G1>>,
}

impl<G0: GetSet<Option<String>> + 'static, G1: GetSet<Model> + 'static> Component
    for Canvas<G0, G1>
{
    fn render(&self) -> VNode {
        let mut foi_mem = FoiMem {
            history: HashMap::new(),
        };

        let container_ref = use_js_ref::<HtmlDivElement>(None);
        let video_ref = use_js_ref(None::<HtmlVideoElement>);
        let canvas_container_ref = use_js_ref(None::<HtmlDivElement>);
        let canvas_skeleton_ref = use_js_ref(None::<HtmlCanvasElement>);
        let canvas_poi_ref = use_js_ref(None::<HtmlCanvasElement>);
        let canvas_plot_ref = use_js_ref(None::<HtmlCanvasElement>);

        let play_future_state = use_play_promise_and_auto_resize_canvas(
            ResizeCanvasInput {
                resize_targets: [
                    canvas_skeleton_ref.clone(),
                    canvas_poi_ref.clone(),
                    canvas_plot_ref.clone(),
                ],
                container_ref: canvas_container_ref.clone(),
                video_ref: video_ref.clone(),
            },
            self.camera_data
                .clone()
                .video_promise
                .as_ref()
                .get_result()
                .unwrap()
                .as_ref()
                .unwrap()
                .clone()
                .dyn_into()
                .unwrap(),
        );

        let media_stream = self
            .camera_data
            .video_promise
            .as_ref()
            .get_result()
            .unwrap()
            .as_ref()
            .unwrap();

        let fps = use_state(|| None::<f64>);

        use_effect(
            {
                let video_ref = video_ref.clone();
                let canvas_skeleton_ref = canvas_skeleton_ref.clone();
                let canvas_plot_ref = canvas_plot_ref.clone();
                let canvas_container_ref = canvas_container_ref.clone();
                let canvas_poi_ref = canvas_poi_ref.clone();
                let model_state = self.detector.model.clone();
                let detector = self
                    .detector
                    .create_detector
                    .as_ref()
                    .get_result()
                    .unwrap()
                    .as_ref()
                    .unwrap()
                    .clone();
                let mut fps = fps.clone();

                move || {
                    let video = video_ref.current().unwrap();
                    let canvas = canvas_skeleton_ref.current().unwrap();
                    let canvas_container = canvas_container_ref.current().unwrap();
                    let pointer_canvas = canvas_poi_ref.current().unwrap();
                    let plots_canvas = canvas_plot_ref.current().unwrap();

                    let (mut raf_loop, canceler) = RafLoop::new();
                    spawn_local(async move {
                        let mut fps_counter = FPSCounter::new();
                        loop {
                            if !raf_loop.next().await {
                                log_1(&"break loop".into());
                                break;
                            };

                            let model = model_state.get().clone();
                            detector_frame(
                                &video,
                                &canvas,
                                &canvas_container,
                                &pointer_canvas,
                                &detector,
                                &model,
                                &mut foi_mem,
                            )
                            .await;

                            plots_frame::plots_frame(&plots_canvas, &mut foi_mem).await;

                            fps.set(|_| Some(fps_counter.tick() as f64));
                        }
                    });
                    move || {
                        spawn_local(async move {
                            canceler.cancel().await;
                            log_1(&"stopped raf loop".into());
                        });
                        log_1(&"stop raf loop".into());
                    }
                }
            },
            Deps::none(),
        );

        let fps = *fps.value();

        create_element(
            &"div".into(),
            &Props::new()
                .key(Some("container"))
                .ref_container(&container_ref)
                .insert(
                    "style",
                    &Style::new()
                        .flex_grow(1)
                        .display("flex")
                        .flex_direction("column")
                        .overflow("hidden")
                        .into(),
                ),
            (

                // FPS readout
                h!(div).style(&Style::new().display("flex")).build((
                    h!(span).style(&Style::new().flex_grow(1)).build((
                        VNode::from("Video FPS: "),
                        h!(code).build(
                            Reflect::get(
                                &media_stream
                                    .get_video_tracks()
                                    .get(0)
                                    .unchecked_into::<MediaStreamTrack>()
                                    .get_settings(),
                                &"frameRate".into(),
                            )
                            .unwrap()
                            .as_f64()
                            .unwrap()
                            .to_string(),
                        ),
                    )),
                    h!(span).style(&Style::new().flex_grow(1)).build((
                        VNode::from("Pose detection FPS: "),
                        h!(code).build({
                            let v_node: VNode = match fps {
                                Some(fps) => fps.into(),
                                None => "Not started".into(),
                            };
                            v_node
                        }),
                    )),
                )),
                // video with canvas overlays
                create_element(
                    &"div".into(),
                    &Props::new()
                        .key(Some("videos_div"))
                        .ref_container(&canvas_container_ref)
                        .insert(
                            "style",
                            &Style::new()
                                .position("relative")
                                // .flex_grow(1)
                                .overflow("hidden")
                                .width("100vw")
                                // .height("100vh")
                                .into(),
                        ),
                    (

                        // video element
                        create_element(
                            &"video".into(),
                            &Props::new()
                                .key(Some("video"))
                                .ref_container(&video_ref)
                                .insert(
                                    "style",
                                    &Style::new()
                                        .width("100vw")
                                        .height("auto")
                                        .into(),
                                )
                                // .insert("hidden", &true.into())
                                ,
                            ().into(),
                        ),
                        // canvas drawing the skeleton
                        create_element(
                            &"canvas".into(),
                            &Props::new()
                                .key(Some("canvas_skeleton"))
                                .ref_container(&canvas_skeleton_ref)
                                .insert(
                                    "style",
                                    &Style::new()
                                        .position("absolute")
                                        .width("100%")
                                        .left(0)
                                        .top(0)
                                        .pointer_events("none")
                                        .into(),
                                ),
                            ().into(),
                        ),
                        // canvas drawing the points of interest
                        create_element(
                            &"canvas".into(),
                            &Props::new()
                                .key(Some("canvas_poi"))
                                .ref_container(&canvas_poi_ref)
                                .insert(
                                    "style",
                                    &Style::new()
                                        .position("absolute")
                                        .width("100%")
                                        .left(0)
                                        .top(0)
                                        .pointer_events("none")
                                        .into(),
                                ),
                            ().into(),
                        ),
                    )
                        .into(),
                ),
                match play_future_state {
                    FutureState::NotStarted => VNode::from("Will play video"),
                    FutureState::Pending => "Playing video".into(),
                    FutureState::Done(result) => match result {
                        Err(_) => "Error playing video".into(),
                        _ => ().into(),
                    },
                },
                // div with horizontal plots of variables
                create_element(
                    &"div".into(),
                    &Props::new()
                        .key(Some("plotters_div"))
                        // .ref_container(&canvas_container_ref)
                        .insert(
                            "style",
                            &Style::new()
                                .position("relative")
                                .into(),
                        ),
                    (create_element(
                        &"canvas".into(),
                        &Props::new()
                            .key(Some("canvas_plot"))
                            .ref_container(&canvas_plot_ref)
                            .insert(
                                "data-proportion",
                                &"0.3".into(),
                            )
                            .insert(
                                "style",
                                &Style::new()
                                    .left(0)
                                    .top(0)
                                    .width("100%")
                                    .border("solid 1px black")
                                    .pointer_events("none")
                                    .into(),
                            ),
                        ().into(),
                    ),)
                        .into(),
                ),
            )
                .into(),
        )
    }
}
