

// re-exportando funções e módulos.
pub mod forma_strings;
pub mod construtor;
mod modelos;
pub use modelos::{Texto, Aninhamento};

// biblioteca externa necessária.
extern crate terminal_size;
use terminal_size::{Width, Height, terminal_size};

// bilioteca padrão do Rust.
use std::collections::LinkedList;


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

pub fn aninha_str_em_texto(string:&str) -> LinkedList<Vec<Vec<char>>> {
   let novo_texto = conserta_acentuacao(string);
   // obtendo todas as palavras.
   let mut palavras = novo_texto.split_ascii_whitespace().peekable();
   // largura da tela.
   let lt = match terminal_size() {
      Some((Width(l), Height(_))) => l,
      None => 0,
   };
   //println!("width_terminal={}", lt);
   let mut linhas:Vec<String> = Vec::new();
   let mut texto:LinkedList<Vec<Vec<char>>> = LinkedList::new();
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
         // uma simples quebra-de-linha.
         //println!(" ");
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
         //println!("draw-width={}", lg);
         if lg > lt-4
            { continue; }
         else { 
            // agora sim, comsumir próximo item.
            let proxima = palavras.next();
            atual_linha += proxima.unwrap_or(""); 
            atual_linha += " ";
            //println!("{}", atual_linha);
         }
      }
   }
   // adicionando todos na lista ligada.
   for l in linhas {
      texto.push_back(
          desenha_frase(l.as_str())
          .unwrap()
      );
   }
   /* consertando... coloca última palavra, que
    * o algoritmo não consegue capturar. */
   texto.push_back(
      match desenha_frase(atual_linha.as_str()) {
         Ok(desenho) => desenho,
         Err(err_msg) => { panic!("{}",err_msg); }
      }
   );
   // retornando texto completo.
   return texto;
}

pub fn desenha_frase(string:&str) -> 
Result<Vec<Vec<char>>, &'static str> {
   /* divindindo a frase por espaços brancos
    * e quebra-de-linha. */
   //let nova_string = conserta_acentuacao(string);
   let mut palavras:Vec<&str> = {
      string
      .split_ascii_whitespace()
      .collect()
   };
   // contando quantia de repartições realizadas.
   let qtd = palavras.len();
   if qtd == 0
      { return Err("string vazias são inválidas."); }
   else if qtd == 1
      { return Ok(forma_strings::desenha_string(string)); }
   // matriz representando espaço vázio.
   let mut espaco:Vec<Vec<char>> = {
      vec![
         vec![' ',' ',],
         vec![' ',' ',],
         vec![' ',' ',],
         vec![' ',' ',],
         vec![' ',' ',],
      ]
   };
   let mut frase:Vec<Vec<char>> = forma_strings::desenha_string(palavras[0]);
   palavras.remove(0);
   for p in palavras {
      forma_strings::aninha_matrizes(&mut frase, &mut espaco);
      forma_strings::concatena_matriz(&mut frase, espaco.clone());
      // faz o desenho da palavra.
      let mut palavra = forma_strings::desenha_string(p);
      forma_strings::aninha_matrizes(&mut frase, &mut palavra);
      forma_strings::concatena_matriz(&mut frase, palavra);
   }
   return Ok(frase);
}

#[cfg(test)]
mod test {
   // importando bagulho acima.
   use super::*;

   #[test]
   fn testa_criacao_texto() {
      let texto = aninha_str_em_texto(
         "hoje é domingo
         pé de cachimbo
         o cachimbo é de ouro
         bate no touro
         o toro e valente
         bate na gente
         a gente é fraco
         cai no buraco
         o buraco é fundo
         acabou-se o mundo."
      );
      for obj in texto 
         { construtor::imprime(&obj); }
      assert!(false);
   }
   
   #[test]
   fn testa_nova_funcao_de_desenho() {
      let frase:&str = "isso e um teste simples";
      let frase_desenho:Vec<Vec<char>> = {
         match desenha_frase(frase) {
            Ok(fd) => fd,
            Err(erro_msg) => { 
               assert!(false); 
               panic!("{}", erro_msg); 
            }
         }
      };
      let frase:&str = "você percebeu isto!";
      let frase_desenho_ii:Vec<Vec<char>> = {
         desenha_frase(frase)
         .expect("erro com a frase dada") 
      };
      // visualizando...
      construtor::imprime(&frase_desenho);
      construtor::imprime(&frase_desenho_ii);
      assert!(true);
   }
}
