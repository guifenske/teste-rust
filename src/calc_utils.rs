use std::io;

enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

struct Calculation {
    op: Operation,
    a: f32,
    b: f32,
}

fn get_calculation(op: &str, a: &str, b: &str) -> Calculation {
    let op = op.trim();
    let a = a.trim().parse().unwrap();
    let b = b.trim().parse().unwrap();
    let op = match op {
        "+" => Operation::Add,
        "-" => Operation::Sub,
        "*" => Operation::Mul,
        "/" => Operation::Div,
        "^" => Operation::Pow,
        _ => panic!("Operação inválida"),
    };
    Calculation { op, a, b }
}

fn calculate(calc: &Calculation) -> f32 {
    match calc.op {
        Operation::Add => calc.a + calc.b,
        Operation::Sub => calc.a - calc.b,
        Operation::Mul => calc.a * calc.b,
        Operation::Div => calc.a / calc.b,
        Operation::Pow => calc.a.powf(calc.b),
    }
}

pub fn init() {
    let mut result: f32 = 0.0;
    let mut should_continue = String::new();
    println!("Calculadora iniciou!");

    loop {
        let mut a = String::new();
        let mut b = String::new();
        let mut op = String::new();
        let calc: Calculation;
        println!("Informe a operação (+, -, *, /):");
        io::stdin().read_line(&mut op).unwrap();

        if should_continue.trim() == "s" {
            println!("Informe o número:");
            io::stdin().read_line(&mut a).unwrap();

            calc = get_calculation(&op, &result.to_string(), &a);
            result = calculate(&calc);
        } else {
            println!("Informe o primeiro número:");
            io::stdin().read_line(&mut a).unwrap();

            println!("Informe o segundo número:");
            io::stdin().read_line(&mut b).unwrap();

            calc = get_calculation(&op, &a, &b);
            result = calculate(&calc);
        }

        should_continue = String::new();

        println!("Resultado da conta: {}", result);
        println!("Deseja realizar outra conta?");

        io::stdin().read_line(&mut should_continue).unwrap();

        if should_continue.trim() != "s" {
            break;
        }
    }
}
