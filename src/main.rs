#[derive(Debug)]
struct Suscriptor {
    es_iteligente: bool,
    nombre: String,
    beautifull: bool,
}
struct RGBColor(i32, i32, i32);

fn se_suscribe(nombre: String) -> Suscriptor {
    Suscriptor {
        es_iteligente: true,
        nombre,
        beautifull: true,
    }
}

impl Suscriptor {
    fn se_desuscribe(&mut self, nuevo_nombre: String) {
        self.nombre = nuevo_nombre;
        self.es_iteligente = false;
        self.beautifull = false;
    }

    fn new_subscriber(nombre: String) -> Suscriptor {
        Suscriptor {
            es_iteligente: true,
            nombre,
            beautifull: true,
        }
    }
}

fn main() {
    let suscriptor: Suscriptor = se_suscribe(String::from("Juan"));

    let mut segund_test: Suscriptor = Suscriptor {
        nombre: String::from("Pedro"),
        ..suscriptor
    };

    segund_test.se_desuscribe(String::from("Pedro"));
    let hola = Suscriptor::new_subscriber(String::from("Juan")); // se puede llamar a un metodo de la estructura sin instanciarla , ya que es un metodo asociado

    println!(
        "El suscriptor {} es inteligente? {}",
        suscriptor.nombre, suscriptor.es_iteligente
    );
    println!(
        "El suscriptor {} es hermoso? {}",
        suscriptor.nombre, suscriptor.beautifull
    );
    let color: RGBColor = RGBColor(255, 0, 0);
}
