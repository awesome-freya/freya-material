use freya::prelude::*;
use freya_material::prelude::*;

fn main() {
    launch_cfg(
        App,
        LaunchConfig::<()>::new()
            .with_title("Component: State Layer")
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
            width: "fill",
            height: "fill",
            background: "{theme.surface}",
            spacing: "8",
            padding: "12",

            Surface {
                height: "128",
                width: "192",
                background: "{theme.primary_container}",
                color: "{theme.on_primary_container}",
                shape: Shape::Large,
                main_align: "center",
                cross_align: "center",
                overflow: "clip",

                StateLayer {
                    color: "{theme.on_primary_container}"
                }

                Typography { "It's state layer!" }
            }

            Surface {
                height: "48",
                width: "48",
                background: "{theme.primary_container}",
                color: "{theme.on_primary_container}",
                shape: Shape::Large,
                main_align: "center",
                cross_align: "center",
                overflow: "clip",

                StateLayer {
                    color: "{theme.on_primary_container}"
                }

                Typography { "Small" }
            }
        }
    }
}
