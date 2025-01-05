use yew::prelude::*;
use yew_router::prelude::*;

use ncaa_data_rs::ncaa::scoreboard::Scoreboard;
use ncaa_data_rs::ncaa::query;

use crate::menus::sports_nav_bar::SportsNavBar;
use crate::cards::game::GameCard;
use crate::pages::sports::ActiveSport;

#[function_component]
pub fn ScoreboardPage() -> Html {
    let navigator = use_navigator().unwrap();
    let active_sport = use_reducer(ActiveSport::default);
    let active_scoreboard = use_state(|| Scoreboard{
        input_md5: "".to_string(),
        updated_at: "".to_string(),
        games: Vec::new(),
    });

    {
        let active_scoreboard = active_scoreboard.clone();
        let active_sport = active_sport.clone();
        use_effect_with(active_sport, move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match query::scoreboard(&active_sport.name, &active_sport.division, &active_sport.date).await {
                    Ok(scoreboard) => {active_scoreboard.set(scoreboard);}
                    Err(_) => {active_scoreboard.set(Scoreboard::default());}
                }
            })
        });
    }
    let games: Vec<_> = active_scoreboard.games.iter().map(|game| {
        html! {
            <GameCard
                id={game.details.url.clone()}
                home_name={game.details.home.names.char6.clone()}
                away_name={game.details.away.names.char6.clone()}
                current_clock={game.details.contest_clock.clone()}
                current_period={game.details.current_period.clone()}
                home_score={game.details.home.score.as_str().unwrap_or_default().to_string().clone()}
                home_record={game.details.home.description.clone()}
                home_rank={game.details.home.rank.clone()}
                away_score={game.details.away.score.as_str().unwrap_or_default().to_string().clone()}
                away_record={game.details.away.description.clone()}
                away_rank={game.details.away.rank.clone()}
                start_time={game.details.start_time.clone()}
            />
        }
    }).collect();

    html! {
        <div>
            <div class="scoreboard-games">
              {games}
            </div>
            <SportsNavBar />
        </div>
    }
}