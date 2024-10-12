use crate::ripple::Ripple;
use freya::prelude::*;

#[derive(Default, PartialEq)]
enum SwitchState {
    #[default]
    Idle,
    // Hovering,
    Pressed,
}

#[derive(Props, PartialEq, Clone)]
pub struct SwitchProps {
    pub width: Option<String>,
    pub height: Option<String>,
    pub enabled: bool,
    pub ontoggled: EventHandler,
}

#[component]
pub fn Switch(props: SwitchProps) -> Element {
    let platform = use_platform();

    let height = props.height.unwrap_or_else(|| "32".into());
    let num_height: f32 = height.parse().unwrap();

    let width = props
        .width
        .unwrap_or_else(|| (num_height * 1.625).to_string());
    let num_width: f32 = width.parse().unwrap();

    let mut state = use_signal(SwitchState::default);

    let animation = use_animation(move |ctx| {
        ctx.with(
            AnimNum::new(0.0, num_width - num_height)
                .time(300)
                .function(Function::Back)
                .ease(Ease::Out),
        )
    });

    let theme = crate::use_theme();
    let theme = theme.read();

    let (offset_x, (background, border, circle)) = (
        animation.get().read().as_f32(),
        if props.enabled {
            (theme.primary, theme.primary, theme.on_primary)
        } else {
            (
                theme.surface_container_highest,
                theme.outline,
                theme.outline,
            )
        },
    );

    let radius_animation = use_animation_with_dependencies(&props.enabled, move |ctx, enabled| {
        ctx.with(
            AnimNum::new(
                if enabled {
                    num_height - 8.0
                } else {
                    num_height / 2.0
                },
                num_height - 4.0,
            )
            .time(100)
            .function(Function::Linear),
        )
    });

    let onmousedown = move |e: MouseEvent| {
        e.stop_propagation();

        state.set(SwitchState::Pressed);

        radius_animation.start();
    };

    let onmouseleave = move |e: MouseEvent| {
        e.stop_propagation();

        if state.read().ne(&SwitchState::Idle) && radius_animation.peek_has_run_yet() {
            radius_animation.reverse();

            state.set(SwitchState::Idle);
        }

        platform.set_cursor(CursorIcon::default());
    };

    let onmouseenter = move |e: MouseEvent| {
        e.stop_propagation();

        // state.set(SwitchState::Hovering);

        platform.set_cursor(CursorIcon::Pointer);
    };

    let onclick = move |e: MouseEvent| {
        e.stop_propagation();

        if radius_animation.peek_has_run_yet() {
            radius_animation.reverse();
        }

        props.ontoggled.call(());

        state.set(SwitchState::Idle);
    };

    use_effect(use_reactive(&props.enabled, move |enabled| {
        if enabled {
            animation.start();
        } else if animation.peek_has_run_yet() {
            animation.reverse();
        }
    }));

    let radius = radius_animation.get();

    rsx! {
        rect {
            width: width.as_str(),
            height: height.as_str(),
            corner_radius: height.as_str(),
            main_align: "center",
            cross_align: "center",
            background: "{background}",
            border: "1 solid {border}",

            onmousedown,
            onmouseenter,
            onmouseleave,
            onclick,

            rect {
                width: "100%",
                offset_x: "{offset_x}",
                padding: "2.5",
                corner_radius: height.as_str(),

                rect {
                    width: "calc({height} - 4)",
                    height: "calc({height} - 4)",
                    main_align: "center",
                    cross_align: "center",
                    corner_radius: height.as_str(),

                    rect {
                        background: "{circle}",
                        width: "{radius.read().as_f32()}",
                        height: "{radius.read().as_f32()}",
                        corner_radius: "{radius.read().as_f32() / 2.0}",
                    }

                    rect {
                        width: "calc({height} + 8)",
                        height: "calc({height} + 8)",
                        position: "absolute",
                        position_left: "-6",
                        position_top: "-6",
                        corner_radius: height.as_str(),
                        overflow: "clip",

                        Ripple { }
                    }
                }
            }
        }
    }
}
