// Elida — une tabla de símbolos y cola; el autómata usa esto para validar y encolar lexemas.

use crate::token::{Token, TipoToken, PALABRAS_RESERVADAS};

use super::cola::ColaTokens;
use super::hash_map::{ResultadoClasificacion, TablaSimbolos};

pub struct GestorEstructuras {
    pub tabla: TablaSimbolos,
    pub cola: ColaTokens,
}

impl GestorEstructuras {
    pub fn nuevo() -> Self {
        let mut gestor = Self {
            tabla: TablaSimbolos::nueva(),
            cola: ColaTokens::nueva(),
        };
        gestor.cargar_palabras_reservadas();
        gestor
    }

    // Registra las reservadas del lenguaje al iniciar (void, int, if, etc.).
    fn cargar_palabras_reservadas(&mut self) {
        for palabra in PALABRAS_RESERVADAS {
            self.tabla.insertar(
                palabra.to_string(),
                TipoToken::Keyword,
                "global",
            );
        }
    }

    /// Valida si el lexema es reservada o identificador, crea el token y lo encola.
    pub fn procesar_lexema(
        &mut self,
        lexema: &str,
        linea: usize,
        columna: usize,
    ) -> ResultadoClasificacion {
        let resultado = self.tabla.clasificar_lexema(lexema);
        let tipo = self.tabla.tipo_para_lexema(lexema);
        self.cola
            .encolar(Token::nuevo(tipo, lexema, linea, columna));
        resultado
    }

    /// Encola tokens que ya vienen clasificados (números, strings, operadores, etc.).
    pub fn encolar_token(&mut self, token: Token) {
        self.cola.encolar(token);
    }

    pub fn ver_salida(&self) {
        self.tabla.ver_tabla();
        self.cola.ver_cola();
    }
}
