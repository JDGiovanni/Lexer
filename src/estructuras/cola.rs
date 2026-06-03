// Cola de tokens — adaptada de la libreria (VecDeque)

use std::collections::VecDeque;

use crate::token::Token;

pub struct ColaTokens {
    elementos: VecDeque<Token>,
}

impl ColaTokens {
    /// Crea cola vacia
    pub fn nueva() -> Self {
        Self {
            elementos: VecDeque::new(),
        }
    }

    /// Agrega token al final
    pub fn encolar(&mut self, token: Token) {
        self.elementos.push_back(token);
    }

    /// Saca el primer token
    pub fn desencolar(&mut self) -> Option<Token> {
        self.elementos.pop_front()
    }

    pub fn esta_vacia(&self) -> bool {
        self.elementos.is_empty()
    }

    pub fn cantidad(&self) -> usize {
        self.elementos.len()
    }

    /// Muestra tokens en orden
    pub fn ver_cola(&self) {
        println!("---- Flujo de tokens ----");
        if self.esta_vacia() {
            println!("(vacia)");
        } else {
            for token in &self.elementos {
                println!("{token}");
            }
        }
        println!("--------------------------");
    }
}
