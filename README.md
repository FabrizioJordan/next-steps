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



## If-let


En el libro oficial de Rust nos comparten una tal vez buena explicación de ```if-let```:

Existe una manera cómoda de probar si un valor se ajusta a un sólo patrón.

Acá te dejo un ejemplo con ```match```, donde se le pasa un valor de tipo ```Option<u8>```.

Si el valor es 7 entonces ejecutará un mensaje, sino entonces no hará nada.

```
let num: Option<u8> = Some(7);
match num {
    Some(7) => println!("Ese es mi número de la suerte"),
    _ => {},
}
```

¿Porqué no se usa ```none``` y sí ```_```?

Esto se debe a que se busca ignorar todos los valores distintos de "7" sin importar nada.

Por eso se usa el patrón comodín ```_```.

Ahora le vamos a agregar el operador ```if-let```:

```
let num: Option<u8> = Some(7);
if let Some(7) = num {
    println!("Ese es mi número de la suerte");
}
```

```if-let``` compara un patrón con una expresión, si los dos coinciden entonces se ejecuta el *if*.

Este operador nos sirve cuando es una sola coincidencia la que buscamos de un patrón, a diferencia de ```match``` que nos sirve para varias coincidencias.


## Conociendo ```unwrap``` y ```expect```

Con el método ```unwrap``` uno puede intentar conseguir el valor interno de un tipo ```Option``` directamente. 

Pero, este método emitirá una alerta de pánico si la variante es ```None```. Se tiene que usar con precaución.

```
// ejemplo 1
let regalo = Some("camionsito");
assert_eq!(regalo.unwrap(), "camionsito"); // funciona bien
    
let regalo_vacio: Option<&str> = None;
assert_eq!(regalo_vacio.unwrap(), "camionsito"); // se produce un panic
```

También se puede usar ```expect```, pero este si permite emitir un mensaje de panico personalizado.

```
// ejemplo 2
let a = Some("valor");
assert_eq!(a.expect("las frutas son buenas"), "valor"); // funciona bien

let b: Option<&str> = None;
b.expect("las frutas son buenas"); // produce un panic con el mensaje "las frutas son buenas"
```

Acá te dejo un ejemplo con las informaciones usadas en la sección  ```if-let```.

```
// ejemplo 3
// utilizando código parecido al de if-let
let num: Option<u8> = Some(7);
//let num: Option<u8> = None;

// unwrap() desempaqueta un Some y devuelve el valor
// si es None, se produce un panic!
println!("Mi número de la suerte es {}", num.unwrap());
```

<br>

***Estos ejemplos muestran lo que no es recomendable utilizar*, debido a que no se están manejado los errores.**

Acá te dejo algunas mejores prácticas.

Usar la coincidencia de patrones y administre el caso ```None``` explícitamente, cómo usar ```match```.

Llamar a métodos parecidos, que no emitan alertas de pánico, como ```unwrap_or```, que devuelve un valor predeterminado si la variante es ```None``` o el valor interno si la variante es ```Some(value)```.

También existe ```unwrap_or_else```.

Entonces ```unwrap_or``` se usa cuando el valor predeterminado es barato de calcular y ```unwrap_or_else``` cuando el cálculo del valor predeterminado es costoso o implica lógica adicional.

Acá hay algunos ejemplos y sus explicaciones usando ```unwrap_or```:

```
assert_eq!(Some("perro").unwrap_or("gato"), "perro");
assert_eq!(None.unwrap_or("gato"), "gato");
```

Básicamente se está verificando el valor de dentro del ```Option```, el cual estamos creando al usar ```Some```.
 
Hay que entender que usamos esto solo para el ejemplo.

Entonces, se crea el ```Option``` con su valor "perro" y se maneja el error con ```.unwrap_or("gato")```, que nos deja el valor "gato" si es que la aserción es errónea. Con "aserción" nos referimos a una verificación de si es verdadera una parte de un código.

Por ende, lo que hace ```.unwrap_or("gato")``` es dejarnos un valor predeterminado ante cualquier error.

Luego viene el último código:
```
assert_eq!(Some...unwrap_or... , "perro")
```

El cual hace la verifiación de la aserción.

Osea que, primero creamos el ```Option``` con su valor, luego manejamos todo ante el posible error permitiendo al programa designar un valor predeterminado y por último verificamos que el valor dentro de ```Option``` sea el del "patrón".

Todo esto es pensando en que el valor y el patrón pueden llegar a ser iguales, pero si queremos que siempre surja un error entonces usamos la segunda línea.

```
assert_eq!(None.unwrap_or("gato"), "gato");
```

Se crea un ```Option``` sin ningún valor, el valor predeterminado que añade ```unwrap_or``` es "gato" y cómo el valor inicial es inexistente, entonces la aserción dará error al ser comparado el valor inicial con el "gato" final. Y al tener un error, el valor predeterminado "gato" es el valor devuelto.


## Ejemplos y prácticas de uso

Ahora vamos a hacer dos ejemplos de uso.

Te voy a dejar dos códigos y vas a tener que implementarle lo pedido a cada uno. Al final de todo te dejo las respuestas.

### Práctica 1 
