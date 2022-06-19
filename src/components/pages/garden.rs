use material_yew::{
    MatButton,
};

use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::pages::Route;

use crate::components::layout::{
    tabs::TopTabs,
};

#[function_component(GardenPage)]
pub fn sports() -> Html {
    let history = use_history().unwrap();
    let onclick = Callback::once(move |_| history.push(Route::Home));

    html! {
        <>
            <TopTabs />
            <h1>{ "Garden" }</h1>
            <span {onclick}>
                <MatButton label="Go Home" ></MatButton>
            </span>
        </>
    }
}