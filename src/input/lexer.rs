use rayon::prelude::*;

use serde::{Deserialize, Serialize};

use crate::types::CmdTokens;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Lexer {
    filter_out: Vec<String>,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            filter_out: vec!["a", "an", "at", "my", "of", "that", "the", "through", "to"]
                .par_iter()
                .map(|&s| s.to_owned())
                .collect(),
        }
    }

    pub fn lex(&self, s: &str) -> CmdTokens {
        let words = self.mod_words(&self.filter_parts(s));

        if words.is_empty() {
            CmdTokens::new(0, None, None, None, None)
        } else if words.len() < 2 {
            CmdTokens::new(1, Some(words[0].to_owned()), None, None, None)
        } else if let Some(pos) = words
            .par_iter()
            .position_any(|r| ["in", "inside", "from", "on", "with"].contains(&r.as_str()))
        {
            CmdTokens::new(
                words.len(),
                Some(words[0].to_owned()),
                Some(words[1..pos].join(" ")),
                Some(words[pos].to_owned()),
                Some(words[pos + 1..].join(" ")),
            )
        } else {
            CmdTokens::new(
                words.len(),
                Some(words[0].to_owned()),
                Some(words[1..].join(" ")),
                None,
                None,
            )
        }
    }

    fn filter_parts(&self, s: &str) -> Vec<String> {
        let words: Vec<String> = s
            .par_split_whitespace()
            .map(str::to_lowercase)
            .filter(|w| !(&self.filter_out).contains(&w))
            .collect();
        words
    }

    fn mod_words(&self, words: &[String]) -> Vec<String> {
        let mut modified = Vec::with_capacity(5 * words.len());
        for w in words {
            modified.push(
                match w.as_str() {
                    "north" => "n",
                    "south" => "s",
                    "east" => "e",
                    "west" => "w",
                    "northeast" => "ne",
                    "northwest" => "nw",
                    "southeast" => "se",
                    "southwest" => "sw",
                    "up" => "u",
                    "down" => "d",
                    _ => w,
                }
                .to_owned(),
            );
        }
        modified.shrink_to_fit();
        modified
    }
}
