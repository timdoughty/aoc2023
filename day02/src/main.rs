use std::fs;
use std::collections::HashMap;

fn main() {
    // Thanks to Giddeon for showing me a more succient way to set variables up
    let input = fs::read_to_string("C:/Users/timot/Documents/Code/aoc2023/day02/input.txt").unwrap();
    let blocks: HashMap<&str, i32> = vec![("red", 12), ("green", 13), ("blue", 14)].into_iter().collect();
    let mut valid_games = 0;
    let mut total_game_count = 0;

    
    for game in input.lines() {
        let (valid, count) = check_game_validity(game, &blocks);
        valid_games += valid;
        total_game_count += count;
    }

    println!("Valid Games: {}", valid_games);
    println!("Total Count: {}", total_game_count);

}

fn check_game_validity(game: &str, blocks: &HashMap<&str, i32>) -> (i32, i32) {
    /*
     *   This function takes in a game string and a hashmap of blocks and their counts
     *   It then checks to see if the game is valid and returns a tuple of the game number
     *   and the total count of blocks necessary to make the game draws valid
     */
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;
    let mut red_drawn = 0;
    let mut green_drawn = 0;
    let mut blue_drawn = 0;
    let mut game_num = 0;

    let (game_number, draws) = game.split_once(':').unwrap_or((game, ""));
    for draw in draws.split(';') {
        for handful in draw.split(',') {
            let (count, color) = handful.trim().split_once(' ').unwrap_or((game, ""));
            match color {
                "red" => red_drawn = count.parse::<i32>().unwrap(),
                "green" => green_drawn = count.parse::<i32>().unwrap(),
                "blue" => blue_drawn = count.parse::<i32>().unwrap(),
                _ => println!("Something went wrong..."),
            }
        }
        if red_drawn > max_red {
            max_red = red_drawn;
        }
        if green_drawn > max_green {
            max_green = green_drawn;
        }
        if blue_drawn > max_blue {
            max_blue = blue_drawn;
        }
    }

    let red_count = blocks.get("red");
    let green_count = blocks.get("green");
    let blue_count = blocks.get("blue");
    match (red_count, green_count, blue_count) {
        (Some(&red_count), Some(&green_count), Some(&blue_count)) if red_count >= max_red && green_count >= max_green && blue_count >= max_blue => {
            game_num = game_number.split_once(' ').unwrap().1.parse::<i32>().unwrap();
        },
        _ => {
            let _ = game_number;
        }
    }
    
    return (game_num, max_red * max_green * max_blue);
}