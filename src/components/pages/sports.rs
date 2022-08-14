use yew::prelude::*;

use material_yew::{
    MatButton,
};

use crate::components::layout::{
    tabs::TopTabs,
};

struct SportLevel {
    inner: bool,  // true for pro, false for NCAA
}
#[function_component(SportsPage)]
pub fn sports() -> Html {
    html! {
        <div>
            <TopTabs />
            <MatButton label="NCAA" icon="school" />
        </div>
    }
}