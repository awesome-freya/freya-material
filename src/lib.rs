pub mod button;
pub mod fab;
pub mod icon_button;
pub mod rail;
pub mod ripple;
pub mod search;
pub mod side_sheet;
pub mod switch;

pub use self::{
    button::{Button, ButtonKind},
    fab::FAB,
    icon_button::{IconButton, IconButtonKind},
    rail::{NavigationRail, RailItemAlignment},
    ripple::Ripple,
    search::Search,
    side_sheet::{SideSheet, SideSheetKind},
    switch::Switch,
};

use freya::prelude::{try_use_context, use_context_provider, Signal, Writable};
use material_colors::{color::Argb, scheme::Scheme, theme::ThemeBuilder};

pub fn use_theme() -> Signal<Scheme> {
    match try_use_context::<Signal<Scheme>>() {
        Some(value) => value,
        None => use_context_provider(|| {
            Signal::new(
                ThemeBuilder::with_source(Argb::from_u32(0xFFBC0D))
                    .build()
                    .schemes
                    .dark,
            )
        }),
    }
}

pub fn set_theme(scheme: Scheme) {
    *use_theme().write() = scheme;
}
