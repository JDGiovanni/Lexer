mod estructuras;
mod input;
mod token;

use estructuras::GestorEstructuras;
use token::{TipoToken, Token};

fn main() {
    let mut gestor = GestorEstructuras::nuevo();

    gestor.procesar_lexema("int", 1, 1);
    gestor.procesar_lexema("suma", 2, 5);
    gestor.encolar_token(Token::nuevo(TipoToken::LiteralNumber, "42", 3, 1));

    gestor.ver_salida();
}
