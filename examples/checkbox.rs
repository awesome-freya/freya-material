use freya::prelude::{
    component, dioxus_core, dioxus_elements, fc_to_builder, launch_cfg, rsx, use_signal, Element,
    GlobalSignal, IntoDynNode, LaunchConfig, Readable, Writable,
};
use freya_material::prelude::*;

fn main() {
    launch_cfg(
        App,
        LaunchConfig::<()>::new()
            .with_title("Component: Checkbox")
            .with_size(200., 200.)
            .with_roboto(),
    );
}

#[component]
fn App() -> Element {
    let theme = use_material_theme();
    let theme = theme.read();

    let mut previous_state = use_signal(CheckboxState::default);
    let mut state = use_signal(CheckboxState::default);

    rsx! {
        Surface {
            direction: "vertical",
            padding: "12",
            spacing: "24",
            background: "{theme.surface}",
            color: "{theme.on_surface}",
            width: "fill",
            height: "fill",

            Checkbox {
                state: CheckboxState::from_bool(state() == CheckboxState::Intermediate),
                on_click: move |_| {
                    if state() == CheckboxState::Intermediate {
                        state.set(previous_state());
                    } else {
                        state.set(CheckboxState::Intermediate);
                    }
                },
            }

            Checkbox {
                state: state(),
                on_click: move |_| {
                    let value = state().into_bool();
                    let value = CheckboxState::from_bool(!value);

                    previous_state.set(value);
                    state.set(value);
                },
            }

            Checkbox {
                state: CheckboxState::Intermediate,
                on_click: |_| { },
            }

            Checkbox {
                state: CheckboxState::Checked,
                on_click: |_| { },
            }

            Checkbox {
                state: state(),
                on_click: |_| { },
                error: true,
            }

            Checkbox {
                state: state(),
                on_click: |_| { },
                disabled: true,
            }
        }
    }
}
