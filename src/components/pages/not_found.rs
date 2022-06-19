use yew::prelude::*;
use yew_router::prelude::*;
use material_yew::MatButton;

use crate::components::pages::Route;

use crate::components::layout::{
    tabs::TopTabs,
};

#[function_component(NotFoundPage)]
pub fn not_found() -> Html {
    let history = use_history().unwrap();
    let onclick = Callback::once(move |_| history.push(Route::Home));

    html! {
        <>
            <TopTabs />
            <h1>{ "Oops, looks like this page does not exist yet." }</h1>
            <span {onclick}>
                <MatButton label="Go Home" ></MatButton>
            </span>
        </>
    }
}
