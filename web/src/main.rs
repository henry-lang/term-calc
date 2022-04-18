use calculate::*;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    (0..10000).map(|i| html!(<p>{i}</p>)).collect::<Html>()
}

fn main() {
    yew::start_app::<App>();
}
