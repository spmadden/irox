// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use irox_dot::drawing::app::{App, InitData};
use irox_egui_extras::composite::CompositeApp;
use irox_egui_extras::eframe;
use irox_egui_extras::eframe::wasm_bindgen::{self, prelude::*};
use irox_egui_extras::styles::StylePersistingApp;
use irox_egui_extras::toolframe::{ToolFrame, ToolFrameOptions};
use irox_log::log;
use std::ops::Deref;

#[derive(Clone)]
#[wasm_bindgen]
pub struct WebHandle {
    runner: eframe::WebRunner,
}
#[wasm_bindgen]
impl WebHandle {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let log_level = if cfg!(debug_assertions) {
            log::LevelFilter::Trace
        } else {
            log::LevelFilter::Info
        };
        eframe::WebLogger::init(log_level).ok();

        Self {
            runner: eframe::WebRunner::new(),
        }
    }
    /// Call this once from JavaScript to start your app.
    #[wasm_bindgen]
    pub async fn start(
        &self,
        doturl: &str,
        scale: f32,
        pos: Box<[f32]>,
        canvas: web_sys::HtmlCanvasElement,
    ) -> Result<(), wasm_bindgen::JsValue> {
        let req = ehttp::Request::get(doturl);
        let data = ehttp::fetch_async(req).await?;
        if !data.ok {
            return Err(wasm_bindgen::JsValue::from_str(&format!(
                "Error loading data: {}/{} for url: {doturl}",
                data.status, data.status_text
            )));
        }
        let data = data.bytes;
        if data.is_empty() {
            return Err(wasm_bindgen::JsValue::from_str(&format!(
                "Response for url {doturl} was empty."
            )));
        }
        let [x, y] = pos.deref() else {
            return Err(wasm_bindgen::JsValue::from_str(
                "Need exactly two values for position",
            ));
        };

        let init = InitData {
            dotjsondata: data,
            scale,
            pos: [*x, *y],
        };
        self.runner
            .start(
                canvas,
                eframe::WebOptions::default(),
                Box::new(|cc| {
                    let mut comp = CompositeApp::default();
                    comp.add(Box::new(StylePersistingApp::new(cc)));
                    comp.add(Box::new(ToolFrame::new_opts(
                        cc,
                        Box::new(App::new(cc, init)),
                        ToolFrameOptions {
                            disable_file_menu: true,
                            ..Default::default()
                        },
                    )));
                    Ok(Box::new(comp))
                }),
            )
            .await
    }

    #[wasm_bindgen]
    pub fn destroy(&self) {
        self.runner.destroy();
    }
    /// The JavaScript can check whether or not your app has crashed:
    #[wasm_bindgen]
    pub fn has_panicked(&self) -> bool {
        self.runner.has_panicked()
    }

    #[wasm_bindgen]
    pub fn panic_message(&self) -> Option<String> {
        self.runner.panic_summary().map(|s| s.message())
    }

    #[wasm_bindgen]
    pub fn panic_callstack(&self) -> Option<String> {
        self.runner.panic_summary().map(|s| s.callstack())
    }
}
