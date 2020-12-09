use std::env;
use std::fs;
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];

    let contents = fs::read_to_string(file)
        .expect("Algo salió mal al leer el archivo.");

    let numeros: Vec<i64> = contents.trim().lines().map(|ele| {ele.parse().unwrap()}).collect();

    let pre_len = 25;

    let salida = (pre_len..numeros.len()).find_map(|ic|{
        let preambulo = &numeros[(ic - pre_len)..ic];
        let numtest = &numeros[ic];
        let left: HashSet<i64> = preambulo.iter().map(|num| *num).collect();
        let right: HashSet<i64> = preambulo.iter().map(|num| numtest - num).collect();
        let intersection: Vec<&i64> = left.intersection(&right).collect();
        match intersection.len() {
            0 => Some(numtest),
            _ => None
        }
    }).unwrap();

    println!("{:?}",salida);

    let suma_extremos = (0..numeros.len()-1).find_map(|ic|{
        let posible = ((ic+1)..numeros.len()).find_map(|fc|{
            let suma = numeros[ic..fc].iter().fold(0, |acc, &x| acc + x);
            if suma >= *salida {
                Some((ic,fc,suma))
            } else {
                None
            }
        });
        match posible {
            Some((li,ls,valor)) => {
                if valor == *salida {
                    Some(numeros[li..ls].iter().min().unwrap() + numeros[li..ls].iter().max().unwrap())
                } else {
                    None
                }
            },
            None => None
        }
    });
    
    println!("{:?}",suma_extremos);

}

