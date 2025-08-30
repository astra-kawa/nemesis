mod cdms;
use crate::cdms::ConjunctionDataMessage;

use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let cdm_result = cdms::xml_parsing::parse_cdm_xml_file("data/sample_cdm.xml");
    let text = match cdm_result {
        Ok(cdm) => format!("{cdm:?}"),
        Err(err) => format!("{err}"),
    };

    // let sample_text = format!("{:?}", cdm.header.originator);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Hero {}
        p { "{text}" }
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            div {
                h1 { "Nemesis" }
            }
        }
    }
}
