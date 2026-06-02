// Elida — cola de tokens adaptada desde LibreriaDeSoporte::ds::queue (VecDeque).

use std::collections::VecDeque;

use crate::token::Token;

pub struct ColaTokens {
    elementos: VecDeque<Token>,
}

impl ColaTokens {
    pub fn nueva() -> Self {
        Self {
            elementos: VecDeque::new(),
        }
    }

    pub fn encolar(&mut self, token: Token) {
        self.elementos.push_back(token);
    }

    pub fn desencolar(&mut self) -> Option<Token> {
        self.elementos.pop_front()
    }

    pub fn esta_vacia(&self) -> bool {
        self.elementos.is_empty()
    }

    pub fn cantidad(&self) -> usize {
        self.elementos.len()
    }

    /// Muestra el flujo de tokens en orden (tipo, valor, línea, columna).
    pub fn ver_cola(&self) {
        println!("---- Flujo de tokens ----");
        if self.esta_vacia() {
            println!("(vacía)");
        } else {
            for token in &self.elementos {
                println!("{token}");
            }
        }
        println!("--------------------------");
    }
}
