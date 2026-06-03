// Cola de tokens — usa Queue<T> de la libreria directamente

use LibreriaDeSoporte::ds::queue::Queue;

use crate::token::Token;

pub struct ColaTokens {
    inner: Queue<Token>,
}

impl ColaTokens {
    /// Crea cola vacia (Queue de la libreria)
    pub fn nueva() -> Self {
        Self {
            inner: Queue::new(),
        }
    }

    /// Agrega token al final
    pub fn encolar(&mut self, token: Token) {
        self.inner.push_back(token);
    }

    /// Saca el primer token
    pub fn desencolar(&mut self) -> Option<Token> {
        self.inner.pop_front()
    }

    /// True si no hay tokens pendientes
    pub fn esta_vacia(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn cantidad(&self) -> usize {
        self.inner.len()
    }

    /// Muestra tokens en orden (depuracion)
    pub fn ver_cola(&self) {
        println!("---- Flujo de tokens ----");
        if self.esta_vacia() {
            println!("(vacia)");
        } else {
            self.inner.ver_cola();
        }
        println!("--------------------------");
    }
}
