use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::events::KeyboardEvent;

struct Model {
    link: ComponentLink<Self>,
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
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { value: 0, link }
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

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::Increment)>{"Increment"}</button>
                <button onclick=self.link.callback(|_| Msg::Reset)>{"Reset"}</button>
                <input
                    readonly=true,
                    value={self.value},
                    onkeydown=self.link.callback(|e: KeyboardEvent| Msg::KeyDown(e.key())),
                />
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
