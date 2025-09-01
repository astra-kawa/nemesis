mod cdms;

use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let cdm_result =
        use_server_future(|| cdms::xml_parsing::parse_cdm_xml_file("data/sample_cdm.xml".into()));

    let text = match cdm_result {
        Ok(cdm) => format!("{cdm:?}"),
        Err(err) => format!("{err}"),
    };

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
