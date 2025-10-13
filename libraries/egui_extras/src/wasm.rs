// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

#![allow(clippy::mem_forget)] // False positives from #[wasm_bindgen] macro

use eframe::wasm_bindgen::{self, prelude::*};
use eframe::{App, CreationContext, Frame};
use egui::{CentralPanel, Context};

type DynError = Box<dyn std::error::Error + Send + Sync>;
pub type AppCreator<'app> =
    Box<dyn 'app + FnOnce(&CreationContext<'_>) -> Result<Box<dyn 'app + App>, DynError>>;
macro_rules! generate_web_handle {
    ($name:ident, $($init:tt)+) => {
        /// Our handle to the web app from JavaScript.
        #[wasm_bindgen]
        pub struct $name {
            runner: eframe::WebRunner,
        }

        #[wasm_bindgen]
        impl $name {
            /// Installs a panic hook, then returns.
            #[allow(clippy::new_without_default, clippy::allow_attributes)]
            #[wasm_bindgen(constructor)]
            pub fn new() -> Self {
                // Redirect [`log`] message to `console.log` and friends:
                let log_level = if cfg!(debug_assertions) {
                    log::LevelFilter::Trace
                } else {
                    log::LevelFilter::Debug
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
                canvas: web_sys::HtmlCanvasElement,
            ) -> Result<(), wasm_bindgen::JsValue> {
                self.runner
                    .start(
                        canvas,
                        eframe::WebOptions::default(),
                        $($init)+
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
    };
}
struct TestApp {}
impl TestApp {
    pub fn new(_cc: &CreationContext) -> Self {
        Self {}
    }
}
impl eframe::App for TestApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| ui.label("it worked!"));
    }
}
generate_web_handle!(TestHandle, Box::new(|cc| Ok(Box::new(TestApp::new(cc)))));
