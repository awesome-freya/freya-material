use freya::prelude::{
    component, dioxus_core, dioxus_elements, fc_to_builder, launch_cfg, rsx, use_signal, Element,
    IntoDynNode, LaunchConfig, Readable, Writable,
};
use freya_material::prelude::*;

fn main() {
    launch_cfg(
        App,
        LaunchConfig::<()>::new()
            .with_title("Component Gallery")
            .with_size(200., 200.)
            .with_roboto(),
    );
}

#[component]
fn App() -> Element {
    let theme = use_material_theme();
    let theme = theme.read();

    let mut selected = use_signal(bool::default);
    let mut current_value = use_signal(i32::default);

    rsx! {
        Surface {
            direction: "vertical",
            padding: "12",
            spacing: "24",
            background: "{theme.surface_container_highest}",
            color: "{theme.on_surface}",
            width: "fill",
            height: "fill",

            Typography {
                variant: TypescaleVariant::Headline,

                "Component Gallery"
            }

            Surface {
                direction: "horizontal",
                spacing: "24",

                Surface {
                    direction: "vertical",
                    spacing: "24",

                    for style in [
                        ButtonStyle::Elevated,
                        ButtonStyle::Filled,
                        ButtonStyle::FilledTonal,
                        ButtonStyle::Outlined,
                        ButtonStyle::Text
                    ] {
                        rect {
                            direction: "horizontal",
                            spacing: "8",

                            Button {
                                label: "Hello world!",
                                on_click: |_| {},
                                style,
                            }

                            Button {
                                label: "Hello world!",
                                icon: IconData {
                                    name: "star",
                                    filled: true,
                                    ..Default::default()
                                },
                                on_click: |_| {},
                                style,
                            }

                            Button {
                                label: "Hello world!",
                                icon: IconData {
                                    name: "star",
                                    filled: true,
                                    ..Default::default()
                                },
                                on_click: |_| {},
                                style,
                                disabled: true
                            }
                        }
                    }

                    for style in [
                        IconButtonStyle::Standard,
                        IconButtonStyle::Filled,
                        IconButtonStyle::FilledTonal,
                        IconButtonStyle::Outlined,
                    ] {
                        rect {
                            direction: "horizontal",
                            spacing: "8",

                            IconButton {
                                style,
                                icon: IconData {
                                    name: "star",
                                    filled: true,
                                    ..Default::default()
                                },
                                on_click: |_| { },
                            }

                            IconButton {
                                style,
                                icon: IconData {
                                    name: "star",
                                    filled: true,
                                    ..Default::default()
                                },
                                on_click: |_| { },
                                disabled: true
                            }

                            IconButton {
                                style,
                                selected: *selected.read(),
                                on_click: move |_| selected.toggle(),
                                icon: IconData {
                                    name: "star",
                                    filled: true,
                                    ..Default::default()
                                },
                            }
                        }
                    }
                }

                Surface {
                    direction: "vertical",
                    spacing: "8",

                    for i in 0..10 {
                        Surface {
                            direction: "horizontal",
                            cross_align: "center",
                            spacing: "8",

                            RadioButton {
                                selected: current_value() == i,
                                on_click: move |_| current_value.set(i),
                            }

                            Typography {
                                "Variant {i + 1}"
                            }
                        }
                    }
                }

                Surface {
                    direction: "horizontal",
                    spacing: "12",

                    for (i, elevation) in [
                        Elevation::Level0,
                        Elevation::Level1,
                        Elevation::Level2,
                        Elevation::Level3,
                        Elevation::Level4,
                        Elevation::Level5
                    ].into_iter().enumerate() {
                        Surface {
                            direction: "vertical",
                            spacing: "8",

                            Typography {
                                "Elevation level {i}"
                            }

                            Surface {
                                background: "{theme.primary}",
                                width: "92",
                                height: "92",
                                shape: Shape::Medium,
                                elevation,
                            }
                        }
                    }
                }
            }
        }
    }
}
