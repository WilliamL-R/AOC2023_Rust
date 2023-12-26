use std::env;
use std::fs;
use std::collections::HashMap;

static DATASET_FOLDER: &str = "dataset/";
static PART_1_NUNMBER : &u16 = &1;
static PART_2_NUNMBER : &u16 = &2;

fn main() {
    let args: Vec<String> = env::args().collect();

    let dataset_file : &str = &args[1];
    // Create a longer lived value.
    let binding = args[2].parse::<u16>();
    let part_to_solve: &u16 = match &binding{
        Ok(number) => number,
        Err(e) => panic!("Not a valid part to solve. {e}"),
    };

    if part_to_solve == PART_1_NUNMBER{
        solve_part_1(dataset_file);
    } else if part_to_solve == PART_2_NUNMBER {
        solve_part_2(dataset_file);
    } else {
        panic!("Not a valid part to solve. Number is not recognised, use '1' or '2'.");
    }

}

fn solve_part_1(dataset_file : &str){
    let file_data = fs::read_to_string(DATASET_FOLDER.to_owned() + dataset_file).expect("Panic: Could not read the provided file.");
    let mut calibration_vector : Vec<u32> = Vec::new();

    for line in file_data.lines(){
        let mut _combined_digit : String = Default::default();
        let converted_line = line.to_string();

        let first_digit_index = converted_line.find(|c: char| c.is_ascii_digit());
        let _first_digit : char;

        match first_digit_index{
            Some(res) =>{ 
                _first_digit = converted_line.chars().nth(res).unwrap();
                _combined_digit.push(_first_digit);
            },
            None => panic!("No digit found"),
        }

        let second_digit_index = converted_line.rfind(|c: char| c.is_ascii_digit());
        match second_digit_index{
            Some(res ) => {
                let second_digit = converted_line.chars().nth(res).unwrap();
                _combined_digit.push(second_digit);
            } ,
            None => panic!("No digit found"),
        }
         calibration_vector.push(_combined_digit.parse().unwrap());
    }   
    let sum : u32 = calibration_vector.iter().sum();
    println!("{sum}");
}

fn solve_part_2(dataset_file : &str) {
    let file_data = fs::read_to_string(DATASET_FOLDER.to_owned() + dataset_file).expect("Panic: Could not read the provided file.");
    let mut calibration_vector : Vec<u32> = Vec::new();

    let regex_set = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]); 
    // Turn lines into digits,
    for line in file_data.lines(){
        let mut numbers_vector: Vec<Option<isize>> = Vec::from_iter((0..line.len()).map(|_| None));
        for i in 0..line.len() {
            for (re, val) in regex_set.iter() {
                if line[i..].starts_with(re) {
                    numbers_vector[i] = Some(*val);
                    break;
                }
            }
        }
        
        let flattened_numbers_vector = numbers_vector.iter().flatten().cloned().collect::<Vec<_>>();

        let first_digit = flattened_numbers_vector.first().unwrap();
        let second_digit = flattened_numbers_vector.last().unwrap();
        let full_number : u32 = ((first_digit * 10) + second_digit).try_into().unwrap();
        calibration_vector.push(full_number)
    } 

    let sum : u32 = calibration_vector.iter().sum();
    println!("{sum}");
}
