use crate::ripple::Ripple;
use freya::prelude::*;
use material_icons::IconStyle;
use freya_transition::{use_transition, Curve};

#[derive(Default, PartialEq, Eq)]
enum FabState {
    #[default]
    Idle,
    Hovering,
    Pressed,
}

#[derive(Props, PartialEq, Clone)]
pub struct FabProps {
    pub extended: bool,
    pub onclick: EventHandler,
    pub icon: String,
    pub label: String,
}

#[component]
#[allow(clippy::needless_pass_by_value)]
pub fn FAB(props: FabProps) -> Element {
    let platform = use_platform();

    let mut state = use_signal(FabState::default);

    let animation = use_transition(move |context| {
        context.add_tween("width", 0.0, Curve::EASE_OUT_EXPO, 300);
    });

    let onmousedown = move |e: MouseEvent| {
        e.stop_propagation();

        state.set(FabState::Pressed);
    };

    let onmouseleave = move |e: MouseEvent| {
        e.stop_propagation();

        state.set(FabState::Idle);

        platform.set_cursor(CursorIcon::default());
    };

    let onmouseenter = move |e: MouseEvent| {
        e.stop_propagation();

        state.set(FabState::Hovering);

        platform.set_cursor(CursorIcon::Pointer);
    };

    let onclick = move |e: MouseEvent| {
        e.stop_propagation();

        props.onclick.call(());

        state.set(FabState::Idle);
    };

    use_effect(use_reactive(&props.extended, move |extended| {
        if extended {
            animation.play(vec![("width", 100.0)]);
        } else {
            animation.play(vec![("width", 0.0)]);
        }
    }));

    let width = animation.get::<f32>("width");

    let icon = static_bytes(material_icons::icon(
        &props.icon,
        IconStyle::Outlined,
        false,
    ));

    let theme = crate::use_theme();
    let theme = theme.read();

    rsx! {
        rect {
            height: "56",
            min_width: "24",
            corner_radius: "16",
            main_align: "center",
            background: "{theme.primary_container}",
            padding: "16",
            direction: "horizontal",
            overflow: "clip",

            onmousedown,
            onmouseenter,
            onmouseleave,
            onclick,

            rect {
                position: "absolute",
                position_top: "-16",
                position_left: "-16",
                height: "56",
                min_width: "56",

                Ripple {
                    color: theme.on_primary_container,
                    height: "56",
                    width: "fill",
                }
            }

            svg {
                width: "24",
                height: "24",
                // fill: "{theme.on_primary_container}",
                svg_data: icon
            }

            rect {
                margin: "0 0 0 4",
                overflow: "clip",
                width: "{width}a",
                height: "100%",

                label {
                    max_lines: "1",
                    width: "auto",
                    font_size: "14",
                    text_overflow: "clip",
                    color: "{theme.on_primary_container}",
                    {props.label.as_str()}
                }
            }
        }
    }
}
