fn first_last(digits: Vec<u8>) -> (u8, u8) {
    match digits.as_slice() {
        // digits is empty return (0,0)
        [] => (0, 0),
        // digits is not empty return (first,last)
        _ => (
            *digits.first().unwrap() as u8,
            *digits.last().unwrap() as u8,
        ),
    }
}
fn line_filter_non_digits(input: &str) -> Vec<u8> {
    input
        .chars()
        .filter_map(|char| char.to_digit(10))
        .map(|digit| digit as u8)
        .collect::<Vec<u8>>()
}

const NUMBERS: [(&str, u8); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn line_parse_text_digits(input: &str) -> Vec<u8> {
    let mut out: Vec<u8> = vec![];
    let chars: Vec<char> = input.chars().collect();
    for i in 0..chars.len() {
        if chars[i].is_ascii_digit() {
            out.push(chars[i].to_digit(10).unwrap() as u8)
        } else {
            let compares: Vec<(&str, u8)> = NUMBERS
                .iter()
                .filter(|(term, _num)| term.chars().next().unwrap() == chars[i])
                .cloned()
                .collect();
            if !compares.is_empty() {
                let input_len = chars.len();
                for (compare, digit) in compares {
                    let compare_len = compare.len();
                    if input_len >= i + compare_len {
                        let attempt = chars[i..i + compare_len].iter().collect::<String>();
                        if attempt == compare {
                            out.push(digit);
                        }
                    }
                }
            }
        }
    }
    out
}

pub mod p1 {
    pub fn line_digit_sum(input: &str) -> u64 {
        input
            .lines()
            .map(super::line_filter_non_digits)
            .map(super::first_last)
            .map(|(first, last)| first as u64 * 10 + last as u64)
            .sum::<u64>()
    }
}

pub mod p2 {

    pub fn line_digit_sum(input: &str) -> u64 {
        input
            .lines()
            .map(super::line_parse_text_digits)
            .map(super::first_last)
            .map(|(first, last)| first as u64 * 10 + last as u64)
            .sum::<u64>()
    }
}
