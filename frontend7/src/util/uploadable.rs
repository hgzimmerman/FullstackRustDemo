use yew::services::fetch::FetchTask;
use yew::prelude::*;
use std::mem;
use context::networking::FtWrapper;
use util::loading::LoadingType;
use std::fmt::Formatter;
use std::fmt::Debug;


pub enum Uploadable<T> {
    NotUploaded(T),
    Uploading(T, FetchTask),
    Failed(T, String)
}

impl <T> Debug for Uploadable<T> where T: Debug {
    fn fmt(&self, f: &mut Formatter) -> Result<(), ::std::fmt::Error> {
        use self::Uploadable::*;
        match self {
            NotUploaded(t) => write!(f, "Unloaded: {:?}", t),
            Uploading(t, _) => write!(f, "Loaded: {:?}", t),
            Failed(t, e) => write!(f, "Failed with error: {} for wrapped: {:?}",e, t),
        }
    }
}

impl <T> FtWrapper for Uploadable<T> where T: Default {
    fn set_ft(&mut self, ft: FetchTask) {
        *self = match *self {
            Uploadable::NotUploaded(ref mut t) => Uploadable::Uploading(mem::replace(t, T::default()), ft),
            Uploadable::Uploading(ref mut t,_) => Uploadable::Uploading(mem::replace(t, T::default()), ft),
            Uploadable::Failed(ref mut t, _) => Uploadable::Uploading(mem::replace(t, T::default()), ft)
        }
    }
}

impl <T> Uploadable<T> where T: Default {
    pub fn default_view<U, CTX>(&self, render_fn: fn(&T) -> Html<CTX, U> ) -> Html<CTX, U>
    where
        CTX: 'static,
        U: Component<CTX>
    {
        match self {
            Uploadable::NotUploaded(ref t) => html! {
                <div class=("loading-container"),>
                    {render_fn(t)}
                </div>
            },
            Uploadable::Uploading(ref t, _) => html! {
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
            Uploadable::Uploading(ref mut t,_) => Uploadable::Failed(mem::replace(t, T::default()), msg),
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
            Uploadable::Uploading(ref t, _) => t,
            Uploadable::Failed(ref t, _) => t
        }
    }
}

impl <T> AsMut<T> for Uploadable<T> {
    fn as_mut(&mut self) -> &mut T {
        match self {
            Uploadable::NotUploaded(ref mut t) => t,
            Uploadable::Uploading(ref mut t, _) => t,
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