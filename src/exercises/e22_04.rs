use super::super::helper;

pub fn main() {
    let input = helper::get_input("\n","./src/txts/22_04.txt".to_string(), true);

    //getting the pairs, probably will use them as pairs for the rest of the day.
    let mut pairs: Vec<Vec<u32>> = vec![];
    
    for i in 0..input.len() {
        
        let mut nums:Vec<u32> = vec![];
        for a in input[i].split(",") {
            for b in a.split("-") {
                let b = match b.parse::<u32>() {
                    Ok(b) => b,
                    Err(_b) => panic!("Not a number!, check input file!"),
                };
                nums.push(b);
            }
        }
        pairs.push(nums)
    }
    // Nice. Just remember that pairs[n][0] goes with pairs[n][1], and pairs[n][2] goes with pairs[n][3]
    //print!("{:?}", &pairs);

    let mut part1 = 0;
    let mut part2 = 0;

    for pair in  pairs {
            if pair[0] <= pair[2] && pair [1] >= pair[3] {part1 +=1;}
            else if pair[0] >= pair[2] && pair[1] <= pair[3] {part1 += 1;};
        
            if pair[0] <= pair[2] && pair[2] <= pair[1] {part2 +=1;}
            // else if pair[0] <= pair[3] && pair[3] <= pair[1] {part2 +=1;}        
            else if pair[2] <= pair[0] && pair[0] <= pair[3] {part2 +=1;}
            // else if pair[2] <= pair[1] && pair[1] <= pair[3] {part2 +=1;};
            // realized later, these two are not needed. whoops, my bad.
        }
        println!("{}",part1);
        println!("{}",part2);
    }