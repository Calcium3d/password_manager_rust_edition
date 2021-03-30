use rand::Rng;

pub fn caeser_encrypt(unencrypted_text: String) -> String {
    //generating a key
    let key: u32 = rand::thread_rng().gen_range(1, 95);

    let mut result: String = "".to_owned();

    //converting to a vector
    let unencrypted_text: Vec<char> = unencrypted_text.chars().collect();

    let mut alphabet: u32 = 0;

    //converting the alphabets into numbers
    for c in unencrypted_text {
        if c == 'a' {
            alphabet = 1;
        }

        if c == 'b' {
            alphabet = 2;
        }

        if c == 'c' {
            alphabet = 3;
        }

        if c == 'd' {
            alphabet = 4;
        }

        if c == 'e' {
            alphabet = 5;
        }

        if c == 'f' {
            alphabet = 6;
        }

        if c == 'g' {
            alphabet = 7;
        }

        if c == 'h' {
            alphabet = 8;
        }

        if c == 'i' {
            alphabet = 9;
        }

        if c == 'j' {
            alphabet = 10;
        }

        if c == 'k' {
            alphabet = 11;
        }

        if c == 'l' {
            alphabet = 12;
        }

        if c == 'm' {
            alphabet = 13;
        }

        if c == 'n' {
            alphabet = 14;
        }

        if c == 'o' {
            alphabet = 15;
        }

        if c == 'p' {
            alphabet = 16;
        }

        if c == 'q' {
            alphabet = 17;
        }

        if c == 'r' {
            alphabet = 18;
        }

        if c == 's' {
            alphabet = 19;
        }

        if c == 't' {
            alphabet = 20;
        }

        if c == 'u' {
            alphabet = 21;
        }

        if c == 'v' {
            alphabet = 22;
        }

        if c == 'w' {
            alphabet = 23;
        }

        if c == 'x' {
            alphabet = 24;
        }
        
        if c == 'y' {
            alphabet = 25;
        }

        if c == 'z' {
            alphabet = 26;
        }

        if c == 'A' {
            alphabet = 27;
        }

        if c == 'B' {
            alphabet = 28;
        }

        if c == 'C' {
            alphabet = 29;
        }

        if c == 'D' {
            alphabet = 30;
        }

        if c == 'E' {
            alphabet = 31;
        }

        if c == 'F' {
            alphabet = 32;
        }

        if c == 'G' {
            alphabet = 33;
        }

        if c == 'H' {
            alphabet = 34;
        }

        if c == 'I' {
            alphabet = 35;
        }

        if c == 'J' {
            alphabet = 36;
        }

        if c == 'K' {
            alphabet = 37;
        }

        if c == 'L' {
            alphabet = 38;
        }

        if c == 'M' {
            alphabet = 39;
        }

        if c == 'N' {
            alphabet = 40;
        }

        if c == 'O' {
            alphabet = 41;
        }

        if c == 'P' {
            alphabet = 42;
        }

        if c == 'Q' {
            alphabet = 43;
        }

        if c == 'R' {
            alphabet = 44;
        }

        if c == 'S' {
            alphabet = 45;
        }

        if c == 'T' {
            alphabet = 46;
        }

        if c == 'U' {
            alphabet = 47;
        }

        if c == 'V' {
            alphabet = 48;
        }

        if c == 'W' {
            alphabet = 49;
        }

        if c == 'X' {
            alphabet = 50;
        }
        
        if c == 'Y' {
            alphabet = 51;
        }

        if c == 'Z' {
            alphabet = 52;
        }

        if c == '1' {
            alphabet = 53;
        }

        if c == '2' {
            alphabet = 54;
        }

        if c == '3' {
            alphabet = 55;
        }

        if c == '4' {
            alphabet = 56;
        }

        if c == '5' {
            alphabet = 57;
        }

        if c == '6' {
            alphabet = 58;
        }

        if c == '7' {
            alphabet = 59;
        }

        if c == '8' {
            alphabet = 60;
        }

        if c == '9' {
            alphabet = 61;
        }

        if c == '0' {
            alphabet = 62;
        }

        if c == '`' {
            alphabet = 63;
        }

        if c == '~' {
            alphabet = 64;
        }

        if c == '!' {
            alphabet = 65;
        }

        if c == '@' {
            alphabet = 66;
        }

        if c == '#' {
            alphabet = 67;
        }

        if c == '$' {
            alphabet = 68;
        }

        if c == '%' {
            alphabet = 69;
        }

        if c == '^' {
            alphabet = 70;
        }

        if c == '&' {
            alphabet = 71;
        }

        if c == '*' {
            alphabet = 72;
        }

        if c == '(' {
            alphabet = 73;
        }

        if c == ')' {
            alphabet = 74;
        }

        if c == '-' {
            alphabet = 75;
        }

        if c == '_' {
            alphabet = 76;
        }

        if c == '{' {
            alphabet = 77;
        }

        if c == '[' {
            alphabet = 78;
        }

        if c == '}' {
            alphabet = 79;
        }

        if c == ']' {
            alphabet = 80;
        }

        if c == '=' {
            alphabet = 81;
        }

        if c == '+' {
            alphabet = 82;
        }

        if c == '\\' {
            alphabet = 83;
        }

        if c == '|' {
            alphabet = 84;
        }

        if c == ':' {
            alphabet = 85;
        }

        if c == ';' {
            alphabet = 86;
        }

        if c == '\'' {
            alphabet = 87;
        }

        if c == '\"' {
            alphabet = 88;
        }

        if c == ',' {
            alphabet = 89;
        }

        if c == '<' {
            alphabet = 90;
        }

        if c == '.' {
            alphabet = 91;
        }

        if c == '>' {
            alphabet = 92;
        }

        if c == '/' {
            alphabet = 93;
        }

        if c == '?' {
            alphabet = 94;
        }

        if c == ' ' {
            result.push_str(" ");
        }

        //displacing with key
        alphabet = alphabet + key;


        //correcting value incase of discrepency
        if alphabet > 94 {
            alphabet = alphabet - 94;
        }

        //converting the numbers back into alphabets
        if alphabet == 1 {
            result.push_str("a");
        }

        if alphabet == 2 {
            result.push_str("b");
        }

        if alphabet == 3 {
            result.push_str("c");
        }

        if alphabet == 4 {
            result.push_str("d");
        }

        if alphabet == 5 {
            result.push_str("e");
        }

        if alphabet == 6 {
            result.push_str("f");
        }

        if alphabet == 7 {
            result.push_str("g");
        }

        if alphabet == 8 {
            result.push_str("h");
        }

        if alphabet == 9 {
            result.push_str("i");
        }

        if alphabet == 10 {
            result.push_str("j");
        }

        if alphabet == 11 {
            result.push_str("k");
        }

        if alphabet == 12 {
            result.push_str("l");
        }

        if alphabet == 13 {
            result.push_str("m");
        }

        if alphabet == 14 {
            result.push_str("n");
        }

        if alphabet == 15 {
            result.push_str("o");
        }

        if alphabet == 16 {
            result.push_str("p");
        }

        if alphabet == 17 {
            result.push_str("q");
        }

        if alphabet == 18 {
            result.push_str("r");
        }

        if alphabet == 19 {
            result.push_str("s");
        }

        if alphabet == 20 {
            result.push_str("t");
        }

        if alphabet == 21 {
            result.push_str("u");
        }

        if alphabet == 22 {
            result.push_str("v");
        }

        if alphabet == 23 {
            result.push_str("w");
        }

        if alphabet == 24 {
            result.push_str("x");
        }

        if alphabet == 25 {
            result.push_str("y");
        }

        if alphabet == 26 {
            result.push_str("z");
        }

        if alphabet == 27 {
            result.push_str("A");
        }

        if alphabet == 28 {
            result.push_str("B");
        }

        if alphabet == 29 {
            result.push_str("C");
        }

        if alphabet == 30 {
            result.push_str("D");
        }

        if alphabet == 31 {
            result.push_str("E");
        }

        if alphabet == 32 {
            result.push_str("F");
        }

        if alphabet == 33 {
            result.push_str("G");
        }

        if alphabet == 34 {
            result.push_str("H");
        }

        if alphabet == 35 {
            result.push_str("I");
        }

        if alphabet == 36 {
            result.push_str("J");
        }

        if alphabet == 37 {
            result.push_str("K");
        }

        if alphabet == 38 {
            result.push_str("L");
        }

        if alphabet == 39 {
            result.push_str("M");
        }

        if alphabet == 40 {
            result.push_str("N");
        }

        if alphabet == 41 {
            result.push_str("O");
        }

        if alphabet == 42 {
            result.push_str("P");
        }

        if alphabet == 43 {
            result.push_str("Q");
        }

        if alphabet == 44 {
            result.push_str("R");
        }

        if alphabet == 45 {
            result.push_str("S");
        }

        if alphabet == 46 {
            result.push_str("T");
        }

        if alphabet == 47 {
            result.push_str("U");
        }

        if alphabet == 48 {
            result.push_str("V");
        }

        if alphabet == 49 {
            result.push_str("W");
        }

        if alphabet == 50 {
            result.push_str("X");
        }

        if alphabet == 51 {
            result.push_str("Y");
        }

        if alphabet == 52 {
            result.push_str("Z");
        }

        if alphabet == 53 {
            result.push_str("1");
        }

        if alphabet == 54 {
            result.push_str("2");
        }

        if alphabet == 55 {
            result.push_str("3");
        }

        if alphabet == 56 {
            result.push_str("4");
        }

        if alphabet == 57 {
            result.push_str("5");
        }

        if alphabet == 58 {
            result.push_str("6");
        }

        if alphabet == 59 {
            result.push_str("7");
        }

        if alphabet == 60 {
            result.push_str("8");
        }

        if alphabet == 61 {
            result.push_str("9");
        }

        if alphabet == 62 {
            result.push_str("0");
        }

        if alphabet == 63 {
            result.push_str("`");
        }

        if alphabet == 64 {
            result.push_str("~");
        }

        if alphabet == 65 {
            result.push_str("!");
        }

        if alphabet == 66 {
            result.push_str("@");
        }

        if alphabet == 67 {
            result.push_str("#");
        }

        if alphabet == 68 {
            result.push_str("$");
        }

        if alphabet == 69 {
            result.push_str("%");
        }

        if alphabet == 70 {
            result.push_str("^");
        }

        if alphabet == 71 {
            result.push_str("&");
        }

        if alphabet == 72 {
            result.push_str("*");
        }

        if alphabet == 73 {
            result.push_str("(");
        }

        if alphabet == 74 {
            result.push_str(")");
        }

        if alphabet == 75 {
            result.push_str("-");
        }

        if alphabet == 76 {
            result.push_str("_");
        }

        if alphabet == 77 {
            result.push_str("{");
        }

        if alphabet == 78 {
            result.push_str("[");
        }

        if alphabet == 79 {
            result.push_str("}");
        }

        if alphabet == 80 {
            result.push_str("]");
        }

        if alphabet == 81 {
            result.push_str("=");
        }

        if alphabet == 82 {
            result.push_str("+");
        }

        if alphabet == 83 {
            result.push_str("\\");
        }

        if alphabet == 84 {
            result.push_str("|");
        }

        if alphabet == 85 {
            result.push_str(":");
        }

        if alphabet == 86 {
            result.push_str(";");
        }

        if alphabet == 87 {
            result.push_str("\'");
        }

        if alphabet == 88 {
            result.push_str("\"");
        }

        if alphabet == 89 {
            result.push_str(",");
        }

        if alphabet == 90 {
            result.push_str("<");
        }

        if alphabet == 91 {
            result.push_str(".");
        }

        if alphabet == 92 {
            result.push_str(">");
        }

        if alphabet == 93 {
            result.push_str("/");
        }

        if alphabet == 94 {
            result.push_str("?");
        }

    }   

    //adding a key to the encryption
    let key_string = key.to_string();

    let key_str: &str = &key_string[..];

    result.push_str(key_str);

    //return the result
    result
}