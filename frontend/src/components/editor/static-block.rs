use crate::components::editor::clipboard::ClipboardComponent;
use yew::{prelude::*, Component};

#[derive(Properties, PartialEq)]
pub struct StaticBlockProps {
    pub content: String,
}

pub struct StaticBlockComponent {
    min_height: i32,
}

impl Component for StaticBlockComponent {
    type Message = ();
    type Properties = StaticBlockProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { min_height: 200 }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="w-full font-mono text-gray-500 border border-gray-700 rounded-lg">
                <div class="w-full flex justify-between items-center pt-4 pb-2 pl-6 pr-4">
                    <p class="text-emerald-400 font-bold">{ "secrets" }</p>
                    <ClipboardComponent text={ctx.props().content.clone()} icon_style={"text-gray-50"}/>
                </div>
                <div class="text-xs lg:text-sm grid grid-cols-[auto,1fr] px-6 pb-4">
                    <div class="pr-3 text-left font-mono text-gray-400">
                        { for rows_vec(&ctx.props().content).iter().map(|num| html! {
                            <div>{if *num < 10 {format!("0{}", num)} else {format!("{}", num)}}</div>
                        }) }
                    </div>
                    <div
                        class="w-full h-full font-mono text-gray-50 bg-transparent whitespace-pre-wrap overflow-auto"
                        style={format!("min-height: {}px", self.min_height)}
                    >
                        { &ctx.props().content }
                    </div>
                </div>
            </div>
        }
    }
}

fn rows(text: &str) -> usize {
    text.len() - text.replace("\n", "").len() + 1
}

fn rows_vec(text: &str) -> Vec<usize> {
    (1..=rows(text)).collect()
}
