use std::io::{self, Write};

fn eval(expression: &str, vars: &mut Vec<(String, f64)>) -> Option<f64> {
 
    let expression = expression.replace(" ", ""); //whitespace için
    let mut pos = 0;
    let result = parse_expr(&expression, vars, &mut pos);
    if pos < expression.len() {
        println!("hata geçersiz karakter: '{}'", expression.chars().nth(pos).unwrap());
        None
    } else {
        result
    }
}

fn parse_expr(expression: &str, vars: &mut Vec<(String, f64)>, pos: &mut usize) -> Option<f64> {
    let mut left = parse_term(expression, vars, pos)?;
    while *pos < expression.len() {
        let ch = expression.chars().nth(*pos)?;
        if ch == '+' {
            *pos += 1;
            let right = parse_term(expression, vars, pos)?;
            left += right;
        } else if ch == '-' {
            *pos += 1;
            let right = parse_term(expression, vars, pos)?;
            left -= right;
        } else {
            break;
        }
    }
    Some(left)
}

fn parse_term(expression: &str, vars: &mut Vec<(String, f64)>, pos: &mut usize) -> Option<f64> {
    let mut left = parse_factor(expression, vars, pos)?;
    while *pos < expression.len() {
        let ch = expression.chars().nth(*pos)?;
        if ch == '*' {
            *pos += 1;
            let right = parse_factor(expression, vars, pos)?;
            left *= right;
        } else if ch == '/' {
            *pos += 1;
            let right = parse_factor(expression, vars, pos)?;
            if right == 0.0 {
                println!("Hata: Sıfıra bölme");
                return None;
            }
            left /= right;
        } else {
            break;
        }
    }
    Some(left)
}

fn parse_factor(expression: &str, vars: &mut Vec<(String, f64)>, pos: &mut usize) -> Option<f64> {
    if *pos >= expression.len() {
        return None;
    }
    let ch = expression.chars().nth(*pos)?;
    if ch.is_digit(10) || ch == '.' {
        parse_number(expression, pos)
    } else if ch.is_alphabetic() {
        // Değişken veya atama
        let start = *pos;
        while *pos < expression.len() && expression.chars().nth(*pos)?.is_alphabetic() {
            *pos += 1;
        }
        let var_name = &expression[start..*pos];
        if *pos < expression.len() && expression.chars().nth(*pos) == Some('=') {
            // atama işlemi
            *pos += 1; //= karakterini atla
            let value = parse_expr(expression, vars, pos)?;
            update_variable(var_name, value, vars);
            Some(value)
        } else {
            // değişkenin değeri
            if let Some(val) = get_variable(var_name, vars) {
                Some(val)
            } else {
                println!("geçersiz: {}", var_name);
                None
            }
        }
    } else if ch == '(' {
        // parantezli ifade
        *pos += 1; // '(' atla
        let value = parse_expr(expression, vars, pos)?;
        if *pos < expression.len() && expression.chars().nth(*pos) == Some(')') {
            *pos += 1; // ')' atla
            Some(value)
        } else {
            println!("hata: ')' ");
            None
        }
    } else if ch == '-' {
        *pos += 1;
        let value = parse_factor(expression, vars, pos)?;
        Some(-value)
    } else {
        println!("geçersiz karakter: {}", ch);
        None
    }
}

fn parse_number(expression: &str, pos: &mut usize) -> Option<f64> {
    let start = *pos;
    while *pos < expression.len() {
        let ch = expression.chars().nth(*pos)?;
        if ch.is_digit(10) || ch == '.' {
            *pos += 1;
        } else {
            break;
        }
    }
    let number_str = &expression[start..*pos];
    number_str.parse::<f64>().ok()
}

fn update_variable(name: &str, value: f64, vars: &mut Vec<(String, f64)>) {
    if let Some(var) = vars.iter_mut().find(|(var_name, _)| var_name == name) {
        var.1 = value;
    } else {
        vars.push((name.to_string(), value));
    }
}

fn get_variable(name: &str, vars: &Vec<(String, f64)>) -> Option<f64> {
    vars.iter().find(|(var_name, _)| var_name == name).map(|(_, val)| *val)
}

fn main() {
    let mut vars: Vec<(String, f64)> = Vec::new();
    loop {
        print!("bir ifade girin: (çıkmak için 'exit')  ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "exit" {
            break;
        }
        match eval(input.trim(), &mut vars) {
            Some(result) => println!("sonuç: {}", result),
            None => println!("hata"),
        }
    }
}
