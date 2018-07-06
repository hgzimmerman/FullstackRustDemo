use yew::prelude::*;
use std::fmt::Formatter;
use std::fmt::Debug;
use common::fetch::FetchResponse;

//use context::networking::FtWrapper;

use loading::LoadingType;

use wrappers::empty_vdom_node;




pub enum Loadable<T> {
    Unloaded,
    Loading,
    Loaded(T),
    Failed(Option<String>)
}

impl<T> PartialEq for Loadable<T> where T: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        match *self {
            Loadable::Unloaded => {
                if let Loadable::Unloaded = other {
                    true
                } else {
                    false
                }
            }
            Loadable::Loading => true, // Just make the assumption that if something is loading its representation does not need to change
            Loadable::Loaded(ref t) => {
                if let Loadable::Loaded(ref t_other) = other {
                    return t == t_other
                } else {
                    false
                }
            }
            Loadable::Failed(ref f) => {
                if let Loadable::Failed(ref f_other) = other {
                   return f == f_other
                } else {
                    false
                }

            }
        }
    }

}

//impl <T> FtWrapper for Loadable<T> where T: Default {
//    fn set_ft(&mut self, ft: FetchTask) {
//        *self = Loadable::Loading(ft)
//    }
//}

impl <T> Debug for Loadable<T> where T: Debug {
    fn fmt(&self, f: &mut Formatter) -> Result<(), ::std::fmt::Error> {
        match self {
            Loadable::Unloaded => write!(f, "Unloaded"),
            Loadable::Loading => write!(f, "Loading"),
            Loadable::Loaded(t) => write!(f, "Loaded: {:?}", t),
            Loadable::Failed(_) => write!(f, "Failed"),
        }
    }
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
            Loadable::Loading => Loadable::Loading, // Any loading loadable throws away the FT because it can't be cloned
            Loadable::Loaded(t) => Loadable::Loaded(t.clone()),
            Loadable::Failed(o) => Loadable::Failed(o.clone())
        }
    }
}



impl <T> Loadable<T> {


    /// Creates a loadable from a fetch response
    pub fn from_fetch_response(fetch_response: FetchResponse<T>) -> Loadable<T> {
        use self::FetchResponse::*;
        match fetch_response {
            Success(t) => Loadable::Loaded(t),
            Error(_) => Loadable::Failed(None),
            Started => Loadable::Loading
        }
    }

    pub fn as_option<'a>(&'a self) -> Option<&'a T> {
        if let Loadable::Loaded(value) = self {
            Some(value)
        } else {
            None
        }
    }

    fn custom_view<U, LoadedFn, FailedFn>(&self,
                                               unloaded: Html<U>,
                                               loading: Html<U>,
                                               loaded_fn: LoadedFn,
                                               failed_fn: FailedFn
    ) -> Html<U>
        where
        U: Component,
        LoadedFn: Fn(&T) -> Html<U>,
        FailedFn: Fn(&Option<String>) -> Html<U>
    {
        match self {
            Loadable::Unloaded => unloaded,
            Loadable::Loading => loading,
            Loadable::Loaded(ref t) => loaded_fn(t),
            Loadable::Failed(ref error) => failed_fn(error)
        }
    }


    fn failed<U> (error: &Option<String>) -> Html<U>
    where
        U: Component
    {
        if let Some(message) = error {
            html! {
                <div class="flexbox-center",>
                    {message}
                </div>
            }
        }
        else {
            html! {
                <div class="flexbox-center",>
                    {"Request Failed"}
                </div>
            }
        }
    }

    /// Uses a 100x100 icons for loading and error.
    /// This should work for medium to large sized views, but if a view dimension can be less than that, visual bugs will result.
    pub fn default_view<U>(&self, render_fn: fn(&T) -> Html<U> ) -> Html<U>
        where
            U: Component
    {
        self.custom_view(
            empty_vdom_node(),
            LoadingType::Fidget{diameter: 100}.view(),
            render_fn,
            Self::failed
        )
    }

    /// Uses text for all error and loading fillers.
    /// This should allow it to be used most flexibly.
    pub fn small_view<U>(&self, render_fn: fn(&T) -> Html<U> ) -> Html<U>
        where
            U: Component
    {
        self.custom_view(
            empty_vdom_node(),
            LoadingType::Empty.view(),
            render_fn,
            Self::failed
        )
    }

    pub fn restricted_custom_view<U, LoadedFn, FailedFn>(&self,
                                     unloaded: Html<U>,
                                     loading_type: LoadingType<U>,
                                     render_fn: LoadedFn,
                                     failed_fn: FailedFn
    ) -> Html<U>
            where
            U: Component,
            LoadedFn: Fn(&T) -> Html<U>,
            FailedFn: Fn(&Option<String>) -> Html<U>
    {
        self.custom_view(
            unloaded,
            loading_type.view(),
            render_fn,
            failed_fn
        )
    }



}