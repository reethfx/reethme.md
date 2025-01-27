use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SidebarItemProps {
    pub icon: &'static str,
    pub label: &'static str,
}

#[function_component(SidebarItem)]
pub fn sidebar_item(props: &SidebarItemProps) -> Html {
    html! {
        <li class="flex items-center p-2 rounded-md hover:bg-[#212121] transition">
            <span class="text-lg mr-3">{ props.icon }</span>
            <span class="text-base">{ props.label }</span>
        </li>
    }
}
