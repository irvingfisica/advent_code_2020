use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];

    let contents = fs::read_to_string(file)
        .expect("Algo salió mal al leer el archivo.");

    let validos = contents.lines().filter_map(|linea| {
        let vector: Vec<&str> = linea.split(|c| c == '-' || c == ':' || c == ' ').collect();  
        let min: usize = vector[0].parse().unwrap();
        let max: usize = vector[1].parse().unwrap();
        let cbu: Vec<char> = vector[2].chars().collect();
        let cuenta = vector[4].chars().filter(|ele| ele == &cbu[0]).count();
        if cuenta >= min && cuenta <= max {
            Some(cuenta)
        } else {
            None
        }
    });

    println!("{}",validos.count());

    let validos2 = contents.lines().filter_map(|linea| {
        let vector: Vec<&str> = linea.split(|c| c == '-' || c == ':' || c == ' ').collect();  
        let first: usize = vector[0].parse().unwrap();
        let second: usize = vector[1].parse().unwrap();
        let cbu: Vec<char> = vector[2].chars().collect();
        let cadena: Vec<char> = vector[4].chars().collect();
        if (cadena[first - 1] == cbu[0]) ^ (cadena[second - 1] == cbu[0]) {
          Some(cbu)
        } else {
            None
        }});

        println!("{}",validos2.count());

}

