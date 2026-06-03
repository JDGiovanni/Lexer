use std::iter::Peekable;
use std::str::Chars;

/// Flujo de entrada: lee el archivo caracter a caracter
pub struct InputStream<'a> {
    fuente: Peekable<Chars<'a>>,
    pub linea: usize,
    pub columna: usize,
}

impl<'a> InputStream<'a> {
    /// Abre el texto fuente (contenido del archivo)
    pub fn new(texto: &'a str) -> Self {
        InputStream {
            fuente: texto.chars().peekable(),
            linea: 1,
            columna: 1,
        }
    }

    /// Mira el siguiente caracter sin consumirlo (lookahead)
    pub fn mirar_siguiente(&mut self) -> Option<&char> {
        self.fuente.peek()
    }

    /// Avanza un caracter y actualiza linea/columna
    pub fn avanzar(&mut self) -> Option<char> {
        let caracter = self.fuente.next();
        if let Some(c) = caracter {
            if c == '\n' {
                self.linea += 1;
                self.columna = 1;
            } else {
                self.columna += 1;
            }
        }
        caracter
    }

    /// Salta espacios, tabs y saltos de linea (llamar antes de cada token)
    pub fn saltar_espacios(&mut self) {
        while let Some(&c) = self.mirar_siguiente() {
            if c.is_whitespace() {
                self.avanzar();
            } else {
                break;
            }
        }
    }
}
