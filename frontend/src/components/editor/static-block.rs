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
            <div class="w-full font-mono text-gray-500 grid h-full grid-cols-[auto,1fr] py-4 px-6 border border-gray-700 rounded-lg">
                <p class="mt-0 text-emerald-400 pb-2 col-span-2 font-bold">{ "secrets" }</p>
                <div class="pr-2 text-left font-mono text-gray-400">
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
        }
    }
}

fn rows(text: &str) -> usize {
    text.len() - text.replace("\n", "").len() + 1
}

fn rows_vec(text: &str) -> Vec<usize> {
    (1..=rows(text)).collect()
}
