use chrono::{Datelike, Local};

use yew::html;
use yew::function_component;
use yew::Html;

use crate::pages::page::*;
use crate::pages::Route;

#[function_component]
pub fn HomePage() -> Html {
    let current_date = Local::now();
    let pages= vec![
        Page {
            id: 1,
            title: "Sports".to_string(),
            description: "Things about sports".to_string(),
            route: Route::Sports{
                sport: "basketball".to_string(),
                variation: "men".to_string(),
                division: "d1".to_string(),
                year: current_date.year(),
                month: current_date.month(),
                day: current_date.day() },
        },
        Page {
            id: 2,
            title: "Robots".to_string(),
            description: "Things about robots".to_string(),
            route: Route::Robots,
        },
        Page {
            id: 3,
            title: "Garden".to_string(),
            description: "Things that grow".to_string(),
            route: Route::Garden,
        },
    ];

    html! {
        <>
            <div class="scroll-container">
                <h1>{"Flynn Labs"}</h1>
                <p>{"Welcome to my website!"}</p>
                <p>{"Here you will find a mix of things that I am interested in enough to actually
                     write a bit of code about them."}</p>
                <PagesList pages={pages} />
            </div>
        </>
    }
}