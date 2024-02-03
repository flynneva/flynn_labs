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

use components::layout::{
  menu::MainMenu,
};


fn switch(routes: &AppRoute) -> Html {
  match routes {
    AppRoute::Home => html! { <HomePage /> },
    AppRoute::Sports => html! { <SportsPage /> },
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
            <MainMenu />
            <Switch <AppRoute> render={Switch::render(switch)} />
          </BrowserRouter>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}