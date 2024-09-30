use super::block::BlockComponent;
use web_sys::{File, HtmlInputElement, HtmlTextAreaElement};
use yew::{prelude::*, Component};

pub enum Msg {
    FocusBlock,
}

#[derive(Default, Debug)]
pub struct FrameComponent {
    focus: usize,
    focus_on_render: bool,
}

#[derive(Properties, PartialEq)]
pub struct FrameProps {
    pub focus_ref: NodeRef,
}

impl Component for FrameComponent {
    type Message = Msg;
    type Properties = FrameProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            focus: 0,
            focus_on_render: true,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FocusBlock => {
                self.focus_on_render = true;
            }
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div style="min-height: 95vh; display: flex; flex-direction: column;" class="text-xs md:text-sm">
                <div style="min-height: 7vh;"/>
                <div class="subpixel-antialiased bg-gray-900 dark:bg-dark-code rounded-md shadow-2xl dark:shadow-gray-400/5">
                    <BlockComponent textarea_ref={ctx.props().focus_ref.clone()}/>
                </div>
                <div class="w-full flex flex-row space-x-2 py-2 text-center h-16">
                    <div class="w-1/2 flex align-center text-center font-mono bg-gray-900 dark:bg-dark-code rounded-md shadow-2xl dark:shadow-gray-400/5 text-gray-500 flex">
                    <p class="w-full text-gray-200 px-2 py-3.5">
                        {"max reads:"}
                    </p>
                    <input
                            type="number"
                            class="w-2/5 no-spinner text-left text-gray-50 bg-transparent appearance-none focus:outline-none focus:ring-0 focus:border-0 active:border-0 py-4"
                            value=10
                        />
                    </div>
                    <div class="w-2/3 flex align-center text-center font-mono bg-gray-900 dark:bg-dark-code rounded-md shadow-2xl dark:shadow-gray-400/5 text-gray-500 flex">
                        <p class="w-full text-gray-200 px-2 py-3.5">{"time to expiry:"}</p>
                        <input type="number" value=7
                            class="w-1/4 no-spinner text-center text-gray-50 bg-transparent appearance-none focus:outline-none focus:ring-0 focus:border-0 active:border-0 py-4"
                        />
                        <select class="w-1/2 text-left text-gray-50 bg-transparent border-0 appearance-none focus:outline-none focus:ring-0 focus:border-0 active:border-0 py-3.5">
                            <option value="minutes">{"minutes"}</option>
                            <option value="hours">{"hours"}</option>
                            <option value="days">{"days"}</option>
                        </select>
                    </div>
                    <button class="w-full text-md md:text-lg font-medium bg-gray-900 dark:bg-dark-code rounded-md shadow-2xl dark:shadow-gray-400/5 text-gray-50 h-full py-2">
                        {"Share"}
                    </button>
                </div>
                <p class="px-2 pt-4 text-gray-500">
                    {"Clicking `Share` will generate a new symmetrical key and encrypt your secrets before sending them (only the encrypted data) to the server."}
                </p>
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if !first_render && self.focus_on_render {
            if let Some(textarea) = ctx.props().focus_ref.cast::<HtmlTextAreaElement>() {
                let _ = textarea.focus();
            }
        }
    }
}
