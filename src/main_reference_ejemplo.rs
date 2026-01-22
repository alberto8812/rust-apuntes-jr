fn main() {
    let mut name: String = String::new(); //se crea un string vacio en el heap
    println!("Ingrese su nombre: ");
    std::io::stdin().read_line(&mut name).unwrap(); //se lee el nombre ingresado por el usuario y se almacena en name
                                                    //unwrap() maneja el error que se puede producir al leer la entrada del usuario
                                                    //stdin() es una funcion que lee la entrada del usuario
                                                    //read_line() es una funcion que lee la entrada del usuario y la almacena en un string
                                                    //io es un modulo que contiene funciones para manejar la entrada y salida de datos
                                                    //std es un modulo que contiene funciones estandar
                                                    //buf es un buffer que almacena la entrada del usuario

    let name_len: usize = calcular_longitud(&name); //se calcula la longitud de name y se almacena en name_len
    add_to_string(&mut name); //se le agrega " mundo" a name que sigue siendo dueño del valor
    println!("Hola, {}! Tu nombre tiene {} letras", name, name_len); //se imprime el nombre y la longitud del nombre
}

fn add_to_string(s: &mut String) {
    s.push_str(" mundo"); //se le agrega " mundo" a s
}

fn calcular_longitud(s: &String) -> usize {
    s.len() //se retorna la longitud de string
}
