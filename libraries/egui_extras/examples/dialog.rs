// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

#![allow(clippy::print_stdout)]

use irox_egui_extras::pwdlg::{dialog_options, DialogType, InteractionType};

fn main() {
    let resp = dialog_options(|opts| {
        opts.title = String::from("Dialog Title");
        opts.details.prompt = Some(String::from("Password: "));
        opts.details.dialog_type = DialogType::Password;
        opts.details.interaction_type = InteractionType::OkCancel;
    });

    println!("{resp:?}");
}
