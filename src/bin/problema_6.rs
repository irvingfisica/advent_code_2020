use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];

    let contents = fs::read_to_string(file)
        .expect("Algo salió mal al leer el archivo.");


    let respuestas: Vec<&str> = contents.trim().split("\n\n").collect();

    let mut suma = 0;
    let mut suma2 = 0;
    for respuesta in respuestas.iter() {
        println!("{:?}", respuesta);
        let mut valcars: Vec<char> = respuesta.replace("\n","").chars().collect();
        valcars.sort();
        valcars.dedup();
        suma = suma + valcars.len();

        let cadenas: Vec<&str> = respuesta.split("\n").collect();
        println!("{:?}", cadenas);

        for car in valcars.into_iter() {
            if cadenas.iter().all(|cad| cad.contains(car)) {
                suma2 = suma2 + 1;
            }
        }
    }

    println!("{}",suma);
    println!("{}",suma2);

}
