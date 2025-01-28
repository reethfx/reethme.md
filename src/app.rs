use yew::prelude::*;
mod sidebar;
mod main_content;

use sidebar::{Sidebar, Block};
use main_content::MainContent;
use std::collections::HashMap;

#[function_component(App)]
pub fn app() -> Html {
    let topics = use_state(|| {
        let mut map = HashMap::new();
        map.insert(
            "General".to_string(),
            vec![
                Block { id: 1, title: "Introduction".into(), content: "Describe your project here...".into() },
                Block { id: 2, title: "Installation".into(), content: "How to install the project...".into() },
            ],
        );
        map.insert(
            "Advanced".to_string(),
            vec![
                Block { id: 3, title: "Usage".into(), content: "How to use the project...".into() },
                Block { id: 4, title: "API Reference".into(), content: "API details here...".into() },
            ],
        );
        map
    });

    let selected_blocks = use_state(|| Vec::<Block>::new());

    let on_add_block = {
        let selected_blocks = selected_blocks.clone();
        Callback::from(move |block: Block| {
            if !selected_blocks.iter().any(|b| b.id == block.id) {
                selected_blocks.set({
                    let mut blocks = (*selected_blocks).clone();
                    blocks.push(block);
                    blocks
                });
            }
        })
    };

    html! {
        <div class="flex h-screen">
            <Sidebar
                topics={(*topics).clone()}
                on_add_block={on_add_block}
            />
            <MainContent
                selected_blocks={(*selected_blocks).clone()}
            />
        </div>
    }
}
