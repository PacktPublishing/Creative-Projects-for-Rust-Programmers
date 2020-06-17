#![recursion_limit = "128"]
#[macro_use]
extern crate yew;
use yew::prelude::*;

struct Model {
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
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            addend1: "".to_string(),
            addend2: "".to_string(),
            sum: None,
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
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let numeric = "text-align: right;";
        html! {
            <table>
                <tr>
                    <td>{"Addend 1:"}</td>
                    <td>
                        <input type="number", style=numeric,
                            oninput=|e| Msg::ChangedAddend1(e.value),
                            />
                    </td>
                </tr>
                <tr>
                    <td>{"Addend 2:"}</td>
                    <td>
                        <input type="number", style=numeric,
                            oninput=|e| Msg::ChangedAddend2(e.value),
                            />
                    </td>
                </tr>
                <tr>
                    <td></td>
                    <td align="center", >
                        <button
                            disabled=self.sum.is_some(),
                            onclick=|_| Msg::ComputeSum,
                        >{"Add"}</button></td>
                </tr>
                <tr>
                    <td>{"Sum:"}</td>
                    <td>
                        <input type="number",
                            style=numeric.to_string()
                                + "background-color: "
                                + if self.sum.is_some() { "lightgreen;" } else { "yellow;" },
                            readonly="true", value={
                                match self.sum { Some(n) => n.to_string(), None => "".to_string() }
                            },
                        />
                    </td>
                </tr>
            </table>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
