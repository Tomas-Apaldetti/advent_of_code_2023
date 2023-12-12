use advent_of_code::common::line_reader::read_lines;

fn main() -> std::io::Result<()>{
    let lines: usize = read_lines(".\\inputs\\day_one.txt")?
    .iter()
    .map(|line|{
        match get_calibration(line){
            Ok(calibration) => {
                calibration
            },
            Err(_) => panic!("Did something wrong {}", line),
        }
    })
    .sum();
    
    println!("{}", lines);
    
    Ok(())
}

fn get_calibration(line: &str) -> Result<usize, ()>{
    let mut rest = line;
    let first_result = get_first_number_appearance(rest);
    rest = first_result?.0;
    let first = first_result?.1;
    let mut second = None;
    loop{
        let following_results = get_first_number_appearance(rest);
        match following_results{
            Ok((r_rest, r_second)) => {
                rest = r_rest;
                second = Some(r_second);
            },
            Err(_) => break,
        }
    }
    if second.is_none(){
        return Ok(first*10 + first);
    }
    
    return Ok(first * 10 + second.unwrap());
}

fn get_first_number_appearance(line: &str) -> Result<(&str, usize), ()>{
    for (i,c) in line.chars().enumerate() {
        if c.is_numeric() {
            return Ok((&line[i+1..], c.to_digit(10).unwrap() as usize));
        }
    }
    Err(())
}

