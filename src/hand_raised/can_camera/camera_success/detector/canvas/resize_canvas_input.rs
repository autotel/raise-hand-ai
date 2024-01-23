use wasm_react::hooks::JsRefContainer;
use web_sys::{HtmlCanvasElement, HtmlDivElement, HtmlVideoElement};

#[derive(Clone)]
pub struct ResizeCanvasInput {
    pub video_ref: JsRefContainer<HtmlVideoElement>,
    pub resize_targets: [JsRefContainer<HtmlCanvasElement>; 3],
    pub container_ref: JsRefContainer<HtmlDivElement>,
}
