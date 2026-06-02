use std::iter::Peekable;
use std::str::Chars;

pub struct InputStream<'a> {
    fuente: Peekable<Chars<'a>>,
    pub linea: usize,
    pub columna: usize,
}

impl<'a> InputStream<'a> {
    
    pub fn new(texto: &'a str) -> Self {
        InputStream {
            fuente: texto.chars().peekable(),
            linea: 1,
            columna: 1,
        }
    }

    pub fn mirar_siguiente(&mut self) -> Option<&char> {
        self.fuente.peek()
    }

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