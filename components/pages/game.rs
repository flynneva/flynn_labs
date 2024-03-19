use yew::prelude::*;

use ncaa_data_rs::ncaa::query;
use ncaa_data_rs::ncaa::basketball::boxscore::Boxscore;


#[derive(Properties, PartialEq)]
pub struct GameProps {
    pub id: String,
}

#[function_component]
pub fn GamePage(props: &GameProps) -> Html {
    let this_game = use_state(|| Boxscore::default());
    let id = props.id.clone();

    {
        let this_game = this_game.clone();
        use_effect_with(props.id.clone(), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match query::boxscore(&id).await {
                    Ok(boxscore) => {
                        this_game.set(boxscore);
                    }
                    Err(_) => {
                        this_game.set(Boxscore::default());
                    }
                }
        })});
    }

    let header: Vec<_> = this_game.meta.teams.iter().map(|team| {
        let Some(ref teams) = this_game.teams else { todo!() };
        let boxscore = if team.id == teams[0].id.to_string() {
            &teams[0]
        } else {
            &teams[1]
        };
        // Determine which team this is (home or away)
        let home_or_away: String = if (
            team.home_team.is_string() &&
            team.home_team.as_str().unwrap().to_string() == "true"
        ) {
            "home".to_string()
        } else {
            "away".to_string()
        };

        html! {
            <div class={home_or_away}>
                <h3>{team.short_name.clone()}</h3>
                <p>{team.home_team.is_string()}</p>
            </div>
        }
    }).collect();

    html! {
        <div>
            <div class="game-header">
                {header}
            </div>
        </div>
    }
}
