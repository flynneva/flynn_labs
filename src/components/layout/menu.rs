use material_yew::{
    MatTabBar,
    MatTab,
};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::pages::Route;

#[function_component(MainMenu)]
pub fn main_menu() -> Html {
    html! {
        <span>
            <MatTabBar>
                <Link<Route> to={Route::Home}><MatTab label="Home" icon="home" is_fading_indicator=true /></Link<Route>>
                <Link<Route> to={Route::Sports}><MatTab label="Sports" icon="sports" is_fading_indicator=true /></Link<Route>>
                <Link<Route> to={Route::Robots}><MatTab label="Robots" icon="smart_toy" is_fading_indicator=true /></Link<Route>>
                <Link<Route> to={Route::Garden}><MatTab label="Garden" icon="nature" is_fading_indicator=true /></Link<Route>>
            </MatTabBar>
        </span>
        
    }
}