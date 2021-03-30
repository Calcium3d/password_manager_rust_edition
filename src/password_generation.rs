use rand::Rng;

pub fn password_generate(length: i32, caps: bool, numbers: bool, symbols: bool) -> String{
    let mut password: String = "".to_owned();
    let mut string: String = "".to_owned();
    let alphabet: &str = "abcdefghijklmnopqrstuvwxyz";
    let number: &str = "1234567890";
    let symbol: &str = "`~!@#$%^&*()-_=+[{]}\\|:;\"\'<,>.?/";
    let alphabet_capital: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    string.push_str(alphabet);

    //adding strings if the user specified
    if caps == true {
        string.push_str(alphabet_capital);
    }

    else if numbers == true {
        string.push_str(number);
    }

    else if symbols == true {
        string.push_str(symbol);
    }

    //converting to vector
    let things: Vec<char> = string.chars().collect();

    let string1: &str = &string[..];

    let length_of_string: i32 = string1.chars().count() as i32;

    //converting the list of characters into the final password
    for _i in 0..length.to_string().len() {
        let letter: usize = rand::thread_rng().gen_range(0, length_of_string) as usize;
        let letter1: String = things[letter].to_string();
        password.push_str(&letter1[..]);
    }

    password
}