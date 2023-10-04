use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main class="container">
            <h1>{ "Wage Calculator" }</h1>
            <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
        </main>
    }
}
