use yew::prelude::*;

// use ncaa_data_rs::ncaa::query;
use ncaa_data_rs::ncaa::{
    sports::supported_sports,
    sports::Sport,
};

use crate::menus::nav_bar::NavBar;
use crate::menus::dropdown::MenuDropdown;
use crate::menus::common::Item;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_sport_select: Callback<String>,
    pub on_variation_select: Callback<String>,
    pub on_division_select: Callback<String>,
}

#[function_component(SportsNavBar)]
pub fn sports_nav_bar(props: &Props) -> Html {
    let sports: Vec<Sport> = supported_sports();
    // Track the currently selected items
    let active_sport = use_state(|| sports[0].clone());
    let active_variation = use_state(|| "men".to_string());
    let active_division = use_state(|| "d1".to_string());
    // TODO: should this be its own fn?
    // Generate all the sports to select
    let mut sport_options: Vec<_> = vec![];
    for (id, sport) in sports.iter().enumerate() {
        let onclick = Callback::from({
            let sport = sport.clone();
            let on_sport_select = props.on_sport_select.clone();
            let active_sport = active_sport.clone();
            move |_| {
                active_sport.set(sport.clone());
                on_sport_select.emit(sport.name.to_string())
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
                let on_division_select = props.on_division_select.clone();
                move |_| { on_division_select.emit(division.to_string())}
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
                    let on_variation_select = props.on_variation_select.clone();
                    move |_| { on_variation_select.emit(sanitized_variation.clone())}
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