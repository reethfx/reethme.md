use yew::prelude::*;
mod sidebar;
mod main_content;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="flex h-screen">
            <sidebar::Sidebar />
            <main_content::MainContent />
        </div>
    }
}
