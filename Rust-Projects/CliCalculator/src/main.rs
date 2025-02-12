use std::io;
use std::process;
fn main() {
    inp();
}

fn inp() {
    let input_date = citire_string(String::from("Enter the expression: "));
    let mut input_date = remove_spaces(input_date);

    loop {
        let value = verif(input_date.clone());
        if value {
            let mut vec: Vec<String> = decipher(input_date);
            let result = calcul_complex(&mut vec);
            println!("{}", result);
            break;
        } else {
            input_date = citire_string(String::from("Enter the expression: "));
            input_date = remove_spaces(input_date);
        }
    }
}

fn citire_string(_afis: String) -> String {
    println!("{}", _afis);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input = input.trim().to_string();
    input
}

fn remove_spaces(input: String) -> String {
    input.replace(" ", "")
}

fn verif(input: String) -> bool {
    let mut ok = true;
    if !paranteze(input.clone()) {
        println!("Invalid parenthesis layout");
        ok = false;
    }
    if !invalid_d(input) {
        println!("Invalid input");
        ok = false;
    }
    ok
}

fn paranteze(input: String) -> bool {
    let mut cnt_d = 0;
    let mut cnt_i = 0;
    let mut cnt_sq_d = 0;
    let mut cnt_sq_i = 0;
    let mut cnt_curly_d = 0;
    let mut cnt_curly_i = 0;
    let mut ok = true;
    for c in input.chars() {
        if c == '(' {
            cnt_d += 1;
        }
        if c == ')' {
            cnt_i += 1;
        }
        if c == '[' {
            cnt_sq_d += 1;
        }
        if c == ']' {
            cnt_sq_i += 1;
        }
        if c == '{' {
            cnt_curly_d += 1;
        }
        if c == '}' {
            cnt_curly_i += 1;
        }
        if cnt_i > cnt_d || cnt_sq_i > cnt_sq_d || cnt_curly_i > cnt_curly_d {
            ok = false;
            break;
        }
    }
    if cnt_d != cnt_i || cnt_sq_d != cnt_sq_i || cnt_curly_d != cnt_curly_i {
        ok = false;
    }

    
    ok
}

fn invalid_d(input: String) -> bool {
    let mut ok = true;
    for c in input.chars() {
        if !c.is_ascii_digit() {
            if c != '+' && c != '-' && c != '*' && c != '/' && c != '(' && c != ')' && c != '.' && c != '[' && c != ']' && c != '{' && c != '}' {
                ok = false;
            }
        }
    }
    ok
}

fn decipher(input: String) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();
    let mut current_number = String::new();
    let mut has_decimal = false;

    for c in input.chars() {
        if c.is_ascii_digit() {
            current_number.push(c);
        } else if c == '.' && !has_decimal {
            current_number.push(c);
            has_decimal = true;
        } else {
            if !current_number.is_empty() {
                vec.push(current_number.clone());
                current_number.clear();
                has_decimal = false;
            }
            vec.push(c.to_string());
        }
    }

    if !current_number.is_empty() {
        vec.push(current_number);
    }

    vec
}

fn calcul_complex(vec: &mut Vec<String>) -> f64 {
    let mut i = 0;
    while i < vec.len() {
        if vec[i] == ")" || vec[i] == "]" || vec[i] == "}" {
            let mut j = i as isize - 1;
            while j >= 0 {
                if vec[j as usize] == "(" && vec[i] == ")" ||
                   vec[j as usize] == "[" && vec[i] == "]" ||
                   vec[j as usize] == "{" && vec[i] == "}" {
                    let mut new_vec: Vec<String> = vec[j as usize + 1..i].to_vec();
                    let rezultat = calcul(&mut new_vec);
                    vec.drain(j as usize..=i);
                    vec.insert(j as usize, rezultat.to_string());
                    i = j as usize;
                    break;
                }
                j -= 1;
            }
        }
        i += 1;
    }
    let cpy = vec;
    calcul(cpy)
}

fn calcul(vec: &mut Vec<String>) -> f64 {
    calcul_sup(vec);
    calcul_inf(vec);
    vec[0].parse::<f64>().unwrap()
}

fn calcul_sup(vec: &mut Vec<String>) -> f64 {
    let mut i = 0;
    while i < vec.len() {
        if vec[i] == "*" {
            let a = vec[i - 1].parse::<f64>().unwrap();
            let b = vec[i + 1].parse::<f64>().unwrap();
            let rezultat = a * b;
            vec.drain(i - 1..=i + 1);
            vec.insert(i - 1, rezultat.to_string());
            i -= 1;
        }
        if vec[i] == "/" {
            let a = vec[i - 1].parse::<f64>().unwrap();
            let b = vec[i + 1].parse::<f64>().unwrap();
            let rezultat = if b == 0.0 {
                println!("Division by zero is not allowed");
                inp();
                process::exit(1);
            } else {
                a / b
            };
            vec.drain(i - 1..=i + 1);
            vec.insert(i - 1, rezultat.to_string());
            i -= 1;
        }
        i += 1;
    }
    vec[0].parse::<f64>().unwrap()
}

fn calcul_inf(vec: &mut Vec<String>) -> f64 {
    let mut i = 0;
    while i < vec.len() {
        if vec[i] == "+" {
            let a = vec[i - 1].parse::<f64>().unwrap();
            let b = vec[i + 1].parse::<f64>().unwrap();
            let rezultat = a + b;
            vec.drain(i - 1..=i + 1);
            vec.insert(i - 1, rezultat.to_string());
            i -= 1;
        }
        if vec[i] == "-" {
            let a = vec[i - 1].parse::<f64>().unwrap();
            let b = vec[i + 1].parse::<f64>().unwrap();
            let rezultat = a - b;
            vec.drain(i - 1..=i + 1);
            vec.insert(i - 1, rezultat.to_string());
            i -= 1;
        }
        i += 1;
    }
    vec[0].parse::<f64>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_spaces() {
        let input = String::from("1 +2");
        assert_eq!(remove_spaces(input),String::from("1+2"));
    }

    #[test]
    fn test_paranteze(){
        let input = String::from("(()");
        assert_eq!(paranteze(input),false);
    }

    #[test]
    fn test_invalid_d(){
        let input = String::from("1+a");
        assert_eq!(invalid_d(input),false);
    }

    #[test]
    fn test_decipher() {
        let input = String::from("3.14+2*(1-5)");
        let expected = vec!["3.14", "+", "2", "*", "(", "1", "-", "5", ")"];
        assert_eq!(decipher(input), expected);
    }

    #[test]
    fn test_calcul_inf() {
        let mut input = decipher(String::from("1+2+3-1"));
        assert_eq!(calcul_inf(&mut input), 5.0);
    }

    #[test]
    fn test_calcul_sup() {
        let mut input = decipher(String::from("2*3/5"));
        assert_eq!(calcul_sup(&mut input), 1.2);
    }

    #[test]
    fn test_calcul() {
        let mut input = decipher(String::from("1-2*35+1"));
        assert_eq!(calcul(&mut input), -68.0);
    }

    #[test]
    fn test_calcul_complex() {
        let mut input = decipher(String::from("(22+44)+(2-(2*4.5))+1"));
        assert_eq!(calcul_complex(&mut input), 60.0);
    }


}
