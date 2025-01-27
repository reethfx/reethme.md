use yew::prelude::*;
mod sidebar_item;
mod footer_button;

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    html! {
        <div class="bg-[#171717] text-gray-200 w-64 h-screen flex flex-col shadow-lg">
            // Encabezado
            <div class="p-4 border-b border-zinc-700">
                <h1 class="text-xl font-bold">{ "reethme." }</h1>
            </div>

            // Menú
            <nav class="flex-1 p-4">
                <ul class="space-y-2">
                    <sidebar_item::SidebarItem icon="🏠" label="Home" />
                    <sidebar_item::SidebarItem icon="👤" label="Profile" />
                    <sidebar_item::SidebarItem icon="⚙️" label="Settings" />
                </ul>
            </nav>

            // Pie de página
            <footer_button::FooterButton />
        </div>
    }
}
