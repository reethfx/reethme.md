
mod sidebar_header;
mod footer_button;

use yew::prelude::*;
use std::collections::HashMap;

use footer_button::FooterButton;

#[derive(Clone, PartialEq, Debug)]
pub struct Block {
    pub id: usize,
    pub title: String,
    pub content: String,
}

#[derive(Properties, PartialEq)]
pub struct SidebarProps {
    pub topics: HashMap<String, Vec<Block>>,
    pub on_add_block: Callback<Block>,
}

#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    let expanded_topics = use_state(|| HashMap::new());

    let toggle_topic = {
        let expanded_topics = expanded_topics.clone();
        Callback::from(move |topic: String| {
            expanded_topics.set({
                let mut current = (*expanded_topics).clone();
                let is_expanded = current.get(&topic).copied().unwrap_or(false);
                current.insert(topic.clone(), !is_expanded);
                current
            });
        })
    };

    html! {
        <div class="bg-[#171717] text-gray-200 w-64 h-screen flex flex-col shadow-lg">
            <div class="p-4 border-b border-zinc-700">
                <h1 class="text-xl font-bold">{ "Proyectos" }</h1>
            </div>
            <nav class="flex-1 p-4 overflow-auto space-y-4">
                { for props.topics.clone().into_iter().map(|(topic, blocks)| {
                    let topic_cloned = topic.clone();
                    let is_expanded = expanded_topics.get(&topic).copied().unwrap_or(false);
                    let toggle = toggle_topic.clone();
                    let on_add = props.on_add_block.clone();

                    html! {
                        <div>
                            <div
                                class="flex items-center space-x-2 cursor-pointer p-1 rounded hover:bg-[#212121]"
                                onclick={Callback::from(move |_| toggle.emit(topic_cloned.clone()))}
                            >
                            <span>{ 
                                if is_expanded 
                                { "-" } else 
                                { "+" } }</span>
                                <span class="text-sm">{ topic }</span>
                            </div>
                            { if is_expanded {
                                html! {
                                    <ul class="pl-4">
                                        { for blocks.iter().map(|block| {
                                            let block_cloned = block.clone();
                                            let on_add_cloned = on_add.clone();
                                            html! {
                                                <li class="flex justify-between items-center p-2 rounded hover:bg-[#212121]">
                                                    <span class="truncate text-sm">{ &block_cloned.title }</span>
                                                    <button
                                                        class="flex items-center justify-center w-6 h-6 rounded-full bg-pink-500 text-white hover:bg-pink-600"
                                                        onclick={Callback::from(move |_| on_add_cloned.emit(block_cloned.clone()))}
                                                    >
                                                        { "+" }
                                                    </button>
                                                </li>
                                            }
                                        })}
                                    </ul>
                                }
                            } else {
                                html! { <></> }
                            }}
                        </div>
                    }
                })}
            </nav>
            <FooterButton />
        </div>
    }
}
