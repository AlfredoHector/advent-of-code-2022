use super::super::helper;

pub fn main() {
    let input = helper::get_input("\n","./src/txts/22_02.txt".to_string(), true);
    
    let mut total = 0;
    let mut total2 = 0 ;

    for line in input{
        let play = &line[..1];
        let response = &line[2..];
        let play = replacements(play);
        let response2 = solve(play,response);
        let response = replacements(response);
        total += reward(play, response);
        total2 += reward(play, response2);
    }

    println!("First answer : {}",total);
    println!("Second answer : {}",total2);
}

fn solve(a: i32, b: &str) -> i32 {
    let c = match b {
        "X" => if a != 1 { a-1 } else { 3 },
        "Y" => {a},
        "Z" => if a != 3 { a+1 } else { 1 },
        &_ => panic!("there was something wrong on the input file!")
    };
    return c
}

fn replacements(a: &str) -> i32 {
    match a {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => panic!("there is something wrong inside of the input!"),
    }
}

fn reward(a:i32, b:i32) -> i32 {
    if a == 1 && b == 3 {
        return b;
    }else if a ==3 && b==1 {
        return b+6
    }else if a < b {
        return b+6; 
    }else if a == b {
        return b+3;
    }else {
        return b
    };
}