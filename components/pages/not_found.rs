use yew::prelude::*;

#[function_component(NotFoundPage)]
pub fn not_found() -> Html {

    html! {
        <>
            <h1>{ "Oops, looks like this page does not exist yet." }</h1>
        </>
    }
}
