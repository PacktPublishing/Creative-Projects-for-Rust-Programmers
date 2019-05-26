use yew::html;
use yew::prelude::*;

struct Model {
    value: u64,
}

enum Msg {
    Increment,
    Reset,
    KeyDown(String),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Increment => {
                self.value += 1;
                true
            }
            Msg::Reset => {
                self.value = 0;
                true
            }
            Msg::KeyDown(s) => match s.as_ref() {
                "+" => {
                    self.value += 1;
                    true
                }
                "0" => {
                    self.value = 0;
                    true
                }
                _ => false,
            },
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <button onclick=|_| Msg::Increment,>{"Increment"}</button>
                <button onclick=|_| Msg::Reset,>{"Reset"}</button>
                <input
                    readonly="true",
                    value={self.value},
                    onkeydown=|e| Msg::KeyDown(e.key()),
                />
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
