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
    on_click: Option<EventHandler<()>>,
) -> Element {
    let theme = use_material_theme();
    let theme = theme.read();

    let (background, color, border) = match (style, selected) {
        (IconButtonStyle::Standard, None | Some(false)) => (None, theme.on_surface_variant, None),
        (IconButtonStyle::Standard, Some(true)) => (None, theme.primary, None),
        (IconButtonStyle::Filled, None | Some(true)) => {
            (Some(theme.primary), theme.on_primary, None)
        }
        (IconButtonStyle::Filled, Some(false)) => {
            (Some(theme.surface_container_highest), theme.primary, None)
        }
        (IconButtonStyle::FilledTonal, None | Some(true)) => (
            Some(theme.secondary_container),
            theme.on_secondary_container,
            None,
        ),
        (IconButtonStyle::FilledTonal, Some(false)) => (
            Some(theme.surface_container_highest),
            theme.on_surface_variant,
            None,
        ),
        (IconButtonStyle::Outlined, None | Some(false)) => (
            None,
            theme.primary,
            Some(format!("1 inner {}", theme.outline)),
        ),
        (IconButtonStyle::Outlined, Some(true)) => {
            (Some(theme.inverse_surface), theme.inverse_on_surface, None)
        }
    };

    let (background, color) = (background.map(|color| color.to_string()), color.to_string());

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

            onclick: move |_| {
                if let Some(on_click) = on_click {
                    on_click(());
                }
            },

            StateLayer {
                color: color.as_str(),
                width: "40",
                height: "40",
                position_left: "-8",
                position_top: "-8"
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
