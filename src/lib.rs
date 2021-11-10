

// re-exportando funções e módulos.
pub mod forma_strings;

// biblioteca externa necessária.
extern crate terminal_size;
use terminal_size::{Width, Height, terminal_size};

// biblioteca padrão do Rust.
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
   texto = texto.replace("é","e");
   texto = texto.replace("á","a");
   texto = texto.replace("ô","o");
   return texto;
}
pub fn aninha_str_em_texto(string:&str) -> LinkedList<Vec<Vec<char>>> {
   // "retirando" a acentuação de algumas vogais.
   let texto_fixed = conserta_acentuacao(string);
   // obtendo todas as palavras.
   let mut palavras = texto_fixed.split_ascii_whitespace().peekable();
   // largura da tela.
   let lt = match terminal_size() {
      Some((Width(l), Height(_))) => l,
      None => 0,
   };
   println!("width_terminal={}", lt);
   let mut linhas:Vec<String> = Vec::new();
   let mut texto:LinkedList<Vec<Vec<char>>> = LinkedList::new();
   let mut lg:u16 = 0;
   let mut atual_linha:String = String::new();
   // até se esgotar.
   'laco:loop {
      /* se exceder a tela, então adicionar 
       * linha e, zerar tanto a 'string concatenadora'
       * como a medida da 'string-desenhada' */
      if lg > lt-3 {
         // adicionando linha formada...
         linhas.push(
             //forma_strings::desenha_string(atual_linha.as_str())
            //forma_strings::desenha_frase(atual_linha.as_str()).unwrap()
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
            let aux = forma_strings::desenha_string(
               proxima
               .unwrap()
            );
            aux[1].len() as u16
            /*
            let aux = forma_strings::desenha_frase(
               proxima
               .unwrap()
            ).unwrap(); 
            2+(aux[1].len() as u16);
            */
         };
         if lg > lt-3
            { continue; }
         else { 
            // agora sim, comsumir próximo item.
            let proxima = palavras.next();
            atual_linha += " ";
            atual_linha += proxima.unwrap_or(""); 
         }
      }
   }
   // adicionando todos na lista ligada.
   for l in linhas {
      texto.push_back(
          forma_strings::desenha_string(l.as_str())
      );
   }
   /* consertando... coloca última palavra, que
    * o algoritmo não consegue capturar. */
   texto.push_back(
      forma_strings::desenha_string(atual_linha.as_str())
   );
   // retornando texto completo.
   return texto;
}


#[cfg(test)]
mod test {
   #[test]
   fn testa_criacao_texto() {
      let texto = super::aninha_str_em_texto(
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
         { super::forma_strings::imprime(&obj); }
      assert!(false);
   }

   
   #[test]
   fn testa_nova_funcao_de_desenho() {
      let frase:&str = "isso e um teste simples";
      let frase_desenho:Vec<Vec<char>> = {
         match super::forma_strings::desenha_frase(frase) {
            Ok(fd) => fd,
            Err(erro_msg) => 
               { assert!(false); panic!("{}", erro_msg); }
         }
      };
      super::forma_strings::imprime(&frase_desenho);
      assert!(true);
   }
}
