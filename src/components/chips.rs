use crate::prelude::*;
use freya::prelude::*;

#[component]
pub fn AssistChip(
    on_click: EventHandler<MouseEvent>,
    label: String,
    leading_icon: Option<IconData>,
    trailing_icon: Option<IconData>,
    #[props(default)] elevated: bool,
) -> Element {
    let theme = use_material_theme();
    let theme = theme.read();

    let (reference, size) = use_node();

    let (background, border, elevation) = if elevated {
        (
            Some(theme.surface_container_low.to_string()),
            None,
            Some(Elevation::Level1),
        )
    } else {
        (None, Some(format!("1 inner {}", theme.outline)), None)
    };

    let left_padding = if leading_icon.is_some() { 8 } else { 16 };

    let right_padding = if trailing_icon.is_some() { 8 } else { 16 };

    rsx! {
        Surface {
            direction: "horizontal",
            cross_align: "center",
            spacing: "8",
            height: "32",
            color: "{theme.on_surface}",
            background,
            border,
            elevation,
            shape: Shape::Small,
            padding: "0 {right_padding} 0 {left_padding}",
            overflow: "clip",

            reference,
            on_click,

            StateLayer {
                color: "{theme.on_surface}",
                width: "{size.area.width()}",
                height: "32",
                position_left: "-{left_padding}",
            }

            if let Some(icon) = leading_icon {
                Icon {
                    color: "{theme.primary}",
                    name: icon.name,
                    style: icon.style,
                    filled: icon.filled,
                    width: "18",
                    height: "18",
                }
            }

            Typography {
                variant: TypescaleVariant::Label,
                size: TypescaleSize::Large,

                {label}
            }

            if let Some(icon) = trailing_icon {
                Icon {
                    color: "{theme.primary}",
                    name: icon.name,
                    style: icon.style,
                    filled: icon.filled,
                    width: "18",
                    height: "18",
                }
            }
        }
    }
}

#[component]
pub fn FilterChip(
    selected: bool,
    on_click: EventHandler<MouseEvent>,
    label: String,
    leading_icon: Option<IconData>,
    trailing_icon: Option<IconData>,
    #[props(default)] elevated: bool,
) -> Element {
    let theme = use_material_theme();
    let theme = theme.read();

    let (reference, size) = use_node();

    let (background, border, elevation) = if selected {
        (
            Some(theme.secondary_container.to_string()),
            None,
            if elevated {
                Some(Elevation::Level1)
            } else {
                None
            },
        )
    } else if elevated {
        (
            Some(theme.surface_container_low.to_string()),
            None,
            Some(Elevation::Level1),
        )
    } else {
        (None, Some(format!("1 inner {}", theme.outline)), None)
    };

    let color = if selected {
        theme.on_secondary_container
    } else {
        theme.on_surface_variant
    };

    let left_padding = if leading_icon.is_some() { 8 } else { 16 };

    let right_padding = if trailing_icon.is_some() { 8 } else { 16 };

    rsx! {
        Surface {
            direction: "horizontal",
            cross_align: "center",
            spacing: "8",
            height: "32",
            color: "{color}",
            background,
            border,
            elevation,
            shape: Shape::Small,
            padding: "0 {right_padding} 0 {left_padding}",
            overflow: "clip",

            reference,
            on_click,

            StateLayer {
                color: "{color}",
                width: "{size.area.width()}",
                height: "32",
                position_left: "-{left_padding}",
            }

            if let Some(icon) = leading_icon {
                Icon {
                    color: "{theme.primary}",
                    name: icon.name,
                    style: icon.style,
                    filled: icon.filled,
                    width: "18",
                    height: "18",
                }
            }

            Typography {
                variant: TypescaleVariant::Label,
                size: TypescaleSize::Large,

                {label}
            }

            if let Some(icon) = trailing_icon {
                Icon {
                    color: "{theme.primary}",
                    name: icon.name,
                    style: icon.style,
                    filled: icon.filled,
                    width: "18",
                    height: "18",
                }
            }
        }
    }
}

#[component]
pub fn InputChip(
    selected: bool,
    on_click: EventHandler<MouseEvent>,
    label: String,
    leading_icon: Option<IconData>,
    avatar: Option<Element>,
    trailing_icon: Option<IconData>,
) -> Element {
    let theme = use_material_theme();
    let theme = theme.read();

    let (reference, size) = use_node();

    let (background, border) = if selected {
        (Some(theme.secondary_container.to_string()), None)
    } else {
        (None, Some(format!("1 inner {}", theme.outline)))
    };

    let color = if selected {
        theme.on_secondary_container
    } else {
        theme.on_surface_variant
    };

    let primary_icon_color = if selected {
        theme.primary
    } else {
        theme.on_surface_variant
    };

    let left_padding = if avatar.is_some() {
        4
    } else if leading_icon.is_some() {
        8
    } else {
        16
    };

    rsx! {
        Surface {
            direction: "horizontal",
            cross_align: "center",
            spacing: "8",
            height: "32",
            color: "{color}",
            background,
            border,
            shape: if avatar.is_some() {
                Shape::Full
            } else {
                Shape::Small
            },
            padding: "0 8 0 {left_padding}",
            overflow: "clip",

            reference,

            StateLayer {
                color: "{color}",
                width: "{size.area.width()}",
                height: "32",
                position_left: "-{left_padding}",
            }

            if let Some(avatar) = avatar {
                Surface {
                    width: "24",
                    height: "24",
                    main_align: "center",
                    cross_align: "center",
                    shape: Shape::Full,
                    overflow: "clip",

                    {avatar}
                }
            } else if let Some(icon) = leading_icon {
                Icon {
                    color: "{primary_icon_color}",
                    name: icon.name,
                    style: icon.style,
                    filled: icon.filled,
                    width: "18",
                    height: "18",
                }
            }

            Typography {
                variant: TypescaleVariant::Label,
                size: TypescaleSize::Large,

                {label}
            }

            Surface {
                width: "24",
                height: "24",
                cross_align: "center",
                main_align: "center",
                layer: "4",

                on_click,

                StateLayer {
                    color: "{color}",
                    shape: Shape::Full,
                    width: "24",
                    height: "24",
                }

                if let Some(icon) = trailing_icon {
                    Icon {
                        color: "{color}",
                        name: icon.name,
                        style: icon.style,
                        filled: icon.filled,
                        width: "18",
                        height: "18",
                    }
                } else {
                    Icon {
                        color: "{color}",
                        name: "star",
                        width: "18",
                        height: "18",
                    }
                }
            }
        }
    }
}
