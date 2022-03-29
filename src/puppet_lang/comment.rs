use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Comment<EXTRA> {
    pub value: String,
    pub extra: EXTRA,
}
