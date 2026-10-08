use std::{io};

#[derive(Debug, Clone, Copy)]
enum Operation { Add, Sub, Mul, Div, Pow }

#[derive(Debug)]
enum Token { Num(f32), Op(Operation) }

struct Calculation {
    tokens: Vec<Token>,
    result: Option<f32>,
}

fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut cur_num = String::new();

    for c in input.chars() {
        if c.is_ascii_digit() || c == '.' || c == ',' {
            cur_num.push(if c == ',' { '.' } else { c });
        } 
        
        else if let Some(op) = get_operation(c) {
            if cur_num.is_empty() {
                return Err(format!("Operador '{}' sem número antes", c));
            }

            let n: f32 = cur_num.parse().map_err(|_| format!("Número inválido: {}", cur_num))?;
            tokens.push(Token::Num(n));
            tokens.push(Token::Op(op));
            cur_num.clear();
        } 
        
        else {
            return Err(format!("Caractere inválido: '{}'", c));
        }
    }

    if cur_num.is_empty() {
        return Err("O cálculo não pode terminar com operador".to_string());
    }

    let n: f32 = cur_num.parse().map_err(|_| format!("Número inválido: {}", cur_num))?;
    tokens.push(Token::Num(n));

    Ok(tokens)
}

fn precedence(op: Operation) -> u8 {
    match op {
        Operation::Add | Operation::Sub => 1,
        Operation::Mul | Operation::Div => 2,
        Operation::Pow => 3,
    }
}

fn apply(op: Operation, a: f32, b: f32) -> f32 {
    match op {
        Operation::Add => a + b,
        Operation::Sub => a - b,
        Operation::Mul => a * b,
        Operation::Div => a / b,
        Operation::Pow => a.powf(b),
    }
}

fn evaluate(tokens: &[Token]) -> f32 {
    let mut nums: Vec<f32> = Vec::new();
    let mut ops: Vec<Operation> = Vec::new();

    for token in tokens {
        match token {
            Token::Num(n) => nums.push(*n),
            Token::Op(op) => {
                // Resolve operações pendentes que têm prioridade maior ou igual.
                // (^ é associativo à direita: 2^3^2 = 2^9, por isso não resolve no "igual")
                
                while let Some(&top) = ops.last() {
                    let resolve = precedence(top) > precedence(*op)
                        || (precedence(top) == precedence(*op) && !matches!(op, Operation::Pow));
                    if !resolve { break; }
                    ops.pop();
                    let b = nums.pop().unwrap();
                    let a = nums.pop().unwrap();
                    nums.push(apply(top, a, b));
                }
                ops.push(*op);
            }
        }
    }

    // Resolve o que sobrou
    while let Some(op) = ops.pop() {
        let b = nums.pop().unwrap();
        let a = nums.pop().unwrap();
        nums.push(apply(op, a, b));
    }
    nums[0]
}

fn get_operation(c: char) -> Option<Operation> {
    match c {
        '+' => Some(Operation::Add),
        '-' => Some(Operation::Sub),
        '*' => Some(Operation::Mul),
        '/' => Some(Operation::Div),
        '^' => Some(Operation::Pow),
        _ => None,
    }
}

fn filter_input(input: &str) -> String {
    let input = input.trim();
    let input: String = input.replace(" ", "");
    input
}

fn mostrar_historico(calculations: Vec<Calculation>){
    println!("Histórico:");
    for (i, calc) in calculations.iter().enumerate() {
        println!("{}: {:?} = {:?}", i + 1, calc.tokens, calc.result);
    }
}

pub fn init() {
    let mut should_continue = String::new();
    let mut calculations: Vec<Calculation> = Vec::new();
    println!("Calculadora iniciou!");

    loop {
        let mut input = String::new();

        println!("Informe o calculo, utilize essas operações (+, -, *, /):");
        io::stdin().read_line(&mut input).unwrap();

        let input = filter_input(&input);
        println!("Filtro: {}", input);

        match tokenize(&input) {
            Ok(tokens) => {
                let result = evaluate(&tokens);
                println!("= {}", result);
                
                calculations.push(Calculation {
                    tokens,
                    result: Some(result),
                });
            },
            Err(e) => println!("Erro: {}", e),
        }

        println!("Continuar?");
        io::stdin().read_line(&mut should_continue).unwrap();

        if should_continue.trim() != "s" {
            break;
        }
    }

    mostrar_historico(calculations);
}
