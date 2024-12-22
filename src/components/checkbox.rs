use crate::prelude::*;
use freya::prelude::*;
use freya_transition::{use_transition, Curve};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CheckboxState {
    #[default]
    Unchecked,
    Intermediate,
    Checked,
}

impl CheckboxState {
    fn into_curve(self) -> Curve {
        if self.into_bool() {
            Easing::EmphasizedAccelerate
        } else {
            Easing::EmphasizedDecelerate
        }
        .into_value()
    }

    fn into_duration(self) -> u64 {
        if self.into_bool() {
            150
        } else {
            350
        }
    }

    fn into_opacity(self) -> f32 {
        if self.into_bool() {
            1.0
        } else {
            0.0
        }
    }

    fn into_radius(self) -> f32 {
        if self.into_bool() {
            18.0
        } else {
            18.0 * 0.6
        }
    }

    fn into_width(self) -> f32 {
        if matches!(self, Self::Checked) {
            128.0f32.sqrt()
        } else {
            10.0
        }
    }

    fn into_height(self) -> f32 {
        if matches!(self, Self::Checked) {
            32.0f32.sqrt()
        } else {
            2.0
        }
    }

    fn into_rotation(self) -> f32 {
        if matches!(self, Self::Intermediate) {
            0.0
        } else {
            -45.0
        }
    }

    fn into_offset(self) -> Point2D {
        if matches!(self, Self::Intermediate) {
            Point2D::new(4.0, 8.0)
        } else {
            Point2D::new(5.0, 8.0)
        }
    }

    pub fn from_bool(value: bool) -> Self {
        if value {
            Self::Checked
        } else {
            Self::Unchecked
        }
    }

    pub fn into_bool(self) -> bool {
        matches!(self, Self::Checked | Self::Intermediate)
    }
}

#[component]
pub fn Checkbox(
    state: CheckboxState,
    on_click: EventHandler<MouseEvent>,
    #[props(default)] error: bool,
    #[props(default)] disabled: bool,
) -> Element {
    let theme = use_material_theme();
    let theme = theme.read();

    let mut is_previous_unchecked = use_signal(bool::default);

    let [background, color, border_color] = match [error, disabled] {
        [false, false] => [theme.primary, theme.on_primary, theme.on_surface_variant],
        [true, false] => [theme.error, theme.on_error, theme.error],
        [_, true] => [
            theme.on_surface.with_alpha_f32(0.38),
            theme.surface,
            if state.into_bool() {
                theme.on_surface.with_alpha_f32(0.0)
            } else {
                theme.on_surface.with_alpha_f32(0.38)
            },
        ],
    };

    let state_layer_color = if error {
        theme.error
    } else if state.into_bool() {
        theme.primary
    } else {
        theme.on_surface
    };

    let [background, color, border_color] = [
        background.as_rgba(),
        color.as_rgba(),
        border_color.as_rgba(),
    ];

    let rect_transition = use_transition(move |context| {
        context.add_tween(
            "radius",
            state.into_radius(),
            state.into_curve(),
            state.into_duration(),
        );
        context.add_tween("opacity", state.into_opacity(), Curve::LINEAR, 50);
    });

    let [radius, opacity] = [
        rect_transition.get::<f32>("radius"),
        rect_transition.get::<f32>("opacity"),
    ];

    let checkmark_transition = use_transition(move |context| {
        context.add_tween(
            "rotation",
            state.into_rotation(),
            state.into_curve(),
            state.into_duration(),
        );

        context.add_tween(
            "offset",
            state.into_offset(),
            state.into_curve(),
            state.into_duration(),
        );

        context.add_tween(
            "width",
            state.into_width(),
            state.into_curve(),
            state.into_duration(),
        );

        context.add_tween(
            "height",
            state.into_height(),
            state.into_curve(),
            state.into_duration(),
        );
    });

    let ([rotation, width, height], offset): ([f32; 3], _) = (
        [
            checkmark_transition.get("rotation"),
            checkmark_transition.get("width"),
            checkmark_transition.get("height"),
        ],
        checkmark_transition.get::<Point2D>("offset"),
    );

    use_effect(use_reactive!(|state| {
        match state {
            CheckboxState::Unchecked => {}
            CheckboxState::Intermediate | CheckboxState::Checked => {
                if *is_previous_unchecked.peek() && state == CheckboxState::Checked {
                    checkmark_transition.forced_set("width", 0.0);
                }

                checkmark_transition.set("rotation", state.into_rotation());
                checkmark_transition.set("offset", state.into_offset());
                checkmark_transition.set("width", state.into_width());
                checkmark_transition.set("height", state.into_height());

                checkmark_transition.play_all();
            }
        }

        rect_transition.play([
            ("radius", state.into_radius()),
            ("opacity", state.into_opacity()),
        ]);

        is_previous_unchecked.set(state == CheckboxState::Unchecked);
    }));

    rsx! {
        rect {
            direction: "horizontal",
            cross_align: "center",
            height: "40",
            width: "40",
            padding: "11",
            corner_radius: "20",
            overflow: "clip",

            onclick: move |data| if !disabled {
                on_click.call(data);
            },

            rect {
                height: "18",
                width: "18",
                border: "2 inner {border_color}",
                corner_radius: "2",
                overflow: "clip",
                main_align: "center",
                cross_align: "center",

                rect {
                    height: "{radius}",
                    width: "{radius}",
                    background,
                    opacity: "{opacity}",

                    // Short mark
                    rect {
                        width: "2",
                        height: "{height}", // sqrt(32) when checked
                        rotate: "{rotation}deg",
                        position: "absolute",
                        position_left: "{offset.x}",
                        position_top: "{offset.y}",
                        background: color.as_str(),
                    }

                    // Long mark
                    rect {
                        width: "{width}", // sqrt(128) when checked
                        height: "2",
                        rotate: "{rotation}deg",
                        position_left: "{offset.x}",
                        position_top: "{offset.y}",
                        background: color.as_str(),
                    }
                }
            }

            if !disabled {
                StateLayer {
                    color: state_layer_color,
                    width: "40",
                    height: "40",
                    position_left: "-11",
                    position_top: "-11"
                }
            }
        }
    }
}
