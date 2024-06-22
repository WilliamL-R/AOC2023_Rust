use std::env;
use std::fs;

static DATASET_FOLDER: &str = "dataset/";

static RED_CUBES_NAME: &str = "red";
static GREEN_CUBES_NAME: &str = "green";
static BLUE_CUBES_NAME: &str = "blue";
static RED_CUBES_ALLOWED: u16 = 12;
static GREEN_CUBES_ALLOWED: u16 = 13;
static BLUE_CUBES_ALLOWED: u16 = 14;

fn main() {
    let args: Vec<String> = env::args().collect();

    let dataset_file : &str = &args[1];
    //let valid_games = get_valid_games(dataset_file);
    //let sum : u16 = valid_games.iter().sum();
    let minimum_games = get_required_cubes(dataset_file);
    let power_sum: u32 = minimum_games.iter().sum();
    //print!("Sum of the valid number of cubes is: {sum}");
    println!("Sum of the valid number of cubes is: {power_sum}");

}

fn get_valid_games(dataset_file : &str) -> Vec<u16> {
    let file_data = fs::read_to_string(DATASET_FOLDER.to_owned() + dataset_file).expect("Panic: Could not read the provided file.");
    let mut game_vector : Vec<u16> = Vec::new();
    
    for line in file_data.lines(){
        let mut valid_game :bool = true;

        let converted_line = line.to_string();
        let game_string: Vec<&str> = converted_line.split(':').collect();

        let cubes_revealed = game_string.last().unwrap().split(';');  

        'cube_game: for cubes in cubes_revealed {
            let reveal: Vec<&str> = cubes.split(',').collect();
            for cubes in reveal {
                let formatted_pull: Vec<&str> = cubes.trim_start().split_whitespace().collect();
                let colour : &str = formatted_pull[1];
                let cubes_pulled: u16 = formatted_pull[0].parse().unwrap();

                let is_valid: bool = match colour {
                    "red" => check_valid_red(cubes_pulled),
                    "blue" => check_valid_blue(cubes_pulled),
                    "green" => check_valid_green(cubes_pulled),
                    _ => false,
                };
                
                if !is_valid {
                    valid_game = false;
                    break 'cube_game;
                }
            }
        }
        if valid_game
        {
            let _game_id : u16;
            let game_id: Vec<&str> = game_string.first().unwrap().split_whitespace().collect();
            _game_id = game_id.last().unwrap().parse().unwrap();
            game_vector.push(_game_id);
        }
    }
    return  game_vector;
}


fn check_valid_blue(cube_count : u16) -> bool {
    return cube_count <= BLUE_CUBES_ALLOWED;}

fn check_valid_red(cube_count : u16) -> bool {
    return cube_count <= RED_CUBES_ALLOWED;}

fn check_valid_green(cube_count : u16) -> bool {
    return cube_count <= GREEN_CUBES_ALLOWED;
}

fn get_required_cubes(dataset_file : &str) -> Vec<u32>{

    let file_data = fs::read_to_string(DATASET_FOLDER.to_owned() + dataset_file).expect("Panic: Could not read the provided file.");
    let mut power_vector : Vec<u32> = Vec::new();
    
    for line in file_data.lines(){
        let converted_line = line.to_string();
        let game_string: Vec<&str> = converted_line.split(':').collect();

        let cubes_revealed = game_string.last().unwrap().split(';');  

        let mut min_red : u16 = 0;
        let mut min_blue : u16 = 0;
        let mut min_green : u16 = 0;

        for cubes in cubes_revealed {
            let reveal: Vec<&str> = cubes.split(',').collect();

            for cubes in reveal {
                let formatted_pull: Vec<&str> = cubes.trim_start().split_whitespace().collect();
                let colour : &str = formatted_pull[1];
                let cubes_pulled: u16 = formatted_pull[0].parse().unwrap();
                
                match colour {
                    "red" => min_red = if cubes_pulled > min_red {cubes_pulled} else {min_red},
                    "blue" => min_blue = if cubes_pulled > min_blue {cubes_pulled} else {min_blue},
                    "green" => min_green = if cubes_pulled > min_green {cubes_pulled} else {min_green},
                    _ => panic!("Not a valid colour."),
                };
            }
        }
        let multiply_result : u32 = multiply_minimum_cubes(min_red, min_green, min_blue);
        power_vector.push(multiply_result);
    }
    return power_vector;
}

fn multiply_minimum_cubes(red : u16, green : u16, blue : u16) -> u32 { return (red*green*blue).into();}