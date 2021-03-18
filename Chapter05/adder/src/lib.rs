#![recursion_limit = "512"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::events::InputData;

struct Model {
    link: ComponentLink<Self>,
    addend1: String,
    addend2: String,
    sum: Option<f64>,
}

enum Msg {
    ChangedAddend1(String),
    ChangedAddend2(String),
    ComputeSum,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            addend1: "".to_string(),
            addend2: "".to_string(),
            sum: None,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ComputeSum => {
                self.sum = match (self.addend1.parse::<f64>(), self.addend2.parse::<f64>()) {
                    (Ok(a1), Ok(a2)) => Some(a1 + a2),
                    _ => None,
                };
            }
            Msg::ChangedAddend1(value) => {
                self.addend1 = value;
                self.sum = None;
            }
            Msg::ChangedAddend2(value) => {
                self.addend2 = value;
                self.sum = None;
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        let numeric = "text-align: right;";
        html! {
            <table>
                <tr>
                    <td>{"Addend 1:"}</td>
                    <td>
                        <input type="number", style=numeric,
                            oninput=self.link.callback(|e: InputData| Msg::ChangedAddend1(e.value)),
                            />
                    </td>
                </tr>
                <tr>
                    <td>{"Addend 2:"}</td>
                    <td>
                        <input type="number", style=numeric,
                            oninput=self.link.callback(|e: InputData| Msg::ChangedAddend2(e.value)),
                            />
                    </td>
                </tr>
                <tr>
                    <td></td>
                    <td align="center", >
                        <button
                            disabled=self.sum.is_some(),
                            onclick=self.link.callback(|_| Msg::ComputeSum),
                        >{"Add"}</button></td>
                </tr>
                <tr>
                    <td>{"Sum:"}</td>
                    <td>
                        <input type="number",
                            style=numeric.to_string()
                                + "background-color: "
                                + if self.sum.is_some() { "lightgreen;" } else { "yellow;" },
                            readonly=true, value={
                                match self.sum { Some(n) => n.to_string(), None => "".to_string() }
                            },
                        />
                    </td>
                </tr>
            </table>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
