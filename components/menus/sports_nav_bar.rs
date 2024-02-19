use yew::prelude::*;

// use ncaa_data_rs::ncaa::query;
use ncaa_data_rs::ncaa::structs::{
    sports::supported_sports,
};

use crate::menus::nav_bar::NavBar;
use crate::menus::dropdown::MenuDropdown;
use crate::menus::common::Item;


#[function_component(SportsNavBar)]
pub fn sports_nav_bar() -> Html {
    let sports = supported_sports();
    // Track the currently selected sport
    let selected_sport = use_state(|| sports[0].clone());

    // Generate all the sports to select
    let sport_options: Vec<_> = sports.iter().enumerate().map(|(id, sport)|
        Item{
            display_name: sport.name.to_string(),
            html: html! {
              <div class="dropdown-item" id={format!("{}", &id)}>
                <a href={sport.name.to_string()} class="link">{&sport.name}</a>
              </div>}}).collect();

    let division_options: Vec<_> = selected_sport.divisions.iter().map(|division|
      Item{
        display_name: division.to_string(),
        html: html! {
            <div class="dropdown-item">
              <a href={division.to_string()} class="link">{&division}</a>
            </div>}}).collect();

    let mut variation_options: Vec<_> = vec![];
    if let Some(variations) = &selected_sport.variations {
        for variation in variations {
            let variation_str = variation.to_string();
            let sanitized_variation  = variation_str.trim_matches('-');
            variation_options.push(
                Item{
                    display_name: sanitized_variation.to_string(),
                    html: html! {
                        <div class="dropdown-item">
                          <a href={sanitized_variation.to_string()} class="link">{&sanitized_variation}</a>
                        </div>}});
        }
    }

    let mut nav_buttons: Vec<_> = vec![
        Item{
            display_name: "Sport selector".to_string(),
            html: html! {<MenuDropdown items={sport_options} />}
        }
    ];

    println!("{:?}", variation_options);
    if !variation_options.is_empty() {
        nav_buttons.push(
            Item{
                display_name: "Variation selector".to_string(),
                html: html! {<MenuDropdown items={variation_options} />},
            }
        );
    };

    nav_buttons.push(
        Item{
            display_name: "Division selector".to_string(),
            html: html! {<MenuDropdown items={division_options} />},
        }
    );

    html! {
        <NavBar items={nav_buttons}/>
    }
}