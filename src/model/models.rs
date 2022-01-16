use crate::model::models::Case::{Genitive, Nominative};

use std::str::FromStr;
use wasm_bindgen::prelude::*;

#[derive(Debug, PartialEq)]
#[wasm_bindgen]
pub enum Question {
    KuiPalju,
    KuiVana,
    Mitmes,
    Mitmendal,
}

impl FromStr for Question {
    type Err = ();

    fn from_str(input: &str) -> Result<Question, ()> {
        match input {
            "KuiPalju"  => Ok(Question::KuiPalju),
            "KuiVana"  => Ok(Question::KuiVana),
            "Mitmes"  => Ok(Question::Mitmes),
            "Mitmendal" => Ok(Question::Mitmendal),
            _      => Err(()),
        }
    }
}

impl Question {
    pub fn get_case(&self) -> Case {
        match self {
            Question::KuiPalju => Nominative,
            _ => Genitive,
        }
    }

    pub fn append_question_case(&self, word: &mut String) {
        match self.get_case() {
            Genitive => {
                match self {
                    Question::KuiPalju => (),
                    Question::KuiVana => {
                        // if word.eq(&"ühe") {
                        if word.eq(&"kolme") {
                            word.clear();
                            word.push_str("kolmaaastane");
                        } else {
                            word.push_str("aastane")
                        }
                    }
                    Question::Mitmes => {
                        if word.eq(&"ühe") {
                            word.clear();
                            word.push_str("esimene");
                        } else if word.eq(&"kahe") {
                            word.clear();
                            word.push_str("teine");
                        } else if word.eq(&"kolme") {
                            word.clear();
                            word.push_str("kolmas");
                        } else {
                            word.push_str("s");
                        }
                    }
                    Question::Mitmendal => {
                        if word.eq(&"ühe") {
                            word.clear();
                            word.push_str("esimesel");
                        } else if word.eq(&"kahe") {
                            word.clear();
                            word.push_str("teisel");
                        } else if word.eq(&"kolme") {
                            word.clear();
                            word.push_str("kolmandal");
                        } else {
                            word.push_str("ndal");
                        }
                    }
                }
            }
            _ => (),
        };
    }

    pub fn append_case_ending(&self, last_word: &mut String) {
        let last_word_str = last_word.as_str();
        match last_word_str {
            "ühe" => match self {
                Question::KuiVana => last_word.push_str("aastane"),
                Question::Mitmes => {
                    last_word.clear();
                    last_word.push_str("esimene")
                }
                Question::Mitmendal => {
                    last_word.clear();
                    last_word.push_str("esimesel")
                }
                Question::KuiPalju => {}
            },
            "kahe" => match self {
                Question::KuiVana => last_word.push_str("aastane"),
                Question::Mitmes => {
                    last_word.clear();
                    last_word.push_str("teine")
                }
                Question::Mitmendal => {
                    last_word.clear();
                    last_word.push_str("teisel")
                }
                _ => {}
            },
            "kolme" => match self {
                Question::KuiVana => last_word.push_str("aastane"),
                Question::Mitmes => {
                    last_word.clear();
                    last_word.push_str("kolmas")
                }
                Question::Mitmendal => {
                    last_word.clear();
                    last_word.push_str("kolmandal")
                }
                _ => {}
            },
            word if word.ends_with("teist") => match self {
                Question::KuiVana => last_word.push_str("kümneaastane"),
                Question::Mitmes => last_word.push_str("kümnes"),
                Question::Mitmendal => last_word.push_str("kümnendal"),
                _ => {}
            },
            _ => {
                match self {
                    Question::KuiVana => last_word.push_str("aastane"),
                    Question::Mitmes => last_word.push_str("s"),
                    Question::Mitmendal => last_word.push_str("ndal"),
                    _ => {}
                }
            }
        }
    }
}

pub enum Case {
    Nominative,
    Genitive,
}

impl Case {
    pub fn get_text_single(&self, number: u32) -> &str {
        match self {
            Case::Nominative => {
                match number {
                    0 => "",
                    1 => "üks",
                    2 => "kaks",
                    3 => "kolm",
                    4 => "neli",
                    5 => "viis",
                    6 => "kuus",
                    7 => "seitse",
                    8 => "kaheksa",
                    9 => "üheksa",
                    10 => "kümme",
                    other => panic!("Unsupported number: {}", other)
                }
            }
            Genitive => {
                match number {
                    0 => "",
                    1 => "ühe",
                    2 => "kahe",
                    3 => "kolme",
                    4 => "nelja",
                    5 => "viie",
                    6 => "kuue",
                    7 => "seitsme",
                    8 => "kaheksa",
                    9 => "üheksa",
                    10 => "kümne",
                    other => panic!("Unsupported number: {}", other)
                }
            }
        }
    }

    pub fn get_quantative(&self, quantative_index: u32, next_numeric_group: &str) -> &str {
        if next_numeric_group.eq("000") {
            return "";
        }
        match quantative_index {
            0 => "",
            1 => match self {
                Nominative => "tuhat",
                Genitive => "tuhande",
            },
            2 => match self {
                Nominative => match next_numeric_group {
                    "1" => "miljon",
                    _ => "miljonit"
                },
                Genitive => "miljoni",
            },
            3 => match self {
                Nominative => match next_numeric_group {
                    "1" => "miljard",
                    _ => "miljardit"
                },
                Genitive => "miljardi",
            },
            4 => match self {
                Nominative => match next_numeric_group {
                    "1" => "triljon",
                    _ => "triljonit"
                },
                Genitive => "triljoni",
            },
            5 => match self {
                Nominative => match next_numeric_group {
                    "1" => "kvadriljon",
                    _ => "kvadriljoni"
                },
                Genitive => "kvadriljoni",
            },
            6 => match self {
                Nominative => match next_numeric_group {
                    "1" => "kvintiljon",
                    _ => "kvintiljoni"
                },
                Genitive => "kvintiljoni",
            },
            7 => match self {
                Nominative => match next_numeric_group {
                    "1" => "sekstillion",
                    _ => "sekstillioni"
                },
                Genitive => "sekstillioni",
            },
            8 => match self {
                Nominative => match next_numeric_group {
                    "1" => "septillion",
                    _ => "septillioni"
                },
                Genitive => "septillioni",
            },
            9 => match self {
                Nominative => match next_numeric_group {
                    "1" => "oktiljon",
                    _ => "oktiljoni"
                },
                Genitive => "oktiljoni",
            },
            _ => panic!("It's over 9000 and even more!"),
        }
    }
}