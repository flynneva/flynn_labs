use yew::prelude::*;

// use ncaa_data_rs::ncaa::query;
use ncaa_data_rs::ncaa::structs::{
    sports::supported_sports,
    sports::Sport
};

use crate::menus::sports_nav_bar::SportsNavBar;


#[function_component(SportsPage)]
pub fn sports() -> Html {
    html! {
        <div>
            <SportsNavBar />
        </div>
    }
}