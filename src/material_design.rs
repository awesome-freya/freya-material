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
