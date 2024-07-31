mod app;
mod components;
mod utils;

use app::App;
use leptos::{mount_to_body, view};

fn main() {
    mount_to_body(|cx| {
        view! { cx,
            <App/>
        }
    });
}
