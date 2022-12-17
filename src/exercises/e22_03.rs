use super::super::helper;

pub fn main() {
    let input = helper::get_input("\n","./src/txts/22_03.txt".to_string(), true);
    
    let mut total = 0;

    for line in &input {
        let half = line.chars().count()/2;
        let pocket1 = &line[..half];
        let pocket2 = &line[half..];

        for cha in pocket1.chars() {
            if pocket2.contains(cha) {
                total += get_value(cha);
                break;
            }
        }
    }
    println!("{}" , total);

    // Part 2:
    total = 0;
    let mut n = 0;
    while n < 300 {
        let line1 = &input[n];
        let line2 = &input[n+1];
        let line3 = &input[n+2];
        n += 3;

        for cha in line1.chars() {
            if line2.contains(cha) && line3.contains(cha) {
                total += get_value(cha);
                break;
            }
        }
    }
    println!("{}" , total);
}


fn get_value(cha: char) -> u32 {
    let value = match cha.is_ascii_lowercase() {
        true => (cha as u32) -96,
        false => (cha as u32) -64+26,
    };
    return value;
}