use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::Route;
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

    // TODO: move this to its own function?
    let on_sport_select: Callback<String> = Callback::from({
        let active_sport = active_sport.clone();
        let active_variation = active_variation.clone();
        let active_division = active_division.clone();
        let navigator = navigator.clone();
        let sports = sports.clone();
        move |sport_name: String| {
            for sport in sports.iter() {
                if sport.name.to_string() == sport_name {
                    active_sport.set(sport.clone());
                    let mut variation = active_variation.to_string();
                    let mut division = active_division.to_string();
                    if !&sport.divisions.contains(&active_division.to_string()) {
                        // If the currently active division is not in the newly
                        // selected sport, default to the first supported division
                        // in the sport.division vector
                        division = sport.divisions[0].clone();
                    }
                    if let Some(variations) = &sport.variations {
                        // If the currently active variation is not in the newly
                        // selected sport, default to the first supported variation
                        // in the sport.variations vector
                        if !variations.contains(&active_variation.to_string()) {
                            variation = variations[0].trim_matches('-').to_string().clone();
                        }
                    } else {
                        // TODO: this now becomes a "magic" string...figure out
                        // a better way to do this
                        // For the selected sport there are no variations
                        variation = "none".to_string().clone();
                    }
                    active_division.set(division.clone());
                    active_variation.set(variation.clone());
                    navigator.push(&Route::Sports {
                        sport: sport.name.to_string().clone(),
                        variation: variation.to_string(),
                        division: division.to_string(),
                    });
                }
            }
        }
    });
    let on_variation_select: Callback<String> = Callback::from({
        let active_variation = active_variation.clone();
        let active_sport = active_sport.clone();
        let active_division = active_division.clone();
        let navigator = navigator.clone();
        move |variation_name: String| {
            if let Some(variations) = &active_sport.variations {
                for variation in variations.iter() {
                    if variation.contains(&variation_name) {
                        active_variation.set(variation_name.clone());
                        navigator.push(&Route::Sports {
                            sport: active_sport.name.to_string().clone(),
                            variation: variation_name.to_string(),
                            division: active_division.to_string(),
                        });
                    }
                }
            }
        }
    });

    let on_division_select: Callback<String> = Callback::from({
        let active_variation = active_variation.clone();
        let active_sport = active_sport.clone();
        let active_division = active_division.clone();
        let navigator = navigator.clone();
        move |division: String| {
            if active_sport.divisions.contains(&division) {
                active_division.set(division.clone());
            } else {
                active_division.set(active_sport.divisions[0].clone());
            }
            navigator.push(&Route::Sports {
                sport: active_sport.name.to_string().clone(),
                variation: active_variation.to_string(),
                division: division.to_string(),
            });
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