use eframe::egui;
use eframe::web::AppRunnerRef;

// fn main() {
//     let native_options = eframe::NativeOptions::default();
//     eframe::run_native("My egui App", native_options, Box::new(|cc| Box::new(MyEguiApp::new(cc))));
// }

#[derive(Default)]
struct MyEguiApp {}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
       egui::CentralPanel::default().show(ctx, |ui| {
           ui.heading("Hello World!");
       });
   }
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Call this once from the HTML.
// #[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
#[cfg(target_arch = "wasm32")]
pub fn start(canvas_id: &str) -> Result<AppRunnerRef, eframe::wasm_bindgen::JsValue> {
    let web_options = eframe::WebOptions::default();
    eframe::start_web(canvas_id, web_options, Box::new(|cc| Box::new(MyEguiApp::new(cc))))
}