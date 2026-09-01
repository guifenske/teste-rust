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
    result: Option<f32>,
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

    Calculation {
        op,
        a,
        b,
        result: None,
    }
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
    let mut calculations: Vec<Calculation> = Vec::new();
    println!("Calculadora iniciou!");

    loop {
        let mut a = String::new();
        let mut b = String::new();
        let mut op = String::new();
        let mut calc: Calculation;
        println!("Informe a operação (+, -, *, /):");
        io::stdin().read_line(&mut op).unwrap();

        if should_continue.trim() == "s" {
            println!("Informe o número:");
            io::stdin().read_line(&mut a).unwrap();

            calc = get_calculation(&op, &result.to_string(), &a);
            result = calculate(&calc);
            calc.result = Some(result);
            calculations.push(calc);
        } else {
            println!("Informe o primeiro número:");
            io::stdin().read_line(&mut a).unwrap();

            println!("Informe o segundo número:");
            io::stdin().read_line(&mut b).unwrap();

            calc = get_calculation(&op, &a, &b);
            result = calculate(&calc);
            calc.result = Some(result);
            calculations.push(calc);
        }

        should_continue = String::new();

        println!("Calculo Atual: ");
        let mut index = 0;
        loop {
            let calc: Option<&Calculation> = calculations.get(index);
            let op: &str;

            if calc.is_none() {
                break;
            }

            match calc.unwrap().op {
                Operation::Add => op = "+",
                Operation::Sub => op = "-",
                Operation::Mul => op = "*",
                Operation::Div => op = "/",
                Operation::Pow => op = "^",
            }

            if index == 0 {
                print!("{} ", calc.unwrap().a);
                print!("{} ", op);
                print!("{} ", calc.unwrap().b);
            } else {
                print!("{} ", op);
                print!("{} ", calc.unwrap().b);
            }

            if index == calculations.len() - 1 {
                break;
            }

            index += 1;
        }

        print!("= {}", calculations.get(index).unwrap().result.unwrap());
        println!();
        println!("Deseja realizar outra conta?");

        io::stdin().read_line(&mut should_continue).unwrap();

        if should_continue.trim() != "s" {
            break;
        }
    }
}
