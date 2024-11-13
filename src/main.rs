fn main() {
    println!("Hello, world!");
    prueba_imutabilidad();
}

fn prueba_imutabilidad() {
    let mut x: i32 = 5; // para que la varaible sea mutable le colocamos mut
    println!("El valor de x es: {}", x);
    x = 2;
    println!("El valor de x es: {}", x);
}

const ESTO_ES_UNA_CONSTANTE: i32 = 5; // las constantes siempre deben ser en mayusculas y se les debe asignar un valor

fn shadowing() {
    let x: i8 = 5; // creamos una variable x de tipo i8
    let x: i8 = x + 1; // creamos una nueva variable x que se llama igual que la anterior pero con un valor diferente
    let x: i8 = x * 2; //destruye la variable anterior y crea una nueva con el mismo nombre pero con un valor diferente
    println!("El valor de x es: {}", x);
}

fn scalares() {
    //enteros
    let x: i8 = 127; // 8 bits -128 a 127
    let x: i16 = 32767; // 16 bits -32768 a 32767
    let x: i32 = 2147483647; // 32 bits -2147483648 a 2147483647
    let x: i64 = 9223372036854775807; // 64 bits -9223372036854775808 a 9223372036854775807
    let x: i128 = 170141183460469231731687303715884105727; // 128 bits -170141183460469231731687303715884105728 a 170141183460469231731687303715884105727

    //igual con los float
    let x: f32 = 3.4; // 32 bits
    let x: f64 = 3.4; // 64 bits
                      /*
                      solo se puede hacer operaciones con variables del mismo tipo
                      let x:i8 = x + 3.4; // error
                      let x:i8 = x + 3; // correcto
                      let x:f32 = x + 3.4; // correcto
                      let x:f32 = x + 3; // error
                      */
    let x: f64 = x + 3.4; // 128 bits

    //booleanos
    let x: bool = true;

    //caracteres
    let x: char = 'a';
}

/**
 *  Tipos compuestos
 *  - Tuplas: son una secuencia de elementos de longitud fija,coleccion de valores de diferentes tipos
 *  - Arreglos: son una secuencia de elementos de longitud fija, obligatorio que todos los elementos sean del mismo tipo
 *  - Slices: son una vista de un arreglo
 *  - Strings: son una secuencia de caracteres
 *  - Structs: son una secuencia de elementos de longitud fija
 *  - Enums: son una secuencia de elementos de longitud fija
 *  - Unions: son una secuencia de elementos de longitud fija
 *  - Option<T>: es un tipo generico que puede tener un valor o no
 *  - Result<T, E>: es un tipo generico que puede tener un valor o un error
 */
fn compound() -> i32 {
    //tuplas
    let x: (i32, f64, u8) = (500, 6.4, 1);

    //arrays
    let x: [i32; 5] = [1, 2, 3, 4, 5];
    let x = [3; 5]; // crea un arreglo de 5 elementos con el valor 3
    let val1: i32 = x[0]; // acceder a un elemento del arreglo
    let val2: i32 = x[1]; // acceder a un elemento del arreglo

    //ciclo for
    for i in 0..5 {
        println!("El valor de x es: {}", x[i]);
    }

    for i in x.iter() {
        println!("El valor de x es: {}", i);
    }

    //while
    let mut i: usize = 0;
    while i < 5 {
        i = i + 1;
    }
    loop {
        i = i + 1;
        if i == 5 {
            break;
        }
    }

    //slices
    let x = [1, 2, 3, 4, 5];
    let y = &x[1..3]; // crea un slice con los elementos 2 y 3
    let y = &x[..3]; // crea un slice con los elementos 1, 2 y 3
    let y = &x[1..]; // crea un slice con los elementos 2, 3, 4 y 5

    //strings
    let x = "Hola mundo";
    let x = String::from("Hola mundo");

    //structs
    struct User {
        username: String,
        email: String,
        age: i32,
    }

    let user = User {
        username: String::from("jose"),
        email: String::from("jose@example.com"),
        age: 30,
    };

    i // se puede retornar un valor de una funcion
}
