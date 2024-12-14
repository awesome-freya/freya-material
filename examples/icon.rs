use freya::prelude::*;
use freya_material::prelude::*;

fn main() {
    launch_cfg(
        App,
        LaunchConfig::<()>::new()
            .with_title("Component: Icon")
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

            Icon {
                name: "star",
                color: "red",
                width: "64",
                height: "64",
            }

            Icon {
                name: "star",
                color: "red",
                filled: true,
                width: "64",
                height: "64",
            }
        }
    }
}
