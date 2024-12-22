use crate::prelude::*;
use freya::prelude::*;
use freya_transition::use_transition;

#[component]
pub fn RadioButton(
    selected: bool,
    on_click: EventHandler<MouseEvent>,
    #[props(default)] disabled: bool,
) -> Element {
    let theme = use_material_theme();
    let theme = theme.read();

    let mut hovered = use_signal(bool::default);

    let color = if disabled {
        theme.on_surface.with_alpha_f32(0.38)
    } else if selected {
        theme.primary
    } else if *hovered.read() {
        theme.on_surface
    } else {
        theme.on_surface_variant
    }
    .as_rgba();

    let onpointerenter = move |_| hovered.set(true);
    let onpointerleave = move |_| hovered.set(false);

    let radius_transition = use_transition(move |context| {
        context.add_tween(
            "radius",
            0.0,
            Easing::EmphasizedDecelerate.as_value(),
            EasingDuration::Medium.as_value()[2],
        );
    });

    let radius = radius_transition.get::<f32>("radius");

    use_effect(use_reactive!(|selected| {
        if selected {
            radius_transition.play([("radius", 8.0)]);
        } else {
            radius_transition.play([("radius", 0.0)]);
        }
    }));

    rsx! {
        rect {
            height: "40",
            width: "40",
            padding: "10",
            corner_radius: "20",
            overflow: "clip",

            onpointerenter,
            onpointerleave,
            onclick: move |data| if !disabled {
                on_click.call(data);
            },

            if !disabled {
                StateLayer {
                    color: color.as_str(),
                    width: "40",
                    height: "40",
                    position_left: "-10",
                    position_top: "-10",
                }
            }

            rect {
                height: "20",
                width: "20",
                main_align: "center",
                cross_align: "center",
                corner_radius: "10",
                border: "2 inner {color}",

                rect {
                    width: "{radius}",
                    height: "{radius}",
                    corner_radius: "{radius / 2.0}",
                    background: color.as_str(),
                }
            }
        }
    }
}
