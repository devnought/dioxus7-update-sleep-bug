use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut counter = use_signal(|| 0);
    use_future(move || async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            counter += 1;
            println!("{counter}");
        }
    });

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        div { "{counter}" }
    }
}
