mod simulacion;
// mod graficar;
extern crate plotly;
use plotly::common::Mode;
use plotly::{Plot, Scatter};
use rayon::prelude::*; // usar paralelismo
use std::sync::{Arc, Mutex}; // para modificar archivos con paralelismo

const N_SIMULACIONES: f32 = 4000.;
const DT: f32 = 0.01;

fn main() {
    // creo donde guardo los datos para graficar
    let mut velocidades = Arc::new(Mutex::new(Vec::new()));
    let mut omegas = Arc::new(Mutex::new(Vec::new()));
    let mut contador_simulaciones_terminadas = Arc::new(Mutex::new(0f32));

    // uso esto para iterar 
    let mut periodos: Vec<f32> = vec![];
    
    // hago np.linspace(10, 20, 201) arcaico
    for i in 0..N_SIMULACIONES as usize {
        periodos.push(5. + i as f32 /N_SIMULACIONES*20.);
    }
    
    // corro muchas simulaciones con multithread con distintos periodos
    periodos.par_iter_mut().for_each(|periodo_segundos| {
        let mut simulacion1: simulacion::Simulacion = simulacion::Simulacion::new(*periodo_segundos, DT, false);
        let contador =Arc::clone(&contador_simulaciones_terminadas);
        loop {
            simulacion1.step();
            if simulacion1.simulacion_prendida == false {
                // obtengo los datos para graficar
                velocidades.lock().unwrap().push(simulacion1.autos[0].velocidad/simulacion1.autos[0].velocidad_maxima);
                omegas.lock().unwrap().push(simulacion1.omega);

                // digo cuantos me faltan
                let mut num = contador.lock().unwrap();
                *num += 1.;
                println!("simulacion terminada, ya van {}, periodo_segundos: {}", *num, periodo_segundos );
                break
            }
        }
    });
        


    // hago el grafico
    let trace = Scatter::new(omegas.lock().unwrap().to_vec(), velocidades.lock().unwrap().to_vec()).mode(Mode::Markers);
    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.show();
       
}





////////////////////////// GRAFICAR /////////////////////////////////////////////
// use macroquad::{prelude::*, color};

// use macroquad::ui::{
//     hash, root_ui,
//     widgets::{self, Group},
//     Drag, Ui,
// };

// #[macroquad::main("InputKeys")]
// async fn main() {
//     // creo la simulacion
//     let mut simulacion1: simulacion::Simulacion = simulacion::Simulacion::new(12.45, DT, false);

//     let mut x = screen_width() / 2.0;
//     let mut y = screen_height() / 2.0;

//     let mut color: char = 'n';

//     let mut SPF = 1; // SIMULATION PER FRAME
//     let mut data0 = "1".to_string(); // SIMULATION PER FRAME

//     loop {

//         clear_background(LIGHTGRAY);

//         set_camera(&Camera2D {
//             target: vec2(x, y),
//             zoom: vec2(0.005, -0.005),
//             ..Default::default()
//         });
        
//         for semaforo in simulacion1.semaforos {
//             x = semaforo.posicion;
//             if (semaforo.posicion - simulacion1.autos[0].posicion).abs() < 300. {
//                 if semaforo.luz == 'g' {draw_circle(x, y, 15.0, GREEN);}
//                 else if semaforo.luz == 'r' {draw_circle(x, y, 15.0, RED);}
//                 else if semaforo.luz == 'y' {draw_circle(x, y, 15.0, YELLOW);}
//                 draw_text(
//                     format!("{}", semaforo.semaforo_j).as_str(),
//                     x,
//                     y -30.,
//                     30.0,
//                     BLACK,
//                 );
//             }
//         }

//         for auto in simulacion1.autos {
//             x = auto.posicion;
//             // draw_circle(x, y, 15.0, YELLOW);
//             if auto.aceleracion >= 0.0 {
//                 draw_circle(x, y, 15.0, PURPLE);
//             }
//             else if auto.aceleracion == 0.0{
//                 draw_circle(x, y, 15.0, BLUE);
//             }
//             else {
//                 draw_circle(x, y, 15.0, ORANGE);
//             }
//             draw_rectangle(x, y+10., auto.distancia_frenado_semaforos, 10., BLACK);
//             draw_rectangle(x, y+20., auto.distancia_frenado_autos, 10., WHITE);

//             draw_text(
//                 format!("aceleracion: {}", auto.aceleracion).as_str(),
//                 x + 10.0,
//                 y + 48.0,
//                 20.0,
//                 BLACK,
//             );
//         }

//         draw_text("white: distancia frenado autos", x-100., y-120., 12.0, DARKGRAY);
//         draw_text("white: distancia frenado autos", x-100., y-120., 12.0, DARKGRAY);

//         // aqui modifico el framerate
//         if data0 == "".to_string(){
//             data0 = "0".to_string()
//         }
//         let mut simulacion_por_frame:i32 = data0.parse::<i32>().unwrap();
//         for i in 0..simulacion_por_frame{
//             simulacion1.step();
//         }

//         if simulacion1.simulacion_prendida == false {break;}




//         widgets::Window::new(hash!(), vec2(470., 50.), vec2(200., 100.))
//             .label("Simulation per frame")
//             .ui(&mut *root_ui(), |ui| {
                

//                 ui.input_text(hash!(), "<- input text 1", &mut data0);
//                 ui.label(
//                     None,
//                     &format!("Text entered: \"{}\"", data0),
//                 );
//             });


//         next_frame().await;
        
//     }
// }


