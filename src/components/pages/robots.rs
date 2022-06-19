use yew::prelude::*;
use yew_router::prelude::*;
use material_yew::MatButton;

use crate::components::pages::Route;

use crate::components::layout::{
    tabs::TopTabs,
};

#[function_component(RobotsPage)]
pub fn robots() -> Html {
    let history = use_history().unwrap();
    let onclick = Callback::once(move |_| history.push(Route::Home));
    
    html! {
        <div>
            <TopTabs />
            <h1>{ "Robots" }</h1>
            <span {onclick}>
                <MatButton label="Go Home" ></MatButton>
            </span>
        </div>
    }
}