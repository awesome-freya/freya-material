use freya::prelude::*;
use freya_material::prelude::*;

fn main() {
    launch_cfg(
        App,
        LaunchConfig::<()>::new()
            .with_title("Component: Radio Button")
            .with_size(200., 200.)
            .with_roboto(),
    );
}

#[component]
fn App() -> Element {
    let theme = use_material_theme();
    let theme = theme.read();

    let mut current_value = use_signal(|| 9);

    rsx! {
        Surface {
            direction: "vertical",
            padding: "12",
            spacing: "8",
            background: "{theme.surface}",
            color: "{theme.on_surface}",
            width: "fill",
            height: "fill",

            for i in 0..10 {
                Surface {
                    direction: "horizontal",
                    cross_align: "center",
                    spacing: "8",

                    if i == 8 || i == 9 {
                        RadioButton {
                            selected: current_value() == i,
                            disabled: true,
                            on_click: move |_| current_value.set(i),
                        }
                    } else {
                        RadioButton {
                            selected: current_value() == i,
                            on_click: move |_| current_value.set(i),
                        }
                    }

                    Typography {
                        "Variant {i + 1}"
                    }
                }
            }
        }
    }
}
