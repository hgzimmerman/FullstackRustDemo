use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub struct Props {
    pub title: String,
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub callback: Option<Callback<f64>>
}

impl Default for Props {
    fn default() -> Self {
        Props {
            title: "Range Picker".to_owned(),
            value: f64::default(),
            min: 0.0,
            max: 100.0,
            step: 0.1,
            callback: None
        }
    }
}

pub enum Msg {
    HandleChange(ChangeData)
}


pub struct RangePicker {
    title: String,
    value: f64,
    min: f64,
    max: f64,
    step: f64,
    callback: Option<Callback<f64>>
}

impl Component for RangePicker {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> RangePicker {
        RangePicker {
            title: props.title,
            value: props.value,
            min: props.min,
            max: props.max,
            step: props.step,
            callback: props.callback
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::HandleChange(cd) => {
                if let Some(ref cb) = self.callback {
                    let new_value = change_data_to_f64(cd);
                    cb.emit(new_value)
                }
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.title = props.title;
        self.value = props.value;
        self.min = props.min;
        self.max = props.max;
        self.step = props.step;
        self.callback = props.callback;

        true
    }
}

impl Renderable<RangePicker> for RangePicker {
    fn view(&self) ->Html<Self> {
        html!{
            <div>
                <label> 
                    {format!("{}: {}", self.title, self.value)}
                </label>
                <input 
                    type="range",
                    min=&self.min,
                    max=&self.max,
                    step=&self.step,
                    value=&self.value,
                    onchange=|e| {
                        Msg::HandleChange(e)
                    }
                ,>
                </input>
            </div>
        }
    }
}

use yew::html::ChangeData;
fn change_data_to_f64(change_data: ChangeData) -> f64 {
    match change_data {
        ChangeData::Value(string) => string.parse::<f64>().expect("js range picker gave us the wrong float format"),
        _ => panic!()
    }
}