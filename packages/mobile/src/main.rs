use dioxus::prelude::*;

use ui::Navbar;
use views::{Blog, Home};

mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(MobileNavbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/tailwind.css") }
        Router::<Route> {}
    }
}

#[component]
fn MobileNavbar() -> Element {
    rsx! {
        Navbar {
            Link {
                to: Route::Home {},
                class: "text-white hover:text-blue-400 transition-colors",
                "Home"
            }
            Link {
                to: Route::Blog { id: 1 },
                class: "text-white hover:text-blue-400 transition-colors",
                "Blog"
            }
        }

        Outlet::<Route> {}
    }
}
