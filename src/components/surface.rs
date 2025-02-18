use dioxus_core::AttributeValue;
use freya::prelude::*;

use crate::{material_design::Shape, prelude::Elevation};

#[component]
pub fn Surface(
    direction: Option<String>,
    main_align: Option<String>,
    cross_align: Option<String>,
    background: Option<String>,
    color: Option<String>,
    width: Option<String>,
    height: Option<String>,
    spacing: Option<String>,
    padding: Option<String>,
    margin: Option<String>,
    opacity: Option<String>,
    overflow: Option<String>,
    border: Option<String>,
    layer: Option<String>,
    shape: Option<Shape>,
    elevation: Option<Elevation>,
    position: Option<String>,
    position_left: Option<String>,
    position_right: Option<String>,
    position_top: Option<String>,
    position_bottom: Option<String>,
    offset_x: Option<String>,
    offset_y: Option<String>,
    reference: Option<AttributeValue>,
    on_click: Option<EventHandler<MouseEvent>>,
    on_pointer_enter: Option<EventHandler<PointerEvent>>,
    on_pointer_leave: Option<EventHandler<PointerEvent>>,
    children: Element,
) -> Element {
    let corner_radius = shape.map(Shape::into_value);
    let shadow = elevation.map(|elevation| {
        elevation
            .into_shadows()
            .map(|value| {
                format!(
                    "{} {} {} {} {}",
                    value.x, value.y, value.blur, value.spread, value.fill
                )
            })
            .join(", ")
    });

    rsx! {
        rect {
            direction,
            main_align,
            cross_align,
            background,
            color,
            width,
            height,
            corner_radius,
            shadow,
            spacing,
            padding,
            border,
            margin,
            opacity,
            overflow,
            position,
            position_left,
            position_right,
            position_top,
            position_bottom,
            offset_x,
            offset_y,
            reference,
            layer,

            onclick: move |data| if let Some(handler) = on_click {
                handler(data);
            },

            onpointerenter: move |data| if let Some(handler) = on_pointer_enter {
                handler(data);
            },

            onpointerleave: move |data| if let Some(handler) = on_pointer_leave {
                handler(data);
            },

            {children}
        }
    }
}
