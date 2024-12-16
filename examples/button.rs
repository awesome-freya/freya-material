use freya::prelude::{
    component, dioxus_core, dioxus_elements, fc_to_builder, launch_cfg, rsx, Element, GlobalSignal,
    IntoDynNode, LaunchConfig, Readable,
};
use freya_material::prelude::*;

fn main() {
    launch_cfg(
        App,
        LaunchConfig::<()>::new()
            .with_title("Component: Button")
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
            padding: "12",
            spacing: "24",
            background: "{theme.surface_container_highest}",
            color: "{theme.on_surface}",
            width: "fill",
            height: "fill",

            for style in [
                ButtonStyle::Elevated,
                ButtonStyle::Filled,
                ButtonStyle::FilledTonal,
                ButtonStyle::Outlined,
                ButtonStyle::Text
            ] {
                rect {
                    direction: "horizontal",
                    spacing: "8",

                    Button {
                        style,

                        "Hello world!"
                    }

                    Button {
                        style,
                        icon: IconData {
                            name: "star",
                            filled: true,
                            ..Default::default()
                        },

                        "Hello world!"
                    }
                }
            }
        }
    }
}
