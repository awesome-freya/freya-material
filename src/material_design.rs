/// Levels: (0, 1, 2, 3, 4, 5)
pub const ELEVATION: [u8; 6] = [0, 1, 3, 6, 8, 12];

pub mod motion {
    pub mod easing {
        use freya_transition::Curve;

        pub const EMPHASIZED: Curve = Curve::EASE_IN_OUT_CUBIC_EMPHASIZED;
        pub const EMPHASIZED_DECELERATE: Curve = Curve::cubic(0.05, 0.7, 0.1, 1.0);
        pub const EMPHASIZED_ACCELERATE: Curve = Curve::cubic(0.3, 0.0, 0.8, 0.15);

        pub const STANDARD: Curve = Curve::cubic(0.2, 0.0, 0.0, 1.0);
        pub const STANDARD_DECELERATE: Curve = Curve::cubic(0.0, 0.0, 0.0, 1.0);
        pub const STANDARD_ACCELERATE: Curve = Curve::cubic(0.3, 0.0, 1.0, 1.0);
    }

    pub mod duration {
        pub const SHORT: [u64; 4] = [50, 100, 150, 200];
        pub const MEDIUM: [u64; 4] = [250, 300, 350, 400];
        pub const LONG: [u64; 4] = [450, 500, 550, 600];
        pub const EXTRA_LONG: [u64; 4] = [700, 800, 900, 1000];
    }
}

pub mod shape {
    pub const NONE: &str = "0";
    pub const EXTRA_SMALL: &str = "4";
    pub const EXTRA_SMALL_TOP: &str = "4 4 0 0";
    pub const SMALL: &str = "8";
    pub const MEDIUM: &str = "12";
    pub const LARGE: &str = "16";
    pub const LARGE_START: &str = "16 0 0 16";
    pub const LARGE_END: &str = "0 16 16 0";
    pub const LARGE_TOP: &str = "16 16 0 0";
    pub const EXTRA_LARGE: &str = "28";
    pub const EXTRA_LARGE_TOP: &str = "28 28 0 0";
    pub const FULL: &str = "9999";
}

mod typescale {
    use super::Typescale;

    pub(super) mod display {
        use super::Typescale;

        pub const LARGE: Typescale = ("Roboto", 400, 57, -0.25, 64);
        pub const MEDIUM: Typescale = ("Roboto", 400, 45, 0.0, 52);
        pub const SMALL: Typescale = ("Roboto", 400, 36, 0.0, 44);
    }

    pub(super) mod headline {
        use super::Typescale;

        pub const LARGE: Typescale = ("Roboto", 400, 32, 0.0, 40);
        pub const MEDIUM: Typescale = ("Roboto", 400, 28, 0.0, 36);
        pub const SMALL: Typescale = ("Roboto", 400, 24, 0.0, 32);
    }

    pub(super) mod title {
        use super::Typescale;

        pub const LARGE: Typescale = ("Roboto", 400, 22, 0.0, 28);
        pub const MEDIUM: Typescale = ("Roboto", 500, 16, 0.15, 24);
        pub const SMALL: Typescale = ("Roboto", 500, 14, 0.1, 20);
    }

    pub(super) mod body {
        use super::Typescale;

        pub const LARGE: Typescale = ("Roboto", 400, 16, 0.5, 24);
        pub const MEDIUM: Typescale = ("Roboto", 400, 14, 0.25, 20);
        pub const SMALL: Typescale = ("Roboto", 400, 12, 0.4, 16);
    }

    pub(super) mod label {
        use super::Typescale;

        pub const LARGE: Typescale = ("Roboto", 500, 14, 0.1, 20);
        pub const LARGE_PROMINENT: Typescale = ("Roboto", 700, 14, 0.1, 20);
        pub const MEDIUM: Typescale = ("Roboto", 500, 12, 0.5, 16);
        pub const MEDIUM_PROMINENT: Typescale = ("Roboto", 700, 12, 0.5, 16);
        pub const SMALL: Typescale = ("Roboto", 500, 11, 0.5, 16);
    }
}

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
pub type Typescale = (&'static str, usize, usize, f32, usize);

pub fn get_type_scale(
    variant: TypescaleVariant,
    size: TypescaleSize,
    prominent: bool,
) -> Typescale {
    match (variant, size, prominent) {
        (TypescaleVariant::Display, TypescaleSize::Large, _) => typescale::display::LARGE,
        (TypescaleVariant::Display, TypescaleSize::Medium, _) => typescale::display::MEDIUM,
        (TypescaleVariant::Display, TypescaleSize::Small, _) => typescale::display::SMALL,

        (TypescaleVariant::Headline, TypescaleSize::Large, _) => typescale::headline::LARGE,
        (TypescaleVariant::Headline, TypescaleSize::Medium, _) => typescale::headline::MEDIUM,
        (TypescaleVariant::Headline, TypescaleSize::Small, _) => typescale::headline::SMALL,

        (TypescaleVariant::Title, TypescaleSize::Large, _) => typescale::title::LARGE,
        (TypescaleVariant::Title, TypescaleSize::Medium, _) => typescale::title::MEDIUM,
        (TypescaleVariant::Title, TypescaleSize::Small, _) => typescale::title::SMALL,

        (TypescaleVariant::Body, TypescaleSize::Large, _) => typescale::body::LARGE,
        (TypescaleVariant::Body, TypescaleSize::Medium, _) => typescale::body::MEDIUM,
        (TypescaleVariant::Body, TypescaleSize::Small, _) => typescale::body::SMALL,

        (TypescaleVariant::Label, TypescaleSize::Large, true) => typescale::label::LARGE_PROMINENT,
        (TypescaleVariant::Label, TypescaleSize::Large, false) => typescale::label::LARGE,
        (TypescaleVariant::Label, TypescaleSize::Medium, true) => {
            typescale::label::MEDIUM_PROMINENT
        }
        (TypescaleVariant::Label, TypescaleSize::Medium, false) => typescale::label::MEDIUM,
        (TypescaleVariant::Label, TypescaleSize::Small, _) => typescale::label::SMALL,
    }
}
