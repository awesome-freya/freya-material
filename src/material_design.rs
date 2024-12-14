#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Elevation {
    Level0,
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
}

impl Elevation {
    pub fn as_value(&self) -> u8 {
        match self {
            Self::Level0 => 0,
            Self::Level1 => 1,
            Self::Level2 => 3,
            Self::Level3 => 6,
            Self::Level4 => 8,
            Self::Level5 => 12,
        }
    }

    pub fn into_value(self) -> u8 {
        self.as_value()
    }

    // Code taken from https://github.com/material-components/material-web/blob/main/elevation/internal/_elevation.scss
    pub fn as_shadow(&self) -> String {
        let level = i32::from(self.as_value());

        let (y1, blur1, color1) = {
            let level1_y = level.clamp(0, 1);
            let level4_y = (level - 3).clamp(0, 1);
            let level5_y = 2 * (level - 4).clamp(0, 1);

            let level1_blur = 2 * level.clamp(0, 1);
            let level3_blur = (level - 2).clamp(0, 1);
            let level5_blur = (level - 4).clamp(0, 1);

            (
                level1_y + level4_y + level5_y,
                level1_blur + level3_blur + level5_blur,
                "rgb(0, 0, 0, 0.3)",
            )
        };

        let (y2, blur2, spread, color2) = {
            let level1_y = level.clamp(0, 1);
            let level2_y = (level - 1).clamp(0, 1);
            let level3to5_y = 2 * (level - 2).clamp(0, 3);
            let level1to2_blur = 3 * level.clamp(0, 2);
            let level3to5_blur = 2 * (level - 2).clamp(0, 3);
            let level1to4_spread = level.clamp(0, 4);
            let level5_spread = 2 * (level - 4).clamp(0, 1);

            (
                level1_y + level2_y + level3to5_y,
                level1to2_blur + level3to5_blur,
                level1to4_spread + level5_spread,
                "rgb(0, 0, 0, 0.15)",
            )
        };

        format!("0 {y1} {blur1} 0 {color1}, 0 {y2} {blur2} {spread} {color2}")
    }

    pub fn into_shadow(self) -> String {
        self.as_shadow()
    }
}

pub mod motion {
    use freya_transition::Curve;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum Easing {
        Emphasized,
        EmphasizedDecelerate,
        EmphasizedAccelerate,
        Standard,
        StandardDecelerate,
        StandardAccelerate,
    }

    impl Easing {
        const EMPHASIZED: Curve = Curve::EASE_IN_OUT_CUBIC_EMPHASIZED;
        const EMPHASIZED_DECELERATE: Curve = Curve::cubic(0.05, 0.7, 0.1, 1.0);
        const EMPHASIZED_ACCELERATE: Curve = Curve::cubic(0.3, 0.0, 0.8, 0.15);
        const STANDARD: Curve = Curve::cubic(0.2, 0.0, 0.0, 1.0);
        const STANDARD_DECELERATE: Curve = Curve::cubic(0.0, 0.0, 0.0, 1.0);
        const STANDARD_ACCELERATE: Curve = Curve::cubic(0.3, 0.0, 1.0, 1.0);

        pub const fn as_value(&self) -> Curve {
            match self {
                Self::Emphasized => Self::EMPHASIZED,
                Self::EmphasizedDecelerate => Self::EMPHASIZED_DECELERATE,
                Self::EmphasizedAccelerate => Self::EMPHASIZED_ACCELERATE,
                Self::Standard => Self::STANDARD,
                Self::StandardDecelerate => Self::STANDARD_DECELERATE,
                Self::StandardAccelerate => Self::STANDARD_ACCELERATE,
            }
        }

        pub const fn into_value(self) -> Curve {
            self.as_value()
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum EasingDuration {
        Short,
        Medium,
        Long,
        ExtraLong,
    }

    impl EasingDuration {
        pub const fn as_value(&self) -> [u64; 4] {
            match self {
                Self::Short => [50, 100, 150, 200],
                Self::Medium => [250, 300, 350, 400],
                Self::Long => [450, 500, 550, 600],
                Self::ExtraLong => [700, 800, 900, 1000],
            }
        }

        pub const fn into_value(self) -> [u64; 4] {
            self.as_value()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Shape {
    None,
    ExtraSmall,
    ExtraSmallTop,
    Small,
    Medium,
    Large,
    LargeStart,
    LargeEnd,
    LargeTop,
    ExtraLarge,
    ExtraLargeTop,
    Full,
}

impl Shape {
    pub const fn as_value(&self) -> &'static str {
        match self {
            Self::None => "0",
            Self::ExtraSmall => "4",
            Self::ExtraSmallTop => "4 4 0 0",
            Self::Small => "8",
            Self::Medium => "12",
            Self::Large => "16",
            Self::LargeStart => "16 0 0 16",
            Self::LargeEnd => "0 16 16 0",
            Self::LargeTop => "16 16 0 0",
            Self::ExtraLarge => "28",
            Self::ExtraLargeTop => "28 28 0 0",
            Self::Full => "9999",
        }
    }

    pub const fn into_value(self) -> &'static str {
        self.as_value()
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TypescaleVariant {
    Display,
    Headline,
    Title,
    Body,
    Label,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TypescaleSize {
    Large,
    Medium,
    Small,
}

/// (Family, Weight, Size, Tracking (?), Line Height)
pub type Typescale = (&'static str, usize, usize, f32, usize);

pub(crate) fn get_type_scale(
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
