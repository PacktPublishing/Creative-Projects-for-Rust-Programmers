#![recursion_limit="128"]
#[macro_use]
extern crate yew;
use yew::prelude::*;

struct Model {
    //addend1: String,
    addend1: f64,
    //addend2: String,
    addend2: f64,
    sum: f64,
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
            //addend1: String::new(),
            addend1: 0.,
            //addend2: String::new(),
            addend2: 0.,
            sum: 0.,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ComputeSum => self.sum =
                //self.addend1.parse::<f64>().unwrap_or(0.) +
                self.addend1 +
                //self.addend2.parse::<f64>().unwrap_or(0.),
                self.addend2,
            Msg::ChangedAddend1(value) =>
                //self.addend1 = value,
                self.addend1 = value.parse::<f64>().unwrap_or(0.),
            Msg::ChangedAddend2(value) =>
                //self.addend2 = value,
                self.addend2 = value.parse::<f64>().unwrap_or(0.),
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
                            //value={&self.addend1},
                            />
                    </td>
                </tr>
                <tr>
                    <td>{"Addend 2:"}</td>
                    <td>
                        <input type="number", style=numeric,
                            oninput=|e| Msg::ChangedAddend2(e.value),
                            //value={&self.addend2},
                            />
                    </td>
                </tr>
                <tr>
                    <td></td>
                    <td align="center", >
                        <button
                            onclick=|_| Msg::ComputeSum,
                        >{"Add"}</button></td>
                </tr>
                <tr>
                    <td>{"Sum:"}</td>
                    <td>
                        <input type="number",
                            style=numeric.to_string()
                                + "background-color: yellow;",
                            readonly="true", value={self.sum},
                        />
                    </td>
                </tr>
            </table>
        }
    }
}

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
