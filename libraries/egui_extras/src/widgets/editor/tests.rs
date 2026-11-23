// SPDX-License-Identifier: MIT
// Copyright 2026 IROX Contributors
//

use super::{
    BooleanEditor, Editor, EditorResponse, Field, FieldDescriptorBuilder, RestrictedValue,
    StringListEditor, UniqueValuesEditor, ValueRelationEditor,
};
use egui::{pos2, vec2, CentralPanel, Context, Event, FullOutput, Key, Modifiers, Pos2, Rect, Ui};

fn field<T: Clone + Default>(value: Option<T>) -> Field<T> {
    Field {
        descriptor: FieldDescriptorBuilder::new(&"Field").build(),
        value,
    }
}

fn frame<R, F: FnMut(&mut Ui) -> R>(
    ctx: &Context,
    events: Vec<Event>,
    mut show: F,
) -> (R, FullOutput) {
    let mut result = None;
    let mut output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(pos2(0.0, 0.0), vec2(640.0, 480.0))),
            predicted_dt: 0.125,
            events,
            ..Default::default()
        },
        |ui| {
            CentralPanel::default().show(ui, |ui| {
                result = Some(show(ui));
            });
        },
    );
    output.textures_delta.clear();
    (result.unwrap(), output)
}

fn editor_frame<T, E>(
    ctx: &Context,
    events: Vec<Event>,
    editor: &E,
    field: &mut Field<T>,
) -> (EditorResponse, FullOutput)
where
    T: Clone + Default,
    E: for<'a> Editor<'a, Value = T>,
{
    frame(ctx, events, |ui| editor.show(field, ui))
}

fn text_positions<'a>(
    output: &'a FullOutput,
    label: &'a str,
) -> impl DoubleEndedIterator<Item = Pos2> + 'a {
    fn find_text(shape: &egui::epaint::Shape, label: &str) -> Option<Pos2> {
        match shape {
            egui::epaint::Shape::Text(text) if text.galley.text() == label => {
                Some(text.visual_bounding_rect().center())
            }
            egui::epaint::Shape::Vec(shapes) => {
                shapes.iter().find_map(|shape| find_text(shape, label))
            }
            _ => None,
        }
    }

    output
        .shapes
        .iter()
        .filter_map(move |shape| find_text(&shape.shape, label))
}

fn text_pos(output: &FullOutput, label: &str) -> Option<Pos2> {
    text_positions(output, label).next()
}

fn pointer_button(pos: Pos2, pressed: bool) -> Event {
    Event::PointerButton {
        pos,
        pressed,
        button: egui::PointerButton::Primary,
        modifiers: Modifiers::NONE,
    }
}

fn click(pos: Pos2) -> Vec<Event> {
    vec![
        Event::PointerMoved(pos),
        pointer_button(pos, true),
        pointer_button(pos, false),
    ]
}

fn key(key: Key) -> Event {
    Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: Modifiers::NONE,
    }
}

#[test]
fn boolean_toggles_from_default_and_can_clear_to_null() {
    let ctx = Context::default();
    let mut field = field(None);
    field.descriptor.default_value = true;
    let (result, _) = editor_frame(&ctx, vec![], &BooleanEditor, &mut field);
    assert!(result.valid);
    assert!(!result.response.changed());
    let checkbox = result.response.rect.center();
    let (result, _) = editor_frame(&ctx, click(checkbox), &BooleanEditor, &mut field);
    assert!(result.response.changed());
    assert_eq!(field.value, Some(false));

    let (result, output) = editor_frame(&ctx, click(checkbox), &BooleanEditor, &mut field);
    assert!(result.response.changed());
    assert_eq!(field.value, Some(true));

    let (result, _) = editor_frame(
        &ctx,
        click(text_pos(&output, "Clear").unwrap()),
        &BooleanEditor,
        &mut field,
    );
    assert!(result.response.changed());
    assert_eq!(field.value, None);

    field.descriptor.nullable = false;
    field.descriptor.enforce_not_null = true;
    let (result, output) = editor_frame(&ctx, vec![], &BooleanEditor, &mut field);
    assert!(!result.valid);
    assert!(!result.response.changed());
    assert!(text_pos(&output, "Clear").is_none());
}

