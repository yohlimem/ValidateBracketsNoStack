use std::io;


fn main() {
    let line = io::stdin().lines().next().unwrap().unwrap();
    println!("the string {} is valid: {}", line, parentasis(line.trim()));
}

fn parentasis(string: &str) -> bool{
    let len = string.chars().count();
    // let j_char = indexes(string, j);
    if len == 0 {
        return true;
    }
    
    
    let i_char = indexes(string, 0);
    if check_if_par(&i_char) == 0 {
        return parentasis(&string[1..]);
    }
    // println!("i: {}, string: {} \n ========", i_char, string);


    if check_if_par(&i_char) == 2 {
        // println!("start par is close!");
        return false;
    }

    if let Some(pos) = find_closing(string, &i_char, 0, 0) {
        if pos == 1 {
            return parentasis(&string[(pos+1)..]);
        }
        
        // println!("big parantesis!!!");
        return parentasis(&string[(pos+1)..]) && parentasis(&string[1..pos]);
        
        
    } else {
        // println!("pos is false");
        return false;
    }


    // [][()]([{}])
    // [()]([{}])
    // ()
    


    // if the next bracket just 1 apart () then just continue
    // if t he next bracket is more than 1 apart {(())} then check its insides with repeatation thingy
    // if we couldnt find a closing bracket than your done.


}

fn find_closing(string: &str, open: &char, i: usize, counter: usize) -> Option<usize>{
    if i > string.chars().count() - 1 {return None};
    let i_char = indexes(string, i);
    if check_if_par(&i_char) == 0 {
        return find_closing(string, open, i+1, counter);
    }
    // (({[()]})}
    // ()
    if check_if_par(&i_char) == 2 {
        if same_type(&open, &i_char) { // iterate over brackets until you find closing
            // println!("i: {}, {}, counter: {} \n ========", i, i_char, counter);
            if counter <= 1 {
                return Some(i);
            }
        }
        return find_closing(string, open, i+1, counter-1);
    }
    return find_closing(string, open, i+1,counter+1);
}

// fn dist(string: &str, i: usize, size: usize) -> usize{
//     if i >= string.chars().count() {return string.chars().count()};

// }

fn indexes(string: &str, index: usize) -> char {
    string.chars().nth(index).unwrap()
}

fn check_if_par(c: &char) -> i32{
    if c == &'{' || c == &'[' || c == &'(' {
        return 1;
    } else if c == &'}' || c == &']' || c == &')' {
        return 2;
    }
    return 0;
}

fn same_type(c: &char, c2: &char) -> bool{
    if c == &'{' && c2 == &'}' {return true;}
    if c == &'[' && c2 == &']' {return true;}
    if c == &'(' && c2 == &')' {return true;}
    return false;
}