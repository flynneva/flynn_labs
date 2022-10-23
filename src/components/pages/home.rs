use yew::prelude::*;

#[function_component(HomePage)]
pub fn home_page() -> Html {

    html! {
        <>
            <h1>{ "Home" }</h1>
            <p>{"Welcome to my site!"}</p>
            <br/>
            <p>{"Here I experiment with ideas and learn new things."}</p>
            <br/>
            <p>{"Hope you enjoy it!"}</p>
        </>
    }
}