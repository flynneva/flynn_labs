use yew::prelude::*;
use yew_router::prelude::*;
use std::rc::Rc;
use chrono::{Datelike, DateTime, FixedOffset, Local, TimeZone};

use ncaa_data_rs::ncaa::{
    sports::supported_sports,
    sports::Sport,
};

use crate::routes::SportsRoute;
use crate::switches::switch_sports;


#[derive(Clone, PartialEq)]
pub struct ActiveSport {
    pub supported_sports: Vec<Sport>,
    pub name: String,
    pub variation: Option<String>,
    pub division: String,
    pub date: DateTime<FixedOffset>,
}

pub enum ActiveSportAction {
    SetSport(String),
}

impl Default for ActiveSport {
    fn default() -> Self {
        let current_date = Local::now();
        Self {
            supported_sports: supported_sports(),
            name: "basketball".to_string(),
            variation: Some("men".to_string()),
            division: "d1".to_string(),
            date: FixedOffset::east_opt(0).unwrap().with_ymd_and_hms(
                current_date.year(),
                current_date.month(),
                current_date.day(),
                0, 0, 0).unwrap(),
        }
    }
}

fn validate_sport(potential_sport: &ActiveSport) -> bool {
    // let navigator = navigator.clone();
    for sport in potential_sport.supported_sports.iter() {
        if sport.name.to_string() == potential_sport.name &&
           !sport.divisions.contains(&potential_sport.division.to_string())
        {
            if sport.variations.is_none() && potential_sport.variation.is_none() ||
               sport.variations.is_some() && potential_sport.variation.is_some()
            {
                return true
            }
        }
    }
    false
}

impl Reducible for ActiveSport {
    type Action = ActiveSportAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let updated_active_sport = match action {
            ActiveSportAction::SetSport(name) => ActiveSport {
                supported_sports: self.supported_sports.clone(),
                name: name,
                variation: self.variation.clone(),
                division: self.division.clone(),
                date: self.date.clone(),
            },
        };
        if !validate_sport(&updated_active_sport) {
            println!("could not update the active sport due to an invalid configuration");
            self
        } else {
            updated_active_sport.into()
        }
    }
}
#[function_component]
pub fn SportsPages() -> Html {
    let active_sport = use_reducer(ActiveSport::default);

    html! {
        <ContextProvider<ActiveSport> context={(*active_sport).clone()}>
            <Switch<SportsRoute> render={switch_sports} />
        </ContextProvider<ActiveSport>>
    }
}
