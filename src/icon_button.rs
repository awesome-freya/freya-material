use freya::prelude::*;
use material_colors::color::Argb;

use crate::{icons, ripple::Ripple};
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
pub enum IconButtonKind {
    #[default]
    Standard,
    Filled,
    Tonal,
    Outlined,
}

#[derive(Props, PartialEq, Clone)]
pub struct IconButtonProps {
    pub kind: Option<IconButtonKind>,
    pub selected: Option<bool>,
    pub onclick: EventHandler,
    pub icon: String,
}

#[component]
#[allow(clippy::needless_pass_by_value)]
pub fn IconButton(props: IconButtonProps) -> Element {
    let platform = use_platform();

    let kind = props.kind.unwrap_or_default();
    let selected = props.selected;

    let mut state = use_signal(ButtonState::default);

    let onmousedown = move |e: MouseEvent| {
        e.stop_propagation();

        state.set(ButtonState::Pressed);
    };

    let onmouseleave = move |e: MouseEvent| {
        e.stop_propagation();

        state.set(ButtonState::Idle);

        platform.set_cursor(CursorIcon::default());
    };

    let onmouseenter = move |e: MouseEvent| {
        e.stop_propagation();

        state.set(ButtonState::Hovering);

        platform.set_cursor(CursorIcon::Pointer);
    };

    let onclick = move |e: MouseEvent| {
        e.stop_propagation();

        props.onclick.call(());

        state.set(ButtonState::Idle);
    };

    let (background, color, border) = match (kind, selected) {
        (IconButtonKind::Standard, None | Some(false)) => (
            TRANSPARENT.to_rgba(),
            THEME.on_surface_variant,
            TRANSPARENT.to_rgba(),
        ),
        (IconButtonKind::Standard, Some(true)) => (
            TRANSPARENT.to_rgba(),
            THEME.primary,
            TRANSPARENT.to_rgba(),
        ),
        (IconButtonKind::Filled, None | Some(true)) => (
            THEME.primary.to_rgb(),
            THEME.on_primary,
            TRANSPARENT.to_rgba(),
        ),
        (IconButtonKind::Filled, Some(false)) => (
            THEME.surface_variant.to_rgb(),
            THEME.primary,
            TRANSPARENT.to_rgba(),
        ),
        (IconButtonKind::Tonal, None | Some(true)) => (
            THEME.secondary_container.to_rgb(),
            THEME.on_secondary_container,
            TRANSPARENT.to_rgba(),
        ),
        (IconButtonKind::Tonal, Some(false)) => (
            THEME.surface_variant.to_rgb(),
            THEME.on_surface_variant,
            TRANSPARENT.to_rgba(),
        ),
        (IconButtonKind::Outlined, None | Some(false)) => (
            TRANSPARENT.to_rgba(),
            THEME.on_surface_variant,
            THEME.outline.to_rgb(),
        ),
        (IconButtonKind::Outlined, Some(true)) => (
            THEME.inverse_surface.to_rgba(),
            THEME.inverse_on_surface,
            THEME.inverse_surface.to_rgb(),
        ),
    };

    let icon = static_bytes(match props.icon.as_str() {
        "settings" => icons::settings(selected.unwrap_or_default()),
        _ => panic!("there is no icon called {}", props.icon),
    });

    rsx! {
        rect {
            height: "40",
            width: "40",
            corner_radius: "20",
            background: "rgb({background})",
            border: "1 solid rgb({border})",
            padding: "8",
            overflow: "clip",

            onmousedown,
            onmouseenter,
            onmouseleave,
            onclick,

            rect {
                position: "absolute",
                height: "40",
                width: "40",
                margin: "-8",

                Ripple {
                    color,
                    height: "40",
                    width: "40",
                }
            }

            svg {
                // fill: "rgb({color})",
                width: "24",
                height: "24",
                svg_data: icon,
            }
        }
    }
}
