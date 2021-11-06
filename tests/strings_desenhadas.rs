
/* Aqui serão feitas vários testes
 * testando todas as possibilidades da 
 * função final "desenha_string" que 
 * produz a arte-texto.
 */

// biblioteca a testar:
extern crate texto_desenho;
use texto_desenho::forma_strings::{imprime, desenha_string};

#[test]
fn arte_de_palavras_basicas() {
   let arte = desenha_string("computador"); 
   imprime(&arte);

   let arte = desenha_string("banana"); 
   imprime(&arte);

   let arte = desenha_string("academia"); 
   imprime(&arte);

   let arte = desenha_string("ovo"); 
   imprime(&arte);

   let arte = desenha_string("jardim"); 
   imprime(&arte);

   assert!(true);
}

#[test]
fn letras_do_alfabeto() {
   let parte_i = desenha_string("abcdefghi"); 
   let parte_ii = desenha_string("jklmnopqr");
   let parte_iii = desenha_string("stuvwxyz");
   imprime(&parte_i);
   imprime(&parte_ii);
   imprime(&parte_iii);
}

#[test]
fn algarismos_numericos() {
   imprime(&desenha_string("0123456789"));
   assert!(true);
}

#[test]
fn pontuacao_presente() {
   imprime(&desenha_string("{} () []"));
   imprime(&desenha_string("+ - * / ="));
   imprime(&desenha_string(". ; , :"));
   imprime(&desenha_string(">< ? \\ ~ ^"));
   assert!(true);
}
