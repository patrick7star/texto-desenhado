
// re-exportando funções e módulos.
pub mod forma_strings;
pub mod construtor;
mod modelos;

pub use modelos::{Texto, Aninhamento};
use forma_strings::{
   aninha_matrizes, 
   concatena_matriz, 
   desenha_string
};
use construtor::Matriz;

// biblioteca externa necessária.
extern crate utilitarios;
use utilitarios::terminal_dimensao::*;

// bilioteca padrão do Rust.
use std::collections::LinkedList;

// substituto dos antigos módulos:
mod matriz_texto;
mod formador;

// apelidos para facilitar codificação.
type ResultadoMatriz = Result<Matriz, &'static str>;
type Linhas = LinkedList<Matriz>;


/**! Pega uma "string" reparte ela e formata
 sua impressão desenhada ajustada com a 
 largura do terminal.
*/
pub fn conserta_acentuacao(s:&str) -> String {
   let mut texto = s.replace("ê","e");
   texto = texto.replace("ã","a");
   texto = texto.replace("ú","u");
   texto = texto.replace("í","i");
   texto = texto.replace("ç","c");
   texto = texto.replace("é","e");
   texto = texto.replace("á","a");
   texto = texto.replace("ô","o");
   texto = texto.replace("à", "a");
   return texto;
}

pub fn aninha_str_em_texto(string:&str) -> Linhas {
   let novo_texto = conserta_acentuacao(string);
   // obtendo todas as palavras.
   let mut palavras = {
      novo_texto
      .split_ascii_whitespace()
      .peekable()
   };
   // largura da tela.
   let lt = terminal_largura().unwrap().0;
   let mut linhas: Vec<String> = Vec::new();
   let mut texto: Linhas = LinkedList::new();
   let mut lg:u16 = 0;
   let mut atual_linha:String = String::new();
   // até se esgotar.
   'laco:loop {
      /* se exceder a tela, então adicionar 
       * linha e, zerar tanto a 'string concatenadora'
       * como a medida da 'string-desenhada' */
      if lg > lt-4 {
         // adicionando linha formada...
         linhas.push(
            atual_linha.clone()
         );
         // zerando coisas...
         atual_linha.clear();
         lg = 0;
      }
      else {
         // não consumir, apenas "dá uma olhada", e 
         // realizar alguns cálculos.
         let proxima = palavras.peek();
         // acabado o iterador, abandornar loop.
         if proxima == None 
            { break 'laco; }
         // mensurando a medida da string-desenhada.
         lg += {
            let aux = desenha_frase(
               proxima
               .unwrap()
            ).unwrap(); 
            (aux[1].len() as u16) + 3
         };
         if lg > lt-4
            { continue; }
         else { 
            // agora sim, comsumir próximo item.
            let proxima = palavras.next();
            atual_linha += proxima.unwrap_or(""); 
            atual_linha += " ";
         }
      }
   }
   // adicionando todos na lista ligada.
   for l in linhas {
      let frase = desenha_frase(l.as_str());
      texto.push_back(frase.unwrap());
   }
   /* consertando... coloca última palavra, que
    * o algoritmo não consegue capturar. */
   texto.push_back(
      match desenha_frase(atual_linha.as_str()) {
         Ok(desenho) => desenho,
         Err(err_msg) => 
            { panic!("{}",err_msg); }
      }
   );
   // retornando texto completo.
   return texto;
}

pub fn desenha_frase(string:&str) -> ResultadoMatriz {
   /* divindindo a frase por espaços brancos
    * e quebra-de-linha. */
   let mut palavras: Vec<&str> = {
      string
      .split_ascii_whitespace()
      .collect()
   };
   // contando quantia de repartições realizadas.
   let qtd = palavras.len();
   if qtd == 0
      { return Err("string vazias são inválidas."); }
   else if qtd == 1
      { return Ok(desenha_string(string)); }
   // matriz representando espaço vázio.
   let mut espaco: Matriz = {
      vec![
         vec![' ',' ',],
         vec![' ',' ',],
         vec![' ',' ',],
         vec![' ',' ',],
         vec![' ',' ',],
      ]
   };
   let mut frase: Matriz = desenha_string(palavras[0]);
   palavras.remove(0);
   for p in palavras {
      aninha_matrizes(&mut frase, &mut espaco);
      concatena_matriz(&mut frase, espaco.clone());
      // faz o desenho da palavra.
      let mut palavra = desenha_string(p);
      aninha_matrizes(&mut frase, &mut palavra);
      concatena_matriz(&mut frase, palavra);
   }
   return Ok(frase);
}

#[cfg(test)]
mod test {
   use super::*;
   use construtor::imprime;

   #[test]
   fn testa_criacao_texto() {
      let texto = aninha_str_em_texto(
         "\rhoje é domingo
         \rpé de cachimbo
         \ro cachimbo é de ouro
         \rbate no touro
         \ro toro e valente
         \rbate na gente
         \ra gente é fraco
         \rcai no buraco
         \ro buraco é fundo
         \racabou-se o mundo."
      );
      for obj in texto 
         { imprime(&obj); }
      assert!(false);
   }
   
   #[test]
   fn testa_nova_funcao_de_desenho() {
      let frase:&str = "isso e um teste simples";
      let frase_desenho: Matriz = {
         match desenha_frase(frase) {
            Ok(fd) => fd,
            Err(erro_msg) => { 
               assert!(false); 
               panic!("{}", erro_msg); 
            }
         }
      };
      let frase:&str = "você percebeu isto!";
      let frase_desenho_ii: Matriz = {
         desenha_frase(frase)
         .expect("erro com a frase dada") 
      };
      // visualizando...
      imprime(&frase_desenho);
      imprime(&frase_desenho_ii);
      assert!(true);
   }
}
