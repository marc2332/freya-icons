use freya::prelude::*;

/// Icon shape trait
pub trait IconShape {
    fn content(&self) -> &'static str;
}

/// Icon component Props
#[derive(PartialEq, Clone, Props)]
pub struct IconProps<T: Into<&'static str> + Clone + 'static + PartialEq> {
    /// The icon shape to use.
    pub icon: T,
    /// The height of the `svg` element. Defaults to 20.
    #[props(default = "20".to_string())]
    pub height: String,
    /// The width of the `svg` element. Defaults to 20.
    #[props(default = "20".to_string())]
    pub width: String,
    /// The color to use for filling the icon. Defaults to `currentColor`.
    #[props(default = "current_color".to_string())]
    pub fill: String,
}

/// Icon component which generates SVG elements
#[allow(non_snake_case)]
pub fn Icon<T: Into<&'static str> + Clone + PartialEq + 'static>(props: IconProps<T>) -> Element {
    let svg_content = props.icon.into();
    rsx!(svg {
        width: props.width,
        height: props.height,
        fill: props.fill,
        svg_content
    })
}
