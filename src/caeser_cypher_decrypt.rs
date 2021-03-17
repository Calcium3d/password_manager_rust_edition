pub fn caeser_decrypt(encrypted: &str) -> String{

	let mut encrypted_text: Vec<char> = encrypted.chars().collect();

    let mut index_2: usize = encrypted_text.len() - 1;

    let mut key: u32 = 0;
    if encrypted_text[index_2] == '1' || encrypted_text[index_2] == '2' || encrypted_text[index_2] == '3' || encrypted_text[index_2] == '4' || encrypted_text[index_2] == '5' || encrypted_text[index_2] == '6' || encrypted_text[index_2] == '7' || encrypted_text[index_2] == '8' || encrypted_text[index_2] == '9' {
        key = (encrypted_text[index_2] as u32 * 10) + (encrypted_text[index_2 + 1] as u32); 
    }
    else {
        key = encrypted_text[index_2 + 1]as u32;
    }

    let mut alphabet: u32 = 0;

	let mut result: String = "".to_owned();

	for c in encrypted_text {
		if c == 'a' {
            alphabet = 1;
        }

        else if c == 'b' {
            alphabet = 2;
        }

        else if c == 'c' {
            alphabet = 3;
        }

        else if c == 'd' {
            alphabet = 4;
        }

        else if c == 'e' {
            alphabet = 5;
        }

        else if c == 'f' {
            alphabet = 6;
        }

        else if c == 'g' {
            alphabet = 7;
        }

        else if c == 'h' {
            alphabet = 8;
        }

        else if c == 'i' {
            alphabet = 9;
        }

        else if c == 'j' {
            alphabet = 10;
        }

        else if c == 'k' {
            alphabet = 11;
        }

        else if c == 'l' {
            alphabet = 12;
        }

        else if c == 'm' {
            alphabet = 13;
        }

        else if c == 'n' {
            alphabet = 14;
        }

        else if c == 'o' {
            alphabet = 15;
        }

        else if c == 'p' {
            alphabet = 16;
        }

        else if c == 'q' {
            alphabet = 17;
        }

        else if c == 'r' {
            alphabet = 18;
        }

        else if c == 's' {
            alphabet = 19;
        }

        else if c == 't' {
            alphabet = 20;
        }

        else if c == 'u' {
            alphabet = 21;
        }

        else if c == 'v' {
            alphabet = 22;
        }

        else if c == 'w' {
            alphabet = 23;
        }

        else if c == 'x' {
            alphabet = 24;
        }
        
        else if c == 'y' {
            alphabet = 25;
        }

        else if c == 'z' {
            alphabet = 26;
        }

        else if c == 'A' {
            alphabet = 27;
        }

        else if c == 'B' {
            alphabet = 28;
        }

        else if c == 'C' {
            alphabet = 29;
        }

        else if c == 'D' {
            alphabet = 30;
        }

        else if c == 'E' {
            alphabet = 31;
        }

        else if c == 'F' {
            alphabet = 32;
        }

        else if c == 'G' {
            alphabet = 33;
        }

        else if c == 'H' {
            alphabet = 34;
        }

        else if c == 'I' {
            alphabet = 35;
        }

        else if c == 'J' {
            alphabet = 36;
        }

        else if c == 'K' {
            alphabet = 37;
        }

        else if c == 'L' {
            alphabet = 38;
        }

        else if c == 'M' {
            alphabet = 39;
        }

        else if c == 'N' {
            alphabet = 40;
        }

        else if c == 'O' {
            alphabet = 41;
        }

        else if c == 'P' {
            alphabet = 42;
        }

        else if c == 'Q' {
            alphabet = 43;
        }

        else if c == 'R' {
            alphabet = 44;
        }

        else if c == 'S' {
            alphabet = 45;
        }

        else if c == 'T' {
            alphabet = 46;
        }

        else if c == 'U' {
            alphabet = 47;
        }

        else if c == 'V' {
            alphabet = 48;
        }

        else if c == 'W' {
            alphabet = 49;
        }

        else if c == 'X' {
            alphabet = 50;
        }
        
        else if c == 'Y' {
            alphabet = 51;
        }

        else if c == 'Z' {
            alphabet = 52;
        }

        else if c == '1' {
            alphabet = 53;
        }

        else if c == '2' {
            alphabet = 54;
        }

        else if c == '3' {
            alphabet = 55;
        }

        else if c == '4' {
            alphabet = 56;
        }

        else if c == '5' {
            alphabet = 57;
        }

        else if c == '6' {
            alphabet = 58;
        }

        else if c == '7' {
            alphabet = 59;
        }

        else if c == '8' {
            alphabet = 60;
        }

        else if c == '9' {
            alphabet = 61;
        }

        else if c == '0' {
            alphabet = 62;
        }

        else if c == '`' {
            alphabet = 63;
        }

        else if c == '~' {
            alphabet = 64;
        }

        else  if c == '!' {
            alphabet = 65;
        }

        else if c == '@' {
            alphabet = 66;
        }

        else if c == '#' {
            alphabet = 67;
        }

        else if c == '$' {
            alphabet = 68;
        }

        else if c == '%' {
            alphabet = 69;
        }

        else if c == '^' {
            alphabet = 70;
        }

        else if c == '&' {
            alphabet = 71;
        }

        else if c == '*' {
            alphabet = 72;
        }

        else if c == '(' {
            alphabet = 73;
        }

        else if c == ')' {
            alphabet = 74;
        }

        else if c == '-' {
            alphabet = 75;
        }

        else if c == '_' {
            alphabet = 76;
        }

        else if c == '{' {
            alphabet = 77;
        }

        else if c == '[' {
            alphabet = 78;
        }

        else if c == '}' {
            alphabet = 79;
        }

        else if c == ']' {
            alphabet = 80;
        }

        else if c == '=' {
            alphabet = 81;
        }

        else if c == '+' {
            alphabet = 82;
        }

        else if c == '\\' {
            alphabet = 83;
        }

        else if c == '|' {
            alphabet = 84;
        }

        else if c == ':' {
            alphabet = 85;
        }

        else if c == ';' {
            alphabet = 86;
        }

        else if c == '\'' {
            alphabet = 87;
        }

        else if c == '\"' {
            alphabet = 88;
        }

        else if c == ',' {
            alphabet = 89;
        }

        else if c == '<' {
            alphabet = 90;
        }

        else if c == '.' {
            alphabet = 91;
        }

        else if c == '>' {
            alphabet = 92;
        }

        else if c == '/' {
            alphabet = 93;
        }

        else if c == '?' {
            alphabet = 94;
        }

        else if c == ' ' {
            result.push_str(" ");
        }

        alphabet = alphabet - key;

        if alphabet < 0 {
            alphabet = alphabet + 94;
        }

        else if alphabet == 1 {
            result.push_str("a");
        }

        else if alphabet == 2 {
            result.push_str("b");
        }

        else if alphabet == 3 {
            result.push_str("c");
        }

        else if alphabet == 4 {
            result.push_str("d");
        }

        else if alphabet == 5 {
            result.push_str("e");
        }

        else if alphabet == 6 {
            result.push_str("f");
        }

        else if alphabet == 7 {
            result.push_str("g");
        }

        else if alphabet == 8 {
            result.push_str("h");
        }

        else if alphabet == 9 {
            result.push_str("i");
        }

        else if alphabet == 10 {
            result.push_str("j");
        }

        else if alphabet == 11 {
            result.push_str("k");
        }

        else if alphabet == 12 {
            result.push_str("l");
        }

        else if alphabet == 13 {
            result.push_str("m");
        }

        else if alphabet == 14 {
            result.push_str("n");
        }

        else if alphabet == 15 {
            result.push_str("o");
        }

        else if alphabet == 16 {
            result.push_str("p");
        }

        else if alphabet == 17 {
            result.push_str("q");
        }

        else if alphabet == 18 {
            result.push_str("r");
        }

        else if alphabet == 19 {
            result.push_str("s");
        }

        else if alphabet == 20 {
            result.push_str("t");
        }

        else if alphabet == 21 {
            result.push_str("u");
        }

        else if alphabet == 22 {
            result.push_str("v");
        }

        else if alphabet == 23 {
            result.push_str("w");
        }

        else if alphabet == 24 {
            result.push_str("x");
        }

        else if alphabet == 25 {
            result.push_str("y");
        }

        else if alphabet == 26 {
            result.push_str("z");
        }

        else if alphabet == 27 {
            result.push_str("A");
        }

        else if alphabet == 28 {
            result.push_str("B");
        }

        else if alphabet == 29 {
            result.push_str("C");
        }

        else if alphabet == 30 {
            result.push_str("D");
        }

        else if alphabet == 31 {
            result.push_str("E");
        }

        else if alphabet == 32 {
            result.push_str("F");
        }

        else if alphabet == 33 {
            result.push_str("G");
        }

        else if alphabet == 34 {
            result.push_str("H");
        }

        else if alphabet == 35 {
            result.push_str("I");
        }

        else if alphabet == 36 {
            result.push_str("J");
        }

        else if alphabet == 37 {
            result.push_str("K");
        }

        else if alphabet == 38 {
            result.push_str("L");
        }

        else if alphabet == 39 {
            result.push_str("M");
        }

        else if alphabet == 40 {
            result.push_str("N");
        }

        else if alphabet == 41 {
            result.push_str("O");
        }

        else if alphabet == 42 {
            result.push_str("P");
        }

        else if alphabet == 43 {
            result.push_str("Q");
        }

        else if alphabet == 44 {
            result.push_str("R");
        }

        else if alphabet == 45 {
            result.push_str("S");
        }

        else if alphabet == 46 {
            result.push_str("T");
        }

        else if alphabet == 47 {
            result.push_str("U");
        }

        else if alphabet == 48 {
            result.push_str("V");
        }

        else if alphabet == 49 {
            result.push_str("W");
        }

        else if alphabet == 50 {
            result.push_str("X");
        }

        else if alphabet == 51 {
            result.push_str("Y");
        }

        else if alphabet == 52 {
            result.push_str("Z");
        }

        else if alphabet == 53 {
            result.push_str("1");
        }

        else if alphabet == 54 {
            result.push_str("2");
        }

        else if alphabet == 55 {
            result.push_str("3");
        }

        else if alphabet == 56 {
            result.push_str("4");
        }

        else if alphabet == 57 {
            result.push_str("5");
        }

        else if alphabet == 58 {
            result.push_str("6");
        }

        else if alphabet == 59 {
            result.push_str("7");
        }

        else if alphabet == 60 {
            result.push_str("8");
        }

        else if alphabet == 61 {
            result.push_str("9");
        }

        else if alphabet == 62 {
            result.push_str("0");
        }

        else if alphabet == 63 {
            result.push_str("`");
        }

        else if alphabet == 64 {
            result.push_str("~");
        }

        else if alphabet == 65 {
            result.push_str("!");
        }

        else if alphabet == 66 {
            result.push_str("@");
        }

        else if alphabet == 67 {
            result.push_str("#");
        }

        else if alphabet == 68 {
            result.push_str("$");
        }

        else if alphabet == 69 {
            result.push_str("%");
        }

        else if alphabet == 70 {
            result.push_str("^");
        }

        else if alphabet == 71 {
            result.push_str("&");
        }

        else if alphabet == 72 {
            result.push_str("*");
        }

        else if alphabet == 73 {
            result.push_str("(");
        }

        else if alphabet == 74 {
            result.push_str(")");
        }

        else if alphabet == 75 {
            result.push_str("-");
        }

        else if alphabet == 76 {
            result.push_str("_");
        }

        else if alphabet == 77 {
            result.push_str("{");
        }

        else if alphabet == 78 {
            result.push_str("[");
        }

        else if alphabet == 79 {
            result.push_str("}");
        }

        else if alphabet == 80 {
            result.push_str("]");
        }

        else if alphabet == 81 {
            result.push_str("=");
        }

        else if alphabet == 82 {
            result.push_str("+");
        }

        else if alphabet == 83 {
            result.push_str("\\");
        }

        else if alphabet == 84 {
            result.push_str("|");
        }

        else if alphabet == 85 {
            result.push_str(":");
        }

        else if alphabet == 86 {
            result.push_str(";");
        }

        else if alphabet == 87 {
            result.push_str("\'");
        }

        else if alphabet == 88 {
            result.push_str("\"");
        }

        else if alphabet == 89 {
            result.push_str(",");
        }

        else if alphabet == 90 {
            result.push_str("<");
        }

        else if alphabet == 91 {
            result.push_str(".");
        }

        else if alphabet == 92 {
            result.push_str(">");
        }

        else if alphabet == 93 {
            result.push_str("/");
        }

        else if alphabet == 94 {
            result.push_str("?");
        }

    }   

    let key_string = key.to_string();

    let key_str: &str = &key_string[..];

    result.push_str(key_str);

    result
	
    

}