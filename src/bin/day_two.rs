use advent_of_code::common::line_reader::read_lines;
use std::collections::HashMap;

type GameInfo = HashMap<String, usize>;
fn main() -> std::io::Result<()>{
    let a: usize = read_lines(".\\inputs\\day_two.txt")?
    .iter()
    .filter_map(|line|{
        let game_info = game_information(line).ok()?;
        let minimum_set = get_minimum_cubes(&game_info.1);
        minimum_set.into_values().reduce(|acc, x| acc * x)
    })
    .sum();

    println!("Result: {}", a);

    Ok(())
}

fn get_minimum_cubes(rounds: &[GameInfo]) -> GameInfo{
    let mut result = HashMap::new();

    for round in rounds{
        for (cube, amount) in round{
            match result.get(cube){
                Some(curr_amount) => {
                    if curr_amount < amount {
                        let _ = result.insert(cube.to_owned(), *amount);
                    }
                },
                None => {
                    let _ = result.insert(cube.to_owned(), *amount);
                },
            }
        }
    }

    result
}

fn is_possible(rounds: &[GameInfo], needs: &[(&str, usize)]) -> bool{
    for round in rounds{
        for (cube, max_amount) in needs{
            match round.get(*cube){
                Some(cube_amount) => {
                    if cube_amount > max_amount{
                        return  false;
                    }
                },
                None => {
                },
            } 
        }
    }
    true
}

fn game_information(line: &str) -> Result<(usize, Vec<GameInfo>), ()>{
    let (round_info_offset, id) = get_game_id(line)?;

    let rounds = get_rounds(&line[round_info_offset..])?;
    Ok((id, rounds))
}

fn get_game_id(line: &str) -> Result<(usize, usize), ()>{
    let split = line.splitn(2, ":").collect::<Vec<&str>>();
    let header = split.get(0).ok_or(())?;
    let offset = header.len() + 1;
    let binding = header.split(" ").collect::<Vec<&str>>();
    let id = binding.get(1).ok_or(())?;
    Ok((
        offset,
        id.parse::<usize>().or(Err(()))?
    ))
}

fn get_rounds(line: &str) -> Result<Vec<GameInfo>, ()>{
    let rounds = line.split(";").map(|round| round.trim()).collect::<Vec<&str>>();
    Ok(rounds.into_iter().map(|round| { count_round(round)}).collect())
}

fn count_round(round_info: &str) -> GameInfo{
    round_info
    .split(",")
    .map(|item| {item.trim()})
    .filter_map(|round_item| {
        let split = round_item.split(" ").collect::<Vec<_>>();
        if split.len() == 2{
            Some((split[1].to_owned(), split[0].parse::<usize>().unwrap()))
        }else{
            None
        }
    })
    .collect()
}