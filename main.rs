use yew::prelude::*;
use yew_router::prelude::*;

use components::pages::{
    AppRoute,
    home::HomePage,
    sports::SportsPage,
    sport_page::SportPage,
    robots::RobotsPage,
    garden::GardenPage,
    not_found::NotFoundPage,
};


fn switch(routes: AppRoute) -> Html {
  match routes {
    AppRoute::Home => html! { <HomePage /> },
    AppRoute::Sports => html! { <SportsPage /> },
    AppRoute::SportPage { sport } => html! { <SportPage sport={sport} />},
    AppRoute::Robots => html! { <RobotsPage /> },
    AppRoute::Garden => html! { <GardenPage /> },
    AppRoute::NotFound => html! { <NotFoundPage /> },
  }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
          <BrowserRouter>
            <Switch <AppRoute> render={switch} />
          </BrowserRouter>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}