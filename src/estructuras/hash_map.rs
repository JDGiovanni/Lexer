// Tabla de simbolos — usa Tabla<K,V> de la libreria directamente

use LibreriaDeSoporte::ds::tabla::Tabla;

use crate::token::{TipoToken, PALABRAS_RESERVADAS};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntradaSimbolo {
    pub nombre: String,
    pub tipo: TipoToken,
    pub ambito: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResultadoClasificacion {
    PalabraReservada,
    IdentificadorExistente,
    IdentificadorNuevo,
}

pub struct TablaSimbolos {
    tabla: Tabla<String, EntradaSimbolo>,
    ambito_actual: String,
}

impl TablaSimbolos {
    pub fn nueva() -> Self {
        Self {
            tabla: Tabla::new(),
            ambito_actual: "global".to_string(),
        }
    }

    /// Inserta o actualiza un simbolo en la tabla de la libreria
    pub fn insertar(&mut self, nombre: String, tipo: TipoToken, ambito: impl Into<String>) {
        let ambito = ambito.into();
        let entrada = EntradaSimbolo {
            nombre: nombre.clone(),
            tipo,
            ambito,
        };
        self.tabla.insertar(nombre, entrada);
    }

    /// Busca por nombre de lexema
    pub fn buscar(&self, nombre: &str) -> Option<&EntradaSimbolo> {
        self.tabla.buscar_str(nombre)
    }

    pub fn contiene(&self, nombre: &str) -> bool {
        self.tabla.contiene_str(nombre)
    }

    pub fn es_palabra_reservada(lexema: &str) -> bool {
        PALABRAS_RESERVADAS.contains(&lexema)
    }

    /// Compara lexema completo contra reservadas e identificadores
    pub fn clasificar_lexema(&mut self, lexema: &str) -> ResultadoClasificacion {
        if Self::es_palabra_reservada(lexema) {
            ResultadoClasificacion::PalabraReservada
        } else if self.contiene(lexema) {
            ResultadoClasificacion::IdentificadorExistente
        } else {
            self.insertar(
                lexema.to_string(),
                TipoToken::Identifier,
                self.ambito_actual.clone(),
            );
            ResultadoClasificacion::IdentificadorNuevo
        }
    }

    /// KEYWORD si esta en PALABRAS_RESERVADAS, si no IDENTIFIER
    pub fn tipo_para_lexema(&self, lexema: &str) -> TipoToken {
        if Self::es_palabra_reservada(lexema) {
            TipoToken::Keyword
        } else {
            TipoToken::Identifier
        }
    }

    pub fn ver_tabla(&self) {
        println!("---- Tabla de simbolos ----");
        for (nombre, entrada) in self.tabla.iter() {
            println!(
                "{} | {} | ambito: {}",
                nombre,
                entrada.tipo.nombre(),
                entrada.ambito
            );
        }
        println!("---------------------------");
    }
}
