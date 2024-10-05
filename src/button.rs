use freya::prelude::*;
use material_colors::color::Argb;

use crate::ripple::Ripple;
use shared::{ColorConversion, THEME};

const TRANSPARENT: Argb = Argb::new(0, 0, 0, 0);

#[derive(Default, PartialEq, Eq)]
enum ButtonState {
    #[default]
    Idle,
    Hovering,
    Pressed,
}

#[derive(Default, PartialEq, Eq, Clone)]
pub enum ButtonKind {
    Elevated,
    #[default]
    Filled,
    Tonal,
    Outlined,
    Text,
}

#[derive(Props, PartialEq, Clone)]
pub struct ButtonProps {
    // pub width: Option<String>,
    // pub height: Option<String>,
    pub kind: Option<ButtonKind>,
    pub onclick: EventHandler,
    pub children: Element,
}

#[component]
#[allow(clippy::needless_pass_by_value)]
pub fn Button(props: ButtonProps) -> Element {
    let platform = use_platform();
    let (node_ref, size) = use_node_signal();

    let kind = props.kind.unwrap_or_default();

    let mut state = use_signal(ButtonState::default);

    let onmousedown = move |e: MouseEvent| {
        e.stop_propagation();

        state.set(ButtonState::Pressed);
    };

    let onmouseleave = move |e: MouseEvent| {
        e.stop_propagation();

        if state.read().ne(&ButtonState::Idle)
        /*  && radius_animation.peek_has_run_yet() */
        {
            state.set(ButtonState::Idle);
        }

        platform.set_cursor(CursorIcon::default());
    };

    let onmouseenter = move |e: MouseEvent| {
        e.stop_propagation();

        state.set(ButtonState::Hovering);

        platform.set_cursor(CursorIcon::Pointer);
    };

    let onclick = move |e: MouseEvent| {
        e.stop_propagation();

        // if radius_animation.peek_has_run_yet() {
        //     radius_animation.reverse();
        // }

        props.onclick.call(());

        state.set(ButtonState::Idle);
    };

    // use_effect(use_reactive(&props.enabled, move |enabled| {
    //     if enabled {
    //         animation.start();
    //     } else if animation.peek_has_run_yet() {
    //         animation.reverse();
    //     }
    // }));

    let (background, color, border) = match kind {
        ButtonKind::Elevated => (
            THEME.surface_container_low.to_rgb(),
            THEME.primary,
            TRANSPARENT.to_rgba(),
        ),
        ButtonKind::Filled => (
            THEME.primary.to_rgb(),
            THEME.on_primary,
            TRANSPARENT.to_rgba(),
        ),
        ButtonKind::Tonal => (
            THEME.primary_container.to_rgb(),
            THEME.on_primary_container,
            TRANSPARENT.to_rgba(),
        ),
        ButtonKind::Outlined => (
            TRANSPARENT.to_rgba(),
            THEME.primary,
            THEME.outline.to_rgb(),
        ),
        ButtonKind::Text => (
            TRANSPARENT.to_rgba(),
            THEME.primary,
            TRANSPARENT.to_rgba(),
        ),
    };

    rsx! {
        rect {
            height: "40",
            corner_radius: "20",
            main_align: "center",
            background: "rgb({background})",
            border: "1 solid rgb({border})",
            padding: "0 24 0 24",
            overflow: "clip",

            reference: node_ref,

            onmousedown,
            onmouseenter,
            onmouseleave,
            onclick,

            rect {
                position: "absolute",
                position_left: "-24",
                height: "40",
                width: "{size.read().area.width()}",

                Ripple {
                    color,
                    height: "40",
                    width: "{size.read().area.width()}",
                }
            }


            label {
                color: "rgb({color.to_rgb()})",
                font_size: "14",

                {&props.children}
            }
        }
    }
}
