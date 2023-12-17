use super::util::*;
use super::{day1, day2, day3};

fn get_test_data(day: u8, part: u8) -> String {
    get_data(day, part, DataType::Test)
}

fn get_test_result_data(day: u8, part: u8) -> String {
    get_data(day, part, DataType::TestResult)
}

fn default_test(day: u8, part: u8, function: fn(&String) -> String) {
    let input = get_test_data(day, part);
    let result = function(&input);
    assert_eq!(result.to_string(), get_test_result_data(day, part))
}

#[test]
fn day1p1() {
    default_test(1, 1, |input| day1::p1::line_digit_sum(input).to_string());
}
#[test]
fn day1p2() {
    default_test(1, 2, |input| day1::p2::line_digit_sum(input).to_string());
}

#[test]
fn day2p1() {
    default_test(2, 1, |input| day2::p1::validate_games(input).to_string());
}

#[test]
fn day2p2() {
    default_test(2, 2, |input| day2::p2::sum_powers(input).to_string());
}

#[test]
fn day3p1() {
    default_test(3, 1, |input| day3::p1::sum_part_numbers(input).to_string());
}
