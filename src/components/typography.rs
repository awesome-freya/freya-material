use crate::material_design::{get_type_scale, TypescaleSize, TypescaleVariant};
use freya::prelude::*;

#[component]
pub fn Typography(
    #[props(default = TypescaleVariant::Body)] variant: TypescaleVariant,
    #[props(default = TypescaleSize::Medium)] size: TypescaleSize,
    #[props(default)] prominent: bool,
    color: Option<String>,
    text_align: Option<String>,
    text_shadow: Option<String>,
    max_lines: Option<String>,
    children: Element,
) -> Element {
    let (font_family, weight, size, _, line_height) = get_type_scale(variant, size, prominent);

    rsx! {
        label {
            font_family,
            font_weight: "{weight}",
            font_size: "{size}",
            line_height: "{line_height / size}",
            color,
            text_align,
            text_shadow,
            max_lines,

            {children}
        }
    }
}
