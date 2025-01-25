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

