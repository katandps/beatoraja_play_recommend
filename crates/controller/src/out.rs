use model::CommandResult;

pub enum Out {
    None,
    Json(String),
    Text(String),
    Result(CommandResult),
}

impl Out {
    pub fn to_string(&self) -> String {
        match self {
            Out::Json(s) => s.into(),
            Out::Text(s) => s.into(),
            Out::Result(cr) => cr.to_string(),
            _ => "".into(),
        }
    }
}
