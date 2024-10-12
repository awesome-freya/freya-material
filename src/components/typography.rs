mod type_scale {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum TypescaleVariant {
        Display,
        Headline,
        Title,
        Body,
        Label,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum TypescaleSize {
        Large,
        Medium,
        Small,
    }

    /// (Family, Weight, Size, Tracking (?), Line Height)
    pub(super) type Typescale = (&'static str, usize, usize, f32, usize);

    pub(super) fn get_type_scale(
        variant: TypescaleVariant,
        size: TypescaleSize,
        prominent: bool,
    ) -> Typescale {
        match (variant, size, prominent) {
            (TypescaleVariant::Display, TypescaleSize::Large, _) => self::display::LARGE,
            (TypescaleVariant::Display, TypescaleSize::Medium, _) => self::display::MEDIUM,
            (TypescaleVariant::Display, TypescaleSize::Small, _) => self::display::SMALL,

            (TypescaleVariant::Headline, TypescaleSize::Large, _) => self::headline::LARGE,
            (TypescaleVariant::Headline, TypescaleSize::Medium, _) => self::headline::MEDIUM,
            (TypescaleVariant::Headline, TypescaleSize::Small, _) => self::headline::SMALL,

            (TypescaleVariant::Title, TypescaleSize::Large, _) => self::title::LARGE,
            (TypescaleVariant::Title, TypescaleSize::Medium, _) => self::title::MEDIUM,
            (TypescaleVariant::Title, TypescaleSize::Small, _) => self::title::SMALL,

            (TypescaleVariant::Body, TypescaleSize::Large, _) => self::body::LARGE,
            (TypescaleVariant::Body, TypescaleSize::Medium, _) => self::body::MEDIUM,
            (TypescaleVariant::Body, TypescaleSize::Small, _) => self::body::SMALL,

            (TypescaleVariant::Label, TypescaleSize::Large, true) => self::label::LARGE_PROMINENT,
            (TypescaleVariant::Label, TypescaleSize::Large, false) => self::label::LARGE,
            (TypescaleVariant::Label, TypescaleSize::Medium, true) => self::label::MEDIUM_PROMINENT,
            (TypescaleVariant::Label, TypescaleSize::Medium, false) => self::label::MEDIUM,
            (TypescaleVariant::Label, TypescaleSize::Small, _) => self::label::SMALL,
        }
    }

    mod display {
        use super::Typescale;

        pub const LARGE: Typescale = ("Roboto", 400, 57, -0.25, 64);
        pub const MEDIUM: Typescale = ("Roboto", 400, 45, 0.0, 52);
        pub const SMALL: Typescale = ("Roboto", 400, 36, 0.0, 44);
    }

    mod headline {
        use super::Typescale;

        pub const LARGE: Typescale = ("Roboto", 400, 32, 0.0, 40);
        pub const MEDIUM: Typescale = ("Roboto", 400, 28, 0.0, 36);
        pub const SMALL: Typescale = ("Roboto", 400, 24, 0.0, 32);
    }

    mod title {
        use super::Typescale;

        pub const LARGE: Typescale = ("Roboto", 400, 22, 0.0, 28);
        pub const MEDIUM: Typescale = ("Roboto", 500, 16, 0.15, 24);
        pub const SMALL: Typescale = ("Roboto", 500, 14, 0.1, 20);
    }

    mod body {
        use super::Typescale;

        pub const LARGE: Typescale = ("Roboto", 400, 16, 0.5, 24);
        pub const MEDIUM: Typescale = ("Roboto", 400, 14, 0.25, 20);
        pub const SMALL: Typescale = ("Roboto", 400, 12, 0.4, 16);
    }

    mod label {
        use super::Typescale;

        pub const LARGE: Typescale = ("Roboto", 500, 14, 0.1, 20);
        pub const LARGE_PROMINENT: Typescale = ("Roboto", 700, 14, 0.1, 20);
        pub const MEDIUM: Typescale = ("Roboto", 500, 12, 0.5, 16);
        pub const MEDIUM_PROMINENT: Typescale = ("Roboto", 700, 12, 0.5, 16);
        pub const SMALL: Typescale = ("Roboto", 500, 11, 0.5, 16);
    }
}

use freya::prelude::*;
use type_scale::get_type_scale;
pub use type_scale::{TypescaleSize, TypescaleVariant};

#[component]
pub fn Typography(
    #[props(default = TypescaleVariant::Body)] variant: TypescaleVariant,
    #[props(default = TypescaleSize::Medium)] size: TypescaleSize,
    #[props(default)] prominent: bool,
    children: Element,
) -> Element {
    let (font_family, weight, size, _, line_height) = get_type_scale(variant, size, prominent);

    rsx! {
        label {
            font_family,
            font_weight: "{weight}",
            font_size: "{size}",
            line_height: "{line_height / size}",

            {children}
        }
    }
}
