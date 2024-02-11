use yew::prelude::*;

// use ncaa_data_rs::ncaa::query;
use ncaa_data_rs::ncaa::structs::{
    sports::supported_sports,
    sports::Sport
};

#[derive(Properties, PartialEq)]
pub struct SportsMenuProps {
    pub sports: Vec<Sport>,
}

#[function_component(SportsSelector)]
pub fn sports_selector(SportsMenuProps {sports}: &SportsMenuProps) -> Html {
    let sport_options: Vec<_> = sports.iter().enumerate().map(|(id, sport)| html! {
        <option id={format!("{}", &id)}>{&sport.name}</option>
    }).collect();

    html! {
        <div class="custom-select">
            <select>
                {sport_options}
            </select>
        </div>
    }
}

#[function_component(SportsBottomMenu)]
pub fn sports_bottom_menu() -> Html {
    let sports = supported_sports();

    html! {
        <div>
          <SportsSelector sports={sports} />
        </div>
    }
}

#[function_component(SportsPage)]
pub fn sports() -> Html {
    html! {
        <div>
            <SportsBottomMenu />
        </div>
    }
}