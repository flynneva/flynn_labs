use yew::prelude::*;

// use ncaa_data_rs::ncaa::query;
use ncaa_data_rs::ncaa::structs::{
    sports::supported_sports,
    sports::Sport
};

use crate::menus::nav_bar::NavBar;
use crate::menus::dropdown::MenuDropdown;
use crate::menus::common::Item;


#[function_component(SportsNavBar)]
pub fn sports_nav_bar() -> Html {
    let sports = supported_sports();
    // Track the currently selected sport
    let selected_sport = use_state(|| sports[0].clone());
    let sport_options: Vec<_> = sports.iter().enumerate().map(|(id, sport)|
        Item{
            display_name: sport.name.clone(),
            html: html! {
              <div class="dropdown-item" id={format!("{}", &id)}>
                <a href={sport.name.clone()} class="link">{&sport.name}</a>
              </div>}}).collect();

    let division_options: Vec<_> = selected_sport.divisions.iter().map(|division|
      Item{
        display_name: division.to_string(),
        html: html! {
            <div class="dropdown-item">
              <a href={division.clone()} class="link">{&division}</a>
            </div>}}).collect();

    let nav_buttons: Vec<_> = vec![
        Item{
            display_name: "Sport selector".to_string(),
            html: html! {<MenuDropdown items={sport_options} />}
        },
        Item{
            display_name: "Division selector".to_string(),
            html: html! {<MenuDropdown items={division_options} />},
        }
    ];

    html! {
        <NavBar items={nav_buttons}/>
    }
}