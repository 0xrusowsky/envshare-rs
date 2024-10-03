mod app;
mod components;

use app::{home::Home, unseal::Unseal};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/share")]
    Share,
    #[at("/unseal/:id")]
    Unseal { id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! {<Home/>},
        Route::Share => html! {<Home/>},
        Route::Unseal { id } => html! { <Unseal encoded_key={id}/> },
        Route::NotFound => html! {<p>{"404"}</p>},
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
