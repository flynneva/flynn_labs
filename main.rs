use yew::prelude::*;
use yew_router::prelude::*;

use components::routes::MainRoute;
use components::switches::switch_main;

#[function_component]
fn App() -> Html {
    html! {
        <>
          <HashRouter>
            <Switch <MainRoute> render={switch_main} />
          </HashRouter>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}