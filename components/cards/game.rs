
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct GameProps {
    pub id: String, 
    pub home_name: String,
    pub away_name: String,
    pub current_clock: String, 
    pub current_period: String, 
    pub home_score: String,
    pub home_record: String,
    pub home_rank: String,
    pub away_score: String,
    pub away_record: String,
    pub away_rank: String,
    pub start_time: String, 
}

#[function_component]
pub fn GameCard(props: &GameProps) -> Html {

    html! {
        <a class="link" href={format!("sports{}", props.id.clone())}>
            <div class="scoreboard-card" id={props.id.clone()}>
    
                <div class="scoreboard-team-container home">
                    <h5>{props.home_name.clone()}</h5>
                    <p>{props.home_record.clone()}</p>
                    <h4>{props.home_score.clone()}</h4>
                </div>
                <div class="scoreboard-team-container away">
                    <h5>{props.away_name.clone()}</h5>
                    <p>{props.away_record.clone()}</p>
                    <h4>{props.away_score.clone()}</h4>
                </div>
                <p>{props.start_time.clone()}</p>
            </div>
        </a>
    }
}
