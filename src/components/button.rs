use crate::prelude::*;
use freya::prelude::*;
use material_colors::color::Argb;

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

    let theme = use_material_theme();
    let theme = theme.read();

    let (background, color, border) = match kind {
        ButtonKind::Elevated => (theme.surface_container_low, theme.primary, TRANSPARENT),
        ButtonKind::Filled => (theme.primary, theme.on_primary, TRANSPARENT),
        ButtonKind::Tonal => (
            theme.primary_container,
            theme.on_primary_container,
            TRANSPARENT,
        ),
        ButtonKind::Outlined => (TRANSPARENT, theme.primary, theme.outline),
        ButtonKind::Text => (TRANSPARENT, theme.primary, TRANSPARENT),
    };

    rsx! {
        rect {
            height: "40",
            corner_radius: material_design::shape::FULL,
            main_align: "center",
            background: "{background.as_rgba()}",
            border: "1 solid {border.as_rgba()}",
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
                color: "{color}",
                font_size: "14",

                {&props.children}
            }
        }
    }
}
