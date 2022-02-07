
/** Com um input dado, será escrito na tela,
  com a estrutura 'Texto', com um 'Aninhamento'
  centralizado.
*/

// biblioteca a testar:
extern crate texto_desenho;
use texto_desenho::{Texto, Aninhamento};

// biblioteca padrão do Rust.
use std::io::stdin;

#[test]
fn escreva_algo() {
   // entrada de dados.
   let mut buffer = String::new();
   stdin().read_line(&mut buffer).unwrap();

   // criando objeto...
   let filtro = {
      buffer
      .strip_suffix("\n")
      .unwrap()
   };
   let texto = Texto::cria(filtro, Aninhamento::Centro);

   // impressão:
   println!("seu texto:\n\t\"{}\"", buffer); 
   println!("{}", texto);
}
