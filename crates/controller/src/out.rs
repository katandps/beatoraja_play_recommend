use model::command::CommandResult;

pub enum Out {
    None,
    Json(String),
    Result(CommandResult),
}

impl Out {
    pub fn to_string(&self) -> String {
        match self {
            Out::Json(s) => s.into(),
            Out::Result(cr) => cr.to_string(),
            _ => "".into(),
        }
    }
}
