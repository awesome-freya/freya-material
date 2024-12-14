use freya::prelude::*;
use freya_material::prelude::*;

fn main() {
    launch_cfg(
        App,
        LaunchConfig::<()>::new()
            .with_title("Material Design: Elevation")
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
            direction: "horizontal",
            padding: "12",
            spacing: "24",
            background: "{theme.surface_bright}",
            color: "{theme.on_surface}",
            width: "fill",
            height: "fill",

            for (i, elevation) in [
                Elevation::Level0,
                Elevation::Level1,
                Elevation::Level2,
                Elevation::Level3,
                Elevation::Level4,
                Elevation::Level5
            ].into_iter().enumerate() {
                Surface {
                    direction: "vertical",
                    spacing: "8",

                    Typography {
                        "Elevation level {i}"
                    }

                    Surface {
                        background: "{theme.primary}",
                        width: "92",
                        height: "92",
                        shape: Shape::Medium,
                        elevation,
                    }
                }
            }
        }
    }
}
