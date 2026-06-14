// SPDX-License-Identifier: MIT
// Copyright 2026 IROX Contributors
//

use egui::load::SizedTexture;
use egui::{Context, ImageData, ImageSource, Response, TextureHandle, TextureOptions, Ui};
use irox_imagery::BoxedImage;
use std::sync::Arc;

pub struct Image {
    pub data: BoxedImage,
    handle: TextureHandle,
}

impl Image {
    pub fn new<T: AsRef<str>>(name: T, data: BoxedImage, ctx: &Context) -> Self {
        let name = name.as_ref().to_string();

        let image = ImageData::Color(Arc::new((&data).into()));

        let handle = ctx.load_texture(name, image, TextureOptions::default());
        Self { data, handle }
    }
    pub fn show(&self, ui: &mut Ui) -> Response {
        ui.image(ImageSource::Texture(SizedTexture::from_handle(
            &self.handle,
        )))
    }
}
