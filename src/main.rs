use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];

    let contents = fs::read_to_string(file)
        .expect("Algo salió mal al leer el archivo.");

    let numbers: Vec<i32> = contents.lines().filter_map(|ele| ele.parse().ok()).collect();

    let mut pareja1: Option<(i32,i32)> = None;

    'outer1: for i in 0..numbers.len() {
        for j in i..numbers.len() {
                if numbers[i] + numbers[j] == 2020 {
                    pareja1 = Some((numbers[i], numbers[j]));
                    break 'outer1
                }
        
        }
    }

    match pareja1 {
        Some(tup) => println!("la primera respuesta es: {}", tup.0*tup.1),
        _ => {}
    }

    let mut pareja2: Option<(i32,i32,i32)> = None;

    'outer2: for i in 0..numbers.len() {
        for j in i..numbers.len() {
            for k in j..numbers.len() {
                if numbers[i] + numbers[j] + numbers[k]== 2020 {
                    pareja2 = Some((numbers[i], numbers[j], numbers[k]));
                    break 'outer2
                }
            }
        }
    }

    match pareja2 {
        Some(tup) => println!("la segunda respuesta es: {}", tup.0*tup.1*tup.2),
        _ => {}
    }
    
}

