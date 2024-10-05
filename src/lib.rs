pub mod button;
pub mod fab;
pub mod icon_button;
pub mod rail;
pub mod ripple;
pub mod search;
pub mod side_sheet;
pub mod switch;

use freya::prelude::*;
use icon_button::{IconButton, IconButtonKind};
use material_colors::{dynamic_color::Variant, image::ImageReader, theme::ThemeBuilder};
use rail::{NavigationRail, RailItemAlignment};
use ripple::Ripple;
use button::{Button, ButtonKind};
use search::Search;
use shared::THEME;
use switch::Switch;
use std::fs;

pub mod icons {
    const SETTINGS: &[u8] = include_bytes!("../../../assets/icons/outlined/settings.svg");
    const SETTINGS_FILLED: &[u8] = include_bytes!("../../../assets/icons/filled/settings.svg");

    #[must_use]
    pub const fn settings(filled: bool) -> &'static [u8] {
        if filled {
            SETTINGS_FILLED
        } else {
            SETTINGS
        }
    }
}

#[derive(Props, PartialEq, Eq, Clone)]
struct TrackProps {
    artist: String,
    title: String,
    image: String,
}

#[component]
fn TrackCard(props: TrackProps) -> Element {
    let image = fs::read(&props.image).expect("failed to read image file");
    let theme_image = ImageReader::read(&image).expect("failed to read image data");
    let color = ImageReader::extract_color(&theme_image);
    let theme = ThemeBuilder::with_source(color)
        .variant(Variant::Content)
        .build()
        .schemes
        .dark;
    let image_data = dynamic_bytes(image);

    rsx! {
        rect {
            background: "{theme.surface_container}",
            corner_radius: "12",
            width: "240",
            height: "80",
            overflow: "clip",

            rect {
                direction: "horizontal",
                padding: "4",

                rect {
                    overflow: "clip",
                    width: "72",
                    height: "72",
                    corner_radius: "12",
    
                    image {
                        image_data,
                        width: "72",
                        height: "72",
                    }
                }
    
                rect {
                    direction: "vertical",
                    margin: "0 4 0 4",
    
                    label {
                        color: "{theme.on_surface}",
                        font_weight: "bold",
                        font_size: "24",
    
                        {props.title.as_str()}
                    }
    
                    label {
                        color: "{theme.on_surface_variant}",
                        font_size: "18",
    
                        {props.artist.as_str()}
                    }
                }
            }

            Ripple {
                color: theme.on_surface,
                width: "240",
                height: "80",
            }
        }
    }
}

pub fn gallery() -> Element {
    let mut value = use_signal(String::default);
    let mut enabled = use_signal(bool::default);

    rsx! {
        rect {
            height: "100%",
            width: "100%",
            background: "{THEME.surface}",
            direction: "horizontal",

            NavigationRail {
                items_align: RailItemAlignment::Top,
            }

            rect {
                height: "fill",
                width: "fill",
                direction: "vertical",
                padding: "8",
                spacing: "8",
                background: "{THEME.surface_container_low}",

                Search {
                    value: value.read().clone(),
                    placeholder: "Enter text here...",
                    onchange: move |text| *value.write() = text,
                }

                TrackCard {
                    title: "CURFEW",
                    artist: "SLYKT",
                    image: "/run/media/aiving/Drive/dev/redo-of-healer/assets/m100x100.jpeg",
                }

                Switch {
                    enabled: *enabled.read(),
                    ontoggled: move |()| enabled.toggle(),
                }

                rect {
                    direction: "horizontal",
                    spacing: "8",

                    Button {
                        kind: ButtonKind::Elevated,
                        onclick: move |()| {},
    
                        "hi!!"
                    }
    
                    Button {
                        kind: ButtonKind::Filled,
                        onclick: move |()| {},
    
                        "hi!!"
                    }
    
                    Button {
                        kind: ButtonKind::Outlined,
                        onclick: move |()| {},
    
                        "hi!!"
                    }
    
                    Button {
                        kind: ButtonKind::Tonal,
                        onclick: move |()| {},
    
                        "hi!!"
                    }
    
    
                    Button {
                        kind: ButtonKind::Text,
                        onclick: move |()| {},
    
                        "hi!!"
                    }    
                }

                rect {
                    direction: "horizontal",
                    spacing: "8",

                    IconButton {
                        kind: IconButtonKind::Standard,
                        selected: *enabled.read(),
                        onclick: move |()| {},
                        icon: "settings",
                    }

                    IconButton {
                        kind: IconButtonKind::Filled,
                        selected: *enabled.read(),
                        onclick: move |()| {},
                        icon: "settings",
                    }

                    IconButton {
                        kind: IconButtonKind::Tonal,
                        selected: *enabled.read(),
                        onclick: move |()| {},
                        icon: "settings",
                    }

                    IconButton {
                        kind: IconButtonKind::Outlined,
                        selected: *enabled.read(),
                        onclick: move |()| {},
                        icon: "settings",
                    }
                }
            }
        }
    }
}
