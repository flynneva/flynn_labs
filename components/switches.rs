use yew::prelude::*;
use yew_router::prelude::*;

use std::str::FromStr;

use crate::routes::MainRoute;

use crate::pages::{
    home::HomePage,
    scoreboard::ScoreboardPage,
    // game::GamePage,
    robots::RobotsPage,
    garden::GardenPage,
    not_found::NotFoundPage,
};

use ncaa_data_rs::ncaa::Sports;
use ncaa_data_rs::ncaa::basketball::{
    Basketball,
    NAME as BASKETBALL_NAME,
    Variation as BASKETBALL_VARIATION,
    Division as BASKETBALL_DIVISION,
};
use ncaa_data_rs::ncaa::football::Football;

pub fn switch_main(routes: MainRoute) -> Html {
    match routes {
      MainRoute::Home => html! { <HomePage /> },
      MainRoute::Sports => html! {
        <Redirect<MainRoute> to={
            MainRoute::Scoreboard {
                sport: BASKETBALL_NAME.to_string(),
                variation: BASKETBALL_VARIATION::MEN.name(),
                division: BASKETBALL_DIVISION::D1.name(),
            }}
        />
      },
      MainRoute::Scoreboard {sport, variation, division} => {
        let selected_sport = Sports::from_str(&sport);
        let scoreboard_page =  match selected_sport {
          Ok(Sports::BASKETBALL) => html!{<ScoreboardPage<Basketball> />},
          Ok(Sports::FOOTBALL) => html!{<ScoreboardPage<Football> />},
          _ => html!{<Redirect<MainRoute> to={MainRoute::NotFound} />},
        };
        scoreboard_page
      },
      MainRoute::Robots => html! { <RobotsPage /> },
      MainRoute::Garden => html! { <GardenPage /> },
      MainRoute::NotFound => html! { <NotFoundPage /> },
    }
}
