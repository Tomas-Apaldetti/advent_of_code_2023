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
    rest = first_result.ok_or(())?.0;
    let first = first_result.ok_or(())?.1;
    let mut second = None;
    loop{
        let following_results = get_first_number_appearance(rest);
        match following_results{
            Some((r_rest, r_second)) => {
                rest = r_rest;
                second = Some(r_second);
            },
            None => break,
        }
    }
    if second.is_none(){
        return Ok(first*10 + first);
    }
    
    return Ok(first * 10 + second.unwrap());
}

fn get_first_number_appearance(line: &str) -> Option<(&str, usize)>{
    for (i,c) in line.chars().enumerate() {
        if c.is_numeric() {
            return Some((
                &line[i+1..], 
                c.to_digit(10).unwrap() as usize
            ));
        }
        match word_numeric(&line[i..]){
            Some((offset,n)) => {
                return Some((
                    &line[i+offset+1..],
                    n
                ))
            },
            None => {
            },
        }
    }
    None
}

fn word_numeric(word: &str) -> Option<(usize,usize)>{
    if word.len() < 3 {
        return None;
    }
    for (i, _) in word.chars().enumerate(){
        if word.len() <= i+1{
            return None
        }
        let a = match &word[..=i+1]{
            "one" => Some(1),
            "two" => Some(2),
            "three" => Some(3),
            "four" => Some(4),
            "five" => Some(5),
            "six" => Some(6),
            "seven" => Some(7),
            "eight" => Some(8),
            "nine" => Some(9),
            "zero" => Some(0),
            _ => None,
        };
        if a.is_some(){
            return Some((i, a.unwrap()));
        }
        if i+1 >= 5 {
            return None
        }
    }

    None
}

