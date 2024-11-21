fn main() {
    cacluar_factorial(5);
    es_primo(5);
}

/**
 * u128 -> unsigned 128 bits  son numeros enteros positivos
 */

fn cacluar_factorial(n: u128) -> u128 {
    if n == 0 || n == 1 {
        1
    } else {
        let mut result: u128 = number;
        //el for iterativo tomará el valor de number y lo decrementará hasta llegar a 0,invirtiendo el valor de result
        for i in (1..number).rev() {
            result = result * i;
        }
        result
    }
}

fn es_primo(number: u128) -> bool {
    let es_primo = true;
    let sum: f64 = number as f64;
    if number > 1 {
        for i in 2..((number.sqrt() as i128) + 1) {
            if (number as i128) % i == 0 {
                es_primo = false;
                break;
            }
        }
    }
    es_primo
}
