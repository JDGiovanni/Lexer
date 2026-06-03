mod estructuras;
mod input;
mod token;
mod reportes;

use estructuras::GestorEstructuras;
use reportes::GestorReportes;
use token::{TipoToken, Token};

fn main() {
    // Demo: mi parte genera tokens
    let mut gestor = GestorEstructuras::nuevo();
    gestor.procesar_lexema("int", 1, 1);
    gestor.procesar_lexema("suma", 2, 5);
    gestor.encolar_token(Token::nuevo(TipoToken::LiteralNumber, "42", 3, 1));

    // Pasa tokens al reporte de Jhonmar
    let mut reportes = GestorReportes::nuevo();
    while let Some(token) = gestor.cola.desencolar() {
        reportes.registrar_token(token);
    }
    reportes.imprimir_reporte_final();
}
