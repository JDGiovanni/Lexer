#[derive(PartialEq, Debug, Clone, Copy)]
pub enum EstadoAFD {
    Inicial,
    Identificador,
    Digito,
    Simbolo,
    Error,
    Aceptacion,
}

/// Cambia de estado segun el caracter actual (un paso del AFD)
pub fn ejecutar_transicion_afd(estado_actual: &EstadoAFD, c: char) -> EstadoAFD {
    match estado_actual {
        EstadoAFD::Inicial => {
            if c.is_alphabetic() || c == '_' {
                return EstadoAFD::Identificador;
            }
            if c.is_ascii_digit() {
                return EstadoAFD::Digito;
            }
            if es_simbolo(c) {
                return EstadoAFD::Simbolo;
            }
            EstadoAFD::Error
        }
        EstadoAFD::Identificador => {
            if c.is_alphanumeric() || c == '_' {
                return EstadoAFD::Identificador;
            }
            EstadoAFD::Aceptacion
        }
        EstadoAFD::Digito => {
            if c.is_ascii_digit() {
                return EstadoAFD::Digito;
            }
            EstadoAFD::Aceptacion
        }
        EstadoAFD::Simbolo => EstadoAFD::Aceptacion,
        _ => EstadoAFD::Error,
    }
}

fn es_simbolo(c: char) -> bool {
    matches!(
        c,
        '+' | '-' | '*' | '/' | '%' | '=' | '<' | '>' | ';' | ',' | '(' | ')' | '{' | '}'
            | '[' | ']'
    )
}
