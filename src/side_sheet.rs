use freya::prelude::*;
use freya_transition::{curves::Curve, use_transition};

#[derive(Default, PartialEq, Eq, Clone)]
pub enum SideSheetKind {
    #[default]
    Docked,
    DockedModal,
    Detached,
}

#[derive(Props, PartialEq, Clone)]
pub struct SideSheetProps {
    pub kind: Option<SideSheetKind>,
    pub width: Option<String>,
    pub open: bool,
    pub children: Element,
}

#[component]
#[allow(clippy::needless_pass_by_value)]
#[must_use]
pub fn SideSheet(props: SideSheetProps) -> Element {
    let kind = props.kind.unwrap_or_default();
    let width = props.width.unwrap_or_else(|| "256".to_string());
    let num_width = width.parse::<f32>().unwrap();

    let transition = use_transition(move |context| {
        context.add_tween("width", 0.0, Curve::EASE_IN_OUT_EXPO, 300);
    });

    use_effect(use_reactive(&props.open, move |open| {
        if open {
            transition.play(vec![("width", num_width)]);
        } else {
            transition.play(vec![("width", 0.0)]);
        }
    }));

    let width = transition.get::<f32>("width");

    let theme = crate::use_theme();
    let theme = theme.read();

    match kind {
        SideSheetKind::Docked => {
            rsx! {
                rect {
                    height: "100%",
                    width: "calc(100% - {width})",

                    {&props.children}
                }

                rect {
                    height: "100%",
                    width: "{width}",
                    direction: "horizontal",
                    corner_radius: "0",

                    rect {
                        height: "100%",
                        width: "1",
                        background: "{theme.outline}",
                    }

                    rect {
                        height: "100%",
                        width: "100%",
                        background: "{theme.surface}",
                        padding: "24",
                    }
                }
            }
        }
        SideSheetKind::DockedModal => {
            rsx! {
                rect {
                    height: "100%",
                    margin: "16 0 16 16",
                    width: "calc(100% - {width} - 128)",
                    corner_radius: "16",
                    overflow: "clip",

                    {&props.children}
                }

                rect {
                    height: "100%",
                    width: "{width}",
                    direction: "horizontal",
                    corner_radius: "16",
                    background: "{theme.surface_container_low}",
                    padding: "24",
                    margin: "16 16 16 16"
                }
            }
        }
        SideSheetKind::Detached => {
            rsx! {
                {&props.children}

                rect {
                    height: "100v",
                    width: "100%",
                    background: "rgb(0, 0, 0, 150)",
                    position: "absolute",
                    position_top: "0",
                    position_left: "0",
                    main_align: "end",
                    cross_align: "end",

                    rect {
                        height: "100%",
                        width: "{width}",
                        direction: "horizontal",
                        corner_radius: "16",
                        background: "{theme.surface}",
                        padding: "24",
                        margin: "16"
                    }
                }
            }
        }
    }
}
