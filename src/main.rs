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