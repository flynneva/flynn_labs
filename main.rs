use yew::prelude::*;
use yew_router::prelude::*;

use components::pages::{
    AppRoute,
    home::HomePage,
    sports::SportsPage,
    robots::RobotsPage,
    garden::GardenPage,
    not_found::NotFoundPage,
};


fn switch(routes: AppRoute) -> Html {
  match routes {
    AppRoute::Home => html! { <HomePage /> },
    AppRoute::Sports { sport, variation, division } => html! { <SportsPage sport={sport} variation={variation} division={division} /> },
    AppRoute::Robots => html! { <RobotsPage /> },
    AppRoute::Garden => html! { <GardenPage /> },
    AppRoute::NotFound => html! { <NotFoundPage /> },
  }
}

#[function_component]
fn App() -> Html {
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