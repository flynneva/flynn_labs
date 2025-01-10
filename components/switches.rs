use yew::prelude::*;
use yew_router::prelude::*;

use std::str::FromStr;

use crate::routes::MainRoute;

use crate::pages::{
    home::HomePage,
    scoreboard::ScoreboardPage,
    game::GamePage,
    robots::RobotsPage,
    garden::GardenPage,
    not_found::NotFoundPage,
};

use ncaa_data_rs::ncaa::Sports;
use ncaa_data_rs::ncaa::sport::SportFromStr;
use ncaa_data_rs::ncaa::basketball::{
    Basketball,
    NAME as BASKETBALL_NAME,
    Variation as BASKETBALL_VARIATION,
    Division as BASKETBALL_DIVISION,
};
use ncaa_data_rs::ncaa::football::Football;

use ncaa_data_rs::ncaa::game::GameFromStr;
use ncaa_data_rs::ncaa::basketball::BasketballGame;
use ncaa_data_rs::ncaa::football::FootballGame;

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
          Ok(Sports::BASKETBALL) => html!{
              <ScoreboardPage<Basketball>
                  sport={<Basketball as SportFromStr<Basketball>>::from_str(&sport, &division, &variation)
                      .expect("Invalid basketball parameters")} />},
          Ok(Sports::FOOTBALL) => html!{
              <ScoreboardPage<Football>
                  sport={<Football as SportFromStr<Football>>::from_str(&sport, &division, &variation)
                      .expect("Invalid football parameters")} />},
          _ => html!{<Redirect<MainRoute> to={MainRoute::NotFound} />},
        };
        scoreboard_page
      },
      MainRoute::Game {sport, variation, division, id} => {
        let selected_sport = Sports::from_str(&sport);
        let game_page =  match selected_sport {
          Ok(Sports::BASKETBALL) => html!{
              <GamePage<BasketballGame>
                  selected_sport={Sports::BASKETBALL}
                  game={<BasketballGame as GameFromStr<BasketballGame>>::from_str(&sport, &division, &variation, &id)
                      .expect("Invalid basketball game parameters")} />},
          Ok(Sports::FOOTBALL) => html!{
              <GamePage<FootballGame>
                  selected_sport={Sports::FOOTBALL}
                  game={<FootballGame as GameFromStr<FootballGame>>::from_str(&sport, &division, &variation, &id)
                      .expect("Invalid football game parameters")} />},
          _ => html!{<Redirect<MainRoute> to={MainRoute::NotFound} />},
        };
        game_page
      },
      MainRoute::Robots => html! { <RobotsPage /> },
      MainRoute::Garden => html! { <GardenPage /> },
      MainRoute::NotFound => html! { <NotFoundPage /> },
    }
}
