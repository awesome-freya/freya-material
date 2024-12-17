use freya::prelude::*;
use freya_material::prelude::{use_material_theme, LaunchConfigExt, Surface, Switch as MSwitch};

fn main() {
    launch_cfg(
        App,
        LaunchConfig::<()>::new()
            .with_title("Component: Switch")
            .with_size(200., 200.)
            .with_roboto(),
    );
}

#[component]
fn App() -> Element {
    let theme = use_material_theme();
    let theme = theme.read();

    let mut toggled = use_signal(bool::default);

    rsx! {
        Surface {
            direction: "vertical",
            spacing: "12",
            background: "{theme.surface}",
            color: "{theme.on_surface}",
            width: "fill",
            height: "fill",

            MSwitch {
                toggled: toggled(),
                on_toggle: move |()| toggled.toggle(),
            }
        }
    }
}
