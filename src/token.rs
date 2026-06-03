/// Contrato compartido del analizador léxico (tipo, valor, ubicación).
///
/// Categorías alineadas con el ejercicio de la calculadora.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TipoToken {
    /// Palabras reservadas: void, float, int, return, if, else, while, break, switch, case, default
    Keyword,
    /// Nombres de funciones y variables
    Identifier,
    /// Cadenas entre comillas
    LiteralString,
    /// Constantes numéricas
    LiteralNumber,
    /// Símbolos aritméticos, lógicos o de asignación
    Operator,
    /// Símbolos de control y agrupación: ;, ,, (, ), {, }, [, ], :
    Punctuator,
    /// para almacenar el error de mensaje lexico
    Error(String),
}

impl TipoToken {
    pub fn nombre(self) -> &'static str {
        match self {
            TipoToken::Keyword => "KEYWORD",
            TipoToken::Identifier => "IDENTIFIER",
            TipoToken::LiteralString => "LITERAL_STRING",
            TipoToken::LiteralNumber => "LITERAL_NUMBER",
            TipoToken::Operator => "OPERATOR",
            TipoToken::Punctuator => "PUNCTUATOR",
            TipoToken::Error(_) => "ERROR",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ubicacion {
    pub linea: usize,
    pub columna: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub tipo: TipoToken,
    pub valor: String,
    pub ubicacion: Ubicacion,
}

impl Token {
    pub fn nuevo(tipo: TipoToken, valor: impl Into<String>, linea: usize, columna: usize) -> Self {
        Token {
            tipo,
            valor: valor.into(),
            ubicacion: Ubicacion { linea, columna },
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ tipo: {}, valor: {:?}, linea: {}, columna: {} }}",
            self.tipo.nombre(),
            self.valor,
            self.ubicacion.linea,
            self.ubicacion.columna
        )
    }
}

/// Palabras reservadas del lenguaje
pub const PALABRAS_RESERVADAS: &[&str] = &[
    "void", "float", "int", "return", "if", "else", "while", "break", "switch", "case", "default",
];
