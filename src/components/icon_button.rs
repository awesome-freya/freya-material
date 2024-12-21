use crate::prelude::*;
use freya::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IconButtonStyle {
    Standard,
    Filled,
    FilledTonal,
    Outlined,
}

#[component]
pub fn IconButton(
    style: IconButtonStyle,
    icon: IconData,
    selected: Option<bool>,
    on_click: EventHandler<MouseEvent>,
    #[props(default)] disabled: bool,
) -> Element {
    let theme = use_material_theme();
    let theme = theme.read();

    let (background, color, border) = match (style, selected, disabled) {
        (IconButtonStyle::Standard, None | Some(false), false) => {
            (None, theme.on_surface_variant, None)
        }
        (IconButtonStyle::Standard, Some(true), false) => (None, theme.primary, None),
        (IconButtonStyle::Standard, _, true) => (None, theme.on_surface.with_alpha_f32(0.38), None),
        (IconButtonStyle::Filled, None | Some(true), false) => {
            (Some(theme.primary), theme.on_primary, None)
        }
        (IconButtonStyle::Filled, Some(false), false) => {
            (Some(theme.surface_container_highest), theme.primary, None)
        }
        (IconButtonStyle::Filled, _, true) => (
            Some(theme.on_surface.with_alpha_f32(0.12)),
            theme.on_surface.with_alpha_f32(0.38),
            None,
        ),
        (IconButtonStyle::FilledTonal, None | Some(true), false) => (
            Some(theme.secondary_container),
            theme.on_secondary_container,
            None,
        ),
        (IconButtonStyle::FilledTonal, Some(false), false) => (
            Some(theme.surface_container_highest),
            theme.on_surface_variant,
            None,
        ),
        (IconButtonStyle::FilledTonal, _, true) => (
            Some(theme.on_surface.with_alpha_f32(0.12)),
            theme.on_surface.with_alpha_f32(0.38),
            None,
        ),
        (IconButtonStyle::Outlined, None | Some(false), false) => (
            None,
            theme.on_surface,
            Some(format!("1 inner {}", theme.outline)),
        ),
        (IconButtonStyle::Outlined, Some(true), false) => {
            (Some(theme.inverse_surface), theme.inverse_on_surface, None)
        }
        (IconButtonStyle::Outlined, _, true) => (
            None,
            theme.on_surface.with_alpha_f32(0.38),
            Some(format!("1 inner {}", theme.on_surface.with_alpha_f32(0.38).as_rgba())),
        ),
    };

    let (background, color) = (background.map(|color| color.as_rgba()), color.as_rgba());

    rsx! {
        rect {
            direction: "horizontal",
            cross_align: "center",
            height: "40",
            width: "40",
            padding: "8",
            corner_radius: "20",
            background,
            color: color.as_str(),
            border,
            overflow: "clip",

            onclick: move |data| if !disabled {
                on_click.call(data)
            },

            if !disabled {
                StateLayer {
                    color: color.as_str(),
                    width: "40",
                    height: "40",
                    position_left: "-8",
                    position_top: "-8"
                }
            }

            Icon {
                name: icon.name,
                style: icon.style,
                filled: selected.unwrap_or(icon.filled),
                color: color.as_str(),
                width: "24",
                height: "24",
            }
        }
    }
}
