//use yew::services::fetch::FetchTask;
use yew::prelude::*;
use std::mem;
//use context::networking::FtWrapper;
use loading::LoadingType;
use std::fmt::Formatter;
use std::fmt::Debug;
use common::fetch::FetchResponse;


pub enum Uploadable<T> {
    NotUploaded(T), // TODO, maybe change to NotSending
    Uploading(T),
    Failed(T, String)
}

impl <T> Debug for Uploadable<T> where T: Debug {
    fn fmt(&self, f: &mut Formatter) -> Result<(), ::std::fmt::Error> {
        use self::Uploadable::*;
        match self {
            NotUploaded(t) => write!(f, "Unloaded: {:?}", t),
            Uploading(t) => write!(f, "Loaded: {:?}", t),
            Failed(t, e) => write!(f, "Failed with error: {} for wrapped: {:?}",e, t),
        }
    }
}
//
//impl <T> FtWrapper for Uploadable<T> where T: Default {
//    fn set_ft(&mut self, ft: FetchTask) {
//        *self = match *self {
//            Uploadable::NotUploaded(ref mut t) => Uploadable::Uploading(mem::replace(t, T::default()), ft),
//            Uploadable::Uploading(ref mut t,_) => Uploadable::Uploading(mem::replace(t, T::default()), ft),
//            Uploadable::Failed(ref mut t, _) => Uploadable::Uploading(mem::replace(t, T::default()), ft)
//        }
//    }
//}

impl <T> Uploadable<T> where T: Default {
    /// Creates a loadable from a fetch response
    pub fn handle_fetch_response(&mut self, fetch_response: FetchResponse<T>) {
        use self::FetchResponse::*;
        *self = match fetch_response {
            Success(t) => Uploadable::NotUploaded(t),
            Error(_) =>{
                let existing_wrapped_value: &mut T = self.as_mut();
                Uploadable::Failed(mem::replace(existing_wrapped_value, T::default()),"Failed".into())
            }
            Started => {
                let existing_wrapped_value: &mut T = self.as_mut();
                Uploadable::Uploading(mem::replace(existing_wrapped_value, T::default()))
            }
        }
    }

    pub fn default_view<U>(&self, render_fn: fn(&T) -> Html<U> ) -> Html<U>
    where
        U: Component
    {
        match self {
            Uploadable::NotUploaded(ref t) => html! {
                <div class=("loading-container"),>
                    {render_fn(t)}
                </div>
            },
            Uploadable::Uploading(ref t) => html! {
                <div class=("loading-container", "full-height","full-width"),>
                    <div class="loading-overlay",>
                        {LoadingType::Fidget{diameter: 100}.view()}
                    </div>
                    {render_fn(t)}
                </div>
            },
            Uploadable::Failed(ref t, err_msg) => html! {
                <div>
                    {err_msg}
                    {render_fn(t)}
                </div>
            }
        }
    }

    pub fn set_failed(&mut self, msg: &str) {
        let msg = msg.to_string();
        *self = match *self {
            Uploadable::NotUploaded(ref mut t) => Uploadable::Failed(mem::replace(t, T::default()), msg),
            Uploadable::Uploading(ref mut t) => Uploadable::Failed(mem::replace(t, T::default()), msg),
            Uploadable::Failed(ref mut t, _) => Uploadable::Failed(mem::replace(t, T::default()), msg)
        }
    }

    pub fn cloned_inner(&self) -> T where T: Clone {
        self.as_ref().clone()
    }
}

impl <T> AsRef<T> for Uploadable<T> {
    fn as_ref(&self) -> &T {
        match self {
            Uploadable::NotUploaded(ref t) => t,
            Uploadable::Uploading(ref t) => t,
            Uploadable::Failed(ref t, _) => t
        }
    }
}

impl <T> AsMut<T> for Uploadable<T> {
    fn as_mut(&mut self) -> &mut T {
        match self {
            Uploadable::NotUploaded(ref mut t) => t,
            Uploadable::Uploading(ref mut t) => t,
            Uploadable::Failed(ref mut t, _) => t
        }
    }
}

impl <T> Default for Uploadable<T>
where T: Default
{
    fn default() -> Self {
        Uploadable::NotUploaded(T::default())
    }
}