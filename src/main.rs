mod app;

use app::App;

#[cfg(not(feature = "hydration"))]
fn main() {
    yew::Renderer::<App>::new().hydrate();
}

#[cfg(feature = "hydration")]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let output = yew::LocalServerRenderer::<App>::new().render().await;
    println!("{}", output);
}
