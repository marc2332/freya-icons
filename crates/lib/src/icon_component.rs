use freya::prelude::*;

/// Icon shape trait
pub trait IconShape {
    fn content(&self) -> &str;
}

/// Icon component Props
#[derive(PartialEq, Props, Clone)]
pub struct IconProps<T: IconShape + Clone + PartialEq + 'static> {
    /// The icon shape to use.
    pub icon: T,
    /// The height of the `<svg>` element. Defaults to 20.
    #[props(default = "20".to_string())]
    pub height: String,
    /// The width of the `<svg>` element. Defaults to 20.
    #[props(default = "20".to_string())]
    pub width: String,
    /// The color to use for filling the icon. Defaults to "currentColor".
    #[props(default = "currentColor".to_string())]
    pub fill: String,
    /// An accessible, short-text description for the icon.
    pub title: Option<String>,
}

/// Icon component which generates SVG elements
#[allow(non_snake_case)]
pub fn Icon<T: IconShape + Clone + PartialEq + 'static>(props: IconProps<T>) -> Element {
    let svg_content = props.icon.content();
    let svg_content = svg_content.replace("currentColor", &props.fill);
    rsx!(svg {
        width: "{props.width}",
        height: "{props.height}",
        svg_content
    })
}
