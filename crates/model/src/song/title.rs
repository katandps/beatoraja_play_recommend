use parse_display::Display;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Display)]
pub struct Title(String);

impl Title {
    pub fn from_title_and_subtitle(title: &str, subtitle: &str) -> Title {
        Title(format!("{}{}", title, subtitle))
    }

    pub fn new(title: String) -> Title {
        let c = title.chars().collect::<Vec<_>>();
        if c.len() > 255 {
            Title(format!(
                "{}...",
                c.into_iter().take(250).collect::<String>()
            ))
        } else {
            Title(title)
        }
    }
}
