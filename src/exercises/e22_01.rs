use super::super::helper;

pub fn main() {
    let input = helper::get_input("\n","./src/txts/22_01.txt".to_string(), true);

    let mut elves:Vec<i32> = vec![];
    let mut contents:Vec<i32> = vec![];
    for line in input {
        if line != "" {
            match line.parse::<i32>() {
                Ok(calories) => contents.push(calories),
                Err(_) => panic!("Couldnt convert string, check your input file")
            }
        }else{
            let mut a = 0;
            for n in &contents {
                a += n
            }
            contents.clear();
            elves.push(a);
        }
    }
    elves.sort_unstable();
    let result1 = elves.pop();
    let result2 = elves.pop();
    let result3 = elves.pop();

    match (result1,result2,result3) {
        (Some(a),Some(b),Some(c)) => {
            println!("{}", a);
            println!("{}", a+b+c);
        }
        (_,_,_) => {
            panic!("There are not enough values in your input!?")
        }
    }

}