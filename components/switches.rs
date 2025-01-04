use yew::prelude::*;
use yew_router::prelude::*;

use chrono::{Datelike, FixedOffset, Local, TimeZone};

use crate::routes::{
    MainRoute,
    SportsRoute,
};

use crate::pages::{
    home::HomePage,
    scoreboard::ScoreboardPage,
    game::GamePage,
    robots::RobotsPage,
    garden::GardenPage,
    not_found::NotFoundPage,
};

pub fn switch_main(routes: MainRoute) -> Html {
    match routes {
      MainRoute::Home => html! { <HomePage /> },
      MainRoute::SportsRoot | MainRoute::Sports => html! { <Switch<SportsRoute> render={switch_sports} />},
      MainRoute::Robots => html! { <RobotsPage /> },
      MainRoute::Garden => html! { <GardenPage /> },
      MainRoute::NotFound => html! { <NotFoundPage /> },
    }
}
  
pub fn switch_sports(routes: SportsRoute) -> Html {
    let current_date = Local::now();
    match routes {
        SportsRoute::Root => html! {<Redirect<SportsRoute> to={SportsRoute::Scoreboard {
            sport: "basketball".to_string(),
            variation: "men".to_string(),
            division: "d1".to_string(),
            year: current_date.year(),
            month: current_date.month(),
            day: current_date.day()}} />},
        SportsRoute::Scoreboard { sport, variation, division, year, month, day} => html! {
            <ScoreboardPage
                sport={sport}
                variation={variation}
                division={division}
                date={FixedOffset::east_opt(0).unwrap().with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap()}
            />
        },
        SportsRoute::Game { id } => html! { <GamePage id={id} />},
        SportsRoute::NotFound => html! {<Redirect<MainRoute> to={MainRoute::NotFound} />},
    }
}
