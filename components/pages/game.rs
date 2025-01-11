mod basketball;
mod football;

use yew::prelude::*;

use ncaa_data_rs::ncaa::sport::Get;
use ncaa_data_rs::ncaa::Sports;
use ncaa_data_rs::ncaa::basketball::BasketballGame;
use ncaa_data_rs::ncaa::football::FootballGame;

use ncaa_data_rs::ncaa::game::GameFromStr;
use ncaa_data_rs::ncaa::game::GameID;

use crate::pages::game::basketball::BasketballGamePage;
use crate::pages::game::football::FootballGamePage;
use std::ops::Deref;

#[derive(Properties, PartialEq)]
pub struct Props<T: PartialEq> {
    pub game: T,
    pub selected_sport: Sports,
}

#[function_component]
pub fn GamePage<T>(props: &Props<T>) -> Html
where
    T: Clone + PartialEq + Default + Get + GameID + 'static,
{
    let game = use_state(|| props.game.clone());
    let selected_sport = use_state(|| props.selected_sport.clone());
    let sport_specific_game_page = match selected_sport.deref() {
        Sports::BASKETBALL => html!{
            <BasketballGamePage game={
                <BasketballGame as GameFromStr<BasketballGame>>::from_str(
                    &game.deref().sport(),
                    &game.deref().division(),
                    &game.deref().variation(),
                    &game.deref().id())
            .expect("Invalid basketball game parameters")}/>},
        Sports::FOOTBALL => html!{
            <FootballGamePage game={
                    <FootballGame as GameFromStr<FootballGame>>::from_str(
                        &game.deref().sport(),
                        &game.deref().division(),
                        &game.deref().variation(),
                        &game.deref().id())
                .expect("Invalid football game parameters")}/>},
        // _ => html!{<div>{"This sports game page has yet to be developed"}</div>},
    };
    html! {
        <div class="game-container">
            {sport_specific_game_page}
        </div>
    }
}
