use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];

    let contents = fs::read_to_string(file)
        .expect("Algo salió mal al leer el archivo.");

    let mut matriz: Vec<Vec<char>> = contents.trim().lines().map(|linea| {
        linea.chars().collect::<Vec<char>>()
    }).collect();

    let mut cambios = 1;
    let mut vueltas = 0;
    while cambios != 0 {
        vueltas = vueltas + 1;
        cambios = evolve(&mut matriz);
        let (vacios,llenos) = inspector(&matriz);
        println!("vuelta: {}, cambios: {}, vacios: {}, llenos: {}",vueltas,cambios,vacios,llenos);
    }

    let mut matriz: Vec<Vec<char>> = contents.trim().lines().map(|linea| {
        linea.chars().collect::<Vec<char>>()
    }).collect();

    let mut cambios = 1;
    let mut vueltas = 0;
    while cambios != 0 {
        vueltas = vueltas + 1;
        cambios = evolve_viendo(&mut matriz);
        let (vacios,llenos) = inspector(&matriz);
        println!("vuelta: {}, cambios: {}, vacios: {}, llenos: {}",vueltas,cambios,vacios,llenos);
    }
    
}

fn inspector(matriz: &Vec<Vec<char>>) -> (usize,usize) {

    let filas = matriz.len();
    let columnas = matriz[0].len();

    let mut vacios = 0;
    let mut llenos = 0;
    
    for col in 0..columnas {
        for fil in 0..filas {
            match matriz[fil][col] {
                'L' => vacios = vacios + 1,
                '#' => llenos = llenos + 1,
                _ => {}
            }
        };
    };

    (vacios,llenos)

}

fn evolve(matriz: &mut Vec<Vec<char>>) -> usize {
    
    let filas = matriz.len();
    let columnas = matriz[0].len();

    let mut salida = matriz.clone();

    let mut cambios = 0;
    for col in 0..columnas {
        for fil in 0..filas {
            salida[fil][col] = update(matriz,col,fil).unwrap();
            if salida[fil][col] != matriz[fil][col] {
                cambios  = cambios + 1;
            }
        }
    }

    *matriz = salida;

    cambios
}

fn evolve_viendo(matriz: &mut Vec<Vec<char>>) -> usize {
    
    let filas = matriz.len();
    let columnas = matriz[0].len();

    let mut salida = matriz.clone();

    let mut cambios = 0;
    for col in 0..columnas {
        for fil in 0..filas {
            salida[fil][col] = update_viendo(matriz,col,fil).unwrap();
            if salida[fil][col] != matriz[fil][col] {
                cambios  = cambios + 1;
            }
        }
    }

    *matriz = salida;

    cambios
}

fn update(matriz: &Vec<Vec<char>>, x: usize, y: usize) -> Option<char> {

    match matriz[y][x] {
        'L' => {
             match ver(matriz,x,y).1 {
                0 => Some('#'),
                _ => Some('L'),
            }
        },
        '#' => {
            if ver(matriz,x,y).1 >= 4 {
                Some('L')
            } else {
                Some('#')
            }
        }
        '.' => Some('.'),
        _ => None
    }
}

fn update_viendo(matriz: &Vec<Vec<char>>, x: usize, y: usize) -> Option<char> {

    match matriz[y][x] {
        'L' => {
             match mirar(matriz,x,y).1 {
                0 => Some('#'),
                _ => Some('L'),
            }
        },
        '#' => {
            if mirar(matriz,x,y).1 >= 5 {
                Some('L')
            } else {
                Some('#')
            }
        }
        '.' => Some('.'),
        _ => None
    }
}

fn mirar(matriz: &Vec<Vec<char>>, x: usize, y: usize) -> (usize,usize) {
    
    let filas = matriz.len() as i32;
    let columnas = matriz[0].len() as i32;

    let mut vecinos = vec![];

    let mut xf = x as i32;
    let mut yf = y as i32;
    let mut visto = '.';
    while xf > 0 && yf > 0 && visto == '.' {
        xf = xf - 1;
        yf = yf - 1;
        visto = matriz[yf as usize][xf as usize];
    }
    vecinos.push(visto);

    let mut xf = x as i32;
    let mut yf = y as i32;
    let mut visto = '.';
    while yf > 0 && visto == '.' {
        xf = xf;
        yf = yf - 1;
        visto = matriz[yf as usize][xf as usize];
    }
    vecinos.push(visto);

    let mut xf = x as i32;
    let mut yf = y as i32;
    let mut visto = '.';
    while xf < columnas - 1 && yf > 0 && visto == '.' {
        xf = xf + 1;
        yf = yf - 1;
        visto = matriz[yf as usize][xf as usize];
    }
    vecinos.push(visto);

    let mut xf = x as i32;
    let mut yf = y as i32;
    let mut visto = '.';
    while xf > 0 && visto == '.' {
        xf = xf - 1;
        yf = yf;
        visto = matriz[yf as usize][xf as usize];
    }
    vecinos.push(visto);

    let mut xf = x as i32;
    let mut yf = y as i32;
    let mut visto = '.';
    while xf < columnas - 1 && visto == '.' {
        xf = xf + 1;
        yf = yf;
        visto = matriz[yf as usize][xf as usize];
    }
    vecinos.push(visto);

    let mut xf = x as i32;
    let mut yf = y as i32;
    let mut visto = '.';
    while xf > 0 && yf < filas - 1 && visto == '.' {
        xf = xf - 1;
        yf = yf + 1;
        visto = matriz[yf as usize][xf as usize];
    }
    vecinos.push(visto);

    let mut xf = x as i32;
    let mut yf = y as i32;
    let mut visto = '.';
    while yf < filas - 1 && visto == '.' {
        xf = xf;
        yf = yf + 1;
        visto = matriz[yf as usize][xf as usize];
    }
    vecinos.push(visto);

    let mut xf = x as i32;
    let mut yf = y as i32;
    let mut visto = '.';
    while xf < columnas - 1 && yf < filas - 1 && visto == '.' {
        xf = xf + 1;
        yf = yf + 1;
        visto = matriz[yf as usize][xf as usize];
    }
    vecinos.push(visto);

    let (vacios, llenos) = inspector(&vec![vecinos]);

    (vacios,llenos)

}

fn ver(matriz: &Vec<Vec<char>>, x: usize, y: usize) -> (usize,usize) {

    let filas = matriz.len();
    let columnas = matriz[0].len();

    let mut vecinos = vec![];

    if x != 0 && y != 0 {
        vecinos.push(matriz[y-1][x-1]);
    };

    if y != 0 {
        vecinos.push(matriz[y-1][x]);
    };

    if x != columnas - 1 && y != 0 {
        vecinos.push(matriz[y-1][x+1]);
    };

    if x != 0 {
        vecinos.push(matriz[y][x-1]);
    };

    if x != columnas - 1 {
        vecinos.push(matriz[y][x+1]);
    };

    if x != 0 && y != filas - 1 {
        vecinos.push(matriz[y+1][x-1]);
    };

    if y != filas - 1 {
        vecinos.push(matriz[y+1][x]);
    };

    if x != columnas - 1 && y != filas - 1 {
        vecinos.push(matriz[y+1][x+1]);
    };

    let (vacios, llenos) = inspector(&vec![vecinos]);

    (vacios,llenos)
}

