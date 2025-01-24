# Next-steps in Rust

## Errores Irrecuperables

### Panic 

Para controlar errores en Rust lo más sencillo es usar ```panic!```

La macro ```panic!``` emite una alerta de **pánico** para el subproceso actual que imprime un mensaje de error, limpia la memporia y sale del programa.

Panic se puede llamar de forma manual o puede ser llamado por el programa debido a un error, cómo por ejemplo el intentar acceder a un elemento de un array cuando este elemento no existe.

Panic es sobre todo usado cuando el estado del programa es irrecuperable.


### ```Option<T>``` (o ```null```)

En otros lenguajes existen o ```null``` o ```nil```, pero en Rust solo existe ```Option<T>```.

Esta opción en realidad es un enum.

```
enum Option<T> {
    None,     // The value doesn't exist
    Some(T),  // The value exists
}
```

Estas dos variantes dentro de enum hablan de los valores que pueden existir, o no, al intentar accesar a una variable.

"La parte ```<T>``` de la declaración de enumeración ```Option<T>``` indica que el tipo ```T``` es genérico y se asociará a la variante ```Some``` de la enumeración ```Option```." comenta la guía de Microsoft.


Hay que recordar que, ```None``` y ```Some``` no son tipos en si mismo, si no variantes de ```Option<T>```, osea que ninguno de estas variantes pueden ser tomadas como argumentos, para eso sirve ```Option<T>```.


Un ejemplo de uso de ```Option<T>``` es cuando se intenta acceder a un índice inexistente dentro de un array.


Ejemplo: (no se ve ```Option<T>``` directamente, pero en la salida se puede ver ```Some``` y ```None```)

```
let autos = vec!["rojo", "verde", "amarillo", "azule", "negro"];

let primer_auto = autos.get(0);
println!("{:?}", primer_auto);

let tercer_auto = autos.get(2);
println!("{:?}", tercer_auto);

// intento de get de un elemento inexistente
let no_existe = autos.get(99);
println!("{:?}", no_existe);
```


Salida:

```
Some("rojo")
Some("amarillo")
None
```

Como se puede ver, la segunda salida es ```Some(amarillo)```, pero cuando se intenta el ```get``` de un elemento que no existe, cómo el 99, entonces la salida es ```None```.

Para cambiar que las salidas sean ```Some(amarillo)``` se puede usar de ```match```. Donde ahora lo que devolvería sería tan solo ```amarillo```.

Match se usa para usar patrones en el control del flujo de un programa. Osea que ```match``` se usa para cuando existen coincidencias (o no) se ejecute código.


```
let autos = vec!["rojo", "verde", "amarillo", "azule", "negro"];

for &index in [0, 2, 99].iter() {
    match autos.get(index) {
        Some(auto) => println!("Mis autos son de color {}!", auto),
        None => println!("No hay autos! :("),
    }
}
```

La salida:

```
Mis autos son de color rojo!
Mis autos son de color amarillo!
No hay autos! :(
```

En ese código se usa un ```for``` para iterar basados en que los valores usados serán ```[0, 2, 99]```, luego se usa ```match``` con ```get``` para encontrar las coincidencias entre los valores pasados y la varible temporal, si el auto es rojo, entonces la salida sería ```Some(rojo)```, pero se usa la variable ```auto``` que corresponde a cualquier valor de dentro ```Some()```.
Y si la salida normalmente ya es ```None```, entonces dentro de ```match``` tan solo se debe usar esta opción de igual manera.

Dentro del vector ```autos``` todos los valores son de tipo ```&str``` por ende, en ese código se usó ```Option<&str>```.


Otro código para seguir entendiendo ```match``` y ```Option<T>```:

```
let colores = vec!["magenta", "violeta", "rosa", "purpura", "naranja"];

for &index in [1, 4, 54].iter() {
    match colores.get(index) {
        Some(&"naranja") => println!("El naranja es mi favorito!!!"),
        Some(color) => println!("El color es {}!", color),
        None => println!("No hay colores en el universo! :("),
    }
}
```

En este código se usa de nuevo ```match```, pero esta vez se usa una coincidencia más exacta. Hay que recordar que al estos datos ser de tipo ```&str``` se necesita poner un ```&``` al inicio, pero al usar ```colores.get(index)``` devuelve ```Option<&&str>```.

La salida es:

```
El color es violeta!
El naranja es mi favorito!!!
No hay colores en el universo! :(
```

¿Porque no se ejecuta dos veces el match con "naranja" si es coincidente en dos ocasiones?
Esto se debe a que ```match``` se ejecuta de arriba para abajo y al encontrar la coincidencia el match pasa al siguiente valor.

También se debe recordar que los casos específicos cómo el del ejemplo y el color "naranja" se deben declarar primero, y luego se declaran los casos más genéricos, y por último, ```match``` debe cubrir todos los casos posibles o habrá un error de compilación.



## Errores Recuperables

Muchos errores que tenemos en nuestro código son errores que permiten que el programa siga ejecutandose.

Hay que recordar el enum ```Result``` con sus variantes ```Ok``` y ```Err``` que ya aprendímos [aquí](https://github.com/FabrizioJordan/guess-and-learn?tab=readme-ov-file)

Vamos a seguir usando esos ejemplos para hablar de otras cosas.


```
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

```T``` y ```E``` son parámetros de tipo genéricos, cuando el resultado de ```Result``` es exitoso entonces se devuelve ```Ok``` y cuando el resultado es un error lo devuleto es ```Err```.

Junto con ```Err``` viene el tipo del error, el cual se representa con ```E```, mientras que en caso de éxito y la devolución de ```Ok``` viene el valor devuelto, el cual se representa con ```T```.




