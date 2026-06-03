pub mod automata;
use automata::{EstadoAFD, ejecutar_transicion_afd};

use crate::estructuras::GestorEstructuras;
use crate::token::{Token, TipoToken};

pub fn siguiente_token(gestor: &mut GestorEstructuras) {
    let mut estado_actual = EstadoAFD::Inicial;
    let mut lexema_actual = String::new();
    
    let linea = 1;   // Temporal para que compile
    let columna = 1; // Temporal para que compile

    while estado_actual != EstadoAFD::Aceptacion && estado_actual != EstadoAFD::Error {

        let c = ' '; 

        let nuevo_estado = ejecutar_transicion_afd(&estado_actual, c);

        if nuevo_estado == EstadoAFD::Aceptacion {
            // Aquí le dices al Integrante 1 (Scanner) que retroceda un espacio
            break; 
        }

        estado_actual = nuevo_estado;

        if estado_actual != EstadoAFD::Error {
            lexema_actual.push(c);
        }
    }

    match estado_actual {
        EstadoAFD::Aceptacion => {
            
            if c.is_alphabetic() {
                gestor.procesar_lexema(lexema_actual, linea, columna);
            } else {
                
                let tipo = TipoToken::NUM; // Esto dependerá de si es Dígito u Operador
                let token = Token::nuevo(tipo, lexema_actual, linea, columna);
                gestor.encolar_token(token);
            }
        }
        EstadoAFD::Error => {

        }
        _ => {}
    }
}