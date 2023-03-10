mod simulacion;
// mod graficar;

fn main() {


    let mut simulacion1: simulacion::Simulacion = simulacion::Simulacion::new(60.0, 0.1);
    // let mut iteracion = 0;
    loop {
        if simulacion1.simulacion_prendida == false {
            println!("simulacion terminada");
            break
        }
        // if iteracion == 1000 {break}
        simulacion1.step();
        // iteracion  += 1; 
    }
}





// fn main() {
//     graficar::graficar()
// }