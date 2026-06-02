mod estructuras;
mod input;
mod token;

fn main() {
    let ejemplo = token::Token::nuevo(token::TipoToken::Keyword, "int", 1, 1);
    println!("Contrato Token: {ejemplo}");
}
