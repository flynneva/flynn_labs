use yew::prelude::*;

use ncaa_data_rs::ncaa::game::QueryBoxscore;
use ncaa_data_rs::ncaa::football::{
    FootballGame,
    boxscore::Boxscore,
};

use std::ops::Deref;

#[derive(Properties, PartialEq)]
pub struct Props{
    pub game: FootballGame,  // always a FootballGame type
}

#[function_component]
pub fn FootballGamePage(props: &Props) -> Html {
    let game = use_state(|| props.game.clone());
    let boxscore = use_state(|| Boxscore::default());

    let boxscore_clone = boxscore.clone();
    use_effect_with(game.clone(), move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            let boxscore = boxscore_clone.clone();
            boxscore.set(game.deref().boxscore().await.expect("No boxscore available"))
        })
    });

    let mut home_tables: Vec<_> = Vec::new();
    let mut away_tables: Vec<_> = Vec::new();
    
    for table in boxscore.deref().tables.as_ref().unwrap().iter() {
        let headers: Vec<_> = table.header.iter().map(|col| {
           html! {<th class={col.class.clone()}>{col.display.clone()}</th>}
        }).collect();

        let table_type = table.header[0].display.clone();

        let data: Vec<_> = table.data.iter().map(|row| {
            let row_data: Vec<_> = row.row.iter().map(|data| {
                html! {<td class={data.class.clone()}>{data.display.clone()}</td>}
            }).collect();

            if table_type == "PUNTING".to_string() {
                if row.row[2].display == "0".to_string() {
                    return html! {<div class="empty-data-row" />}
                }
            }

            let mut row_class = "table-row-data";
            if row.row[0].display == "Total".to_string() {
                row_class = "table-row-data-total";
            }
            html! {
                <tr class={row_class}>
                  {row_data}
                </tr>
            }
        }).collect();

        let new_table = html! {
            <div class={"football-table-div"} >
              <table id={table.id.clone()} class={"football-table"}>
                <thead class={table.header_class.clone()} style={"background-color:".to_owned() + &table.header_color.clone()}>
                  <tr>
                    {headers}
                  </tr>
                </thead>
                <tbody>
                  {data}
                </tbody>
              </table>
            </div>
        };

        if table.id.ends_with("home") {
            home_tables.push(new_table);
        } else {
            away_tables.push(new_table);
        }
    }


    let mut home_team_name_header = html! {<div></div>};
    let mut away_team_name_header = html! {<div></div>};
    if boxscore.meta.teams.len() > 0 {
        if boxscore.meta.teams[0].home_team.as_str() == Some("true") {
            home_team_name_header = html! {
                <h4>{boxscore.meta.teams[0].short_name.clone()}</h4>
            };
            away_team_name_header = html! {
                <h4>{boxscore.meta.teams[1].short_name.clone()}</h4>
            };
        } else {
            home_team_name_header = html! {
                <h4>{boxscore.meta.teams[1].short_name.clone()}</h4>
            };
            away_team_name_header = html! {
                <h4>{boxscore.meta.teams[0].short_name.clone()}</h4>
            };
        }
    }
    html! {
        <div class={"football-game-page"}>
            <div class={"football-game-home-tables"}>
                {home_team_name_header}
                {home_tables}
            </div>
            <div class={"football-game-away-tables"}>
                {away_team_name_header}
                {away_tables}
            </div>
        </div>
    }
}
