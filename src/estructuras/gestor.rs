// Une tabla de simbolos (Tabla libreria) y cola (Queue libreria)

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

    /// Registra void, int, if... en la tabla al iniciar
    fn cargar_palabras_reservadas(&mut self) {
        for palabra in PALABRAS_RESERVADAS {
            self.tabla.insertar(
                palabra.to_string(),
                TipoToken::Keyword,
                "global",
            );
        }
    }

    /// Lexema alfabetico: clasifica KEYWORD/IDENTIFIER y encola
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

    /// Numeros, operadores y simbolos ya clasificados
    pub fn encolar_token(&mut self, token: Token) {
        self.cola.encolar(token);
    }

    pub fn ver_salida(&self) {
        self.tabla.ver_tabla();
        self.cola.ver_cola();
    }
}
