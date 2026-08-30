use std::collections::HashMap;

enum LoginStatus {
    Success(String),
    //Error(String),
}

fn main() {
    let mut nome = String::from("Guilherme");
    let nome1 = &mut nome;

    nome1.push_str(" Melo");

    println!("{}", nome1);

    let frutas = ["Pera", "Banana", "Laranja"];

    println!("Ultima fruta: {}", frutas[2]);

    let mut frutas2 = vec!["Pera", "Uva"];
    frutas2.push("Banana");

    println!("Ultima fruta vetor: {}", frutas2[frutas2.len() - 1]);

    //Tuple
    let pessoa = get_pessoa();

    println!("Nome: {}", pessoa.0);
    println!("Idade: {}", pessoa.1);
    println!("Ativa: {}", pessoa.2);

    //HashMap
    let mut cidade_capital = HashMap::new();
    cidade_capital.insert("Rio Grande do Sul", "Porto Alegre");
    cidade_capital.insert("Distrito Federal", "Brasilia");

    if let Some(cidade) = cidade_capital.get("Distrito Federal") {
        println!("Capitadal do Distrito Federal: {}", cidade);
    }

    let frutas3 = ["Pera", "Abacaxi", "Laranja"];

    for fruta in frutas3 {
        println!("Fruta: {}", fruta);

        println!("Teste")
    }

    //ou

    println!("{:?}", frutas3);

    frutas2.pop(); //frutas2.remove(index);

    for fruta in &frutas2 {
        println!("Eu gosto de: {}", fruta);
    }

    println!("{:?}", frutas2);

    for (estado, cidade) in &cidade_capital {
        println!("Capital do {} e {}", estado, cidade);
    }

    print!("{:?}", cidade_capital);

    struct Cidadao {
        nome: String,
        idade: u32, //sempre sera positivo
    }

    let mut guilherme = Cidadao {
        nome: String::from("Guilherme Melo"),
        idade: 18,
    };

    guilherme.idade = 19;

    println!("Nome: {}", guilherme.nome);
    println!("Idade: {}", guilherme.idade);

    //enum

    let result1 = LoginStatus::Success(String::from("Welcome, {name}!"));

    match result1 {
        LoginStatus::Success(message) => println!("Success: {}", message),
        //LoginStatus::Error(message) => println!("Error: {}", message),
    }
}

/*
fn teste(name: &str) {
    println!("Hello {}!", name);
}

fn soma(a: i32, b: i32) -> i32 {
    a + b
}

fn soma(a: i32, b: i32) -> i32 {
    return a + b;
}
*/

fn get_pessoa() -> (String, i32, bool) {
    (String::from("Guilherme"), 19, true)
}

//unsigned int nao permite valores negativos
//o signed int permite valores negativos, e o bit mais importante
//  (o primeiro), sempre sera 1 quando o nnumero for negativo.
//  ex: em i4 0111 = 7, 1000 = -8, 1001 = -7
