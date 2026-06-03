// Elida — tabla de símbolos adaptada desde LibreriaDeSoporte::ds::map (Vec de entradas).

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
    datos: Vec<EntradaSimbolo>,
    ambito_actual: String,
}

impl TablaSimbolos {
    pub fn nueva() -> Self {
        Self {
            datos: Vec::new(),
            ambito_actual: "global".to_string(),
        }
    }

    pub fn insertar(&mut self, nombre: String, tipo: TipoToken, ambito: impl Into<String>) {
        let ambito = ambito.into();
        if let Some(entrada) = self
            .datos
            .iter_mut()
            .find(|e| e.nombre == nombre && e.ambito == ambito)
        {
            entrada.tipo = tipo;
        } else {
            self.datos.push(EntradaSimbolo {
                nombre,
                tipo,
                ambito,
            });
        }
    }

    pub fn buscar(&self, nombre: &str) -> Option<&EntradaSimbolo> {
        self.datos.iter().find(|e| e.nombre == nombre)
    }

    pub fn buscar_en_ambito(&self, nombre: &str, ambito: &str) -> Option<&EntradaSimbolo> {
        self.datos
            .iter()
            .find(|e| e.nombre == nombre && e.ambito == ambito)
    }

    pub fn contiene(&self, nombre: &str) -> bool {
        self.buscar(nombre).is_some()
    }

    pub fn es_palabra_reservada(lexema: &str) -> bool {
        PALABRAS_RESERVADAS.contains(&lexema)
    }

    /// Distingue palabra reservada, identificador ya registrado o identificador nuevo.
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

    pub fn tipo_para_lexema(&self, lexema: &str) -> TipoToken {
        if Self::es_palabra_reservada(lexema) {
            TipoToken::Keyword
        } else {
            TipoToken::Identifier
        }
    }

    pub fn ver_tabla(&self) {
        println!("---- Tabla de símbolos ----");
        if self.datos.is_empty() {
            println!("(vacía)");
        } else {
            for entrada in &self.datos {
                println!(
                    "{} | {} | ámbito: {}",
                    entrada.nombre,
                    entrada.tipo.nombre(),
                    entrada.ambito
                );
            }
        }
        println!("---------------------------");
    }
}
