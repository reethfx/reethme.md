use yew::prelude::*;
use pulldown_cmark::{Parser, html};

use super::sidebar::Block;

#[derive(Properties, PartialEq)]
pub struct MainContentProps {
    pub selected_blocks: Vec<Block>,
}

#[function_component(MainContent)]
pub fn main_content(props: &MainContentProps) -> Html {
    let is_preview = use_state(|| false);

    fn render_markdown(markdown: &str) -> String {
        let parser = Parser::new(markdown);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        html_output
    }

    let toggle_preview = {
        let is_preview = is_preview.clone();
        Callback::from(move |_| {
            is_preview.set(!*is_preview);
        })
    };

    html! {
        <div class="flex-1 flex flex-col h-full bg-[#262626] text-white">
            <div class="flex items-center p-3.5 bg-[#262626] border-b border-[#333]">
                <button
                    onclick={toggle_preview.clone()}
                    class={if !*is_preview { "transition bg-[#393939] px-2 py-1 rounded" } else { "transition hover:bg-[#393939] bg-[#262626] px-2 py-1 rounded" }}
                >
                    <svg class="h-6 w-6" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g><g id="SVGRepo_iconCarrier"> <path d="M9 8L5 11.6923L9 16M15 8L19 11.6923L15 16" stroke="#FFF" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"></path> </g></svg>
                </button>
                <button
                    onclick={toggle_preview}
                    class={if *is_preview { "transition bg-[#393939] px-2 py-1 rounded ml-2" } else { "transition hover:bg-[#393939] bg-[#262626] px-2 py-1 rounded ml-2" }}
                >
                    <svg class="h-6 w-6" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g><g id="SVGRepo_iconCarrier"> <path d="M4.52393 6.25C4.52393 5.83579 4.85971 5.5 5.27393 5.5H18.7297C19.1439 5.5 19.4797 5.83579 19.4797 6.25V9.75C19.4797 10.1642 19.1439 10.5 18.7297 10.5H5.27393C4.85971 10.5 4.52393 10.1642 4.52393 9.75V6.25ZM6.02393 7V9H17.9797V7H6.02393Z" fill="#FFF"></path> <path d="M14.2297 11.979C13.8155 11.979 13.4797 12.3148 13.4797 12.729V17.229C13.4797 17.6432 13.8155 17.979 14.2297 17.979H18.7297C19.1439 17.979 19.4797 17.6432 19.4797 17.229V12.729C19.4797 12.3148 19.1439 11.979 18.7297 11.979H14.2297ZM14.9797 16.479V13.479H17.9797V16.479H14.9797Z" fill="#FFF"></path> <path d="M4.52393 13.25C4.52393 12.8358 4.85971 12.5 5.27393 12.5H11.25C11.6642 12.5 12 12.8358 12 13.25C12 13.6642 11.6642 14 11.25 14H5.27393C4.85971 14 4.52393 13.6642 4.52393 13.25Z" fill="#FFF"></path> <path d="M5.27393 16C4.85971 16 4.52393 16.3358 4.52393 16.75C4.52393 17.1642 4.85971 17.5 5.27393 17.5H11.25C11.6642 17.5 12 17.1642 12 16.75C12 16.3358 11.6642 16 11.25 16H5.27393Z" fill="#FFF"></path> <path d="M2 5.75C2 4.23122 3.23122 3 4.75 3H19.25C20.7688 3 22 4.23122 22 5.75V18.25C22 19.7688 20.7688 21 19.25 21H4.75C3.23122 21 2 19.7688 2 18.25V5.75ZM4.75 4.5C4.05964 4.5 3.5 5.05964 3.5 5.75V18.25C3.5 18.9404 4.05964 19.5 4.75 19.5H19.25C19.9404 19.5 20.5 18.9404 20.5 18.25V5.75C20.5 5.05964 19.9404 4.5 19.25 4.5H4.75Z" fill="#FFF"></path> </g></svg>
                </button>
            </div>

            <div class="flex-1 flex justify-center items-start p-8 overflow-y-auto">
                <div class="w-full max-w-4xl">
                    { for props.selected_blocks.iter().map(|block| {
                        if *is_preview {
                            // Preview mode
                            html! {
                                <div class="mb-6 bg-[#262626] rounded-lg overflow-hidden">
                                    // Header
                                    <div class="flex items-center justify-between p-2 bg-[#393939] border-b border-[#333]">
                                        <div class="flex space-x-2">
                                            <div class="w-3 h-3 bg-[#ff5f56] rounded-full"></div>
                                            <div class="w-3 h-3 bg-[#ffbd2e] rounded-full"></div>
                                            <div class="w-3 h-3 bg-[#27c93f] rounded-full"></div>
                                        </div>
                                        <div class="text-sm text-gray-400">
                                            { "README.md" }
                                        </div>
                                    </div>                                   
                                    <h2 class="text-xl font-bold mb-2">{ &block.title }</h2>
                                    <div
                                        class="prose prose-invert"
                                        inner_html={render_markdown(&block.content)}
                                    />
                                </div>
                            }
                        } else {
                            html! {
                                <div class="mb-6 bg-[#262626] rounded-lg overflow-hidden">
                                    // Header
                                    <div class="flex items-center justify-between p-2 bg-[#393939] border-b border-[#333]">
                                        <div class="flex space-x-2">
                                            <div class="w-3 h-3 bg-[#ff5f56] rounded-full"></div>
                                            <div class="w-3 h-3 bg-[#ffbd2e] rounded-full"></div>
                                            <div class="w-3 h-3 bg-[#27c93f] rounded-full"></div>
                                        </div>
                                        <div class="text-sm text-gray-400">
                                            { "README.md" }
                                        </div>
                                    </div>

                                    // Codearea
                                    <textarea
                                        class="w-full h-64 p-4 bg-[#2F2F2F] text-white resize-none font-mono focus:outline-none"
                                        value={block.content.clone()}
                                    />
                                </div>
                            }
                        }
                    })}
                </div>
            </div>
        </div>
    }
}