//! first and last of each line, sum of these, only one digit means double
use util::{get_data, DataType};
fn main() {
    let input = get_data(3, 1, DataType::Input);
    dbg!(day3::p1::sum_part_numbers(&input));
}

pub mod util {
    pub enum DataType {
        Input,
        Test,
        TestResult,
    }

    pub fn get_data(day: u8, part: u8, data_type: DataType) -> String {
        let filename = match data_type {
            DataType::Input => "input",
            DataType::Test => "test",
            DataType::TestResult => "test_result",
        };
        let path = match data_type {
            DataType::Input => format!("inputs/{day}/{filename}.txt"),
            _ => format!("inputs/{day}/{part}/{filename}.txt"),
        };
        std::fs::read_to_string(path).unwrap()
    }
}

pub mod day1;

pub mod day2;

pub mod day3;

#[cfg(test)]
mod tests;
