use yew::prelude::*;

use ncaa_data_rs::ncaa::sport::Get;
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
    T: Clone + PartialEq + Default + BaseQueries + Get + 'static,
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
        let game_id = scoreboard.get_id_of_game(index);
        games.push(html! {<GameCard<T> sport={sport.deref().clone()} game={game.clone()} id={game_id} />})
    }

    html! {
        <div class="scoreboard">
            <div class="scoreboard-games">
                {games}
            </div>
            <h4 class="scoreboard-sport-name">{
                format!(
                    "{} {} games",
                    sport.deref().division(),
                    sport.deref().sport(),
                )
            }</h4>
        </div>
    }
}