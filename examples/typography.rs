use freya::prelude::*;
use freya_material::prelude::*;

fn main() {
    launch_cfg(
        App,
        LaunchConfig::<()>::new()
            .with_title("Component: Typography")
            .with_size(200., 200.)
            .with_roboto(),
    );
}

#[component]
fn App() -> Element {
    let theme = use_material_theme();
    let theme = theme.read();

    rsx! {
        Surface {
            direction: "vertical",
            spacing: "12",
            background: "{theme.surface}",
            color: "{theme.on_surface}",
            width: "fill",
            height: "fill",

            for variant in [
                TypescaleVariant::Display,
                TypescaleVariant::Headline,
                TypescaleVariant::Title,
                TypescaleVariant::Body,
                TypescaleVariant::Label
            ] {
                for size in [
                    TypescaleSize::Large,
                    TypescaleSize::Medium,
                    TypescaleSize::Small
                ] {
                    if variant == TypescaleVariant::Label && size != TypescaleSize::Small {
                        Typography {
                            variant,
                            size,
                            prominent: true,

                            "{size:?} Prominent {variant:?}"
                        }
                    }

                    Typography {
                        variant,
                        size,

                        "{size:?} {variant:?}"
                    }
                }
            }
        }
    }
}
