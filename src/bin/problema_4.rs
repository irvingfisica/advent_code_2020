use std::env;
use std::fs;
use std::error::Error;


fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];

    let contents = fs::read_to_string(file)
        .expect("Algo salió mal al leer el archivo.");

    let entradas : Vec<Documento> = contents.split("\n\n").map(|datos| lector(datos)).collect();

    let validos = entradas.iter().filter_map(|docu| docu.primer_filtro().ok()).count();

    let validos2 = entradas.iter().filter_map(|docu| docu.segundo_filtro().ok()).count();

    println!("{},{}",validos,validos2);

}

#[derive(Default, Debug)]
struct  Documento {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Documento {
    fn load_data(&mut self, key: &str, val: &str) {
        match key {

            "byr" => self.byr = Some(String::from(val)),
            "iyr" => self.iyr = Some(String::from(val)),
            "eyr" => self.eyr = Some(String::from(val)),
            "hgt" => self.hgt = Some(String::from(val)),
            "hcl" => self.hcl = Some(String::from(val)),
            "ecl" => self.ecl = Some(String::from(val)),
            "pid" => self.pid = Some(String::from(val)),
            "cid" => self.cid = Some(String::from(val)),
            _ => {}
        }
    }

    fn primer_filtro(&self) -> Result<(), &str> {
       
        self.byr.as_ref().ok_or("")?;
        self.iyr.as_ref().ok_or("")?;
        self.eyr.as_ref().ok_or("")?;
        self.hgt.as_ref().ok_or("")?;
        self.hcl.as_ref().ok_or("")?;
        self.ecl.as_ref().ok_or("")?;
        self.pid.as_ref().ok_or("")?;

        Ok(())
    }

    fn segundo_filtro(&self) -> Result<(), Box<dyn Error>> {

        byr_validator(self.byr.as_ref().ok_or("")?)?;
        iyr_validator(self.iyr.as_ref().ok_or("")?)?;
        eyr_validator(self.eyr.as_ref().ok_or("")?)?;
        hgt_validator(self.hgt.as_ref().ok_or("")?)?;
        hcl_validator(self.hcl.as_ref().ok_or("")?)?;
        ecl_validator(self.ecl.as_ref().ok_or("")?)?;
        pid_validator(self.pid.as_ref().ok_or("")?)?;
            
        Ok(())
    }
}

fn byr_validator(byr: &str) -> Result<(), Box<dyn Error>> {
    
    let numero: usize = byr.parse()?;

    if numero >= 1920 && numero <= 2002 {
        Ok(())
    } else {
        Err(From::from(""))
    }
}

fn iyr_validator(iyr: &str) -> Result<(), Box<dyn Error>> {
    
    let numero: usize = iyr.parse()?;

    if numero >= 2010 && numero <= 2020 {
        Ok(())
    } else {
        Err(From::from(""))
    }
}

fn eyr_validator(eyr: &str) -> Result<(), Box<dyn Error>> {
    
    let numero: usize = eyr.parse()?;

    if numero >= 2020 && numero <= 2030 {
        Ok(())
    } else {
        Err(From::from(""))
    }
}

fn hgt_validator(hgt: &str) -> Result<(), Box<dyn Error>> {
    
    let len = hgt.len();
    let numero: usize = hgt[..len-2].parse()?;
    let unidad = &hgt[len-2..len];

    match unidad {
        "in" => {
            if numero >= 59 && numero <= 76 {
                return Ok(())
            } else {
                return Err(From::from(""))
            }
        },
        "cm" => {
            if numero >= 150 && numero <= 193 {
                return Ok(())
            } else {
                return Err(From::from(""))
            }
        },
        _ => return Err(From::from("")),
    }

}

fn hcl_validator(hcl: &str) -> Result<(), Box<dyn Error>> {

    let len = hcl.len();
    if len == 7 && hcl.starts_with('#') {
        
        if hcl[1..].chars().all(|c| "0123456789abcdef".contains(c)) {
            return Ok(())
        } else {
            return Err(From::from(""))
        }
    } else {
        return Err(From::from(""))
    }

}

fn ecl_validator(ecl: &str) -> Result<(), Box<dyn Error>> {

    match ecl {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => Ok(()),
        _ => Err(From::from(""))
    }
}

fn pid_validator(pid: &str) -> Result<(), Box<dyn Error>> {
    if pid.len() != 9 {
        return Err(From::from(""))
    }

    match pid.parse::<usize>() {
        Ok(_) => Ok(()),
        _ => Err(From::from(""))
    }
}


fn lector(datos: &str) -> Documento {
    let elementos: Vec<&str> = datos.split(|c| c == ' ' || c == '\n').collect();

    let mut docu: Documento = Default::default();

    for elemento in elementos {
        let pareja: Vec<&str> = elemento.split(":").collect();
        if pareja.len() == 2 {
            docu.load_data(pareja[0], pareja[1]);
        }
    };

    docu
}





