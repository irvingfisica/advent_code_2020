use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];

    let contents = fs::read_to_string(file)
        .expect("Algo salió mal al leer el archivo.");

    

    let mut instrucciones: Vec<Operacion> = contents.lines().map(|ele| {
        match &ele[..3] {
            "nop" => Operacion::Nop(ele[4..].parse::<i32>().unwrap(),0),
            "acc" => Operacion::Acc(ele[4..].parse::<i32>().unwrap(),0),
            "jmp" => Operacion::Jmp(ele[4..].parse::<i32>().unwrap(),0),
            _ => Operacion::None
        }
    }).collect();

    let mut control = 0;
    let mut posicion = 0;
    let mut accumulator = 0;

    while control == 0 {
        match evaluador(&mut instrucciones, posicion, accumulator) {
            Some((pos,acc)) => {
                posicion = pos;
                accumulator = acc;
            },
            None => break
        }

        control = instrucciones[posicion as usize].check();
    }

    println!("valor de acc en la primera parte: {}",accumulator);


    'outer: for index in 0..instrucciones.len() {

        let mut temp_inst: Vec<Operacion> = instrucciones.iter().map(|ele| reset(ele)).collect();

        match instrucciones[index] {
            Operacion::Nop(_,_) | Operacion::Jmp(_,_) => temp_inst[index] = mutar(&temp_inst[index]),
            _ => continue
        }

        let mut control = 0;
        let mut posicion = 0;
        let mut accumulator = 0;

        while control == 0 {
            match evaluador(&mut temp_inst, posicion, accumulator) {
                Some((pos,acc)) => {
                    posicion = pos;
                    accumulator = acc;
                },
                None => {
                    break 'outer
                }
            }

            if (posicion as usize) < temp_inst.len() {
                control = temp_inst[posicion as usize].check();
            } else {
                control = 1;
                println!("valor de acc en la segunda parte: {}", accumulator);
            }
        
        }

    }


}

#[derive(Debug)]
enum Operacion {
    Nop(i32,usize),
    Acc(i32,usize),
    Jmp(i32,usize),
    None
}

impl Operacion {

    fn check(&self) -> usize {
        match self {
            Operacion::Nop(_,cnt) => *cnt,
            Operacion::Jmp(_,cnt) => *cnt,
            Operacion::Acc(_,cnt) => *cnt,
            Operacion::None => 1,
        }
    }
}

fn evaluador(instrucciones: &mut Vec<Operacion>, posicion: i32, accumulator: i32) -> Option<(i32, i32)> {

    if posicion >= instrucciones.len() as i32 {
        return None
    }

    instrucciones[posicion as usize] = pasar(&instrucciones[posicion as usize]);

    match instrucciones[posicion as usize] {
        Operacion::Nop(_,_) => Some((posicion + 1, accumulator)),
        Operacion::Acc(valor,_) => Some((posicion + 1, accumulator + valor)),
        Operacion::Jmp(valor,_) => Some((posicion + valor, accumulator)),
        _ => None
    }
}

fn pasar(op: &Operacion) -> Operacion {
    match op {
        Operacion::Nop(val,_) => Operacion::Nop(*val,1),
        Operacion::Jmp(val,_) => Operacion::Jmp(*val,1),
        Operacion::Acc(val,_) => Operacion::Acc(*val,1),
        Operacion::None => Operacion::None,
    }
}

fn reset(op: &Operacion) -> Operacion {
    match op {
        Operacion::Nop(val,_) => Operacion::Nop(*val,0),
        Operacion::Jmp(val,_) => Operacion::Jmp(*val,0),
        Operacion::Acc(val,_) => Operacion::Acc(*val,0),
        Operacion::None => Operacion::None,
    }
}

fn mutar(op: &Operacion) -> Operacion {
    match op {
        Operacion::Nop(val,_) => Operacion::Jmp(*val,0),
        Operacion::Jmp(val,_) => Operacion::Nop(*val,0),
        Operacion::Acc(val,_) => Operacion::Acc(*val,0),
        Operacion::None => Operacion::None,
    }
}
