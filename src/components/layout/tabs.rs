use material_yew::{
    MatButton,
    MatTabBar,
    MatTab,
};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::pages::Route;

#[function_component(TopTabs)]
pub fn top_tabs() -> Html {
    html! {
        <MatTabBar>
            <Link<Route> to={Route::Home}><MatTab label="Home" icon="home" is_fading_indicator=true /></Link<Route>>
            <Link<Route> to={Route::Sports}><MatTab label="Sports" icon="sports" is_fading_indicator=true /></Link<Route>>
            <Link<Route> to={Route::Robots}><MatTab label="Robots" icon="smart_toy" is_fading_indicator=true /></Link<Route>>
            <Link<Route> to={Route::Garden}><MatTab label="Garden" icon="nature" is_fading_indicator=true /></Link<Route>>
        </MatTabBar>
    }
}

#[function_component(SportsTabs)]
pub fn sports_tabs() -> Html {

    html! {
        <MatTabBar>
            <Link<Route> to={Route::Home}><MatTab label="NCAA" icon="school" is_fading_indicator=true /></Link<Route>>
            <Link<Route> to={Route::Sports}><MatTab label="Professional" icon="sports" is_fading_indicator=true /></Link<Route>>
        </MatTabBar>
    }
}