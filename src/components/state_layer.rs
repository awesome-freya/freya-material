use freya::prelude::*;
use freya_transition::{use_transition, Curve};

const INITIAL_ORIGIN_SCALE: f32 = 0.2;
const PADDING: f32 = 10.0;
const SOFT_EDGE_MINIMUM_SIZE: f32 = 75.0;
const SOFT_EDGE_CONTAINER_RATIO: f32 = 0.35;

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum State {
    #[default]
    Idle,
    Hover,
    Press,
    // Focus,
    // Drag (TODO?)
}

impl State {
    fn is_press(&self) -> bool {
        matches!(self, Self::Press)
    }

    fn opacity(&self) -> f32 {
        match self {
            State::Idle => 0.0,
            State::Hover => 0.08,
            State::Press => 0.1,
            // State::Focus => 0.1,
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
struct RipplePosition {
    start: Point2D,
    end: Point2D,
}

impl RipplePosition {
    fn new(cursor: Point2D, radius: f32, scale: f32, (height, width): (f32, f32)) -> Self {
        Self {
            start: Point2D::new(cursor.x - radius / 2.0, cursor.y - radius / 2.0),
            end: Point2D::new(
                (width - radius * scale) / 2.0,
                (height - radius * scale) / 2.0,
            ),
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
struct RippleInfo {
    position: RipplePosition,
    radius: f32,
    scale: f32,
}

impl RippleInfo {
    fn new(cursor: Point2D, area: Rect<f32, Measure>) -> Self {
        let (size, origin) = ((area.height(), area.width()), area.origin);
        let (radius, scale) = Self::get_size(size);
        let position = RipplePosition::new((cursor - origin).to_point(), radius, scale, size);

        Self {
            position,
            radius,
            scale,
        }
    }

    fn get_size((height, width): (f32, f32)) -> (f32, f32) {
        let max_dim = height.max(width);
        let soft_edge_size = SOFT_EDGE_MINIMUM_SIZE.max(SOFT_EDGE_CONTAINER_RATIO * max_dim);

        let initial_size = (max_dim * INITIAL_ORIGIN_SCALE).floor();
        let hypotenuse = width.hypot(height);
        let max_radius = hypotenuse + PADDING;

        (initial_size, (max_radius + soft_edge_size) / initial_size)
    }
}

#[component]
pub fn StateLayer(
    position_left: Option<String>,
    position_top: Option<String>,
    color: String,
    height: Option<String>,
    width: Option<String>,
) -> Element {
    let (reference, size) = use_node_signal();

    let width = width.unwrap_or_else(|| "fill".into());
    let height = height.unwrap_or_else(|| "fill".into());

    let mut state = use_signal(State::default);

    let transition = use_transition(move |context| {
        context.add_tween("opacity", 0.0, Curve::LINEAR, 15);
    });

    let ripple_opacity_transition =
        use_transition(move |context| context.add_tween("opacity", 0.0, Curve::LINEAR, 75));
    let ripple_transition = use_transition(move |context| {
        context.add_tween("radius", 0.0, Curve::FAST_OUT_SLOW_IN, 450);
        context.add_tween("origin", Point2D::zero(), Curve::FAST_OUT_SLOW_IN, 450);
    });

    let onpointerenter = move |_| state.set(State::Hover);
    let onpointerleave = move |_| state.set(State::Idle);
    let onpointerup = move |_| {
        state.set(State::Hover);
    };

    let onpointerdown = move |event: PointerEvent| {
        let info = RippleInfo::new(event.get_screen_coordinates().to_f32(), size.read().area);

        ripple_opacity_transition.set_duration("opacity", 75);
        ripple_opacity_transition.forced_set("opacity", 0.0);
        ripple_transition.forced_set("radius", info.radius);
        ripple_transition.forced_set("origin", info.position.start);
        ripple_transition.set("radius", info.radius * info.scale);
        ripple_transition.set("origin", info.position.end);
        ripple_opacity_transition.play([("opacity", 0.1)]);
        ripple_transition.play_all();

        state.set(State::Press);
    };

    use_effect(move || {
        transition.play([("opacity", state.read().opacity())]);
    });

    use_effect(move || {
        if !ripple_transition.is_playing() && !state.read().is_press() {
            ripple_opacity_transition.set_duration("opacity", 150);
            ripple_opacity_transition.play([("opacity", 0.0)]);
        }
    });

    let opacity: f32 = transition.get("opacity");
    let (ripple_opacity, radius, origin): (f32, f32, Point2D) = (
        ripple_opacity_transition.get("opacity"),
        ripple_transition.get("radius"),
        ripple_transition.get("origin"),
    );

    rsx! {
        rect {
            width: "{width}",
            height: "{height}",
            position: "absolute",
            position_left,
            position_top,
            overflow: "clip",
            layer: "-999",

            reference,

            onpointerenter,
            onpointerleave,
            onpointerup,
            onpointerdown,

            rect {
                background: "{color}",
                opacity: "{opacity}",
                width: "fill",
                height: "fill"
            }

            rect {
                background: "radial-gradient({color} 65%, transparent 100%)",
                opacity: "{ripple_opacity}",
                width: "{radius}",
                height: "{radius}",
                corner_radius: "{radius / 2.0}",
                position: "absolute",
                position_top: "{origin.y}",
                position_left: "{origin.x}",
            }
        }
    }
}
