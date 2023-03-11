extern crate array_init;
use array_init::array_init;
// use std::thread;

// valores iniciales importantes    
const N_AUTOS: usize = 1;
const N_SEMAFOROS: usize = 100;


// Clase semaforo
pub struct Semaforo {
    pub posicion: f32,
    pub luz: char,
    semaforo_j: i32,
}
impl Copy for Semaforo {}
impl Clone for Semaforo {
    fn clone(&self) -> Semaforo {
        Semaforo{posicion: self.posicion, luz: self.luz, semaforo_j:self.semaforo_j}
    }
}
impl Semaforo {
    fn new() -> Semaforo {
        Semaforo {posicion: 200.0, luz: 'g', semaforo_j: 0}
    }
}
impl Semaforo {
    fn change_color(&mut self, iteracion:f32, periodo:f32) {
        if iteracion%(periodo/2.0) < 0.01 {
            if self.luz == 'g'{
                self.luz = 'r';
            }
            else {
                self.luz = 'g';
            }
        }
    }
}
// Clase autos
pub struct Auto {
    // datos internos 
    prendido: bool,
    pub posicion: f32,
    pub velocidad: f32, // para hacer el grafico
    aceleracion: f32,
    modulo_aceleracion: f32,
    auto_i: i32,
    factor_frenado: f32,
    pub velocidad_maxima: f32, // para normalizar el grafico
    distancia_no_pegado: f32,
    tiempo_reaccion: f32,
    distancia_reaccion: f32,
    distancia_maniobra_autos: f32,
    distancia_maniobra_semaforos: f32,
    distancia_frenado_autos: f32,
    distancia_frenado_semaforos: f32,

    // semaforo obstaculo actual
    semaforo_j: i32,

    // valores de distancia dentro de la simulacion
    dist_auto_obstaculo: f32,
    velocidad_relativa_auto_obstaculo: f32,
    dist_semaforo_obstaculo:f32,