#[test]
fn string_list_preserves_empty_array_and_edit_order() {
    let ctx = Context::default();
    let mut field = field(None);
    let mut show = |events| editor_frame(&ctx, events, &StringListEditor, &mut field);

    let (_, output) = show(vec![]);
    let (result, _) = show(click(text_pos(&output, "Add item").unwrap()));
    assert!(result.response.changed());
    let (_, output) = show(vec![]);
    let (result, output) = show(click(text_pos(&output, "Remove").unwrap()));
    assert!(result.response.changed());
    assert!(text_pos(&output, "NULL").is_none());
    assert_eq!(field.value, Some(vec![]));

    field.value = Some(["first", "", "first"].map(str::to_owned).to_vec());
    let (_, output) = editor_frame(&ctx, vec![], &StringListEditor, &mut field);
    let (result, _) = editor_frame(
        &ctx,
        click(text_pos(&output, "first").unwrap()),
        &StringListEditor,
        &mut field,
    );
    assert!(!result.response.changed());
    let (result, output) = editor_frame(
        &ctx,
        vec![key(Key::End), Event::Text(" edited".to_owned())],
        &StringListEditor,
        &mut field,
    );
    assert!(result.response.changed());
    assert_eq!(
        field.value,
        Some(["first edited", "", "first"].map(str::to_owned).to_vec())
    );

    let (result, _) = editor_frame(
        &ctx,
        click(text_pos(&output, "Clear").unwrap()),
        &StringListEditor,
        &mut field,
    );
    assert!(result.response.changed());
    assert_eq!(field.value, None);
}

#[test]
fn relation_preserves_missing_keys_and_stores_keys_instead_of_labels() {
    let ctx = Context::default();
    let mut field = field(Some(205));
    let first_page = [(101, "Central")];
    let (result, output) = editor_frame(
        &ctx,
        vec![],
        &ValueRelationEditor::new(&first_page),
        &mut field,
    );
    assert!(!result.valid);
    assert!(!result.response.changed());
    assert_eq!(field.value, Some(205));
    assert!(text_pos(&output, "Unavailable: 205").is_some());

    let all_rows = [(205, "North"), (101, "Central")];
    let relation = ValueRelationEditor::new(&all_rows);
    let (result, output) = editor_frame(&ctx, vec![], &relation, &mut field);
    assert!(result.valid);
    assert!(!result.response.changed());
    assert!(text_pos(&output, "North").is_some());
    let dropdown = result.response.rect.center();

    editor_frame(&ctx, click(dropdown), &relation, &mut field);
    let (_, output) = editor_frame(&ctx, vec![], &relation, &mut field);
    let central = text_pos(&output, "Central").unwrap();
    let north = text_positions(&output, "North").next_back().unwrap();
    assert!(north.y < central.y);
    let (result, _) = editor_frame(&ctx, click(central), &relation, &mut field);
    assert!(result.valid);
    assert!(result.response.changed());
    assert_eq!(field.value, Some(101));

    let (result, _) = editor_frame(&ctx, vec![], &relation, &mut field);
    editor_frame(
        &ctx,
        click(result.response.rect.center()),
        &relation,
        &mut field,
    );
    let (_, output) = editor_frame(&ctx, vec![], &relation, &mut field);
    let current_option = text_positions(&output, "Central").next_back().unwrap();
    let (result, _) = editor_frame(&ctx, click(current_option), &relation, &mut field);
    assert!(result.valid);
    assert!(!result.response.changed());
    assert_eq!(field.value, Some(101));
    assert!(!egui::ComboBox::is_open(&ctx, result.response.id));

    let (result, _) = editor_frame(&ctx, vec![], &relation, &mut field);
    editor_frame(
        &ctx,
        click(result.response.rect.center()),
        &relation,
        &mut field,
    );
    let (_, output) = editor_frame(&ctx, vec![], &relation, &mut field);
    let (result, _) = editor_frame(
        &ctx,
        click(text_pos(&output, "NULL").unwrap()),
        &relation,
        &mut field,
    );
    assert!(result.valid);
    assert!(result.response.changed());
    assert_eq!(field.value, None);
}

#[test]
fn restricted_unique_values_flag_unknown_text() {
    let ctx = Context::default();
    let mut field = field(Some("custom".to_owned()));
    let known = ["Maple Avenue".to_owned()];
    let (result, _) = editor_frame(
        &ctx,
        vec![],
        &UniqueValuesEditor::restricted(&known),
        &mut field,
    );
    assert!(!result.valid);
    assert!(!result.response.changed());
    assert_eq!(field.value.as_deref(), Some("custom"));
}

