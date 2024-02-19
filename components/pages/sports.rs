use yew::prelude::*;

use crate::menus::sports_nav_bar::SportsNavBar;


#[function_component(SportsPage)]
pub fn sports() -> Html {
    html! {
        <div>
            <SportsNavBar />
        </div>
    }
}