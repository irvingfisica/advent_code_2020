use std::env;
use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];

    let contents = fs::read_to_string(file)
        .expect("Algo salió mal al leer el archivo.");

    let mut red = Red {
                    nodos:HashSet::new(),
                    cabe_en:HashMap::new(),
                    capacidad:HashMap::new(),
                    le_caben:HashMap::new(),
                };

    contents.trim().lines().for_each(|line| parse_line(line, &mut red));

    let mut set = HashSet::new();
    set.insert("shiny gold".to_string());

    let mut set_salida = HashSet::new();

    let mut flag = 1;
    while flag != 0 {
        match red.check_set_up(&set) {
            None => flag = 0,
            Some(temp) => {
                set_salida = set_salida.union(&temp)
                    .map(|ele| ele.clone()).collect();
                set = temp;
            }
        }
    }

    println!("{:?}",set_salida.len());

    let mut vector =  vec![("shiny gold".to_string(),1)];
    let mut total = 0;

    let mut flag = 1;
    while flag != 0 {
        match red.check_set_down(&vector) {
            None => flag = 0,
            Some((val,temp)) => {
                total = total + val;
                vector = temp;
            }
        }
    }

    println!("{:?}",total);
    
}

fn parse_line(linea: &str, red: &mut Red) {
    
    let partes: Vec<&str> = linea.split(" contain ").collect();
    let nodo0 = partes[0].replace(" bags","");

    red.nodos.insert(nodo0.clone());

    let target = partes[1].replace(" bags","")
                        .replace(" bag","")
                        .replace(".","");

    target.split(", ").for_each(|targ| {
        match parse_target(targ) {
            Some((val,nodo)) => {
                red.nodos.insert(nodo.clone());
                let couple = (nodo0.clone(),val);
                let contenedores = red.cabe_en.entry(nodo.clone()).or_insert(vec![]);
                contenedores.push(couple);
                let capsuma = red.capacidad.entry(nodo0.clone()).or_insert(0);
                *capsuma += val;
                let couple = (nodo.clone(),val);
                let contenido = red.le_caben.entry(nodo0.clone()).or_insert(vec![]);
                contenido.push(couple);
            },
            _ => {},
        }
    })
}

fn parse_target(target: &str) -> Option<(usize,String)> {
    match target {
        "no other" => None,
        _ => {
            let partes: Vec<&str> = target.splitn(2,' ').collect();
            Some((partes[0].parse().unwrap(),partes[1].to_string()))
        }
    }
}

#[derive(Debug)]
struct Red {
    nodos: HashSet<String>,
    cabe_en: HashMap<String,Vec<(String,usize)>>,
    capacidad: HashMap<String,usize>,
    le_caben: HashMap<String,Vec<(String,usize)>>
}

impl Red {
    fn check_bag_up(&self, bag: &str) -> Option<HashSet<String>> {
        let contenedores = self.cabe_en.get(bag);
        match contenedores {
            None => None,
            Some(vector) => {
                Some(vector.iter().map(|couple| couple.0.clone()).collect())
            }
        }
    }

    fn check_bag_down(&self, bag: &str, existencia: usize) -> Option<(usize,Vec<(String,usize)>)> {
        let contenido = self.le_caben.get(bag);
        match contenido {
            None => None,
            Some(vector) => {
                match self.capacidad.get(bag) {
                    Some(cap) => {
                        let newvec = vector.iter().map(|(bag,val)| (bag.clone(),val*existencia)).collect();
                        Some((cap*existencia,newvec))},
                    None => None
                }
            }
        }
    }

    fn check_set_down(&self, bags: &Vec<(String,usize)>) -> Option<(usize,Vec<(String,usize)>)> {

        let mut salida = Vec::new();

        let mut total = 0;
        bags.iter().for_each(|bag| {
            match self.check_bag_down(&bag.0,bag.1) {
                Some((suma,contenido)) => {
                    total = total + suma;
                    contenido.iter().for_each(|ele| {salida.push(ele.clone());});
                },
                None => {},
            }
        });

        match salida.len() {
            0 => None,
            _ => Some((total,salida))
        }
    }

    fn check_set_up(&self, bags: &HashSet<String>) -> Option<HashSet<String>> {

        let mut salida = HashSet::new();

        bags.iter().for_each(|bag| {
            match self.check_bag_up(bag) {
                Some(set) => {
                    set.iter().for_each(|ele| {salida.insert(ele.clone());});
                },
                None => {},
            }
        });

        match salida.len() {
            0 => None,
            _ => Some(salida)
        }
    }

}


