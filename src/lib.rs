pub(crate) mod components;
pub mod material_design;

pub mod prelude {
    pub use crate::{set_material_theme, use_material_theme, ArgbExt, material_design};
}

use freya::prelude::{try_use_context, use_context_provider, Signal, Writable};
use material_colors::{color::Argb, scheme::Scheme, theme::ThemeBuilder};

pub trait ArgbExt {
    fn as_rgba(&self) -> String;
}

impl ArgbExt for Argb {
    fn as_rgba(&self) -> String {
        format!(
            "rgb({}, {}, {}, {})",
            self.red, self.blue, self.green, self.alpha
        )
    }
}

pub fn use_material_theme() -> Signal<Scheme> {
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

pub fn set_material_theme(scheme: Scheme) {
    *use_material_theme().write() = scheme;
}
