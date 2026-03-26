use crate::views::{theme_view, upgrade_context_view};
use sycamore::prelude::*;

#[component]
pub fn App() -> View {
    view! {
        main(class="min-h-screen w-full") {
            div(class="mx-auto flex w-full max-w-6xl flex-col gap-4 p-4 md:gap-6 md:p-6") {
                theme_view::ThemeView()
                upgrade_context_view::UpgradeContextView()
            }
        }
    }
}
