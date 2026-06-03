#[derive(PartialEq, Debug)]
pub enum EstadoAFD {
    Inicial,
    Identificador,
    Digito,
    Operador,
    Error,
    Aceptacion,
}

pub fn ejecutar_transicion_afd(estado_actual: &EstadoAFD, c: char) -> EstadoAFD {
    match estado_actual {
        EstadoAFD::Inicial => {
            if c.is_alphabetic() { return EstadoAFD::Identificador; }
            if c.is_digit(10)    { return EstadoAFD::Digito; }
            if c == '+' || c == '-' || c == '*' || c == '=' { return EstadoAFD::Operador; }
            EstadoAFD::Error
        }

        EstadoAFD::Identificador => {
            if c.is_alphanumeric() || c == '_' { return EstadoAFD::Identificador; }
            EstadoAFD::Aceptacion
        }

        EstadoAFD::Digito => {
            if c.is_digit(10) { return EstadoAFD::Digito; }
            EstadoAFD::Aceptacion
        }

        EstadoAFD::Operador => {
            EstadoAFD::Aceptacion
        }

        _ => EstadoAFD::Error,
    }
}