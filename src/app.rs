use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main class="container">
            <h1>{ "Wage Calculator" }</h1>
            <div class="grid">
                <article>
                    <header>
                        { "Hourly" }
                    </header>
                    <label for="hourly">
                        { "Hourly Rate" }
                        <input type="text" />
                    </label>
                </article>
                <article>
                    <header>
                        { "Bi-Weekly" }
                    </header>
                </article>
            </div>
            <div class="grid">
                <article>
                    <header>
                        { "Semi-Monthly" }
                    </header>
                </article>
                <article>
                    <header>
                        { "Monthly" }
                    </header>
                </article>
            </div>
        </main>
    }
}
