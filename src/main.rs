mod calc_utils;

fn main() {
    calc_utils::init();
}

/*
fn teste(name: &str) {
    println!("Hello {}!", name);
}



fn get_pessoa() -> (String, i32, bool) {
    (String::from("Guilherme"), 19, true)
}
*/

//unsigned int nao permite valores negativos
//o signed int permite valores negativos, e o bit mais importante
//  (o primeiro), sempre sera 1 quando o nnumero for negativo.
//  ex: em i4 0111 = 7, 1000 = -8, 1001 = -7
