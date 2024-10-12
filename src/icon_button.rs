use crate::ripple::Ripple;
use freya::prelude::*;
use material_colors::color::Argb;
use material_icons::IconStyle;

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

    let theme = crate::use_theme();
    let theme = theme.read();

    let (background, color, border) = match (kind, selected) {
        (IconButtonKind::Standard, None | Some(false)) => {
            (TRANSPARENT, theme.on_surface_variant, TRANSPARENT)
        }
        (IconButtonKind::Standard, Some(true)) => (TRANSPARENT, theme.primary, TRANSPARENT),
        (IconButtonKind::Filled, None | Some(true)) => {
            (theme.primary, theme.on_primary, TRANSPARENT)
        }
        (IconButtonKind::Filled, Some(false)) => {
            (theme.surface_variant, theme.primary, TRANSPARENT)
        }
        (IconButtonKind::Tonal, None | Some(true)) => (
            theme.secondary_container,
            theme.on_secondary_container,
            TRANSPARENT,
        ),
        (IconButtonKind::Tonal, Some(false)) => {
            (theme.surface_variant, theme.on_surface_variant, TRANSPARENT)
        }
        (IconButtonKind::Outlined, None | Some(false)) => {
            (TRANSPARENT, theme.on_surface_variant, theme.outline)
        }
        (IconButtonKind::Outlined, Some(true)) => (
            theme.inverse_surface,
            theme.inverse_on_surface,
            theme.inverse_surface,
        ),
    };

    let icon = static_bytes(material_icons::icon(
        &props.icon,
        IconStyle::Outlined,
        false,
    ));

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
