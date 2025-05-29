//! # freya-icons
//!
//! Use svg icons in your Freya projects easily with freya-icons.
//! This library provides Icon component, which will generate a `svg` element.
//!
//! Basic usage:
//! ```ignore
//! use freya::prelude::*;
//! use freya_icons::prelude::fa_brands_icons::FaRust;
//! use freya_icons::prelude::Icon;
//!
//! fn RustIcon() -> Element {
//!     rsx!(
//!         Icon {
//!             width: 30,
//!             height: 30,
//!             fill: "black",
//!             icon: FaRust,
//!         }
//!     )
//! }
//! ```
pub mod icon_component;

/// Collections of icons.
pub mod icons;

pub mod prelude {
    pub use crate::icon_component::*;
    pub use crate::icons::*;
}