#[test]
fn editable_unique_values_support_keyboard_pointer_and_independent_records() {
    let ctx = Context::default();
    let mut first = field(Some("ma".to_owned()));
    let mut second = field(Some("oa".to_owned()));
    let known = ["Maple Avenue", "Maple Court", "Oak Street"].map(str::to_owned);
    let editor = UniqueValuesEditor::editable(&known);
    let mut draw = |events| {
        frame(&ctx, events, |ui| {
            let first = ui.push_id("first", |ui| editor.show(&mut first, ui)).inner;
            let second = ui
                .push_id("second", |ui| editor.show(&mut second, ui))
                .inner;
            (first.response.changed(), second.response.changed())
        })
    };

    let (_, output) = draw(vec![]);
    draw(click(text_pos(&output, "ma").unwrap()));
    let (_, output) = draw(vec![]);
    assert!(text_pos(&output, "Maple Avenue").is_some());
    assert!(text_pos(&output, "Oak Street").is_none());

    draw(vec![key(Key::ArrowDown)]);
    let (changed, output) = draw(vec![key(Key::Enter)]);
    assert_eq!(changed, (true, false));
    draw(click(text_pos(&output, "oa").unwrap()));

    let (_, output) = draw(vec![]);
    assert!(text_pos(&output, "Oak Street").is_some());
    assert!(text_pos(&output, "Maple Avenue").is_none());
    let (changed, output) = draw(vec![key(Key::Escape)]);
    assert_eq!(changed, (false, false));
    assert!(text_pos(&output, "Oak Street").is_none());

    let (changed, _) = draw(vec![key(Key::End), Event::Text("k".to_owned())]);
    assert_eq!(changed, (false, true));
    let (_, output) = draw(vec![]);
    assert!(text_pos(&output, "Oak Street").is_some());

    let (changed, output) = draw(vec![key(Key::Tab)]);
    assert_eq!(changed, (false, false));
    assert!(text_pos(&output, "Oak Street").is_none());
    draw(click(text_pos(&output, "oak").unwrap()));
    let (_, output) = draw(vec![]);
    let suggestion = text_pos(&output, "Oak Street").unwrap();
    ctx.options_mut(|options| {
        options.input_options.surrender_focus_on = egui::SurrenderFocusOn::Presses;
    });
    draw(vec![
        Event::PointerMoved(suggestion),
        pointer_button(suggestion, true),
    ]);
    let (changed, _) = draw(vec![pointer_button(suggestion, false)]);
    assert_eq!(changed, (false, true));
    assert_eq!(first.value.as_deref(), Some("Maple Court"));
    assert_eq!(second.value.as_deref(), Some("Oak Street"));
}

#[test]
fn editable_unique_values_keep_empty_text_distinct_from_null() {
    let ctx = Context::default();
    let mut field = field(Some("x".to_owned()));
    let editor = UniqueValuesEditor::editable(&[]);

    let (_, output) = editor_frame(&ctx, vec![], &editor, &mut field);
    editor_frame(
        &ctx,
        click(text_pos(&output, "x").unwrap()),
        &editor,
        &mut field,
    );
    let (result, output) = editor_frame(
        &ctx,
        vec![key(Key::End), key(Key::Backspace)],
        &editor,
        &mut field,
    );
    assert!(result.valid);
    assert!(result.response.changed());
    assert_eq!(field.value.as_deref(), Some(""));

    let (result, _) = editor_frame(
        &ctx,
        click(text_pos(&output, "Clear").unwrap()),
        &editor,
        &mut field,
    );
    assert!(result.response.changed());
    assert_eq!(field.value, None);
}

fn tooltip_context() -> Context {
    let ctx = Context::default();
    ctx.all_styles_mut(|style| {
        style.interaction.tooltip_delay = 0.0;
        style.interaction.show_tooltips_only_when_still = false;
    });
    ctx
}

#[test]
fn editors_show_descriptor_help_on_hover() {
    fn assert_field_help<T, E>(editor: &E, value: T, hover_target: &str)
    where
        T: Clone + Default,
        E: for<'a> Editor<'a, Value = T>,
    {
        let ctx = tooltip_context();
        let mut field = field(Some(value));
        field.descriptor.help_description = Some("Field help".to_owned());

        let (_, output) = editor_frame(&ctx, vec![], editor, &mut field);
        assert!(text_pos(&output, "Field help").is_none());
        let position = text_pos(&output, hover_target).unwrap();
        editor_frame(
            &ctx,
            vec![Event::PointerMoved(position)],
            editor,
            &mut field,
        );
        let (result, output) = editor_frame(&ctx, vec![], editor, &mut field);
        assert!(text_pos(&output, "Field help").is_some());
        assert!(!result.response.changed());
    }

    assert_field_help(&BooleanEditor, true, "Field");
    assert_field_help(&StringListEditor, vec![], "Field");
    assert_field_help(&UniqueValuesEditor::editable(&[]), String::new(), "Field");
    let unique_values = ["Oak".to_owned()];
    assert_field_help(
        &UniqueValuesEditor::restricted(&unique_values),
        "Oak".to_owned(),
        "Oak",
    );
    assert_field_help(&ValueRelationEditor::new(&[(12, "Central")]), 12, "Central");
}

