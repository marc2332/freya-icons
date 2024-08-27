use freya::prelude::*;

use freya_icons::icons::bs_icons::BsAlarm;
use freya_icons::Icon;

fn main() {
    launch(app);
}

fn app() -> Element {
    rsx!(
        rect {
            width: "fill",
            height: "fill",
            main_align: "center",
            cross_align: "center",
            Icon {
                width: 200,
                height: 200,
                fill: "#007FFF",
                icon: BsAlarm,
            }
        }
    )
}
