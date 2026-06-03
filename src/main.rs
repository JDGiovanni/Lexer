mod estructuras;
mod input;
mod token;
mod reportes;

use reportes::GestorReportes;
fn main() {
    let mut gestor = GestorReportes::new();
    
    let ejemplo = token::Token::nuevo(token::TipoToken::Keyword, "int", 1, 1);
    println!("Contrato Token: {ejemplo}");

    gestor.registrar_token(ejemplo);

    gestor.imprimir_reporte_final();
}
