use dioxus_native::prelude::*;

fn main() {
    dioxus_native::launch(App);
}

#[component]
fn App() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        Stylesheet { href: asset!("/assets/tailwind.css") }
        div { class: "min-h-screen bg-gray-950 text-white flex flex-col items-center justify-center p-8",
            h1 { class: "text-5xl font-bold mb-4 bg-gradient-to-r from-blue-400 to-purple-500 bg-clip-text text-transparent",
                "Dioxus Native"
            }
            p { class: "text-gray-400 text-lg mb-8 max-w-md text-center",
                "GPU-rendered native app via Blitz — no webview needed."
            }
            div { class: "text-6xl font-bold mb-6", "{count}" }
            div { class: "flex gap-4 mb-8",
                button {
                    class: "px-6 py-3 bg-blue-600 rounded-lg hover:bg-blue-500 transition-colors text-white",
                    onclick: move |_| count += 1,
                    "Increment"
                }
                button {
                    class: "px-6 py-3 bg-red-600 rounded-lg hover:bg-red-500 transition-colors text-white",
                    onclick: move |_| count -= 1,
                    "Decrement"
                }
            }
            div { class: "grid grid-cols-2 gap-4",
                a {
                    href: "https://dioxuslabs.com/learn/0.7/",
                    class: "px-6 py-3 bg-gray-800 rounded-lg hover:bg-gray-700 transition-colors text-center",
                    "Learn Dioxus"
                }
                a {
                    href: "https://github.com/DioxusLabs/blitz",
                    class: "px-6 py-3 bg-gray-800 rounded-lg hover:bg-gray-700 transition-colors text-center",
                    "Blitz Engine"
                }
            }
        }
    }
}
