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
        rect {
            direction: "vertical",
            spacing: "12",
            background: "{theme.surface}",
            color: "{theme.on_surface}",
            width: "fill",
            height: "fill",

            for variant in [
                material_design::TypescaleVariant::Display,
                material_design::TypescaleVariant::Headline,
                material_design::TypescaleVariant::Title,
                material_design::TypescaleVariant::Body,
                material_design::TypescaleVariant::Label
            ] {
                for size in [
                    material_design::TypescaleSize::Large,
                    material_design::TypescaleSize::Medium,
                    material_design::TypescaleSize::Small
                ] {
                    if variant == material_design::TypescaleVariant::Label && size != material_design::TypescaleSize::Small {
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
