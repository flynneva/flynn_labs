use yew::prelude::*;
use yew_router::prelude::*;
use material_yew::MatButton;

use crate::components::layout::{
    tabs::TopTabs,
};

use crate::components::pages::Route;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    let history = use_history().unwrap();
    let onclick = Callback::once(move |_| history.push(Route::Home));

    html! {
        <>
            <TopTabs />
            <h1>{ "Home" }</h1>
            <span {onclick}>
                <MatButton label="Go Home" ></MatButton>
            </span>
        </>
    }
}