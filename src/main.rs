mod estructuras;
mod input;
mod lexer;
mod token;
mod reportes;

use estructuras::GestorEstructuras;
use input::InputStream;
use lexer::analizar_fuente;
use reportes::GestorReportes;

/// Archivo de prueba: debe estar en la raiz del proyecto (junto a Cargo.toml)
const ARCHIVO_FUENTE: &str = "codigo_fuente.txt";

fn main() {
    let contenido = std::fs::read_to_string(ARCHIVO_FUENTE)
        .unwrap_or_else(|_| panic!("No se encontro {ARCHIVO_FUENTE} en la raiz del proyecto"));

    let mut scanner = InputStream::new(&contenido);
    let mut gestor = GestorEstructuras::nuevo();

    analizar_fuente(&mut scanner, &mut gestor);

    let mut reportes = GestorReportes::nuevo();
    while let Some(token) = gestor.cola.desencolar() {
        reportes.registrar_token(token);
    }
    reportes.imprimir_reporte_final();
}
