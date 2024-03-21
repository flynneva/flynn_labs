use chrono::{FixedOffset, TimeZone};
use yew::prelude::*;
use yew_router::prelude::*;

use components::pages::{
    Route,
    DATE_FMT,
    home::HomePage,
    sports::SportsPage,
    game::GamePage,
    robots::RobotsPage,
    garden::GardenPage,
    not_found::NotFoundPage,
};


fn switch(routes: Route) -> Html {
  match routes {
    Route::Home => html! { <HomePage /> },
    Route::Sports { sport, variation, division, year, month, day} => html! {
      <SportsPage sport={sport} variation={variation} division={division}
        date={FixedOffset::east_opt(0).unwrap().with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap()} />
    },
    Route::Game { id } => html! { <GamePage id={id} />},
    Route::Robots => html! { <RobotsPage /> },
    Route::Garden => html! { <GardenPage /> },
    Route::NotFound => html! { <NotFoundPage /> },
  }
}

#[function_component]
fn App() -> Html {
    html! {
        <>
          <BrowserRouter>
            <Switch <Route> render={switch} />
          </BrowserRouter>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}