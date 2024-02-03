use material_yew::{
    MatTabBar,
    MatTab,
};
use yew::prelude::*;

use crate::pages::AppRoute;
use crate::pages::AppLink;

#[function_component(MainMenu)]
pub fn main_menu() -> Html {
    html! {
        <span>
            <MatTabBar>
                <AppLink to={AppRoute::Home}><MatTab label="Home" icon="home" is_fading_indicator=true /></AppLink>
                <AppLink to={AppRoute::Sports}><MatTab label="Sports" icon="sports" is_fading_indicator=true /></AppLink>
                <AppLink to={AppRoute::Robots}><MatTab label="Robots" icon="smart_toy" is_fading_indicator=true /></AppLink>
                <AppLink to={AppRoute::Garden}><MatTab label="Garden" icon="nature" is_fading_indicator=true /></AppLink>
            </MatTabBar>
        </span>
        
    }
}