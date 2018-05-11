use yew::services::fetch::FetchTask;
use yew::prelude::*;

pub enum Loadable<T> {
    Unloaded,
    Loading(FetchTask),
    Loaded(T),
    Failed(Option<String>)
}

impl <T> Default for Loadable<T> {
    fn default() -> Self {
        Loadable::Unloaded
    }
}

impl <T> Clone for Loadable<T>
    where T: Clone
{
    fn clone(&self) -> Self {
        match self {
            Loadable::Unloaded => Loadable::Unloaded,
            Loadable::Loading(_) => Loadable::Unloaded, // Any loading loadable throws away the FT because it can't be cloned
            Loadable::Loaded(t) => Loadable::Loaded(t.clone()),
            Loadable::Failed(o) => Loadable::Failed(o.clone())
        }
    }
}

impl <T> Loadable<T> {
    pub fn loaded<'a>(&'a self) -> Option<&'a T> {
        if let Loadable::Loaded(ref t) = self {
            Some(t)
        } else {
            None
        }
    }

    pub fn default_view<U, CTX>(&self, render_fn: fn(&T) -> Html<CTX, U> ) -> Html<CTX, U>
        where
            CTX: 'static,
            U: Component<CTX>

    {
        match self {
            Loadable::Unloaded => html! {
                <>
                </>
            },
            Loadable::Loading(_) => html! {
                <>
                    {"Loading..."}
                </>
            },
            Loadable::Loaded(ref t) => render_fn(t),
            Loadable::Failed(error) => {
                if let Some(message) = error {
                    html! {
                        <>
                            {message}
                        </>
                    }
                }
                else {
                    html! {
                        <>
                            {"Request Failed"}
                        </>
                    }
                }
            }
        }
    }
}

pub enum Uploadable<T> {
    NotUploaded(T),
    Uploading(T, FetchTask),
//    Failed(E) // TODO, provide a way to swap in a component that displays errors.
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

}

#[derive(Clone, Debug)]
pub enum Either<L,R> {
    Left(L),
    Right(R)
}
impl <L,R> Default for Either<L,R>
    where
        L: Default
{
    fn default() -> Self {
        Either::Left(L::default())
    }
}