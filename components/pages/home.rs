use yew::html;
use yew::function_component;
use yew::Html;

use crate::pages::page::*;

#[function_component(HomePage)]
pub fn home_page() -> Html {

    let pages= vec![
        Page {
            id: 0,
            title: "Home".to_string(),
            description: "The homepage".to_string(),
            url: "/".to_string(),
        },
        Page {
            id: 1,
            title: "Sports".to_string(),
            description: "The page about sports".to_string(),
            url: "/sports".to_string(),
        },
        Page {
            id: 2,
            title: "Robots".to_string(),
            description: "Things about robots".to_string(),
            url: "/robots".to_string(),
        },
        Page {
            id: 3,
            title: "Garden".to_string(),
            description: "Things that grow".to_string(),
            url: "/garden".to_string(),
        },
        
    ];

    html! {
        <>
            <div class="scroll-container">
                <PagesList pages={pages} />
            </div>
        </>
    }
}