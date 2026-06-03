use crate::token::{Token, TipoToken};

pub struct GestorReportes {
    cola_tokens: Vec<Token>,
    lista_errores: Vec<Token>,
}

impl GestorReportes {
    /// Inicializa el gestor de reportes y errores
    pub fn nuevo() -> Self {
        GestorReportes {
            cola_tokens: Vec::new(),
            lista_errores: Vec::new(),
        }
    }

    /// Recibe los tokens del automata y los clasifica
    pub fn registrar_token(&mut self, token: Token) {
        match token.tipo {
            TipoToken::Error(_) => {
                // Modo recuperacion: guardamos el error sin detener el programa
                self.lista_errores.push(token);
            }
            _ => {
                // Flujo normal
                self.cola_tokens.push(token);
            }
        }
    }

    pub fn obtener_cola_tokens(&self) -> &Vec<Token> {
        &self.cola_tokens
    }

    /// Imprime el reporte estructurado usando los campos exactos del equipo
    pub fn imprimir_reporte_final(&self) {
        println!("\n======================================================================");
        println!("               REPORTE FINAL DEL ANALIZADOR LEXICO               ");
        println!("======================================================================");

        // 1. TOKENS VALIDOS
        println!("\n[TOKENS RECONOCIDOS EXITOSAMENTE]:");
        println!("{:<18} | {:<25} | {:<6} | {:<7}", "Tipo de Token", "Valor", "Linea", "Columna");
        println!("{}", "-".repeat(70));
        
        for token in &self.cola_tokens {
            println!(
                "{:<18} | {:<25} | {:<6} | {:<7}",
                token.tipo.nombre(),
                token.valor,
                token.ubicacion.linea,
                token.ubicacion.columna
            );
        }

        println!("\n======================================================================");

        // 2. DETECCION DE ERRORES
        if self.lista_errores.is_empty() {
            println!("Analisis finalizado con exito! 0 errores lexicos detectados.");
        } else {
            println!("ERRORES ENCONTRADOS: Se detectaron {} errores lexicos.", self.lista_errores.len());
            println!("{}", "-".repeat(70));
            println!("{:<6} | {:<7} | {:<15} | {:<35}", "Linea", "Columna", "Caracter/Texto", "Descripcion del Error");
            println!("{}", "-".repeat(70));

            for err in &self.lista_errores {
                let detalle = match &err.tipo {
                    TipoToken::Error(msg) => msg.as_str(),
                    _ => "Caracter no reconocido por el lenguaje",
                };

                println!(
                    "ERR  {:<4} | {:<7} | {:<15} | {:<35}",
                    err.ubicacion.linea,
                    err.ubicacion.columna,
                    err.valor,
                    detalle
                );
            }
        }
        println!("======================================================================\n");
    }
}
