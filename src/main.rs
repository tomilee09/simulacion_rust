mod simulacion;
// mod graficar;
use std::thread;
extern crate plotly;
use plotly::common::Mode;
use plotly::{Plot, Scatter};
const N_SIMULACIONES: f32 = 202.;

fn main() {
    // creo donde guardo los datos para graficar
    let mut velocidades: Vec<f32> = vec![];
    let mut omegas: Vec<f32> = vec![];

    // uso esto para iterar 
    let mut periodos: Vec<f32> = vec![];
    
    // hago np.linspace(10, 20, 201) arcaico
    for i in 0..N_SIMULACIONES as usize {
        periodos.push(10. + i as f32 /N_SIMULACIONES*10.);
        println!("{}", 10. + i as f32 /N_SIMULACIONES*10.)
    }

    // // creo este vector para hacer un thread pool
    // let mut handle = vec!();
    
    // corro muchas simulaciones con multithread con distintos periodos
    for periodo_segundos in periodos {
        let mut simulacion1: simulacion::Simulacion = simulacion::Simulacion::new(periodo_segundos, 0.1);
        loop {
            if simulacion1.simulacion_prendida == false {
                velocidades.push(simulacion1.autos[0].velocidad/simulacion1.autos[0].velocidad_maxima);
                omegas.push(simulacion1.omega);

                println!("simulacion terminada");
                break
            }
            simulacion1.step();
        }
    }

    // hago el grafico
    let trace = Scatter::new(omegas, velocidades).mode(Mode::Markers);
    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.show();

    
}





////////////////////////// GRAFICAR /////////////////////////////////////////////
// use macroquad::{prelude::*, color};

// #[macroquad::main("InputKeys")]
// async fn main() {
//     // creo la simulacion
//     let mut simulacion1: simulacion::Simulacion = simulacion::Simulacion::new(60.0, 0.1);
//     // for semaforo in simulacion1.semaforos {
//     //     println!("{}", semaforo.posicion)
//     // }

//     let mut x = screen_width() / 2.0;
//     let mut y = screen_height() / 2.0;
    
//     let mut color: char = 'n';

//     loop {

//         clear_background(LIGHTGRAY);

//         set_camera(&Camera2D {
//             target: vec2(x, y),
//             zoom: vec2(0.005, -0.005),
//             ..Default::default()
//         });

//         draw_text("Run!", x-10., y-20., 20.0, DARKGRAY);
        
//         for semaforo in simulacion1.semaforos {
//             x = semaforo.posicion;
//             if semaforo.luz == 'g' {draw_circle(x, y, 15.0, GREEN);}
//             else {draw_circle(x, y, 15.0, RED);}
//         }

//         for auto in simulacion1.autos {
//             x = auto.posicion;
//             // draw_circle(x, y, 15.0, YELLOW);
//             println!("{}", auto.accion);
//             if auto.accion >= 0.0 {
//                 draw_circle(x, y, 15.0, YELLOW);
//             }
//             else {
//                 draw_circle(x, y, 15.0, RED);
//             }
//         }

//         simulacion1.step();

//         if simulacion1.simulacion_prendida == false {break;}

//         next_frame().await;
        
//     }
// }


