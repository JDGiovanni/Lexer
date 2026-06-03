//! Elida — Integrante 3: tabla de símbolos, cola de tokens y gestor de integración.

pub mod cola;
pub mod gestor;
pub mod hash_map;

pub use cola::ColaTokens;
pub use gestor::GestorEstructuras;
pub use hash_map::{EntradaSimbolo, ResultadoClasificacion, TablaSimbolos};
