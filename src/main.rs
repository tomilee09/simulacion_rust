mod simulacion;
// mod graficar;

// fn main() {
    // let mut simulacion1: simulacion::Simulacion = simulacion::Simulacion::new(60.0, 0.1);
    // // let mut iteracion = 0;
    // loop {
    //     if simulacion1.simulacion_prendida == false {
    //         println!("simulacion terminada");
    //         break
    //     }
    //     // if iteracion == 1000 {break}
    //     simulacion1.step();
    //     // iteracion  += 1; 
    // }
// }



// fn main() {
//     graficar::graficar()
// }

use macroquad::{prelude::*, color};

#[macroquad::main("InputKeys")]
async fn main() {
    // creo la simulacion
    let mut simulacion1: simulacion::Simulacion = simulacion::Simulacion::new(60.0, 0.1);
    // for semaforo in simulacion1.semaforos {
    //     println!("{}", semaforo.posicion)
    // }

    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;
    
    let mut color: char = 'n';

    let car: Texture2D = load_texture("src/car.png").await.unwrap();




    loop {

        clear_background(LIGHTGRAY);

        set_camera(&Camera2D {
            target: vec2(x, y),
            zoom: vec2(0.005, -0.005),
            ..Default::default()
        });

        draw_text("move the ball with arrow keys", 20.0, 20.0, 20.0, DARKGRAY);
        
        for semaforo in simulacion1.semaforos {
            x = semaforo.posicion;
            if semaforo.luz == 'g' {draw_circle(x, y, 15.0, GREEN);}
            else {draw_circle(x, y, 15.0, RED);}
        }

        for auto in simulacion1.autos {
            x = auto.posicion;
            draw_texture_ex(car, x- car.width() / 2., y- car.height() / 2., RED, DrawTextureParams { dest_size: vec2(15., 15.), ..Default::default()});
            // draw_circle(x, y, 15.0, YELLOW);
        }

        
        simulacion1.step();

        // set_default_camera();
        
        next_frame().await;
        
    }
}


