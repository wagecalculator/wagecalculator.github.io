mod calc;

use calc::*;
use yew::prelude::*;
use web_sys::{EventTarget, HtmlInputElement};
use wasm_bindgen::JsCast;



#[derive(PartialEq, Debug, Clone)]
struct State {
    annual: f64,
    hourly: f64,
    hours: f64,
}

impl State {
    fn propogate_annual(&mut self, new_annual: f64) {
        self.annual = new_annual;
        let new_weekly = annual_to_weekly(new_annual);
        self.hourly = weekly_to_hourly(self.hours, new_weekly);
    }

    fn propogate_hourly(&mut self, new_hourly: f64) {
        self.hourly = new_hourly;
        let new_weekly = hourly_to_weekly(self.hours, new_hourly);
        self.annual = weekly_to_annual(new_weekly);
    }

    fn propogate_hours(&mut self, new_hours: f64) {
        self.hours = new_hours;
        let new_weekly = hourly_to_weekly(new_hours, self.hourly);
        self.annual = weekly_to_annual(new_weekly);
    }
}

impl Default for State {
    fn default() -> Self {
        const INIT_HOURLY: f64 = 15.0;
        const INIT_HOURS: f64 = 40.0;
        let weekly = hourly_to_weekly(INIT_HOURS, INIT_HOURLY);
        let annual = weekly_to_annual(weekly);
        State {
            annual,
            hourly: INIT_HOURLY,
            hours: INIT_HOURS,
        }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let state = use_state(|| State::default());
    let state_value = (*state).clone();
    let on_change_annual = {
        let state = state.clone();
        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(x) = input.and_then(|i| i.value().parse::<f64>().ok()) {
                state.set({
                    let mut s = (*state).clone();
                    s.propogate_annual(x);
                    s
                })
            }
        })
    };
    let on_change_hourly = {
        let state = state.clone();
        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(x) = input.and_then(|i| i.value().parse::<f64>().ok()) {
                state.set({
                    let mut s = (*state).clone();
                    s.propogate_hourly(x);
                    s
                })
            }
        })
    };
    let on_change_hours = {
        let state = state.clone();
        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(x) = input.and_then(|i| i.value().parse::<f64>().ok()) {
                state.set({
                    let mut s = (*state).clone();
                    s.propogate_hours(x);
                    s
                })
            }
        })
    };
    let on_change_biweekly = {
        let state = state.clone();
        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(x) = input.and_then(|i| i.value().parse::<f64>().ok()) {
                state.set({
                    let mut s = (*state).clone();
                    s.propogate_annual((x / 2.0) * WEEKS_PER_YEAR);
                    s
                })
            }
        })
    };
    let on_change_monthly = {
        let state = state.clone();
        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(x) = input.and_then(|i| i.value().parse::<f64>().ok()) {
                state.set({
                    let mut s = (*state).clone();
                    s.propogate_annual(x * 12.0);
                    s
                })
            }
        })
    };
    let on_change_semimonthly = {
        let state = state.clone();
        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(x) = input.and_then(|i| i.value().parse::<f64>().ok()) {
                state.set({
                    let mut s = (*state).clone();
                    s.propogate_annual(x * 24.0);
                    s
                })
            }
        })
    };
    html! {
        <>
            <main class="container">
                <h1>{ "Wage Calculator" }</h1>
                <div class="grid">
                    <article>
                        <label for="annual-salary">
                            { "Annual Salary" }
                            <input
                                type="number"
                                id="annual-salary"
                                value={ format!("{:.2}", state_value.annual) }
                                onchange={ on_change_annual }
                            />
                        </label>
                    </article>
                    <article>
                        <label for="hourly-rate">
                            { "Hourly Rate" }
                            <input
                                type="number"
                                id="hourly-rate"
                                value={ format!("{:.2}", state_value.hourly) }
                                onchange={ on_change_hourly }
                            />
                        </label>
                        <label for="hours-per-week">
                            { "Hours per Week" }
                            <input
                                type="number"
                                id="hours-per-week"
                                value={ state_value.hours.to_string() }
                                onchange={ on_change_hours }
                            />
                        </label>
                    </article>
                </div>
                <div class="grid">
                    <article>
                        <label for="biweekly-rate">
                            { "Bi-Weekly" }
                            <input
                                type="number"
                                id="biweekly-rate"
                                value={ format!("{:.2}", state_value.annual / (WEEKS_PER_YEAR / 2.0)) }
                                onchange={ on_change_biweekly }
                            />
                        </label>
                    </article>
                    <article>
                        <label for="semimonthly-rate">
                            { "Semi-Monthly" }
                            <input
                                type="number"
                                id="semimonthly-rate"
                                value={ format!("{:.2}", state_value.annual / 24.0) }
                                onchange={ on_change_semimonthly }
                            />
                        </label>
                    </article>
                    <article>
                        <label for="monthly-rate">
                            { "Monthly" }
                            <input
                                type="number"
                                id="monthly-rate"
                                value={ format!("{:.2}", state_value.annual / 12.0) }
                                onchange={ on_change_monthly }
                            />
                        </label>
                    </article>
                </div>
                <h2 id="purpose">{ "Purpose" }</h2>
                <p>{"This website is intended to help people recognize the different rates at which their wage equates to, without being overly complicated. Just adjust the values and see the other ones update!"}</p>
                <hr />
                <div style="text-align: center">
                    <p><strong>{" Disclaimer: "}</strong>{ DISCLAIMER }</p>
                    <p>
                        { "Built with " }
                        <a href="https://yew.rs/" target="__blank">{"Yew"}</a>
                        { " and " }
                        <a href="https://picocss.com/" target="__blank">{"Pico"}</a>
                        { ". Released under the " }
                        <a href="https://github.com/wagecalculator/wagecalculator.github.io/blob/master/LICENSE-MIT" target="__blank">{"MIT License"}</a>
                        { ". Source code available on " }
                        <a href="https://github.com/wagecalculator/wagecalculator.github.io" target="__blank">{"GitHub"}</a>
                        { "." }
                    </p>
                    <p>
                        { "Copyright © 2023 Athan Clark." }
                    </p>
                </div>
            </main>
            <a
                href="https://github.com/wagecalculator/wagecalculator.github.io"
                target="__blank"
                id="github-corner"
                aria-label="View source on GitHub"
                >
                <svg
                    width="80"
                    height="80"
                    viewBox="0 0 250 250"
                    aria-hidden="true"
                    >
                    <path
                        d="M0,0 L115,115 L130,115 L142,142 L250,250 L250,0 Z">
                    </path>
                    <path
                        d="M128.3,109.0 C113.8,99.7 119.0,89.6 119.0,89.6 C122.0,82.7 120.5,78.6 120.5,78.6 C119.2,72.0 123.4,76.3 123.4,76.3 C127.3,80.9 125.5,87.3 125.5,87.3 C122.9,97.6 130.6,101.9 134.4,103.2"
                        fill="currentColor"
                        style="transform-origin: 130px 106px;"
                        class="octo-arm">
                    </path>
                    <path
                        d="M115.0,115.0 C114.9,115.1 118.7,116.5 119.8,115.4 L133.7,101.6 C136.9,99.2 139.9,98.4 142.2,98.6 C133.8,88.0 127.5,74.4 143.8,58.0 C148.5,53.4 154.0,51.2 159.7,51.0 C160.3,49.4 163.2,43.6 171.4,40.1 C171.4,40.1 176.1,42.5 178.8,56.2 C183.1,58.6 187.2,61.8 190.9,65.4 C194.5,69.0 197.7,73.2 200.1,77.6 C213.8,80.2 216.3,84.9 216.3,84.9 C212.7,93.1 206.9,96.0 205.4,96.6 C205.1,102.4 203.0,107.8 198.3,112.5 C181.9,128.9 168.3,122.5 157.7,114.1 C157.9,116.9 156.7,120.9 152.7,124.9 L141.0,136.5 C139.8,137.7 141.6,141.9 141.8,141.8 Z"
                        fill="currentColor"
                        class="octo-body">
                    </path>
                </svg>
            </a>
        </>
    }
}


const DISCLAIMER: &'static str = "
Nothing in this Site constitutes professional and/or financial advise, nor does any information on the Site constitute a comprehensive or complete statement of the matters discussed or the law relating thereto.
";
