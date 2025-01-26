use std::fmt;

fn main() {
    //println!("Using panic!");
    //panic!("PAAANIC")


    /*
    let autos = vec!["rojos", "verdes", "amarillos", "azules", "negros"];

    let primer_auto = autos.get(0);
    println!("{:?}", primer_auto);

    let tercer_auto = autos.get(2);
    println!("{:?}", tercer_auto);

    // intento de get de un elemento inexistente
    let no_existe = autos.get(99);
    println!("{:?}", no_existe);
    */


    /*
    let autos = vec!["rojo", "verde", "amarillo", "azule", "negro"];
    for &index in [0, 2, 99].iter() {
        match autos.get(index) {
            Some(auto) => println!("Mis autos son de color {}!", auto),
            None => println!("No hay autos! :("),
        }
    }
    */


    /* 
    // se cambia un poco los valores
    // ahora se usa match con una coincidencia exacta 
    let colores = vec!["magenta", "violeta", "rosa", "purpura", "naranja"];
    for &index in [1, 4, 54].iter() {
        match colores.get(index) {
            Some(&"naranja") => println!("El naranja es mi favorito!!!"),
            Some(color) => println!("El color es {}!", color),
            None => println!("No hay colores en el universo! :("),
        }
    }
    */

    creador_de_casas();
    transformers();
}


/* 
fn if_let(){

    // sin if-let
    let num: Option<u8> = Some(7);
    match num {
        Some(7) => println!("Ese es mi número de la suerte"),
        _ => {},
    }

    // con if-let
    let num: Option<u8> = Some(7);
    if let Some(7) = num {
        println!("Ese es mi número de la suerte");
    }
}
*/

/*
fn unwrap(){
    // ejemplo 1
    let regalo = Some("camionsito");
    assert_eq!(regalo.unwrap(), "camionsito"); // funciona bien
    
    let regalo_vacio: Option<&str> = None;
    //assert_eq!(regalo_vacio.unwrap(), "camionsito"); // se produce un panic


    // ejemplo 2
    let a = Some("valor");
    assert_eq!(a.expect("las frutas son buenas"), "valor"); // funciona bien

    let b: Option<&str> = None;
    //b.expect("las frutas son buenas"); // produce un panic con el mensaje "las frutas son buenas"

    // ejemplo 3
    // utilizando código parecido al de if-let
    let num: Option<u8> = Some(7);
    //let num: Option<u8> = None; // descomentar para probar el panic

    // unwrap() desempaqueta un Some y devuelve el valor
    // si es None, se produce un panic!
    println!("Mi número de la suerte es {}", num.unwrap());


    // esos ejemplos muestran algunas malas prácticas, lo mejor sería manejar los errores
    // no se recomiendan debido a esa alerta de panico que pueden generar
    
    // recomendados
    assert_eq!(Some("perro").unwrap_or("gato"), "perro");
    assert_eq!(None.unwrap_or("gato"), "gato");
}
*/

// ejemplos de uso


// ejemplo 1
enum Opcion{
    Tiene,
    NoTiene
}

impl fmt::Display for Opcion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Opcion::Tiene => write!(f, "tiene"),
            Opcion::NoTiene => write!(f, "no tiene"),
        }
    }
}

struct Casa{
    direccion: String,
    habitaciones: u8,
    garaje: Opcion,
    piscina: Opcion
}

fn crea_casa(casa: &Casa) -> String {

    let mut descripcion = String::new();
    descripcion.push_str("
    ");
    
    descripcion.push_str("Una casa en ");
    descripcion.push_str(&casa.direccion);
    
    descripcion.push_str(" con ");
    
    descripcion.push_str(&casa.habitaciones.to_string());
    descripcion.push_str(" habitaciones");

    descripcion.push_str(", que ");
    descripcion.push_str(&casa.garaje.to_string());
    descripcion.push_str(" garaje");


    descripcion.push_str(" y ");
    descripcion.push_str(&casa.piscina.to_string());
    descripcion.push_str(" piscina");

    descripcion
}

fn creador_de_casas() {
    let casa1 = Casa {
        direccion: String::from("Calle 123"),
        habitaciones: 3,
        garaje: Opcion::Tiene,
        piscina: Opcion::NoTiene,
    };

    let casa2 = Casa {
        direccion: String::from("Calle 456"),
        habitaciones: 1,
        garaje: Opcion::NoTiene,
        piscina: Opcion::NoTiene
    };

    println!("{}", crea_casa(&casa1));
    println!("{}", crea_casa(&casa2));
    println!("");

}

////

// ejemplo 2
enum Bando{
    Decepticons,
    Autobots
}

enum Amenaza{
    Media,
    Alta,
    DestruccionMasiva
}

impl fmt::Display for Bando {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Bando::Decepticons => write!(f, "Decepticons"),
            Bando::Autobots => write!(f, "Autobots"),
        }
    }
}

impl fmt::Display for Amenaza {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Amenaza::Media => write!(f, "Media"),
            Amenaza::Alta => write!(f, "Alta"),
            Amenaza::DestruccionMasiva => write!(f, "Destrucción Masiva"),
        }
    }
}

struct Transformer {
    nombre: Option<String>,
    rango: u8,
    bando: Bando,
    amenaza: Amenaza
}

fn meet_the_transformers(transformer: &Transformer) -> String {
    let mut description = String::new();
    description.push_str("
    ");

    if let Some(name) = &transformer.nombre {
        description.push_str(name);
    } else {
        description.push_str("Unknown name");
    }

    description.push_str(" is a rank ");

    description.push_str(&transformer.rango.to_string());

    description.push_str(" and is part of the ");
    description.push_str(&transformer.bando.to_string());
    description.push_str(" faction.");

    description.push_str(" The threat level is ");
    description.push_str(&transformer.amenaza.to_string());

    description
}


fn transformers() {
    let optimus = Transformer {
        nombre: Some(String::from("Optimus Prime")),
        rango: 10,
        bando: Bando::Autobots,
        amenaza: Amenaza::DestruccionMasiva,
    };

    let megatron = Transformer {
        nombre: Some(String::from("Megatron")),
        rango: 10,
        bando: Bando::Decepticons,
        amenaza: Amenaza::DestruccionMasiva,
    };

    let bumblebee = Transformer {
        nombre: Some(String::from("Bumblebee")),
        rango: 7,
        bando: Bando::Autobots,
        amenaza: Amenaza::Media,
    };

    let unknown = Transformer {
        nombre: None,
        rango: 0,
        bando: Bando::Decepticons,
        amenaza: Amenaza::Alta,
    };

    println!("{}", meet_the_transformers(&optimus));
    println!("{}", meet_the_transformers(&megatron));
    println!("{}", meet_the_transformers(&bumblebee));
    println!("{}", meet_the_transformers(&unknown));
    println!("");
}
