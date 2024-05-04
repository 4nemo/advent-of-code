use std::{convert::From, fs, ops::Index};

static U8_DIGIT_MIN: u8 = 48;
static U8_DIGIT_MAX: u8 = 57;
static DIGIT_AS_CHARS: [&'static str; 10] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
];

type ByteChar = u8;

#[derive(Debug, PartialEq)]
enum CharType {
    Alpha,
    Digit,
    Newline,
    Eof,
}

#[derive(Debug, PartialEq)]
struct CharToken {
    char_type: CharType,
    literal: ByteChar,
}

#[derive(Debug)]
struct Parser {
    input_str: String,
    position: usize,
}

impl From<String> for Parser {
    fn from(item: String) -> Self {
        Parser { input_str: item, position: 0 }
    }
}

impl Parser {
    fn next_char_token(&mut self) -> CharToken {
        self.position += 1;

        if self.position > self.input_str.len() {
            return CharToken { char_type: CharType::Eof, literal: b'\0' };
        }
        match self.input_str.as_bytes()[self.position - 1] {
            b'\n' => return CharToken { char_type: CharType::Newline, literal: b'\n' },
            other => {
                for i in U8_DIGIT_MIN..=U8_DIGIT_MAX {
                    if other == i {
                        return CharToken { char_type: CharType::Digit, literal: other }
                    }
                }
                if other.is_ascii() {
                    return CharToken { char_type: CharType::Alpha, literal: other }
                }
                else {
                    panic!("Invalid CharType found: must be a lowercase letter, digit, EoF, or newline");
                }
            }
        }
    }
}

fn calibrate(value: String) -> u16 {
    let mut engine = Parser::from(value);
    let mut sum = 0u16;
    let mut prev_digit = 0u16;
    let mut num_digits = 0u8;
    let mut word_index = 0u8;
    let mut char_buf: Vec<ByteChar> = vec![];

    loop {
        let ch_tok = engine.next_char_token();

        match ch_tok.char_type {
            CharType::Alpha => {
                if contains_digit_substr(ch_tok.literal, word_index) {
                    char_buf.push(ch_tok.literal);
                }
                if let Some(digit) = contains_digit_substr(ch_tok.literal, word_index) {
                    
                }
                if is_digit_str(char_buf) {
                    num_digits += 1;
                    prev_digit
                }
            },
            CharType::Digit => {
                num_digits += 1;
                prev_digit = u16::from(ch_tok.literal - U8_DIGIT_MIN);
                if num_digits == 1 { sum += prev_digit * 10; }
            },
            CharType::Newline => {
                num_digits = 0;
                sum += prev_digit;
            },
            CharType::Eof => {
                if num_digits > 0 { break sum + prev_digit }
                else { break sum }
            }
        }
    }
}

fn contains_digit_substr(input: u8, index: u8) -> bool {
    
}

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("input")?;
    let calibration_value = calibrate(input);

    println!("\nThe calibration value is: {calibration_value}\n");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct CharTestCase {
        expected_char: ByteChar,
        expected_type: CharType,
    }

    struct StringTestCase {
        input_str: String,
        expected_sum: u16,
    }

    #[test]
    fn test_next_char() {
        let input = String::from("ab12dyr90\nx\n");
        let test_cases: [CharTestCase; 13] = [
            CharTestCase { expected_char: b'a', expected_type: CharType::Alpha, },
            CharTestCase { expected_char: b'b', expected_type: CharType::Alpha, },
            CharTestCase { expected_char: b'1', expected_type: CharType::Digit, },
            CharTestCase { expected_char: b'2', expected_type: CharType::Digit, },
            CharTestCase { expected_char: b'd', expected_type: CharType::Alpha, },
            CharTestCase { expected_char: b'y', expected_type: CharType::Alpha, },
            CharTestCase { expected_char: b'r', expected_type: CharType::Alpha, },
            CharTestCase { expected_char: b'9', expected_type: CharType::Digit, },
            CharTestCase { expected_char: b'0', expected_type: CharType::Digit, },
            CharTestCase { expected_char: b'\n', expected_type: CharType::Newline, },
            CharTestCase { expected_char: b'x', expected_type: CharType::Alpha, },
            CharTestCase { expected_char: b'\n', expected_type: CharType::Newline, },
            CharTestCase { expected_char: b'\0', expected_type: CharType::Eof, },
        ];

        let mut engine = Parser::from(input);

        for case in test_cases {
            let ch_tok: CharToken = engine.next_char_token();

            assert_eq!(ch_tok.char_type, case.expected_type);
            assert_eq!(ch_tok.literal, case.expected_char);
        }
    }

    #[test]
    fn test_calibration_loop() {
        let test_cases: [StringTestCase; 6] = [
            StringTestCase { input_str: String::from("ab1de"), expected_sum: 11u16 },
            StringTestCase { input_str: String::from("a2i3e"), expected_sum: 23u16 },
            StringTestCase { input_str: String::from("a4b5d6e"), expected_sum: 46u16 },
            StringTestCase { input_str: String::from("a7bi89d0e"), expected_sum: 70u16 },
            StringTestCase { input_str: String::from("abide"), expected_sum: 0u16 },
            StringTestCase { input_str: String::from("a1b2\nid3"), expected_sum: 45u16 },
        ];

        for case in test_cases {
            let calibration_value = calibrate(case.input_str);
            assert_eq!(calibration_value, case.expected_sum);
        }
    }

    #[test]
    fn test_calibration_loop_v2() {
        let test_cases: [StringTestCase; 12] = [
            StringTestCase { input_str: String::from("one23456789"), expected_sum: 19u16 },
            StringTestCase { input_str: String::from("onetwo3456789"), expected_sum: 19u16 },
            StringTestCase { input_str: String::from("otthreeff6789"), expected_sum: 39u16 },
            StringTestCase { input_str: String::from("two1nine"), expected_sum: 29u16 },
            StringTestCase { input_str: String::from("eightwothree"), expected_sum: 83u16 },
            StringTestCase { input_str: String::from("abcone2threexyz"), expected_sum: 13u16 },
            StringTestCase { input_str: String::from("xtwone3four"), expected_sum: 24u16 },
            StringTestCase { input_str: String::from("4nineeightseven2"), expected_sum: 42u16 },
            StringTestCase { input_str: String::from("zoneight234"), expected_sum: 14u16 },
            StringTestCase { input_str: String::from("7pqrstsixteen"), expected_sum: 76u16 },
            StringTestCase { input_str: String::from("xtwone3four\n4nineeightseven2"), expected_sum: 66u16 },
            StringTestCase { input_str: String::from("4nineeightseven2\nzoneight234\n7pqrstsixteen"), expected_sum: 132u16 },
        ];

        for case in test_cases {
            let calibration_value = calibrate(case.input_str);
            assert_eq!(calibration_value, case.expected_sum);
        }
    }
}