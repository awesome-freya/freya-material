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
    reference: Option<AttributeValue>,
    on_click: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let corner_radius = shape.map(Shape::into_value);
    let shadow = elevation.map(Elevation::into_shadow);

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
            reference,
            layer,

            onclick: move |data| if let Some(handler) = on_click {
                handler(data)
            },

            {children}
        }
    }
}
