use yew::services::fetch::FetchTask;
use yew::prelude::*;



pub enum Uploadable<T> {
    NotUploaded(T),
    Uploading(T, FetchTask),
//    Failed(T) // TODO, this may not be necessary, as the responsibility for showing errors may best be shown in the NotUploaded section
}

impl <T> Uploadable<T> {
    pub fn default_view<U, CTX>(&self, render_fn: fn(&T) -> Html<CTX, U> ) -> Html<CTX, U>
    where
        CTX: 'static,
        U: Component<CTX>
    {
        match self {
            Uploadable::NotUploaded(ref t) => render_fn(t),
            Uploadable::Uploading(ref t, _) => html! {
                <div>
                    {"sending..."}
                    {render_fn(t)}
                </div>
            }
        }
    }

    pub fn get_inner(&self) -> &T {
        match self {
            Uploadable::NotUploaded(ref t) => t,
            Uploadable::Uploading(ref t, _) => t,
        }
    }
    #[allow(dead_code)]
    pub fn get_cloned_inner(&self) -> T where T: Clone {
        self.get_inner().clone()
    }
}

impl <T> Default for Uploadable<T>
where T: Default
{
    fn default() -> Self {
        Uploadable::NotUploaded(T::default())
    }
}