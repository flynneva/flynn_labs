use yew::prelude::*;

use crate::pages::page::*;
use crate::routes::MainRoute;

#[function_component]
pub fn HomePage() -> Html {
    let pages= vec![
        Page {
            id: 1,
            title: "Sports".to_string(),
            description: "Things about sports".to_string(),
            route: MainRoute::Sports,
        },
        Page {
            id: 2,
            title: "Robots".to_string(),
            description: "Things about robots".to_string(),
            route: MainRoute::Robots,
        },
        Page {
            id: 3,
            title: "Garden".to_string(),
            description: "Things that grow".to_string(),
            route: MainRoute::Garden,
        },
    ];

    html! {
        <>
            <div class="scroll-container">
                <h1>{"Flynn Labs"}</h1>
                <p>{"Welcome to my website!"}</p>
                <p>{"Here you will find a mix of things that I am interested in enough to actually
                     write a bit of code about them."}</p>
                <div class="page-cards">
                    <PagesList pages={pages} />
                </div>
            </div>
        </>
    }
}