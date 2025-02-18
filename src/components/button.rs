use crate::prelude::*;
use freya::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ButtonStyle {
    Elevated,
    Filled,
    FilledTonal,
    Outlined,
    Text,
}

#[component]
pub fn Button(
    style: ButtonStyle,
    icon: Option<IconData>,
    label: String,
    on_click: EventHandler<MouseEvent>,
    #[props(default)] disabled: bool,
) -> Element {
    let theme = use_material_theme();
    let theme = theme.read();

    let (background, color, border) = match (style, disabled) {
        (ButtonStyle::Elevated, false) => (Some(theme.surface_container_low), theme.primary, None),
        (ButtonStyle::Filled, false) => (Some(theme.primary), theme.on_primary, None),
        (ButtonStyle::FilledTonal, false) => (
            Some(theme.secondary_container),
            theme.on_secondary_container,
            None,
        ),
        (ButtonStyle::Outlined, false) => (
            None,
            theme.primary,
            Some(format!("1 inner {}", theme.outline)),
        ),
        (ButtonStyle::Outlined, true) => (
            None,
            theme.on_surface.with_alpha_f32(0.38),
            Some(format!(
                "1 inner {}",
                theme.on_surface.with_alpha_f32(0.12).as_rgba()
            )),
        ),
        (ButtonStyle::Text, false) => (None, theme.primary, None),
        (ButtonStyle::Text, true) => (None, theme.on_surface.with_alpha_f32(0.38), None),
        (ButtonStyle::Elevated | ButtonStyle::Filled | ButtonStyle::FilledTonal, true) => (
            Some(theme.on_surface.with_alpha_f32(0.12)),
            theme.on_surface.with_alpha_f32(0.38),
            None,
        ),
    };

    let (background, color) = (background.map(|color| color.as_rgba()), color.as_rgba());

    let padding = if style == ButtonStyle::Text { 12 } else { 24 };

    let padding_left = if icon.is_some() && style != ButtonStyle::Text {
        16
    } else {
        padding
    };

    let (reference, size) = use_node();

    rsx! {
        rect {
            direction: "horizontal",
            cross_align: "center",
            height: "40",
            padding: "0 {padding} 0 {padding_left}",
            corner_radius: "20",
            spacing: "8",
            background,
            color: color.as_str(),
            border,
            shadow: if style == ButtonStyle::Elevated && !disabled {
                Some(
                    Elevation::Level1
                        .as_shadows()
                        .map(|value| format!("{} {} {} {} {}", value.x, value.y, value.blur, value.spread, value.fill))
                        .join(", ")
                )
            } else {
                None
            },
            overflow: "clip",

            reference,

            onclick: move |data| if !disabled {
                on_click.call(data);
            },

            if !disabled {
                StateLayer {
                    color: color.as_str(),
                    width: "{size.area.size.width}",
                    height: "40",
                    position_left: "-{padding_left}"
                }
            }

            if let Some(icon) = icon {
                Icon {
                    name: icon.name,
                    style: icon.style,
                    filled: icon.filled,
                    color: color.as_str(),
                    width: "18",
                    height: "18",
                }
            }

            Typography {
                variant: TypescaleVariant::Label,
                size: TypescaleSize::Large,

                {label}
            }
        }
    }
}
