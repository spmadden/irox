use egui::epaint::RectShape;
use egui::*;
use irox_egui_extras::drawpanel::DrawPanel;
use irox_egui_extras::fonts::{load_fonts, FontSet};
use irox_egui_extras::toolframe::{ToolApp, ToolFrame};
use log::error;

pub fn main() {
    let viewport = ViewportBuilder::default().with_inner_size(Vec2::new(1024., 800.));

    let native_options = eframe::NativeOptions {
        viewport,

        ..Default::default()
    };
    if let Err(e) = eframe::run_native(
        "draw-panels",
        native_options,
        Box::new(|cc| Ok(Box::new(ToolFrame::new(cc, Box::new(TestApp::new(cc)))))),
    ) {
        error!("{e:?}");
    };
}
pub struct TestApp {
    panel: DrawPanel,
    init: bool,
}
impl TestApp {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        load_fonts(FontSet::all(), &cc.egui_ctx);
        TestApp {
            panel: DrawPanel::default(),
            init: false,
        }
    }
    pub fn init(&mut self) {
        if self.init {
            return;
        }
        self.init = true;

        self.panel.shapes.push(Shape::Rect(RectShape::new(
            Rect::from_x_y_ranges(-50.0..=50.0, -50.0..=50.0),
            CornerRadius::ZERO,
            Color32::BLACK,
            Stroke::new(1.0, Color32::BLACK),
            StrokeKind::Middle,
        )));
    }
}
impl eframe::App for TestApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            self.panel.show(ui);
        });
    }
}

impl ToolApp for TestApp {}
