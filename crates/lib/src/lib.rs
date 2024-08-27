//! # dioxus-free-icons
//!
//! Use free svg icons in your Dioxus projects easily with dioxus-free-icons.
//! This library provides Icon component, which will generate SVG for a Font Awesome icon.
//!
//! Basic usage:
//! ```ignore
//! use dioxus::prelude::*;
//! use freya_icons::icons::fa_brands_icons::FaRust;
//! use freya_icons::Icon;
//!
//! fn RustIcon() -> Element {
//!     rsx!(
//!         Icon {
//!             width: 30,
//!             height: 30,
//!             fill: "black",
//!             icon: Icon::FaRust,
//!         }
//!     )
//! }
//! ```
mod icon_component;

/// a collections of free icons
pub mod icons;
pub use crate::icon_component::{Icon, IconProps, IconShape};
