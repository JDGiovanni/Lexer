pub mod automata;

use automata::{EstadoAFD, ejecutar_transicion_afd};

use crate::estructuras::GestorEstructuras;
use crate::input::InputStream;
use crate::token::{Token, TipoToken};

/// Recorre el archivo completo y encola todos los tokens
pub fn analizar_fuente(scanner: &mut InputStream<'_>, gestor: &mut GestorEstructuras) {
    loop {
        scanner.saltar_espacios();
        if scanner.mirar_siguiente().is_none() {
            break;
        }
        if !siguiente_token(scanner, gestor) {
            break;
        }
    }
}

/// Reconoce un token: salta espacios, lee caracteres con el AFD, clasifica y encola
pub fn siguiente_token(scanner: &mut InputStream<'_>, gestor: &mut GestorEstructuras) -> bool {
    scanner.saltar_espacios();
    let Some(&c) = scanner.mirar_siguiente() else {
        return false;
    };

    let linea_inicio = scanner.linea;
    let columna_inicio = scanner.columna;

    let mut estado = ejecutar_transicion_afd(&EstadoAFD::Inicial, c);
    if estado == EstadoAFD::Error {
        let ch = scanner.avanzar().unwrap();
        gestor.encolar_token(Token::nuevo(
            TipoToken::Error("caracter no reconocido".to_string()),
            ch.to_string(),
            linea_inicio,
            columna_inicio,
        ));
        return true;
    }

    let mut lexema = String::new();
    let ch = scanner.avanzar().unwrap();
    lexema.push(ch);

    loop {
        let Some(&siguiente) = scanner.mirar_siguiente() else {
            break;
        };
        let nuevo = ejecutar_transicion_afd(&estado, siguiente);
        if nuevo == EstadoAFD::Aceptacion {
            break;
        }
        if nuevo == EstadoAFD::Error {
            break;
        }
        let leido = scanner.avanzar().unwrap();
        lexema.push(leido);
        estado = nuevo;
    }

    if lexema.is_empty() {
        return false;
    }

    clasificar_y_encolar(gestor, &lexema, linea_inicio, columna_inicio);
    true
}

/// Tipo segun lexema completo: "int" se compara en la tabla de reservadas
fn clasificar_y_encolar(
    gestor: &mut GestorEstructuras,
    lexema: &str,
    linea: usize,
    columna: usize,
) {
    let primer = lexema.chars().next().unwrap();
    if primer.is_alphabetic() || primer == '_' {
        gestor.procesar_lexema(lexema, linea, columna);
    } else if lexema.chars().all(|c| c.is_ascii_digit()) {
        gestor.encolar_token(Token::nuevo(
            TipoToken::LiteralNumber,
            lexema,
            linea,
            columna,
        ));
    } else if lexema.len() == 1 && es_puntuador(primer) {
        gestor.encolar_token(Token::nuevo(
            TipoToken::Punctuator,
            lexema,
            linea,
            columna,
        ));
    } else {
        gestor.encolar_token(Token::nuevo(
            TipoToken::Operator,
            lexema,
            linea,
            columna,
        ));
    }
}

fn es_puntuador(c: char) -> bool {
    matches!(c, ';' | ',' | '(' | ')' | '{' | '}' | '[' | ']')
}
