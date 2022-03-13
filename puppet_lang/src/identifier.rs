use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LowerIdentifier<EXTRA> {
    pub name: Vec<String>,
    pub is_toplevel: bool,
    pub extra: EXTRA,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CamelIdentifier<EXTRA> {
    pub name: Vec<String>,
    pub extra: EXTRA,
}
