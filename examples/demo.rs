use freya::prelude::*;

use freya_icons::icons::bs_icons::BsAlarm;
use freya_icons::Icon;

fn main() {
    launch(app);
}

fn app() -> Element {
    rsx!(Icon {
        width: 200,
        height: 200,
        fill: "#007FFF",
        icon: BsAlarm,
    })
}
