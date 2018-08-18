
use yew::prelude::*;

use wrappers;

pub type InputValidator = fn(String) -> Result<String, String>;
/// Parent components will own input states and pass them to the Input Components.
#[derive(Debug, Clone, PartialEq)]
pub enum InputState{
    /// A string that has not yet been validated, but isn't necessarily incorrect.
    /// Selecting the input and then unselecting it should make this convert to a Validated variant.
    Unvalidated{text: String},
    /// Just a normal string
    Validated{text: String},
    /// Typing anything should convert the error into a Normal again.
    Error{text: String, error_text: String},
}

impl Default for InputState {
    fn default() -> Self {
        InputState::Unvalidated {text: String::default()}
    }
}

impl InputState {
    pub fn validate(text: String, validation_fn: fn(String) -> Result<String, String>) -> InputState {
        match validation_fn(text.clone()) {
            Ok(validated_text) => InputState::Validated {text: validated_text},
            Err(error_text) => InputState::Error {text, error_text}
        }
    }

    pub fn set_text(&mut self, text: String) {
        use self::InputState::*;
        *self = match *self {
            Unvalidated{..} => Unvalidated{text},
            Validated{..} => Validated {text},
            Error{ref error_text,..} => Error{text, error_text: error_text.clone()},
        }
    }

    pub fn inner_text(&self) -> String {
        use self::InputState::*;
        match self {
            Unvalidated {text} => text.clone(),
            Validated {text} => text.clone(),
            Error{text, ..} => text.clone()
        }
    }
    fn error_text(&self) -> Option<&String> {
        use self::InputState::*;
        match self {
            Unvalidated {..} => None,
            Validated {..} => None,
            Error{error_text, ..} => Some(error_text)
        }
    }
}

pub struct Input {
    input_state: InputState,
    required: bool,
    placeholder: String,
    label: String,
    disabled: bool,
    is_password: bool,
    /// When an enter key is encountered in the input box, this callback will fire
    on_enter: Option<Callback<()>>,
    /// Whenever a the user types a word, or the element loses focus, this callback will fire.
    on_change: Callback<InputState>,
    /// Whenever the element loses focus, this function will be used to determine if the string is valid.
    /// This validator has the ability to change the input string, but under most circumstances, it should not.
    /// That could be used for normalizing date values ond decimals though.
    validator: InputValidator
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub input_state: InputState,
    pub placeholder: String,
    pub label: String,
    pub required: bool,
    pub disabled: bool,
    pub is_password: bool,
    /// When an enter key is encountered in the input box, this callback will fire.
    pub on_enter: Option<Callback<()>>,
    /// Whenever a the user types a word, or the element loses focus, this callback will fire.
    pub on_change: Option<Callback<InputState>>,
    /// Whenever the element loses focus, this function will be used to determine if the string is valid.
    /// The `Box<>` appears to be necessary to overcome the coercion of FN pointers to `Closures` by the `html!` macro
    pub validator: Box<InputValidator>
}

impl Default for Props {
    fn default() -> Props {
        Props {
            input_state: InputState::Unvalidated {text: String::default()},
            placeholder: String::default(),
            label: String::default(),
            required: false,
            disabled: false,
            is_password: false,
            on_enter: None,
            on_change: None,
            validator: Box::new(Ok) // by default, the validator will approve any String
        }
    }
}



pub enum Msg {
    UpdateText(String),
    EnterPressed,
    Validate,
    NoOp
}

impl Component for Input {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Input {
            input_state: props.input_state,
            placeholder: props.placeholder,
            label: props.label,
            required: props.required,
            is_password: props.is_password,
            disabled: props.disabled,
            on_enter: props.on_enter,
            on_change: props.on_change.expect("Developer error, on_change callback MUST be supplied when creating a Input component."),
            validator: *props.validator,
        }
    }

    fn update(&mut self, msg: Msg) -> ShouldRender {
        use self::Msg::*;

        match msg {
            UpdateText(new_text) => {
                // If the input is an error, try to get out of the error ASAP, otherwise, don't validate until they click away.
                if let InputState::Error{..} = self.input_state {
                    self.on_change.emit(InputState::validate(new_text, self.validator))
                } else {
                    let mut is = self.input_state.clone();
                    is.set_text(new_text);
                    // Generate a new input state, pass it up to the parent, and expect it to be propagated back to this component
                    self.on_change.emit(is);
                }

                false
            },
            EnterPressed => {
                if let Some(ref cb) = self.on_enter {
                    cb.emit(())
                }
                false
            }
            Validate => {
                self.on_change.emit(InputState::validate(self.input_state.inner_text(), self.validator));
                false
            }
            NoOp => {false}
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {

        self.input_state = props.input_state;
        self.required = props.required;
        self.placeholder = props.placeholder;
        self.disabled = props.disabled;
        self.on_enter = props.on_enter;
        self.on_change = props.on_change.expect("Developer error, on_change callback MUST be supplied when changing a Input component.");
        self.validator = *props.validator;

        true
    }
}


impl Renderable<Input> for Input
{
    fn view(&self) -> Html<Input> {

//        fn required_star(is_required: bool) -> Html<Context, Input> {
//            if is_required {
//                html!{
//
//                    <>
//                        {"*"}
//                    </>
//                }
//            } else {
//                wrappers::empty_vdom_node()
//            }
//        }

        fn error_view(error: Option<&String>) ->  Html<Input> {
            match error {
                Some(e) => html!{
                    <div>
                        {e}
                    </div>
                },
                None => wrappers::empty_vdom_node()
            }
        }

        let error_visibility: &str = if self.input_state.error_text().is_some() {
           "visibility: visible"
        } else {
           "visibility: hidden"
        };

        let input_type = if self.is_password {
            "password"
        } else {
            "text"
        };

        html!{
            <div class=("flexbox-vert", "input-and-error-container"), >
                <div class="form-error-message", style=error_visibility, >
                    {error_view(self.input_state.error_text())}
                </div>
                <div class=("input-group"),>
//                    <div class="form-control-start", >
//                        {required_star(self.required)}
//                    </div>
                    <input
                        class="form-control",
                        disabled=self.disabled,
                        placeholder=&self.placeholder,
                        value=self.input_state.inner_text(),
                        onblur=|_| Msg::Validate,
                        oninput=|e| Msg::UpdateText(e.value),
                        onkeypress=|e| {
                            if e.key() == "Enter" { Msg::EnterPressed } else {Msg::NoOp}
                        },
                        type=input_type,
                    />
                </div>
            </div>
        }
    }
}
