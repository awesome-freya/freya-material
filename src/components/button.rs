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
pub fn Button(style: ButtonStyle, icon: Option<IconData>, children: Element) -> Element {
    let theme = use_material_theme();
    let theme = theme.read();

    let (background, color, border) = match style {
        ButtonStyle::Elevated => (Some(theme.surface_container_low), theme.primary, None),
        ButtonStyle::Filled => (Some(theme.primary), theme.on_primary, None),
        ButtonStyle::FilledTonal => (
            Some(theme.secondary_container),
            theme.on_secondary_container,
            None,
        ),
        ButtonStyle::Outlined => (
            None,
            theme.primary,
            Some(format!("1 inner {}", theme.outline)),
        ),
        ButtonStyle::Text => (None, theme.primary, None),
    };

    let (background, color) = (background.map(|color| color.to_string()), color.to_string());

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
            shadow: if style == ButtonStyle::Elevated {
                Some(Elevation::Level1.as_shadow())
            } else {
                None
            },
            overflow: "clip",

            reference,

            StateLayer {
                color: color.as_str(),
                width: "{size.area.size.width}",
                height: "40",
                position_left: "-{padding_left}"
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

                {children}
            }
        }
    }
}