    // la accion
    pub accion: f32,
}
impl Copy for Auto {}
impl Clone for Auto {
    fn clone(&self) -> Auto {
        Auto { prendido: self.prendido, 
            posicion: self.posicion, 
            velocidad: self.velocidad, 
            aceleracion: self.aceleracion, 
            modulo_aceleracion: self.modulo_aceleracion, 
            auto_i: self.auto_i, 
            factor_frenado: self.factor_frenado, 
            velocidad_maxima: self.velocidad_maxima, 
            distancia_no_pegado: self.distancia_no_pegado, 
            tiempo_reaccion: self.tiempo_reaccion, 
            distancia_reaccion: self.distancia_reaccion, 
            distancia_maniobra_autos: self.distancia_maniobra_autos, 
            distancia_maniobra_semaforos: self.distancia_maniobra_semaforos, 
            distancia_frenado_autos: self.distancia_frenado_autos, 
            distancia_frenado_semaforos: self.distancia_frenado_semaforos, 
            semaforo_j: self.semaforo_j, 
            dist_auto_obstaculo: self.dist_auto_obstaculo,
            velocidad_relativa_auto_obstaculo: self.velocidad_relativa_auto_obstaculo,
            dist_semaforo_obstaculo: self.dist_semaforo_obstaculo,
            accion: self.accion,
        
        }
    }
}
impl Auto {
    fn new() -> Auto {
        Auto {
            // datos internos 
            prendido: true,
            posicion: 0.0,
            velocidad: 0.0,
            aceleracion: 0.0,
            modulo_aceleracion: 1.0,
            auto_i: 0,
            factor_frenado: 3.0,
            velocidad_maxima: 10.0,
            distancia_no_pegado: 1.0,
            tiempo_reaccion: 0.5,
            distancia_reaccion: 0.0,
            distancia_maniobra_autos: 0.0,
            distancia_maniobra_semaforos: 0.0,
            distancia_frenado_autos: 0.0,
            distancia_frenado_semaforos: 0.0,
            // semaforo obstaculo actual
            semaforo_j: 0,
            // valores de distancia dentro de la simulacion
            dist_auto_obstaculo: 0.0,
            velocidad_relativa_auto_obstaculo: 0.0,
            dist_semaforo_obstaculo: 0.0,
            // accion
            accion:1.
        }
    }
}
impl Auto {
    fn maneja(&mut self, lista_autos: [Auto; N_AUTOS], lista_semaforos: [Semaforo; N_SEMAFOROS], dt: f32, amarillo: bool) {
        if self.prendido{
            // Calculo distancia siguiente auto //////
            // distancia hasta el auto el cual es un obstaculo ahora
            if self.auto_i != N_AUTOS as i32 -1 && lista_autos[(self.auto_i+1) as usize].prendido != false{
                self.dist_auto_obstaculo = lista_autos[(self.auto_i+1) as usize].posicion - self.posicion;}
            else{
                self.dist_auto_obstaculo = 10000000.0;} // numero muy grande para que se note que no tiene un auto al frente
            
            // Calculo velocidad relativa entre los autos 
            if self.auto_i as i32 != N_AUTOS as i32 -1 {
                self.velocidad_relativa_auto_obstaculo = lista_autos[(self.auto_i+1) as usize].velocidad - self.velocidad;}
            else {
                self.velocidad_relativa_auto_obstaculo = 0.0; }
            // Calculo distancia siguiente semaforo 
            // distancia con el siguiente semaforo
            self.dist_semaforo_obstaculo = lista_semaforos[self.semaforo_j as usize].posicion - self.posicion;
   
            ////// hago cosas con los semaforos //////
            // si adelanta al semaforo, usar como semaforo actual al siguiente semaforo
            if self.dist_semaforo_obstaculo < 0.0 {
                self.semaforo_j += 1; 
                
                let n_semf:i32 = N_SEMAFOROS as i32;
                if self.semaforo_j == n_semf {
                    self.prendido = false;
                    return
                }

                // descomentar para comprobar que funcionan los semaforos
                // if self.semaforo_obstaculo.luz == "G":
                //     print("SIIIIIIII")
                if lista_semaforos[self.semaforo_j as usize].luz == 'r' {
                    println!("NOOOOOOOO"); // hay algunos que pasan en rojo
                    // si se pasan todos los semaforos deten el auto
                }
            }
            
            //////////////////////////// AQUI PONER EL COMO SE DEBE MANEJAR //////////////////////////////////////////////////
            // accion puede tomar valores de -3, 0, 1, o sea, frena, nada, acelera
            self.accion = 1.0;
            
            ////// acciones relacionadas con otros autos //////
            self.distancia_reaccion = self.tiempo_reaccion * self.velocidad;
            self.distancia_maniobra_autos = ((self.velocidad-self.velocidad_relativa_auto_obstaculo).powi(2))/(self.modulo_aceleracion*self.factor_frenado*2.0);
            self.distancia_frenado_autos = self.distancia_reaccion + self.distancia_maniobra_autos + self.distancia_no_pegado;
            if self.dist_auto_obstaculo < self.distancia_frenado_autos {
                self.accion = -1.0*self.factor_frenado;
            }
            ////// acciones relacionadas con los semaforos //////
            // frenado con un semaforo si no aparece de repente el rojo
            self.distancia_maniobra_semaforos = (self.velocidad.powi(2))/(self.modulo_aceleracion*self.factor_frenado.powi(2));
            self.distancia_frenado_semaforos = self.distancia_maniobra_semaforos + self.distancia_reaccion + self.distancia_no_pegado;
            if self.dist_semaforo_obstaculo < self.distancia_frenado_semaforos && lista_semaforos[self.semaforo_j as usize].luz == 'r' {
                self.accion = -1.0*self.factor_frenado;
            }
            // println!("{}", self.dist_semaforo_obstaculo );
            // println!("{}", self.distancia_frenado_semaforos);
            // println!("{}", lista_semaforos[0].luz);
            // println!("{}", accion);
            // thread::sleep_ms(1000);

            //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
            
            ////// Calculo del movimiento del auto //////
            self.aceleracion = self.accion * self.modulo_aceleracion;
            // println!("{}", self.aceleracion);
            // println!("{}", self.semaforo_obstaculo.posicion);
            // thread::sleep_ms(1000);


            // calculo de velocidad
            self.velocidad = self.velocidad + self.aceleracion*dt;
            if self.velocidad > self.velocidad_maxima { // implemento limite de velocidad
                self.velocidad = self.velocidad_maxima;
                self.aceleracion = 0.0;
            }
            if self.velocidad < 0.0 {
                self.velocidad = 0.0;
                self.aceleracion = 0.0;
            }
            // calculo la posicion
            self.posicion = self.posicion + self.velocidad*dt + self.aceleracion*dt.powi(2)/2.0; //- roce*dt
        }
    }
}


