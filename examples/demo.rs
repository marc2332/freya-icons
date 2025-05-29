use freya::prelude::*;
use freya_icons::prelude::{
    bs_icons::{Bs123, BsActivity, BsApp},
    Icon,
};

fn main() {
    launch(app);
}

fn app() -> Element {
    use_init_theme(|| DARK_THEME);
    let icons: [&'static str; 2] = [Bs123.into(), BsActivity.into()];

    rsx!(
        Body {
            rect {
                width: "fill",
                height: "fill",
                main_align: "center",
                cross_align: "center",
                Icon {
                    width: "48",
                    height: "48",
                    icon: BsApp
                }
                for icon in icons {
                    Icon {
                        icon
                    }
                }
            }
        }
    )
}
