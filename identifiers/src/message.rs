use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MessageUuid(pub Uuid);

#[cfg(feature="rocket_support")]
mod rocket {
    use super::*;
    use rocket::http::RawStr;
    use rocket::request::FromParam;
    use uuid_from_param;
    use uuid_from_form;
    use rocket::request::{FromForm, FormItems};

    impl<'a> FromParam<'a> for MessageUuid {
        type Error = &'a RawStr;

        #[inline]
        fn from_param(param: &'a RawStr) -> Result<Self, Self::Error> {
            uuid_from_param(param).map(MessageUuid)
        }
    }

    const PARAM_NAME: &'static str ="message_uuid";

    impl<'f> FromForm<'f> for MessageUuid {
        type Error = ();

        #[inline]
        fn from_form(items: &mut FormItems<'f>, strict: bool) -> Result<Self, ()> {
            uuid_from_form(items, strict, PARAM_NAME).map(MessageUuid)
        }
    }
}
