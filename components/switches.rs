use yew::prelude::*;
use yew_router::prelude::*;

use chrono::{Datelike, FixedOffset, TimeZone};

use crate::routes::{
    MainRoute,
    SportsRoute,
};

use crate::pages::{
    home::HomePage,
    sports::SportsPages,
    scoreboard::ScoreboardPage,
    game::GamePage,
    robots::RobotsPage,
    garden::GardenPage,
    not_found::NotFoundPage,
};

use crate::pages::sports::ActiveSport;

pub fn switch_main(routes: MainRoute) -> Html {
    match routes {
      MainRoute::Home => html! { <HomePage /> },
      MainRoute::SportsRoot | MainRoute::Sports => html! { <SportsPages />},
      MainRoute::Robots => html! { <RobotsPage /> },
      MainRoute::Garden => html! { <GardenPage /> },
      MainRoute::NotFound => html! { <NotFoundPage /> },
    }
}
  
pub fn switch_sports(routes: SportsRoute) -> Html {
    let active_sport = ActiveSport::default();

    match routes {
        SportsRoute::Root => html! {<Redirect<SportsRoute> to={SportsRoute::Scoreboard {
            sport: active_sport.name,
            variation: active_sport.variation.expect("todo"),
            division: active_sport.division,
            year: active_sport.date.year(),
            month: active_sport.date.month(),
            day: active_sport.date.day()}} />},
        SportsRoute::Scoreboard { sport, variation, division, year, month, day} => html! {
            <ScoreboardPage
                sport={sport}
                variation={variation}
                division={division}
                date={FixedOffset::east_opt(0).unwrap().with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap()}
            />
        },
        SportsRoute::GameRoot => html! { <Redirect<SportsRoute> to={SportsRoute::Root} /> },
        SportsRoute::Game { id } => html! { <GamePage id={id} />},
        SportsRoute::NotFound => html! {<Redirect<MainRoute> to={MainRoute::NotFound} />},
    }
}
