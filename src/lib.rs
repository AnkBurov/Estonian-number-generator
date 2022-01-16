mod model;

pub mod functions {
    use std::cmp::Ordering;

    use wasm_bindgen::prelude::*;

    use crate::model::models::{Case, Question};

    #[wasm_bindgen]
    pub fn to_letters_js(number: String, question: Question) -> String {
        let number: i128 = number.parse().expect("Expected a number");
        to_letters(number, &question)
            .into()
    }

    pub fn to_letters(number: i128, question: &Question) -> String {
        let mut result: Vec<String> = Vec::new();
        let abs_number = number.abs();
        let text = abs_number.to_string();

        let mut accumulator: String = String::new();
        let accumulator_length = 3; // defines numeric group length
        let mut quantative_index: u32 = 0;
        for char in text.chars().rev() {
            if accumulator.len() < accumulator_length {
                accumulator.push(char);
            }
            if accumulator.len() >= accumulator_length {
                handle_numeric_group(quantative_index, &question, &mut result, &mut accumulator);
                accumulator.clear();
                quantative_index += 1;
            }
        };
        if !accumulator.is_empty() {
            handle_numeric_group(quantative_index, &question, &mut result, &mut accumulator);
        }

        let result = to_numeric_string(&mut result, &question);
        match number.cmp(&(0 as i128)) {
            Ordering::Less => format!("miinus {}", result),
            Ordering::Equal => "null".to_string(),
            Ordering::Greater => result,
        }
    }

    fn to_numeric_string(result: &Vec<String>, question: &Question) -> String {
        let mut words: Vec<String> = result.into_iter()
            .rev()// reverse text groups
            // flatmap text group words into a flat vector
            .flat_map(|numeric_group| numeric_group.split(" "))
            .map(|word| word.to_string())
            .filter(|word| !word.is_empty())
            .collect();

        match words.last_mut() {
            None => (),
            Some(word) => question.append_case_ending(word),
        }

        words.join(" ")
    }

    fn handle_numeric_group(index: u32, question: &Question, result: &mut Vec<String>, accumulator: &mut String) {
        let numeric_group = accumulator.chars().rev().collect::<String>();
        let quantative = question.get_case().get_quantative(index, &numeric_group).to_string();
        result.push(quantative);

        let texted_numeric_group = get_numeric_text_group(&numeric_group, &question.get_case());
        result.push(texted_numeric_group);
    }

    pub fn get_numeric_text_group(numeric: &str, case: &Case) -> String {
        let number: u32 = numeric.parse().expect("expected a number");
        let number_text = number.to_string();
        let result = match number {
            0..=10 => String::from(case.get_text_single(number)),
            11..=19 => {
                let second_simbol = &number_text[1..2];
                get_numeric_text_group(&second_simbol, case) + "teist"
            }
            20..=99 => {
                let first_symbol = &number_text[0..1];
                let second_symbol = &number_text[1..2];
                match case {
                    Case::Nominative => {
                        format!("{}kümmend {}", get_numeric_text_group(&first_symbol, case), get_numeric_text_group(second_symbol, case))
                    }
                    Case::Genitive => {
                        format!("{}kümne {}", get_numeric_text_group(&first_symbol, case), get_numeric_text_group(second_symbol, case))
                    }
                }
            }
            100..=999 => {
                let first_symbol = &number_text[0..1];
                let second_third_symbols = &number_text[1..3];
                match case {
                    Case::Nominative => {
                        format!("{} sada {}", get_numeric_text_group(first_symbol, case), get_numeric_text_group(second_third_symbols, case))
                    }
                    Case::Genitive => {
                        format!("{} saja {}", get_numeric_text_group(first_symbol, case), get_numeric_text_group(second_third_symbols, case))
                    }
                }
            }
            other => panic!("Unsupported value: {}", other),
        };
        return String::from(result.trim());
    }
}

#[cfg(test)]
mod tests {
    use crate::functions::{get_numeric_text_group, to_letters};
    use crate::model::models::Case;
    use crate::model::models::Question::{KuiPalju, KuiVana, Mitmendal, Mitmes};

    #[test]
    fn test_get_nominative() {
        assert_eq!("viis", get_numeric_text_group("5", &Case::Nominative));
        assert_eq!("kümme", get_numeric_text_group("10", &Case::Nominative));
        assert_eq!("üksteist", get_numeric_text_group("11", &Case::Nominative));
        assert_eq!("üheksateist", get_numeric_text_group("19", &Case::Nominative));
        assert_eq!("üheksateist", get_numeric_text_group("19", &Case::Nominative));
        assert_eq!("kakskümmend", get_numeric_text_group("20", &Case::Nominative));
        assert_eq!("kolmkümmend neli", get_numeric_text_group("34", &Case::Nominative));
        assert_eq!("üks sada", get_numeric_text_group("100", &Case::Nominative));
        assert_eq!("üks sada kolmteist", get_numeric_text_group("113", &Case::Nominative));
        assert_eq!("üheksa sada üheksakümmend üheksa", get_numeric_text_group("999", &Case::Nominative));
    }

    #[test]
    #[should_panic]
    fn test_get_nominative_invalid_value() {
        get_numeric_text_group("1000", &Case::Nominative);
    }

    #[test]
    fn test_to_letters() {
        assert_eq!("null", to_letters(0, &KuiPalju));
        assert_eq!("kümme", to_letters(10, &KuiPalju));
        assert_eq!("üheksateistkümneaastane", to_letters(19, &KuiVana));
        assert_eq!("ühe tuhande kahe saja kaheteistkümnendal", to_letters(1212, &Mitmendal));
        assert_eq!("miinus üheksa sada üheksakümmend üheksa", to_letters(-999, &KuiPalju));
        assert_eq!("ühe tuhande kolme saja kolmekümne seitsmendal", to_letters(1337, &Mitmendal));
        assert_eq!("kümne tuhande nelja saja viiekümne kolmas", to_letters(10453, &Mitmes));
        assert_eq!("üheksa saja üheksakümne üheksa tuhande üheksa saja üheksakümne üheksas", to_letters(999_999, &Mitmes));
        assert_eq!("üks miljon", to_letters(1_000_000, &KuiPalju));
        assert_eq!("üks miljon üks tuhat", to_letters(1_001_000, &KuiPalju));
        assert_eq!("üks sada viiskümmend miljonit üks", to_letters(150_000_001, &KuiPalju));
        assert_eq!("ühe miljardi ühe tuhande esimene", to_letters(1_000_001_001, &Mitmes));
        assert_eq!("ühe triljoni kahe miljardi kolme miljoni nelja tuhande viiendal", to_letters(1_002_003_004_005, &Mitmendal));
        assert_eq!("üks kvadriljon", to_letters(1_000_000_000_000_000, &KuiPalju));
        assert_eq!("üks kvintiljon", to_letters(1_000_000_000_000_000_000, &KuiPalju));
        assert_eq!("üks sekstillion", to_letters(1_000_000_000_000_000_000_000, &KuiPalju));
        assert_eq!("üks septillion", to_letters(1_000_000_000_000_000_000_000_000, &KuiPalju));
        assert_eq!("üks oktiljon", to_letters(1_000_000_000_000_000_000_000_000_000, &KuiPalju));
    }
}