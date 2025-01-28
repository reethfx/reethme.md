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
                <h1 class="text-xl font-bold">{ "reethme." }</h1>
            </div>
            <nav class="flex-1 p-4 overflow-auto space-y-2">
                { for props.topics.clone().into_iter().map(|(topic, blocks)| {
                    let topic_cloned = topic.clone();
                    let is_expanded = expanded_topics.get(&topic).copied().unwrap_or(false);
                    let toggle = toggle_topic.clone();
                    let on_add = props.on_add_block.clone();

                    html! {
                        <div>
                            <div
                                class="flex items-center space-x-2 cursor-pointer p-2 rounded-md hover:bg-[#212121]"
                                onclick={Callback::from(move |_| toggle.emit(topic_cloned.clone()))}
                            >
                                { if is_expanded {
                                    html! {
                                        <svg class="stroke-gray-200 text-gray-200 h-5 w-5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g><g id="SVGRepo_iconCarrier"> <path d="M3 9.312C3 4.93757 3.93757 4 8.312 4H9.92963C10.5983 4 11.2228 4.3342 11.5937 4.8906L12.4063 6.1094C12.7772 6.6658 13.4017 7 14.0704 7C15.9647 7 17.8145 7 19.1258 7C20.1807 7 21.0128 7.82095 21.0029 8.8758C21.0013 9.05376 21 9.20638 21 9.312V14.688C21 19.0624 20.0624 20 15.688 20H8.312C3.93757 20 3 19.0624 3 14.688V9.312Z" stroke="#currentColor" stroke-width="2" stroke-linejoin="round"></path> <path d="M21 11H17.688H12.312C7.93757 11 7 11.9376 7 16.312V19.5" stroke="#currentColor" stroke-width="2" stroke-linecap="round"></path> </g></svg>
                                    }
                                } else {
                                    html! {
                                        <svg class="stroke-gray-200 text-gray-200 h-5 w-5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g><g id="SVGRepo_iconCarrier"> <path opacity="0.1" d="M3 9.312C3 4.93757 3.93757 4 8.312 4H9.92963C10.5983 4 11.2228 4.3342 11.5937 4.8906L12.4063 6.1094C12.7772 6.6658 13.4017 7 14.0704 7C15.9647 7 17.8145 7 19.1258 7C20.1807 7 21.0128 7.82095 21.0029 8.8758C21.0013 9.05376 21 9.20638 21 9.312V14.688C21 19.0624 20.0624 20 15.688 20H8.312C3.93757 20 3 19.0624 3 14.688V9.312Z" fill="#currentColor"></path> <path d="M3 9.312C3 4.93757 3.93757 4 8.312 4H9.92963C10.5983 4 11.2228 4.3342 11.5937 4.8906L12.4063 6.1094C12.7772 6.6658 13.4017 7 14.0704 7C15.9647 7 17.8145 7 19.1258 7C20.1807 7 21.0128 7.82095 21.0029 8.8758C21.0013 9.05376 21 9.20638 21 9.312V14.688C21 19.0624 20.0624 20 15.688 20H8.312C3.93757 20 3 19.0624 3 14.688V9.312Z" stroke="#currentColor" stroke-width="2"></path> </g></svg>
                                    }
                                }}
                                <span class="text-sm">{ topic }</span>
                            </div>
                            { if is_expanded {
                                html! {
                                    <ul class="pl-4">
                                        { for blocks.iter().map(|block| {
                                            let block_cloned = block.clone();
                                            let on_add_cloned = on_add.clone();
                                            html! {
                                                <li class="flex justify-between items-center p-2 rounded-md hover:bg-[#212121]">
                                                    <span class="truncate text-sm">{ &block_cloned.title }</span>
                                                    <button
                                                        class="flex items-center justify-center w-6 h-6 rounded-full bg-pink-500 text-white hover:bg-pink-600"
                                                        onclick={Callback::from(move |_| on_add_cloned.emit(block_cloned.clone()))}
                                                    >
                                                        { "" }
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
