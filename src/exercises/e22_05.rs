use super::super::helper;

pub fn main() {
    let input = helper::get_input("\n","./src/txts/22_05.txt".to_string(), true);

    let mut labels = 0;                       //this is the place where the input changes from "picture" to orders
    for i in 0..input.len() {
        if input[i] == "" {labels = i-1;break;}
    }

    let mut stacks: Vec<Vec<char>> = vec![];    // there is probably a good reason to use a stack instead of a vector,
                                                // but I dont know what they are gonna ask on part 2.

    for x in 0..input[labels].len() {       // this reads through the first few lines, and makes the stacks doesnt matter if they are empty.
        let mut stack: Vec<char> = vec![];
        for y in (0..labels).rev() {
            let a = input[y].chars().nth(x).unwrap();
            if a != ' ' && a != '[' && a!= ']' {
                stack.push(a);
            }
        }
        if stack.len() != 0{                        //removing the empty ones.
            stacks.push(stack);
        }
    }
    // Not really needed, but time saving measure.. I am gonna copy the input, so it doesnt have to read it two times.
    let mut stacks2 = stacks.clone();
    
    // reading the instructions:
    for x in labels+2..input.len() {
        let mut instruction = input[x].split(' ');
        let mut data: Vec<usize> = vec![];
        
        for a in instruction {
            match a.parse::<usize>() {
                Ok(b) => data.push(b),
                Err(_) => continue,
            }
        }

        //This is part 1
        for a in 0..data[0]{                // actually moving around the characters.
            let b = stacks[data[1]-1].pop().unwrap();
            stacks[data[2]-1].push(b);
        }
        //This is part 2
        
        let mut c: Vec<char> = vec![];
        for a in 0..data[0]{
            let b = stacks2[data[1]-1].pop().unwrap();
            c.push(b);
        }
        c.reverse();
        stacks2[data[2]-1].append(&mut c); 
    }

    // Calculating both answers.
    let mut answer:Vec<String> = vec![];
    for a in stacks{
        answer.push(a[a.len()-1].to_string());
    }
    let mut answer2:Vec<String> = vec![];
    for a in stacks2{
        answer2.push(a[a.len()-1].to_string());
    }

    println!("Part 1 : {}",answer.concat());
    println!("Part 2 : {}",answer2.concat());
}