use calculate::{calculate, Config, Context as CalculatorContext, Mode};
use web_sys::{HtmlInputElement, InputEvent};
use yew::{html, Component, Context, Html, TargetCast};

enum CalculatorMessage {
    TextUpdate(String),
}
use CalculatorMessage::*;

struct Calculator {
    value: Result<f64, String>,
    context: CalculatorContext,
    config: Config,
}

impl Component for Calculator {
    type Message = CalculatorMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let config = Config {
            mode: Mode::Degrees,
            debug: false,
        };

        Self {
            value: Ok(0f64),
            config,
            context: CalculatorContext::create(&config),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TextUpdate(expression) => {
                self.value = calculate(&expression, &self.context, &self.config);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let oninput = link.callback(|e: InputEvent| {
            TextUpdate(e.target_unchecked_into::<HtmlInputElement>().value())
        });

        match &self.value {
            Ok(num) => {
                html! {
                    <div>
                        <input {oninput} />
                        <p>{num}</p>
                    </div>
                }
            }

            Err(error) => {
                html! {
                    <div>
                        <input {oninput} />
                        <p>{error}</p>
                    </div>
                }
            }
        }
    }
}

fn main() {
    yew::Renderer::<Calculator>::new().render();
}
