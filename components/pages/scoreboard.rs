use yew::prelude::*;
use yew::html::IntoPropValue;

use ncaa_data_rs::ncaa::sport::{
    Get,
    SportFromStr,
};
use ncaa_data_rs::ncaa::scoreboard::Scoreboard;
use ncaa_data_rs::ncaa::sport::BaseQueries;

use crate::cards::game::GameCard;

use std::ops::Deref;

#[derive(Properties, PartialEq)]
pub struct Props<T: PartialEq> {
    pub sport: T,
}

#[function_component]
pub fn ScoreboardPage<T>(props: &Props<T>) -> Html
where
    T: Clone + PartialEq + Default + BaseQueries + Get + SportFromStr<T> + 'static,
{
    let sport = use_state(|| props.sport.clone());
    let scoreboard = use_state(|| Scoreboard::default());
    // extra clone here to use the sport in the closure here and below
    let sport_clone = sport.clone();
    let scoreboard_clone = scoreboard.clone();
    use_effect_with((), move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            let scoreboard = scoreboard_clone.clone();
            scoreboard.set(sport_clone.scoreboard(None).await.expect("No scoreboard available"));
        });
    });

    let mut games: Vec<_> = Vec::new();
    
    for (index, game) in scoreboard.games.iter().enumerate() {
        games.push(html! {<GameCard game={game.clone()} index={index} />})
    }

    html! {
        <div class="scoreboard">
            <h1>{
                format!(
                    "{} {} games",
                    sport.deref().division(),
                    sport.deref().name(),
                )
            }</h1>
            <div class="scoreboard-games">
                {games}
            </div>
        </div>
    }
}