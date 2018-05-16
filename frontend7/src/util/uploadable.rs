use yew::services::fetch::FetchTask;
use yew::prelude::*;
use std::mem;
use context::networking::FtWrapper;
use util::loading::LoadingType;



pub enum Uploadable<T> {
    NotUploaded(T),
    Uploading(T, FetchTask),
    Failed(T, String)
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
            Uploadable::NotUploaded(ref t) => render_fn(t),
            Uploadable::Uploading(ref t, _) => html! {
                <div class="loading-container",>
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

    pub fn as_mut(&mut self) -> &mut T {
        match self {
            Uploadable::NotUploaded(ref mut t) => t,
            Uploadable::Uploading(ref mut t, _) => t,
            Uploadable::Failed(ref mut t, _) => t
        }
    }

    pub fn as_ref(&self) -> &T {
        match self {
            Uploadable::NotUploaded(ref t) => t,
            Uploadable::Uploading(ref t, _) => t,
            Uploadable::Failed(ref t, _) => t
        }
    }

    pub fn cloned_inner(&self) -> T where T: Clone {
        self.as_ref().clone()
    }
}

impl <T> Default for Uploadable<T>
where T: Default
{
    fn default() -> Self {
        Uploadable::NotUploaded(T::default())
    }
}