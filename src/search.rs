use crate::prelude::*;
use freya::prelude::*;
use freya_transition::{use_transition, Curve};

#[derive(Default, PartialEq, Eq)]
enum SearchState {
    #[default]
    Idle,
    Hovering,
}

#[component]
pub fn Search(
    value: String,
    placeholder: Option<String>,
    onchange: EventHandler<String>,
) -> Element {
    let platform = use_platform();

    let (reference, size) = use_node();

    let mut state = use_signal(SearchState::default);

    let animation = use_transition(move |context| {
        context.add_tween("offset", 20.0, Curve::EASE_OUT_EXPO, 500);
    });

    let mut editable = use_editable(
        || EditableConfig::new(value.to_string()),
        EditableMode::MultipleLinesSingleEditor,
    );

    use_drop(move || {
        if *state.peek() == SearchState::Hovering {
            platform.set_cursor(CursorIcon::default());
        }
    });

    let mut focus = use_focus();

    let is_focused = focus.is_focused();
    let display_placeholder = value.is_empty() && placeholder.is_some() && !is_focused;

    if &value != editable.editor().read().rope() {
        editable.editor_mut().write().set(&value);
    }

    let onkeydown = move |e: Event<KeyboardData>| {
        if e.data.key != Key::Enter && e.data.key != Key::Tab {
            e.stop_propagation();

            editable.process_event(&EditableEvent::KeyDown(e.data));

            onchange.call(editable.editor().peek().to_string());
        }
    };

    let onkeyup = move |e: Event<KeyboardData>| {
        e.stop_propagation();

        if e.data.key == Key::Escape {
            focus.unfocus();
        }

        editable.process_event(&EditableEvent::KeyUp(e.data));
    };

    let onmousedown = move |e: MouseEvent| {
        if !display_placeholder {
            editable.process_event(&EditableEvent::MouseDown(e.data, 0));
        }

        focus.focus();
    };

    let onmousemove = move |e: MouseEvent| {
        editable.process_event(&EditableEvent::MouseMove(e.data, 0));
    };

    let onmouseenter = move |_: MouseEvent| {
        platform.set_cursor(CursorIcon::Text);

        *state.write() = SearchState::Hovering;
    };

    let onmouseleave = move |_: MouseEvent| {
        platform.set_cursor(CursorIcon::default());

        *state.write() = SearchState::Idle;
    };

    let onglobalclick = move |_| match *state.read() {
        SearchState::Idle if focus.is_focused() => {
            focus.unfocus();

            editable.process_event(&EditableEvent::Click);
        }
        SearchState::Hovering => {
            editable.process_event(&EditableEvent::Click);
        }
        _ => {}
    };

    let a11y_id = focus.attribute();
    let cursor_reference = editable.cursor_attr();
    let highlights = editable.highlights_attr(0);

    let cursor_char = if focus.is_focused() {
        editable.editor().read().cursor_pos().to_string()
    } else {
        "none".into()
    };

    let theme = use_material_theme();
    let theme = theme.read();

    let color = if display_placeholder {
        &theme.on_surface_variant
    } else {
        &theme.on_surface
    };

    let text = if display_placeholder {
        placeholder.unwrap_or_default()
    } else {
        value
    };

    let offset = animation.get::<f32>("offset");

    use_effect(use_reactive!(|is_focused| {
        if is_focused {
            animation.set("offset", 20.0);
            // animation.set("radius", 0.0);
        } else {
            animation.set("offset", -size.area.size.height);
            // animation.set("radius", 16.0);
        }

        animation.play_all();
    }));

    rsx! {
        rect {
            height: "56",
            width: "fill",
            min_width: "360",
            max_width: "720",

            rect {
                height: "56",
                width: "fill",
                padding: "8",
                spacing: "8",
                direction: "horizontal",
                corner_radius: "16",
                background: "{theme.surface_container_high}",
                overflow: "clip",
                layer: "-20",

                // rect {
                //     position: "absolute",
                //     width: "fill",
                //     height: "56",
                //     margin: "-16",

                //     Ripple {
                //         color: theme.on_surface,
                //     }
                // }

                IconButton {
                    onclick: move |_| {},
                    icon: "settings",
                }

                // Input {}

                rect {
                    color: "{color}",
                    width: "calc(100% - 96)",
                    height: "fill",
                    direction: "vertical",
                    main_align: "center",
                    a11y_id,
                    a11y_role: "textInput",
                    cursor_reference,

                    onkeydown,
                    onkeyup,


                    paragraph {
                        onglobalclick,
                        onmouseenter,
                        onmouseleave,
                        onmousedown,
                        onmousemove,

                        width: "100%",
                        cursor_id: "0",
                        cursor_index: "{cursor_char}",
                        cursor_mode: "editable",
                        cursor_color: "{color}",
                        max_lines: "1",
                        highlights,

                        text {
                            "{text}"
                        }
                    }
                }

                IconButton {
                    onclick: move |_| {},
                    icon: "settings",
                }
            }

            rect {
                min_height: "240",
                width: "fill",
                direction: "vertical",
                min_width: "360",
                max_width: "720",
                overflow: "clip",
                position: "absolute",
                position_top: "20",
                layer: "-11",
                offset_y: "{offset}",

                rect {
                    width: "fill",
                    height: "16",
                    background: "{theme.surface_container_high}",
                }

                rect {
                    width: "fill",
                    height: "1",
                    background: "{theme.outline}",
                }

                rect {
                    width: "fill",
                    corner_radius: "0 0 16 16",
                    background: "{theme.surface_container_high}",
                    padding: "8",
                    spacing: "8",
                    reference,

                    rect {
                        width: "fill",
                        corner_radius: "16",
                        background: "{theme.surface_container_highest}",
                        color: "{theme.on_surface}",
                        padding: "8",

                        label { "nice suggestion item" }
                    }

                    rect {
                        width: "fill",
                        corner_radius: "16",
                        background: "{theme.surface_container_highest}",
                        color: "{theme.on_surface}",
                        padding: "8",

                        label { "nice suggestion item" }
                    }

                    rect {
                        width: "fill",
                        corner_radius: "16",
                        background: "{theme.surface_container_highest}",
                        color: "{theme.on_surface}",
                        padding: "8",

                        label { "nice suggestion item" }
                    }

                    rect {
                        width: "fill",
                        corner_radius: "16",
                        background: "{theme.surface_container_highest}",
                        color: "{theme.on_surface}",
                        padding: "8",

                        label { "nice suggestion item" }
                    }

                    label {
                        color: "{theme.on_surface}",
                        font_size: "18",
                        font_weight: "bold",

                        "btw maybe you searching for someone?"
                    }

                    rect {
                        direction: "horizontal",
                        spacing: "8",

                        rect {
                            height: "48",
                            width: "48",
                            corner_radius: "24",
                            main_align: "center",
                            cross_align: "center",
                            background: "{theme.surface_container_highest}",
                            color: "{theme.on_surface}",

                            label { "AM" }
                        }

                        rect {
                            height: "48",
                            width: "48",
                            corner_radius: "24",
                            main_align: "center",
                            cross_align: "center",
                            background: "{theme.surface_container_highest}",
                            color: "{theme.on_surface}",

                            label { "AM" }
                        }

                        rect {
                            height: "48",
                            width: "48",
                            corner_radius: "24",
                            main_align: "center",
                            cross_align: "center",
                            background: "{theme.surface_container_highest}",
                            color: "{theme.on_surface}",

                            label { "AM" }
                        }

                        rect {
                            height: "48",
                            width: "48",
                            corner_radius: "24",
                            main_align: "center",
                            cross_align: "center",
                            background: "{theme.surface_container_highest}",
                            color: "{theme.on_surface}",

                            label { "AM" }
                        }

                        rect {
                            height: "48",
                            width: "48",
                            corner_radius: "24",
                            main_align: "center",
                            cross_align: "center",
                            background: "{theme.surface_container_highest}",
                            color: "{theme.on_surface}",

                            label { "AM" }
                        }
                    }
                }
            }
        }
    }
}
