use crate::prelude::*;
use freya::prelude::*;
use freya_transition::{use_transition, Curve};

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum SwitchState {
    #[default]
    Idle,
    Hovering,
    Pressed,
}

#[component]
pub fn Switch(toggled: bool, on_toggle: EventHandler<()>) -> Element {
    let theme = use_material_theme();
    let theme = theme.read();

    let mut state = use_signal(SwitchState::default);

    let (background, handle_color, border_color) = if toggled {
        (theme.primary, theme.on_primary, theme.primary)
    } else {
        (
            theme.surface_container_highest,
            theme.outline,
            theme.outline,
        )
    };

    let radius_transition = use_transition(move |context| {
        context.add_tween(
            "radius",
            if toggled { 24.0 } else { 16.0 },
            Easing::Standard.as_value(),
            250,
        );
    });

    let offset_transition = use_transition(move |context| {
        context.add_tween(
            "offset",
            if toggled { 21.0 } else { 0.0 },
            Curve::cubic(0.175, 0.885, 0.32, 1.275),
            300,
        );
    });

    let onpointerenter = move |_| state.set(SwitchState::Hovering);
    let onpointerleave = move |_| state.set(SwitchState::Idle);
    let onpointerdown = move |_| state.set(SwitchState::Pressed);
    let onpointerup = move |_| {
        state.set(SwitchState::Hovering);

        on_toggle(());
    };

    let radius = radius_transition.get::<f32>("radius");
    let offset = offset_transition.get::<f32>("offset");

    use_effect(use_reactive!(|toggled| {
        offset_transition.play([("offset", if toggled { 21.0 } else { 0.0 })]);
    }));

    use_effect(use_reactive!(|state| {
        match *state.read() {
            SwitchState::Idle => {
                radius_transition.play([("radius", if toggled { 24.0 } else { 16.0 })])
            }
            SwitchState::Hovering => radius_transition.play([("radius", 24.0)]),
            SwitchState::Pressed => radius_transition.play([("radius", 28.0)]),
        }
    }));

    rsx! {
        rect {
            height: "32",
            width: "52",
            corner_radius: "16",
            background: "{background}",
            border: "2 inner {border_color}",
            padding: "2",

            onpointerenter,
            onpointerleave,
            onpointerup,
            onpointerdown,

            rect {
                offset_x: "{offset}",
                width: "28",
                height: "28",
                main_align: "center",
                cross_align: "center",

                rect {
                    width: "{radius}",
                    height: "{radius}",
                    corner_radius: "{radius / 2.0}",
                    background: "{handle_color}",
                }
            }
        }
    }
}
