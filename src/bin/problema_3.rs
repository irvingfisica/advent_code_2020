use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];

    let contents = fs::read_to_string(file)
        .expect("Algo salió mal al leer el archivo.");

    let arboles = toboggan(3,1,&contents);

    println!("{}",arboles);

    let condiciones: Vec<(usize,usize)> = vec![(1,1),(3,1),(5,1),(7,1),(1,2)];

    let arboles: usize = condiciones.iter().map(|(right,down)| toboggan(*right, *down, &contents)).product();

    println!("{:?}",arboles);
    
}

fn toboggan(right: usize, down: usize, bosque: &String) -> usize {

    bosque.lines().enumerate().filter_map(|(ind, cad)|{
        if ind%down == 0 {
            Some((ind, cad))
        } else { 
            None
        }
    }).filter_map(|(ind,cad)|{
        let largo = cad.len();
        match cad.chars().nth(ind*right%largo) {
            Some('#') => Some(ind),
            _ => None,
        }
    }).count()
}

