use std::io;

pub fn user_input_string(msg: &str) -> io::Result<String> {
    println!("{}", msg);
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string().to_lowercase())
}

pub fn user_input_number(msg: &str) -> Result<usize, std::num::ParseIntError> {
    println!("{}", msg);
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let number: usize = input.trim().parse()?;
    Ok(number)
}
