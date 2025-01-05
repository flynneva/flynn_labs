use yew::prelude::*;

use crate::menus::nav_bar::NavBar;
use crate::menus::dropdown::MenuDropdown;
use crate::menus::common::Item;

use crate::pages::sports::{ActiveSport, ActiveSportAction};

#[function_component(SportsNavBar)]
pub fn sports_nav_bar() -> Html {
    // Track the currently selected items
    let ctx_sport = use_reducer(ActiveSport::default);
    let active_sport = use_state(|| ctx_sport.supported_sports[0].clone());

    // Generate all the sports to select
    let mut sport_options: Vec<_> = vec![];
    for (_, sport) in ctx_sport.supported_sports.iter().enumerate() {
        let onclick = Callback::from({
            let sport = sport.clone();
            let active_sport = active_sport.clone();
            let ctx_sport = ctx_sport.clone();
            move |_| {
                active_sport.set(sport.clone());
                ctx_sport.dispatch(ActiveSportAction::SetSport(sport.name.to_string()));
        }});
        sport_options.push(Item{
            display_name: sport.name.clone(),
            html: html! {<button {onclick} class="link">{sport.name.to_string()}</button>}});
    }

    let mut division_options: Vec<_> = vec![];
    for division in active_sport.divisions.iter() {
        division_options.push({
            let onclick = Callback::from({
                let division = division.clone();
                move |_| {ctx_sport.dispatch(ActiveSportAction::SetDivision(division.to_string()))}
            });
            Item{
                display_name: division.to_string(),
                html: html! {<button {onclick} class="link">{division.to_string()}</button>}
    }})};

    let mut variation_options: Vec<_> = vec![];
    if let Some(variations) = &active_sport.variations {
        for variation in variations {
            let variation_str = variation.clone();
            let sanitized_variation  = variation_str.trim_matches('-').to_string();
            variation_options.push({
                let onclick = Callback::from({
                    let sanitized_variation = sanitized_variation.clone();
                    move |_| {
                        ctx_sport.dispatch(ActiveSportAction::SetVariation(sanitized_variation.to_string()))}
                });
                Item{
                    display_name: sanitized_variation.to_string(),
                    html: html! {<button {onclick} class="link">{&sanitized_variation}</button>}
    }})}};

    let mut nav_buttons: Vec<_> = vec![
        Item{
            display_name: "Sport selector".to_string(),
            html: html! {<MenuDropdown key={"sport_selector".to_string()} items={sport_options} />}
        }
    ];

    if !variation_options.is_empty() {
        nav_buttons.push(
            Item{
                display_name: "Variation selector".to_string(),
                html: html! {<MenuDropdown key={"variation_selector".to_string()} items={variation_options} />},
            }
        );
    };

    nav_buttons.push(
        Item{
            display_name: "Division selector".to_string(),
            html: html! {<MenuDropdown key={"division_selector".to_string()} items={division_options} />},
        }
    );

    html! {
        <div>
          <NavBar items={nav_buttons}/>
        </div>
    }
}