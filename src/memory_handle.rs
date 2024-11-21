fn main() {
    let hola: &str = "Hola mundo"; //hola se crea en la pila y al ser inmutable ,va a stack
                                   //&str es un tipo de dato que representa una referencia a un string,usa un slice de un string
    println!("{}", hola); //se imprime el valor de hola
                          //hola deja  de estar en el scope,se elimina de la pila

    //sigo con mi codigo

    let s: str = String::from("Hola mundo"); //s se crea en la pila y al ser inmutable,va al stack
    let mut s = String::from("Hola mundo"); //s se crea en la pila y al ser mutable,va al heap
                                            //String::from("Hola mundo") crea un string en el heap y devuelve un puntero a ese string

    // Move
    let var1 = 1; //var1 se crea en la pila y al ser inmutable,va al stack
    let var2 = var1; //var2 se crea en la pila y al ser inmutable,va al stack
    println!("var1: {}", var1); //se imprime el valor de var1

    //aqui esto se copia en cambio en el caso de los strings no se copia
    let s1 = String::from("Hola mundo"); //s1 se crea en la pila y al ser mutable,va al heap
    let s2 = s1; //s2 se crea en la pila y al ser mutable,va al heap
    println!("s1: {}", s1); //se imprime el valor de s1


    // Clone
    let s1 = String::from("Hola mundo"); //s1 se crea en la pila y al ser mutable,va al heap
    let s2 = s1.clone(); //s2 se crea en la pila y al ser mutable,va al heap
    println!("s1: {}", s1); //se imprime el valor de s1

    // esto daria error
    // let sy:string=string::from("hola mundo");
    // let sy: String = String::from("hola mundo");
    // let sc: String = nuevo_return(sy); //sy desaparece de la memoria
    // println!("sy: {}", sy); //se imprime el valor de sy

    let sa: String = String::from("Hola mundo"); //sa se crea en la pila y al ser mutable,va al heap
    let sa_len: usize = calcular_longitud(sting:&sa); // se pasa la referencia de sa para no perder la propiedad de sa
    // sa vuvelva a ser duena de "sy"
    println!("sa: {} a respuesta  {}", sa,sa_len); //se imprime el valor de sa


}

fn nuevo_return(sting: &String) -> String { // le pasamos la referencia de sting retorno el valor de sting ya que es un prestamo
    s //se retorna el valor de s y se elimina de la memoria,hace un move de s para que no se pueda usar despues
}