pub struct Simulacion {
    // tiempo
    t_0: f32,
    dt: f32, 
    // tamaño
    separacion_semaforos:f32,
    size: f32,
    // creo una lista con todos los semaforos
    pub semaforos: [Semaforo; N_SEMAFOROS],
    // creo una lista con todos los autos
    pub autos:[Auto; N_AUTOS],
    // cantidad de iteraciones realizadas
    iteracion:i32,
    // duracion del ciclo de semaforos
    periodo:f32,
    // para apagar la simulacion
    pub simulacion_prendida:bool,

    pub omega: f32,
}

impl Simulacion {
    pub fn new(periodo_segundos: f32, dt: f32) -> Simulacion {
        // tamaño
        let separacion_semaforos:f32 = 200.0;
        let size: f32 = N_SEMAFOROS as f32 * separacion_semaforos;
        
        // posicion semaforos
        let mut posicion_semaforos:[f32; N_SEMAFOROS] = [0.0; N_SEMAFOROS]; // np.linspace(100, size, N_SEMAFOROS)
        for i in 0..N_SEMAFOROS {
            posicion_semaforos[i] = (i as f32 +1.0)*separacion_semaforos as f32;
        }
        
        // creo una lista con todos los semaforos
        let mut semaforos:[Semaforo; N_SEMAFOROS] = array_init(|_| Semaforo::new());
        for (i, posicion) in posicion_semaforos.iter().enumerate() {
            let mut semaforo_n = Semaforo {
                posicion: *posicion,
                luz: 'g',
                semaforo_j: i as i32,
            };
            semaforos[i as usize] = semaforo_n;
        }

        // posicion inicial de los autos
        let posicion_autos:[f32; N_AUTOS] = [0.0; N_AUTOS];
        // creo una lista con todos los autos
        let auto_comun:Auto = Auto::new();
        let mut autos:[Auto; N_AUTOS] = array_init(|_| Auto::new());
        for (i, posicion) in posicion_autos.iter().enumerate() {
            let mut auto_n:Auto = Auto {
                auto_i: i as i32,
                posicion: *posicion,
                ..auto_comun
            };
            autos[i as usize] = auto_n;
        }



        // les calculo el semaforo obstaculo inicial a los autos
        for mut auto in autos {
            let mut dist_semaforos:f32 = -1.0;
            let mut j = 0;
            while dist_semaforos <= 0.0 {
                dist_semaforos = semaforos[j].posicion-auto.posicion;
                j += 1;
            }
            auto.semaforo_j = j as i32 - 1;
        }
        
        for mut auto in autos {
        // println!("{}", auto.semaforo_obstaculo.posicion)
        }


        // duracion del ciclo de semaforos
        let periodo:f32 = (1.0/dt)*periodo_segundos; // 60 segundos

        // calculo omega
        let T_c:f32 = separacion_semaforos/autos[0].velocidad_maxima;
        let P = dt*periodo;
        let omega = T_c/P; 

        // esto es lo que realmente me interesa
        let mut datos:Simulacion = Simulacion { 
            t_0: 0.0, 
            dt: dt,
            separacion_semaforos: 200.0, 
            size: size, 
            semaforos: semaforos, 
            autos: autos, 
            iteracion: 0, 
            periodo: periodo, 
            simulacion_prendida: true,
            omega: omega };

        return datos
    }
}




impl Simulacion {
    pub fn step(&mut self) {
        let autos_copy = self.autos; // para que todos los autos usen el mismo frame, y no vaya cambiando con el movimiento de los autos anteriores
        for mut auto in self.autos.iter_mut() {
            auto.maneja(autos_copy, self.semaforos, self.dt, false);
        }
        self.iteracion += 1;

        for mut semaforo in self.semaforos.iter_mut() {
            semaforo.change_color(self.iteracion as f32, self.periodo);
        }
        // println!("{}", self.iteracion);
        if self.autos[0].prendido == false {
            self.simulacion_prendida = false;
        }
    }
}