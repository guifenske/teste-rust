use std::collections::HashMap;

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
    let pessoa = (nome1, 19, true);

    println!("Nome: {}", pessoa.0);
    println!("Idade: {}", pessoa.1);
    println!("Ativa: {}", pessoa.2);

    //HashMap
    let mut cidadeCapital = HashMap::new();
    cidadeCapital.insert("Rio Grande do Sul", "Porto Alegre");
    cidadeCapital.insert("Distrito Federal", "Brasilia");

    println!("Capitadal do Distrito Federal: {}", cidadeCapital["Distrito Federal"]);

}

fn teste(name: &str){
    println!("Hello {}!", name);
}

fn soma(a: i32, b: i32) -> i32 { 
    a + b
}

/*
fn soma(a: i32, b: i32) -> i32 { 
    return a + b;
}
*/
