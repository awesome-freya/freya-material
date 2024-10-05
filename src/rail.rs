use freya::prelude::*;

use crate::ripple::Ripple;
use shared::THEME;
use shared::{ColorConversion, Direction, WithSpacing};

#[component]
fn NavigationRailItem() -> Element {
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
                background: "{THEME.secondary_container}",
                overflow: "clip",

                rect {
                    position: "absolute",
                    height: "32",
                    width: "56",

                    Ripple {
                        color: THEME.on_secondary_container,
                        height: "32",
                        width: "56",
                    }
                }

                label {
                    color: "{THEME.on_secondary_container}",
                    font_size: "24",

                    "RE"
                }
            }

            label {
                margin: "4 0 0 0",
                color: "{THEME.on_surface}",

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

#[derive(Props, PartialEq, Clone)]
pub struct NavigationRailProps {
    items_align: Option<RailItemAlignment>,
}

#[component]
pub fn NavigationRail(props: NavigationRailProps) -> Element {
    let align = props.items_align.unwrap_or_default();
    let align = match align {
        RailItemAlignment::Top => "start",
        RailItemAlignment::Center => "center",
        RailItemAlignment::Bottom => "end",
    };

    rsx! {
        rect {
            height: "100%",
            width: "80",
            background: "{THEME.surface_container_highest}",
            main_align: align,
            padding: "24 0 24 0",
            cross_align: "center",

            WithSpacing {
                spacing: 12,
                direction: Direction::Vertical,
                elements: [
                    rsx!(NavigationRailItem {}),
                    rsx!(NavigationRailItem {}),
                    rsx!(NavigationRailItem {}),
                ]
            }
        }
    }
}
