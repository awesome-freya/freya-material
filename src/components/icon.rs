use freya::prelude::*;
pub use material_icons::IconStyle;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IconData {
    pub name: &'static str,
    pub style: IconStyle,
    pub filled: bool,
}

impl Default for IconData {
    fn default() -> Self {
        Self {
            name: "",
            style: IconStyle::Outlined,
            filled: false,
        }
    }
}

#[component]
pub fn Icon(
    name: String,
    color: Option<String>,
    width: Option<String>,
    height: Option<String>,
    #[props(default = IconStyle::Outlined)] style: IconStyle,
    #[props(default)] filled: bool,
) -> Element {
    let svg_data = static_bytes(material_icons::icon(&name, style, filled));

    rsx! {
        svg {
            fill: color.clone(),
            color,
            width,
            height,
            svg_data,
        }
    }
}
