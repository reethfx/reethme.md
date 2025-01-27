use yew::prelude::*;

#[function_component(MainContent)]
pub fn main_content() -> Html {
    html! {
        <div class="flex-1 p-8 bg-[#212121]">
            <h1 class="text-3xl font-bold mb-4">{ "Main Content" }</h1>
            <p class="text-lg">{ "This is where your main content goes." }</p>
        </div>
    }
}
