# dioxus-free-icons

Use free svg icons in your [Dioxus](https://dioxuslabs.com/) projects easily with dioxus-free-icons.

More information about this crate can be found in the [crate documentation](https://docs.rs/dioxus-free-icons/0.1.1/dioxus_free_icons/).

## Install

To use `dioxus-free-icons`, add this to your Cargo.toml:

```toml
[dependencies]
dioxus-free-icons = "0.1.1"
```

## Example

This library provides Icon component, which will generate SVG for a Font Awesome icon.

```rust
use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_brands_icons::FaRust;
use dioxus_free_icons::Icon;

fn RustIcon(cx: Scope) -> Element {
    cx.render(rsx! {
        Icon {
            size: 30,
            fill: "black",
            icon: Icon::FaRust,
        }
    })
}
```

## License

This project is licensed under the MIT license.

### Icon

Icon Library|License|Version
---|---|---
[Font Awesome](https://fontawesome.com/)|[CC BY 4.0 License](https://creativecommons.org/licenses/by/4.0/)| [6.1.1](https://github.com/FortAwesome/Font-Awesome/tree/6.1.1)
[heroicons](https://heroicons.com/)|[MIT License](https://github.com/tailwindlabs/heroicons/blob/master/LICENSE)| [1.0.6](https://github.com/tailwindlabs/heroicons/tree/v1.0.6)

## Contribution

The project welcomes all contributions from anyone willing to work in good faith with other contributors and the community. 
In particular, contributions regarding support for other free icons such as Material Design icons or Ionicons are welcome. 
This library aims to be a react-icons-like library for dioxus projects.

### Development

```sh
// generate icon files
cd packages/codegen
cargo run
```

### Preview

```sh
cd packages/exmaple
cargo install dioxus-cli
dioxus serve
```
