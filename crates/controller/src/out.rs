use crate::output::Output;
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
            Out::Result(cr) => cr.to_text(),
            _ => "".into(),
        }
    }

    pub fn convert(&self, out_type: Output) -> Self {
        if let Self::Result(r) = self {
            match out_type {
                Output::JSON => Self::Json(r.to_json()),
                Output::TEXT => Self::Text(r.to_text()),
                Output::STDOUT => {
                    println!("{}", r.to_text());
                    Self::None
                }
                Output::SLACK => {
                    println!("Can not send to slack");
                    Self::None
                }
            }
        } else {
            Self::None
        }
    }

    pub async fn convert_async(&self, out_type: Output) -> Self {
        if let Output::SLACK = out_type {
            if let Self::Result(r) = self {
                match send_slack::send_async(r.to_text()).await {
                    Ok(_) => Self::None,
                    Err(e) => panic!(e),
                }
            } else {
                Self::None
            }
        } else {
            self.convert(out_type)
        }
    }
}
