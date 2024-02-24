use yew::prelude::*;
use yew_router::prelude::*;

use crate::menus::sports_nav_bar::{
    SportsNavBar,
};
use ncaa_data_rs::ncaa::structs::sports::Sport;
use ncaa_data_rs::ncaa::structs::sports::supported_sports;

#[derive(Properties, PartialEq)]
pub struct SportProps {
    pub sport: String,
    pub variation: String,
    pub division: String,
}

#[function_component]
pub fn SportsPage(props: &SportProps) -> Html {
    let navigator = use_navigator().unwrap();
    let sports: Vec<Sport> = supported_sports();
    // Track the currently selected items
    let active_sport = use_state(|| sports[0].clone());
    let active_variation = use_state(|| "men".to_string());
    let active_division = use_state(|| "d1".to_string());

    let on_sport_select: Callback<String> = Callback::from({
        let active_sport = active_sport.clone();
        let sports = sports.clone();
        move |sport_name: String| {
            for sport in sports.iter() {
                if sport.name.to_string() == sport_name {
                    active_sport.set(sport.clone());
                }
            }
        }
    });
    let on_variation_select: Callback<String> = Callback::from({
        let active_variation = active_variation.clone();
        let active_sport = active_sport.clone();
        move |variation_name: String| {
            if let Some(variations) = &active_sport.variations {
                for variation in variations.iter() {
                    if variation_name == variation.to_string() {
                        active_variation.set(variation_name.clone());
                    }
                }
            }
        }
    });

    let on_division_select: Callback<String> = Callback::from({
        let active_division = active_division.clone();
        move |name: String| {
            // TODO: check if new division is supported by active sport and variation?
            active_division.set(name);
        }
    });

    html! {
        <div>
            <p>{&*active_sport.name.clone()}</p>
            <p>{&*active_variation.clone()}</p>
            <p>{&*active_division.clone()}</p>
            <SportsNavBar {on_sport_select} {on_variation_select} {on_division_select} />
        </div>
    }
}