//! Elida: tabla de símbolos y cola de tokens.
//!
//! `cola` y `hash_map` se adaptan desde la librería 

pub mod cola;
pub mod hash_map;

pub use cola::ColaTokens;
pub use hash_map::{EntradaSimbolo, ResultadoClasificacion, TablaSimbolos};
