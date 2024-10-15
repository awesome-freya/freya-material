use freya::prelude::*;
use freya_transition::{curves::ICurve, tween::lerp::Lerp, use_transition, Curve};
use material_colors::color::Argb;
use std::time::Duration;

const INITIAL_ORIGIN_SCALE: f32 = 0.2;
const PADDING: f32 = 10.0;
const SOFT_EDGE_MINIMUM_SIZE: f32 = 75.0;
const SOFT_EDGE_CONTAINER_RATIO: f32 = 0.35;

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub enum RippleState {
    #[default]
    Inactive,
    Hovering,
    Holding,
}

impl RippleState {
    pub fn is_holding(&self) -> bool {
        matches!(self, Self::Holding)
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

#[derive(Props, Default, PartialEq, Eq, Clone)]
pub struct RippleProps {
    width: Option<String>,
    height: Option<String>,
    color: Option<Argb>,
}

pub struct AdvancedAnimNum {
    origin: f32,
    destination: f32,
    time: Duration,
    curve: Curve,
    value: f32,
}

impl AdvancedAnimNum {
    pub fn new(origin: f32, destination: f32, time: u64, curve: Curve) -> Self {
        Self {
            origin,
            destination,
            time: Duration::from_millis(time),
            curve,
            value: origin,
        }
    }
}

impl AnimatedValue for AdvancedAnimNum {
    fn time(&self) -> Duration {
        self.time
    }

    fn as_f32(&self) -> f32 {
        self.value
    }

    fn as_string(&self) -> String {
        panic!("This is not a String");
    }

    fn prepare(&mut self, direction: AnimDirection) {
        match direction {
            AnimDirection::Forward => self.value = self.origin,
            AnimDirection::Reverse => {
                self.value = self.destination;
            }
        }
    }

    fn is_finished(&self, index: i32, direction: AnimDirection) -> bool {
        match direction {
            AnimDirection::Forward => {
                index > self.time.as_millis() as i32 && self.value >= self.destination
            }
            AnimDirection::Reverse => {
                index > self.time.as_millis() as i32 && self.value <= self.origin
            }
        }
    }

    fn advance(&mut self, index: i32, direction: AnimDirection) {
        if !self.is_finished(index, direction) {
            let time = self.time.as_millis() as f32;
            let (origin, destination) = match direction {
                AnimDirection::Forward => (self.origin, self.destination),
                AnimDirection::Reverse => (self.destination, self.origin),
            };

            self.value = match &self.curve {
                Curve::None => destination,
                curve => origin.lerp(
                    &destination,
                    curve.transform((index as f32).min(time) / time),
                ),
            };

            if origin == 8.0 {
                println!("advance: \x1b[32;1m{:10}\x1b[0m (from \x1b[32;1m{}\x1b[0m to \x1b[32;1m{}\x1b[0m) [\x1b[32;1m{:3}\x1b[0m / \x1b[32;1m{}\x1b[0m]", self.value, origin, destination, index, time);
            }
        }
    }
}

#[component]
#[allow(clippy::needless_pass_by_value)]
pub fn Ripple(props: RippleProps) -> Element {
    let (node_ref, size) = use_node_signal();

    let mut state = use_signal(RippleState::default);

    let mut info = use_signal(RippleInfo::default);

    let bg = props.color.unwrap_or_else(|| Argb::new(255, 255, 255, 255));

    let background_animation = use_transition(move |context| {
        context.add_tween("background", 0.0, Curve::LINEAR, 15);
    });

    let opacity_animation = use_transition(|context| {
        context.add_tween("opacity", 0.0, Curve::LINEAR, 75);
    });

    let animation = use_animation(move |context| {
        let info = info.read();

        (
            context.with(AdvancedAnimNum::new(
                info.radius,
                info.radius * info.scale,
                450,
                Curve::cubic(0.2, 0.0, 0.0, 1.0),
            )),
            context.with(AdvancedAnimNum::new(
                info.position.start.x,
                info.position.end.x,
                450,
                Curve::cubic(0.2, 0.0, 0.0, 1.0),
            )),
            context.with(AdvancedAnimNum::new(
                info.position.start.y,
                info.position.end.y,
                450,
                Curve::cubic(0.2, 0.0, 0.0, 1.0),
            )),
        )
    });

    let opacity = opacity_animation.get::<f32>("opacity");

    let background_opacity = background_animation.get::<f32>("background");

    let pointerdown = move |event: PointerEvent| {
        // animation.reset();

        info.set(RippleInfo::new(
            event.get_screen_coordinates().to_f32(),
            size.read().area,
        ));

        state.set(RippleState::Holding);
    };

    let pointerup = move |_| {
        if !animation.is_running() {
            opacity_animation.play([("opacity", 0.0)]);
        }

        state.set(RippleState::Hovering);
    };

    let pointerenter = move |_| {
        background_animation.play([("background", 0.08)]);

        state.set(RippleState::Hovering);
    };

    let pointerleave = move |_| {
        animation.reset();

        background_animation.play([("background", 0.0)]);
        opacity_animation.play([("opacity", 0.0)]);

        state.set(RippleState::Inactive);
    };

    use_effect(move || {
        if !animation.is_running() && !state.read().is_holding() {
            opacity_animation.set_duration("opacity", 150);
            opacity_animation.play([("opacity", 0.0)]);
        }
    });

    use_effect(move || {
        if *state.read() == RippleState::Holding {
            opacity_animation.set_duration("opacity", 375);
            opacity_animation.play([("opacity", 0.12)]);

            animation.start();
        }
    });

    let (radius, x, y) = animation.get();

    use_effect(move || {
        println!("{}", radius.read().as_f32());
    });

    // use_effect(move || {
    //     if !opacity_animation.is_playing() && opacity == 0.0 && opacity_animation.peek_has_run_yet() && radius.read().as_f32() != 0.0 {
    //         // animation.reset();
    //     }
    // });

    rsx! {
        rect {
            width: props.width.as_deref().unwrap_or("fill"),
            height: props.height.as_deref().unwrap_or("fill"),
            overflow: "clip",
            position: "absolute",
            layer: "-10",

            onpointerdown: pointerdown,
            onpointerup: pointerup,
            onpointerenter: pointerenter,
            onpointerleave: pointerleave,

            reference: node_ref,

            rect {
                background: "{bg}",
                opacity: "{background_opacity}",
                width: "100%",
                height: "100%",
            },

            rect {
                background: "radial-gradient({bg} 65%, transparent 100%)",
                width: "{radius.read().as_f32()}",
                height: "{radius.read().as_f32()}",
                opacity: "{opacity}",
                position: "absolute",
                position_left: "{x.read().as_f32()}",
                position_top: "{y.read().as_f32()}",
                corner_radius: "{radius.read().as_f32() / 2.0}",
            }
        }
    }
}
