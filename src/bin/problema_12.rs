use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];

    let contents = fs::read_to_string(file)
        .expect("Algo salió mal al leer el archivo.");

    let mut boat = Boat {x: 0, y: 0, facing: 1};

    contents.trim().lines().for_each(|ele|{
        let inst = ele.get(..1).unwrap();
        let valor: i32 = ele.get(1..).unwrap().parse().unwrap();

        match inst {
            "E" => boat.x = boat.x + valor,
            "W" => boat.x = boat.x - valor,
            "N" => boat.y = boat.y + valor,
            "S" => boat.y = boat.y - valor,
            "R" => {
                let angpos = valor / 90;
                boat.facing = (boat.facing + angpos) % 4;
            },
            "L" => {
                let angpos = valor / 90;
                let temp = (boat.facing - angpos) % 4;
                if temp < 0 {
                    boat.facing = 4 + temp
                } else {
                    boat.facing = temp
                }
            },
            "F" => {
                match boat.facing {
                    0 => boat.y = boat.y + valor,
                    1 => boat.x = boat.x + valor,
                    2 => boat.y = boat.y - valor,
                    3 => boat.x = boat.x - valor,
                    _ => ()
                }
            },
            _ => ()
        }
    });

    println!("{:?}",boat);
    println!("{}",boat.x.abs() + boat.y.abs());

    let mut boat = Boat {x: 0, y: 0, facing: 1};
    let mut wpoint = Boat {x: 10, y:1, facing: 0};

    contents.trim().lines().for_each(|ele|{
        let inst = ele.get(..1).unwrap();
        let valor: i32 = ele.get(1..).unwrap().parse().unwrap();

        match inst {
            "E" => wpoint.x = wpoint.x + valor,
            "W" => wpoint.x = wpoint.x - valor,
            "N" => wpoint.y = wpoint.y + valor,
            "S" => wpoint.y = wpoint.y - valor,
            "R" => {
                let angpos = valor / 90;
                match angpos {
                    0 => (),
                    1 => {
                        let old_x = wpoint.x;
                        wpoint.x = wpoint.y;
                        wpoint.y = -1 * old_x;
                    },
                    2 => {
                        wpoint.x = -1 * wpoint.x;
                        wpoint.y = -1 * wpoint.y;
                    },
                    3 => {
                        let old_x = wpoint.x;
                        wpoint.x = -1 * wpoint.y;
                        wpoint.y = old_x;
                    },
                    _ => ()
                }
            },
            "L" => {
                let angpos = valor / 90;
                match angpos {
                    0 => (),
                    3 => {
                        let old_x = wpoint.x;
                        wpoint.x = wpoint.y;
                        wpoint.y = -1 * old_x;
                    },
                    2 => {
                        wpoint.x = -1 * wpoint.x;
                        wpoint.y = -1 * wpoint.y;
                    },
                    1 => {
                        let old_x = wpoint.x;
                        wpoint.x = -1 * wpoint.y;
                        wpoint.y = old_x;
                    },
                    _ => ()
                }
            },
            "F" => {
                boat.x = boat.x + (valor * wpoint.x);
                boat.y = boat.y + (valor * wpoint.y)
            },
            _ => ()
        }
        println!("boat: {:?}",boat);
        println!("wpoint: {:?}",wpoint);
    });

    println!("{:?}",boat);
    println!("{}",boat.x.abs() + boat.y.abs());
    
}

#[derive(Debug)]
struct Boat {
    x: i32,
    y: i32,
    facing: i32,
}
