use uuid::{
    Uuid,
    ParseError
};
use std::{
    fmt::{
        Display,
        Formatter,
        Result as FormatResult
    }
};

#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq, Default, Hash, Eq)]
pub struct QuestionUuid(pub Uuid);

const PARAM_NAME: &str = "question_uuid";
impl QuestionUuid {
    pub fn to_query_parameter(self) -> String {
        format!("{}={}", PARAM_NAME, self.0)
    }
    pub fn parse_str(input: &str) -> Result<Self, ParseError> {
        Uuid::parse_str(input).map(QuestionUuid)
    }
}

impl Display for QuestionUuid {
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for QuestionUuid {
    fn from(uuid: Uuid) -> QuestionUuid {
        QuestionUuid(uuid)
    }
}

#[cfg(feature = "rocket_support")]
mod rocket {
    use super::*;
    use ::rocket::http::RawStr;
    use ::rocket::request::FromParam;
    use crate::uuid_from_param;
    use crate::uuid_from_form;
    use ::rocket::request::{FromForm, FormItems};

    impl<'a> FromParam<'a> for QuestionUuid {
        type Error = &'a RawStr;

        #[inline]
        fn from_param(param: &'a RawStr) -> Result<Self, Self::Error> {
            uuid_from_param(param).map(QuestionUuid)
        }
    }


    impl<'f> FromForm<'f> for QuestionUuid {
        type Error = ();

        #[inline]
        fn from_form(items: &mut FormItems<'f>, strict: bool) -> Result<Self, ()> {
            uuid_from_form(items, strict, PARAM_NAME)
                .map(QuestionUuid)
        }
    }
}