#[test]
fn restricted_option_help_is_optional_and_does_not_change_selection() {
    let ctx = tooltip_context();
    let choices = [
        RestrictedValue {
            value: "Oak".to_owned(),
            help_description: Some("A deciduous tree".to_owned()),
        },
        RestrictedValue::new("Pine".to_owned()),
    ];
    let editor = UniqueValuesEditor::restricted_with_options(&choices);
    let mut field = field(None);

    let (result, _) = editor_frame(&ctx, vec![], &editor, &mut field);
    editor_frame(
        &ctx,
        click(result.response.rect.center()),
        &editor,
        &mut field,
    );
    let (_, output) = editor_frame(&ctx, vec![], &editor, &mut field);
    let pine = text_pos(&output, "Pine").unwrap();
    editor_frame(&ctx, vec![Event::PointerMoved(pine)], &editor, &mut field);
    let (_, output) = editor_frame(&ctx, vec![], &editor, &mut field);
    assert!(text_pos(&output, "A deciduous tree").is_none());

    let oak = text_pos(&output, "Oak").unwrap();
    editor_frame(&ctx, vec![Event::PointerMoved(oak)], &editor, &mut field);
    let (result, output) = editor_frame(&ctx, vec![], &editor, &mut field);
    assert!(text_pos(&output, "A deciduous tree").is_some());
    assert!(!result.response.changed());
    assert_eq!(field.value, None);

    let (result, _) = editor_frame(&ctx, click(oak), &editor, &mut field);
    assert!(result.valid);
    assert!(result.response.changed());
    assert_eq!(field.value.as_deref(), Some("Oak"));
}

#[test]
fn disabled_editors_hide_edit_actions_and_keep_warnings() {
    let ctx = Context::default();
    let mut list = field(Some(vec!["Existing item".to_owned()]));
    let mut required = field::<bool>(None);
    required.descriptor.name = "required".to_owned();
    required.descriptor.nullable = false;
    required.descriptor.enforce_not_null = true;
    let mut checkbox = field(Some(true));
    let mut relation = field(Some(99));

    let (_, output) = frame(&ctx, vec![], |ui| {
        ui.add_enabled_ui(false, |ui| {
            let list = StringListEditor.show(&mut list, ui);
            let required = BooleanEditor.show(&mut required, ui);
            let checkbox = BooleanEditor.show(&mut checkbox, ui);
            let relation = ValueRelationEditor::new(&[(1, "Known option")]).show(&mut relation, ui);
            assert!(!required.valid);
            assert!(!relation.valid);
            assert!([list, required, checkbox, relation]
                .iter()
                .all(|editor| !editor.response.changed()));
        });
    });

    for hidden in ["Add item", "Remove", "Clear", "Known option"] {
        assert!(text_pos(&output, hidden).is_none(), "{hidden}");
    }
    for visible in [
        "Existing item",
        "A value is required",
        "Unavailable: 99",
        "Value is absent from the options",
    ] {
        assert!(text_pos(&output, visible).is_some(), "{visible}");
    }
}

#[test]
fn disabling_autocomplete_dismisses_suggestions_without_applying_them() {
    let ctx = Context::default();
    let mut field = field(Some("Map".to_owned()));
    let known = ["Maple".to_owned()];
    let editor = UniqueValuesEditor::editable(&known);
    let (_, output) = editor_frame(&ctx, vec![], &editor, &mut field);
    editor_frame(
        &ctx,
        click(text_pos(&output, "Map").unwrap()),
        &editor,
        &mut field,
    );
    let (_, output) = editor_frame(&ctx, vec![], &editor, &mut field);
    assert!(text_pos(&output, "Maple").is_some());

    let (result, output) = frame(&ctx, vec![key(Key::Enter)], |ui| {
        ui.add_enabled_ui(false, |ui| editor.show(&mut field, ui))
            .inner
    });
    assert!(text_pos(&output, "Map").is_some());
    assert!(text_pos(&output, "Maple").is_none());
    assert!(text_pos(&output, "Clear").is_none());
    assert!(!result.response.changed());
    assert_eq!(field.value.as_deref(), Some("Map"));
}
