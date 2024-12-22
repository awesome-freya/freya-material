use crate::prelude::*;
use freya::prelude::*;
use freya_transition::use_transition;
use material_colors::color::Argb;

#[component]
pub fn NavigationRailItem(
    active: bool,
    icon: IconData,
    label: Option<String>,
    #[props(default = true)] always_show_label: bool,
    on_click: EventHandler<MouseEvent>,
) -> Element {
    let theme = use_material_theme();
    let theme = theme.read();

    let mut hovered = use_signal(bool::default);

    let [background, color, icon_color] = if active {
        [
            theme.secondary_container,
            theme.on_surface,
            theme.on_secondary_container,
        ]
    } else {
        let color = if hovered() {
            theme.on_surface
        } else {
            theme.on_surface_variant
        };

        [Argb::new(0, 255, 255, 255), color, color]
    };

    let [background, color, icon_color] = [
        background.as_rgba(),
        color.to_string(),
        icon_color.to_string(),
    ];

    let on_pointer_enter = move |_| hovered.set(true);
    let on_pointer_leave = move |_| hovered.set(false);

    let have_label = label.is_some();
    let pill_height = if have_label { 32.0 } else { 56.0 };

    let radius_transition = use_transition(move |context| {
        context.add_tween(
            "radius",
            if active { 56.0 } else { 0.0 },
            Easing::EmphasizedDecelerate.as_value(),
            EasingDuration::Medium.as_value()[2],
        );
    });

    let label_transition = use_transition(move |context| {
        context.add_tween(
            "icon_offset",
            if active { 12.0 } else { 0.0 },
            Easing::EmphasizedDecelerate.as_value(),
            EasingDuration::Medium.as_value()[2],
        );

        context.add_tween(
            "opacity",
            if active { 0.0 } else { 1.0 },
            Easing::EmphasizedDecelerate.as_value(),
            EasingDuration::Medium.as_value()[2],
        );
    });

    let radius = radius_transition.get::<f32>("radius");
    let [icon_offset, label_opacity] = [
        label_transition.get::<f32>("icon_offset"),
        label_transition.get::<f32>("opacity"),
    ];

    use_effect(use_reactive!(|active| {
        if active {
            radius_transition.play([("radius", 56.0)]);

            if !always_show_label && have_label {
                label_transition.play([("icon_offset", 0.0), ("opacity", 1.0)]);
            }
        } else {
            radius_transition.play([("radius", 0.0)]);

            if !always_show_label && have_label {
                label_transition.play([("icon_offset", 12.0), ("opacity", 0.0)]);
            }
        }
    }));

    rsx! {
        Surface {
            direction: "vertical",
            cross_align: "center",
            width: "80",
            height: "56",

            on_pointer_enter,
            on_pointer_leave,
            on_click,

            Surface {
                main_align: "center",
                cross_align: "center",
                width: "56",
                height: "{pill_height}",
                margin: if always_show_label || !have_label {
                    0.0.to_string()
                } else {
                    format!("{icon_offset} 0 0 0")
                },

                Surface {
                    main_align: "center",
                    cross_align: "center",
                    width: "56",
                    height: "{pill_height}",
                    position: "absolute",
                    layer: "1",

                    Surface {
                        width: "{radius}",
                        height: if have_label {
                            32.0.to_string()
                        } else {
                            radius.to_string()
                        },
                        background,
                        shape: Shape::Full,
                    }
                }

                Icon {
                    name: icon.name,
                    style: icon.style,
                    filled: active,
                    width: "24",
                    height: "24",
                    color: icon_color,
                }

                StateLayer {
                    color: "{theme.on_surface}",
                    width: "56",
                    height: "{pill_height}",
                    shape: Shape::Full,
                }
            }

            if let Some(label) = label {
                Surface {
                    opacity: if always_show_label {
                        1.0
                    } else {
                        label_opacity
                    }.to_string(),

                    Typography {
                        variant: TypescaleVariant::Label,
                        color,

                        {label}
                    }
                }
            }
        }
    }
}

#[component]
pub fn NavigationRail(container_color: Option<String>, children: Element) -> Element {
    let theme = use_material_theme();
    let theme = theme.read();

    rsx! {
        Surface {
            direction: "vertical",
            height: "fill",
            width: "80",
            shape: Shape::None,
            background: container_color.unwrap_or_else(|| format!("{}", theme.surface)),
            main_align: "center",
            cross_align: "center",

            Surface {
                direction: "vertical",
                spacing: "12",
                children,
            }
        }
    }
}
