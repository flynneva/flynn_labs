use yew::prelude::*;

use crate::components::layout::{
    menu::MainMenu,
};

#[function_component(HomePage)]
pub fn home_page() -> Html {

    html! {
        <>
            <MainMenu />
            <h1>{ "Home" }</h1>
            <p>{"Welcome, welcome, welcome!"}</p>
        </>
    }
}