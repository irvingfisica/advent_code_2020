use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];

    let contents = fs::read_to_string(file)
        .expect("Algo salió mal al leer el archivo.");

    let salida = contents.lines().map(|line| id(line)).max();

    println!("{}",salida.unwrap());

    let mut ids: Vec<usize> = contents.lines().map(|line| id(line)).collect();
    ids.sort();

    let hueco = ids[1..].iter().zip(ids[..ids.len()-1].iter()).find(|(a, b)| **a - (**b + 1) != 0).unwrap();
    let mio = hueco.1 + 1;

    println!("{:?},asiento: {}",hueco,mio);

}

fn filas(letra: char, resto: &[usize]) -> &[usize] {
    let len = resto.len()/2;

    match letra {
        'F' => &resto[..len],
        'B' => &resto[len..],
        _ => &[]
    }
}

fn columnas(letra: char, resto: &[usize]) -> &[usize] {
    let len = resto.len()/2;

    match letra {
        'L' => &resto[..len],
        'R' => &resto[len..],
        _ => &[]
    }
}

fn id(cadena: &str) -> usize {

    let fils: Vec<usize> = (0..128).collect();
    let cols: Vec<usize> = (0..8).collect();

    let fila = cadena[..7].chars().fold(&fils[..],|acc, letra| filas(letra, acc));
    let columna = cadena[7..].chars().fold(&cols[..],|acc, letra| columnas(letra, acc));

    (fila[0] * 8) + columna[0] 

}