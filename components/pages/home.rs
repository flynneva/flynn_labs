use chrono::{Utc, FixedOffset};

use yew::html;
use yew::function_component;
use yew::Html;

use crate::pages::page::*;

#[function_component]
pub fn HomePage() -> Html {
    let current_date = Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap());
    let current_date_str = current_date.format("%Y/%m/%d").to_string();
    let pages= vec![
        Page {
            id: 1,
            title: "Sports".to_string(),
            description: "Things about sports".to_string(),
            url: format!("sports/basketball/men/d1/{}", current_date_str),
        },
        Page {
            id: 2,
            title: "Robots".to_string(),
            description: "Things about robots".to_string(),
            url: "robots".to_string(),
        },
        Page {
            id: 3,
            title: "Garden".to_string(),
            description: "Things that grow".to_string(),
            url: "garden".to_string(),
        },
        
    ];

    html! {
        <>
            <div class="scroll-container">
                <h1>{"Flynn Labs"}</h1>
                <PagesList pages={pages} />
            </div>
        </>
    }
}