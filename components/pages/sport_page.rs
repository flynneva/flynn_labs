use yew::prelude::*;

// use ncaa_data_rs::ncaa::query;
use ncaa_data_rs::ncaa::structs::{
    sports::supported_sports,
    sports::Sport
};

use crate::menus::dropdown::MenuDropdown;

#[derive(Properties, PartialEq)]
pub struct SportPageProps {
    pub sport: String,
}


#[function_component(SportPage)]
pub fn sport_page(SportPageProps {sport}: &SportPageProps) -> Html {
    let sports = supported_sports();

    html! {
        <div>
            {"Sport page"}
        </div>
    }
}