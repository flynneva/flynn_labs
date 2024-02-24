use yew::prelude::*;

#[function_component]
pub fn NotFoundPage() -> Html {
    html! {
        <>
            <h1>{ "Oops, looks like this page does not exist yet." }</h1>
        </>
    }
}
