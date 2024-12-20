use freya::prelude::*;
use freya_material::prelude::*;

const IMAGE: &[u8] = include_bytes!("/home/aiving/Pictures/c6oi.png");

fn main() {
    launch_cfg(
        App,
        LaunchConfig::<()>::new()
            .with_title("Component: Chips")
            .with_size(200., 200.)
            .with_roboto(),
    );
}

#[component]
fn App() -> Element {
    let theme = use_material_theme();
    let theme = theme.read();

    let mut selected = use_signal(bool::default);

    let mut values = use_signal(Vec::new);

    rsx! {
        Surface {
            direction: "vertical",
            padding: "12",
            spacing: "24",
            background: "{theme.surface_bright}",
            color: "{theme.on_surface}",
            width: "fill",
            height: "fill",

            Typography {
                variant: TypescaleVariant::Headline,

                "Assist chips"
            }

            Surface {
                direction: "horizontal",
                spacing: "8",

                AssistChip {
                    on_click: |_| {},
                    label: "Hello?",
                }

                AssistChip {
                    on_click: |_| {},
                    label: "Hello?",
                    elevated: true
                }

                AssistChip {
                    on_click: |_| {},
                    label: "Hello?",
                    leading_icon: IconData {
                        name: "star",
                        ..Default::default()
                    },
                }

                AssistChip {
                    on_click: |_| {},
                    label: "Hello?",
                    leading_icon: IconData {
                        name: "star",
                        ..Default::default()
                    },
                    trailing_icon: IconData {
                        name: "star",
                        ..Default::default()
                    },
                }
            }

            Typography {
                variant: TypescaleVariant::Headline,

                "Filter chips"
            }

            Surface {
                direction: "horizontal",
                spacing: "8",

                FilterChip {
                    selected: selected(),
                    on_click: move |_| selected.toggle(),
                    label: "Apples",
                }

                FilterChip {
                    selected: selected(),
                    on_click: move |_| selected.toggle(),
                    label: "Apples",
                    elevated: true
                }

                FilterChip {
                    selected: selected(),
                    on_click: move |_| selected.toggle(),
                    label: "Apples",
                    leading_icon: IconData {
                        name: "star",
                        ..Default::default()
                    },
                }

                FilterChip {
                    selected: selected(),
                    on_click: move |_| selected.toggle(),
                    label: "Apples",
                    leading_icon: IconData {
                        name: "star",
                        ..Default::default()
                    },
                    trailing_icon: IconData {
                        name: "star",
                        ..Default::default()
                    },
                }
            }

            Typography {
                variant: TypescaleVariant::Headline,

                "Input chips"
            }

            Surface {
                direction: "horizontal",
                spacing: "8",

                AssistChip {
                    on_click: move |_| values.write().push(()),
                    label: "Add input chip",
                }

                for (value, ()) in values.read().iter().enumerate() {
                    InputChip {
                        selected: selected(),
                        on_click: move |_| values.write().remove(value),
                        label: "{value + 1} value",
                        avatar: if value % 2 == 0 {
                            Some(rsx! {
                                image {
                                    image_data: static_bytes(IMAGE),
                                    width: "24",
                                    height: "24",
                                }
                            })
                        } else {
                            None
                        },
                        leading_icon: if value % 3 == 0 {
                            IconData {
                                name: "star",
                                filled: true,
                                ..Default::default()
                            }
                        },
                        trailing_icon: if value % 4 == 0 {
                            IconData {
                                name: "star",
                                ..Default::default()
                            }
                        }
                    }
                }
            }
        }
    }
}
