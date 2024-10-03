use crate::components::{editor::frame::FrameComponent, theme::ThemeComponent};

use gloo::events::EventListener;
use gloo::timers::callback::Timeout;
use web_sys::wasm_bindgen::JsCast;
use web_sys::{HtmlElement, KeyboardEvent, Window};
use yew::{prelude::*, Component};
use yew_router::prelude::*;

pub enum Msg {
    SwitchTheme(bool),
    LandingOff,
    GoToLanding,
    GoToeditor,
}

pub struct Home {
    dark_mode: bool,
    work_mode: bool,
    landing_ref: NodeRef,
    editor_ref: NodeRef,
    _timeout: Option<Timeout>,
}

impl Home {
    fn is_dark_mode(&self) -> bool {
        self.dark_mode
    }
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            dark_mode: true,
            work_mode: false,
            landing_ref: NodeRef::default(),
            editor_ref: NodeRef::default(),
            _timeout: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SwitchTheme(dark_mode) => {
                self.dark_mode = dark_mode;
            }
            Msg::LandingOff => {
                self.work_mode = true;
            }
            Msg::GoToLanding => {
                self.work_mode = false;
                if let Some(landing) = self.landing_ref.cast::<HtmlElement>() {
                    landing.scroll_into_view();
                }
            }
            Msg::GoToeditor => {
                if let Some(pg) = self.editor_ref.cast::<HtmlElement>() {
                    pg.scroll_into_view();
                }
                let link = ctx.link().clone();
                let pg = self.editor_ref.clone();
                let timeout = Timeout::new(800, move || {
                    // 0.8sec delay to scroll to editor
                    if let Some(pg) = pg.cast::<HtmlElement>() {
                        let _ = pg.focus();
                    };
                    link.send_message(Msg::LandingOff);
                });
                self._timeout = Some(timeout);
                return false;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
        <div class={if self.is_dark_mode() { "dark scroll-smooth" } else { "scroll-smooth" }}>
        <div class="w-full flex flex-col bg-gray-100 dark:bg-dark-primary min-h-screen">
            // navbar
            <button onclick={ctx.link().callback(|_| Msg::GoToLanding)}>
            <div class="w-full bg-gray-100 dark:bg-dark-primary" style="position: fixed; top: 0; z-index: 10;">
            <div class="max-w-md md:max-w-2xl lg:max-w-4xl 2xl:max-w-6xl 4xl:max-w-8xl mx-auto">
            <div class="flex items-center justify-between px-0 py-4 border-b border-gray-200 dark:border-gray-700">
                <h1 class="text-2xl max-md:text-lg font-bold tracking-tight text-gray-800 dark:text-gray-200">
                    {"envshare-rs"}
                </h1>
                <div class="flex items-center space-x-2">
                    // theme
                    <ThemeComponent on_clicked={ctx.link().callback(Msg::SwitchTheme)}/>
                    // docs
                    <a href="https://github.com/0xrusowsky/envshare-rs" target="_blank" class="group pr-1">
                        <div class="flex items-center space-x-2 transition-colors relative z-10 text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-300">
                            <svg role="img" width="20" height="20" viewBox="0 0 24 24" fill="currentColor" class="scale-100 group-hover:scale-110 transition-transform"><path fill-rule="evenodd" d="m22.903 11.728-4.528-1.697V4.945a1.69 1.69 0 0 0-1.097-1.58l-4.687-1.757a1.668 1.668 0 0 0-1.186 0L6.717 3.366a1.687 1.687 0 0 0-1.097 1.58v5.085l-4.528 1.697A1.69 1.69 0 0 0 0 13.308v5.16c0 .638.36 1.224.933 1.51l4.687 2.344a1.68 1.68 0 0 0 1.51 0L12 19.884l4.87 2.438a1.68 1.68 0 0 0 1.51 0l4.687-2.344a1.69 1.69 0 0 0 .933-1.51v-5.16c0-.703-.436-1.331-1.097-1.58zm-6.122-1.66-3.984 1.496V8.367l3.984-1.734zM7.22 4.88 12 3.09l4.781 1.79v.028L12 6.848l-4.781-1.94Zm3.937 13.645-3.984 1.992V16.81l3.984-1.818zm0-5.25-4.781 1.94-4.781-1.94v-.028l4.781-1.79 4.781 1.79zm11.25 5.25-3.984 1.992V16.81l3.984-1.818zm0-5.25-4.781 1.94-4.781-1.94v-.028l4.781-1.79 4.781 1.79z"/></svg>
                            <p class="text-sm hidden sm:inline">{"Docs"}</p>
                        </div>
                    </a>

                    // github
                    <a href="https://github.com/0xrusowsky/envshare-rs" target="_blank" class="text-gray-600 dark:text-gray-400 transition-colors duration-200 hover:scale-110 hover:text-gray-900 dark:hover:text-gray-100">
                    <svg width="22.5" height="22.5" viewBox="0 -.75 16 16" fill="currentColor"><path fill-rule="evenodd" d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z"></path></svg>
                    </a>
                </div>
            </div>
            </div>
            </div>
            </button>
        // landing
        if !self.work_mode {
        <div id="landing" ref={self.landing_ref.clone()} class="bg-gray-100 dark:bg-dark-primary"
            style="min-height: 100vh; display: flex; flex-direction: column;">
        <div class="flex-grow flex flex-col justify-between text-gray-800 dark:text-gray-200">
            <div class="flex flex-grow items-center justify-center">
            <div class="text-lg max-sm:text-sm text-center">
                <br/>
                <br/>
                <br/>
                <br/>
                <h1 class="text-6xl max-sm:text-4xl text-center font-extrabold tracking-wide py-8">
                    <div class="flex items-center justify-center text-center"><p class="pr-6">{"Share"}</p><p>{"Environment"}</p></div>
                    <div class="flex items-center justify-center text-center pt-2"><p class="pr-6">{"Variables"}</p><p>{"Securely"}</p></div>
                </h1>
                <p class="text-lg max-sm:text-md text-center"> {"Your secret is encrypted in your browser before being stored for a limited period of time and read operations."} </p>
                <p class="text-lg max-sm:text-md text-center"> {"Unencrypted data never leaves your browser."} </p>
                <br/>
                <br/>
                <br/>
                <br/>
                <br/>
                <div class="flex justify-center py-4"><img src="assets/ferris.svg" class="w-20"/></div>
                <div class="flex items-center text-center justify-center">
                    <p>{"Inspired by"}</p><a href="https://envshare.dev" class="font-bold hidden sm:inline pl-1 hover:scale-105">{"EnvShare"}</a>
                    <p>{", rebuilt from the ground up in"}</p><a href="https://www.rust-lang.org/" class="font-bold pl-1 hover:scale-105">{"Rust"}</a><p>{"."}</p>
                </div>
            </div>
            </div>
            <div class="p-4 w-full">
                <a class="transition-all" onclick={ctx.link().callback(|_| Msg::GoToeditor)}>
                    <button class="btn mx-auto block hover:font-semibold animate-bounce w-100 h-6">
                        {"Try it out"}
                    </button>
                    <svg class="mx-auto animate-bounce w-100 h-6" width="24" height="24" viewBox="0 0 22 22" fill="currentColor"><path d="M12 16a1 1 0 0 1-.64-.23l-6-5a1 1 0 1 1 1.28-1.54L12 13.71l5.36-4.32a1 1 0 0 1 1.41.15 1 1 0 0 1-.14 1.46l-6 4.83A1 1 0 0 1 12 16z"/></svg>
                </a>
            </div>
            </div>
        </div>
        }
        // editor
        <div ref={self.editor_ref.clone()} class="px-3 bg-gray-100 dark:bg-dark-primary md:px-0 flex flex-col">
        <div class="flex flex-col items-center justify-center w-full space-y-8">
        <div class="w-full max-w-md md:max-w-2xl lg:max-w-4xl 2xl:max-w-6xl 4xl:max-w-8xl 8xl:max-w-10xl">
            <div id="editor"><FrameComponent focus_ref={self.editor_ref.clone()}/></div>
            // footer
            <div class="max-height: 10vh">
            <div class="text-sm text-gray-400 dark:text-gray-600 flex flex-col sm:flex-row justify-center items-center space-x-2">
                <p> {"© 2024 envshare-rs"} </p>
                <p class="hidden sm:inline px-1"> {"|"} </p>
                <a class="transition-colors duration-200 hover:scale-105 hover:text-gray-900 dark:hover:text-gray-100"
                    href="https://0xrusowsky.github.io/blog/"> {"0xrusowsky"} </a>
            </div>
            <div class="text-sm text-gray-400 dark:text-gray-600 flex flex-col sm:flex-row justify-center items-center pt-0.5 pb-2">
                <p>{"Inspired by"}</p>
                <div class="flex hover:scale-105 font-bold hover:font-medium">
                    <a class="pl-1 transition-colors duration-200 hover:text-gray-900 dark:hover:text-gray-100" href="https://envshare.dev">{"EnvShare"}</a>
                    <p class="pr-1">{","}</p>
                </div>
                <p>{"rebuilt from the ground up in"}</p>
                <div class="flex hover:scale-105 font-bold hover:font-medium">
                    <a class="pl-1 transition-colors duration-200 hover:text-gray-900 dark:hover:text-gray-100" href="https://www.rust-lang.org">{"Rust"}</a>
                    <p class="pr-1">{"."}</p>
                </div>
                </div>
            </div>
        </div>
        </div>
        </div>
        </div>
        </div>
        }
    }
}
