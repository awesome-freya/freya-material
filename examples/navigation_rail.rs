use freya::prelude::*;
use freya_material::prelude::*;

fn main() {
    launch_cfg(
        App,
        LaunchConfig::<()>::new()
            .with_title("Component: Navigation Rail")
            .with_size(200., 200.)
            .with_roboto(),
    );
}

#[component]
fn App() -> Element {
    let theme = use_material_theme();
    let theme = theme.read();

    let mut current_value = use_signal(usize::default);

    rsx! {
        Surface {
            direction: "horizontal",
            background: "{theme.surface}",
            color: "{theme.on_surface}",
            width: "fill",
            height: "fill",

            NavigationRail {
                container_color: "{theme.surface_container}",

                for item in 0..4 {
                    NavigationRailItem {
                        active: current_value() == item,
                        icon: IconData {
                            name: "star",
                            ..Default::default()
                        },
                        label: if item != 3 {
                            Some("Stargazers".to_string())
                        } else {
                            None
                        },
                        on_click: move |_| current_value.set(item),
                        always_show_label: false
                    }
                }
            }
        }
    }
}
