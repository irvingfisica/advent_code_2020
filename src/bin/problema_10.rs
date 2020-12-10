use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];

    let contents = fs::read_to_string(file)
        .expect("Algo salió mal al leer el archivo.");

    let mut numeros_temp: Vec<usize> = contents.trim().lines().map(|ele| {ele.parse().unwrap()}).collect();

    let mut numeros: Vec<usize> = vec![0];

    numeros.append(&mut numeros_temp);

    numeros.sort();

    let mut cta1 = 0;
    let mut cta3 = 0;

    let mut out = 0;
    for num in numeros.iter() {
        match num - out {
            1 => {
                cta1 = cta1 + 1;
                out = *num;
            },
            3 => {
                cta3 = cta3 + 1;
                out = *num;
            },
            _ => {}
        }
    };

    println!("{}",cta1*(cta3+1));

    let saltos: Vec<usize> = numeros[1..].iter().zip(numeros[..(numeros.len()-1)].iter()).map(|(a,b)|a-b).collect();

    let mut count = 0;
    let mut salt = vec![];
    for salto in saltos.iter() {
        match salto {
            1 => {
                count = count + 1
            },
            3 => {
                if count != 0 {
                    salt.push(count);
                    count = 0;
                    salt.push(1);
                } else {
                    salt.push(1);
                }
            },
            _ => {}
        }
    }
    if count != 0 {
        salt.push(count);
    } else {
    }

    let pows: Vec<usize> = salt.iter().map(|ele| {
        match ele {
            // Aquí me aprovecho de que nunca el número de 1s es mayor a 4. El caso general impleca calcular la cantidad a restar en cada caso > 4
            4 => usize::pow(2,ele-1) - 1,
            _ => usize::pow(2,ele-1),
        }
    }).collect();

    println!("{:?}",pows.iter().fold(1, |acc, &x| acc * x));

}

