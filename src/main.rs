use yew::prelude::*;
use yew_router::prelude::*;

use flynn_labs::components::pages::{
    Route,
    home::HomePage,
    sports::SportsPage,
    robots::RobotsPage,
    garden::GardenPage,
    not_found::NotFoundPage,
};

fn switch(routes: &Route) -> Html {
  match routes {
    Route::Home => html! { <HomePage /> },
    Route::Sports => html! { <SportsPage /> },
    Route::Robots => html! { <RobotsPage /> },
    Route::Garden => html! { <GardenPage /> },
    Route::NotFound => html! { <NotFoundPage /> },
  }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
          <BrowserRouter>
            <Switch <Route> render={Switch::render(switch)} />
          </BrowserRouter>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}