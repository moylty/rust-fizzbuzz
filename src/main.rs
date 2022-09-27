use std::{convert::TryInto, io};

fn main()
{
    let mut str_input = String::new();
    io::stdin().read_line(&mut str_input)
        .expect("read error");

    let mut int_input: i32 = 0;
    
    let trimmed_str = str_input.trim();
    match trimmed_str.parse::<i32>() {
        Ok(i) => int_input = i,
        Err(..) => println!("this was not an integer: {}", trimmed_str),
    };

    println!("{:?}", fizz_buzz(int_input));
}

fn fizz_buzz(n: i32) -> Vec<String>
{
        
    // create a vector of empty strings, size: n
    let mut list = vec![String::new(); n.try_into().unwrap()];
    
    for i in 1..n+1
    {
        // convert our int to usize for use with vector
        let usize_i = i as usize;
        
        //concatenate the correct string
        if i.rem_euclid(3) == 0
        {
            list[usize_i-1].push_str("Fizz");
        }
        
        if i.rem_euclid(5) == 0
        {
            list[usize_i-1].push_str("Buzz");
        }
        
        if i.rem_euclid(3) != 0 && i.rem_euclid(5) != 0
        {
            list[usize_i-1] = i.to_string();
        }
    }
    
    list    // return the list
}