use yew::prelude::*;
use yew_router::prelude::*;
use material_yew::MatButton;

use crate::components::pages::Route;

use crate::components::layout::{
    tabs::TopTabs,
    tabs::SportsTabs,
};

#[function_component(SportsPage)]
pub fn sports() -> Html {
    let history = use_history().unwrap();
    
    html! {
        <div>
            <TopTabs />
            <SportsTabs />
            <h1>{ "Sports" }</h1>
        </div>
    }
}