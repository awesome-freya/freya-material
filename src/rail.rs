use crate::ripple::Ripple;
use freya::prelude::*;
use std::fmt;

#[component]
fn NavigationRailItem() -> Element {
    let theme = crate::use_theme();
    let theme = theme.read();

    rsx! {
        rect {
            height: "56",
            main_align: "center",
            cross_align: "center",

            rect {
                width: "56",
                height: "32",
                corner_radius: "16",
                margin: "0 12 0 12",
                main_align: "center",
                cross_align: "center",
                background: "{theme.secondary_container}",
                overflow: "clip",

                rect {
                    position: "absolute",
                    height: "32",
                    width: "56",

                    Ripple {
                        color: theme.on_secondary_container,
                        height: "32",
                        width: "56",
                    }
                }

                label {
                    color: "{theme.on_secondary_container}",
                    font_size: "24",

                    "RE"
                }
            }

            label {
                margin: "4 0 0 0",
                color: "{theme.on_surface}",

                "Label"
            }
        }
    }
}

#[derive(Default, PartialEq, Eq, Clone)]
pub enum RailItemAlignment {
    Top,
    #[default]
    Center,
    Bottom,
}

impl fmt::Display for RailItemAlignment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RailItemAlignment::Top => f.write_str("start"),
            RailItemAlignment::Center => f.write_str("center"),
            RailItemAlignment::Bottom => f.write_str("end"),
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct NavigationRailProps {
    items_align: Option<RailItemAlignment>,
}

#[component]
pub fn NavigationRail(props: NavigationRailProps) -> Element {
    let align = props.items_align.unwrap_or_default();

    let theme = crate::use_theme();
    let theme = theme.read();

    rsx! {
        rect {
            height: "100%",
            width: "80",
            background: "{theme.surface_container_highest}",
            main_align: "{align}",
            padding: "24 0 24 0",
            cross_align: "center",
            spacing: "12",
            direction: "vertical",

            NavigationRailItem {}
            NavigationRailItem {}
            NavigationRailItem {}
        }
    }
}
