use std::fs;
use std::vec;

pub fn get_input(separator: &str, input_path: String, leave_empty_spaces: bool) -> Vec<String> {
    let f  = fs::read_to_string(input_path);
    let f = match f {
        Ok(file) => file,
        Err(e) => panic!("Could not read file! {}",e)
    };
    let mut res: Vec<String> = vec![];

    for line in f.split(separator){
        let mut a = String::from(line);
        a = remove_newlines(a);
        if !leave_empty_spaces {
            if a != "" {
                res.push(a);
            }
        }else{
            res.push(a)
        }
    }
    res
}

pub fn remove_newlines(mut a : String) -> String {
    a = a.replace("\n", "");
    return a;
}
