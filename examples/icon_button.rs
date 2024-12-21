use freya::prelude::{
    component, dioxus_core, dioxus_elements, fc_to_builder, launch_cfg, rsx, use_signal, Element,
    GlobalSignal, IntoDynNode, LaunchConfig, Readable, Writable,
};
use freya_material::prelude::*;

fn main() {
    launch_cfg(
        App,
        LaunchConfig::<()>::new()
            .with_title("Component: Icon Button")
            .with_size(200., 200.)
            .with_roboto(),
    );
}

#[component]
fn App() -> Element {
    let theme = use_material_theme();
    let theme = theme.read();

    let mut selected = use_signal(bool::default);

    rsx! {
        Surface {
            direction: "vertical",
            padding: "12",
            spacing: "24",
            background: "{theme.surface}",
            color: "{theme.on_surface}",
            width: "fill",
            height: "fill",

            for style in [
                IconButtonStyle::Standard,
                IconButtonStyle::Filled,
                IconButtonStyle::FilledTonal,
                IconButtonStyle::Outlined,
            ] {
                rect {
                    direction: "horizontal",
                    spacing: "8",

                    IconButton {
                        style,
                        icon: IconData {
                            name: "star",
                            filled: true,
                            ..Default::default()
                        },
                        on_click: |_| { },
                    }

                    IconButton {
                        style,
                        icon: IconData {
                            name: "star",
                            filled: true,
                            ..Default::default()
                        },
                        on_click: |_| { },
                        disabled: true
                    }

                    IconButton {
                        style,
                        selected: *selected.read(),
                        on_click: move |_| {
                            selected.toggle();
                        },
                        icon: IconData {
                            name: "star",
                            filled: true,
                            ..Default::default()
                        },
                    }
                }
            }
        }
    }
}
