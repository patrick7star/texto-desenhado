

// re-exportando funções e módulos.
pub mod forma_strings;

// biblioteca externa necessária.
extern crate terminal_size;
use terminal_size::{Width, Height, terminal_size};

// biblioteca padrão do Rust.
use std::collections::LinkedList;
use std::fmt::{Formatter, Display, Result as String_fmt};


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

#[derive(Clone, Copy)]
pub enum Aninhamento {
   Centro,
   Direita,
   Esquerda,
}
pub struct Texto {
   linhas:LinkedList<Vec<Vec<char>>>,
   posicao:Aninhamento,
}
impl Texto {
   /* computa o espaço em branco entre o texto 
    * impresso e o limite da tela. */
   fn calculo_espaco_vago(matriz:&Vec<Vec<char>>) -> u16 {
      let lm:u16 = matriz[1].len() as u16;
      let lt:u16 = match terminal_size() {
         Some((Width(l),Height(_))) => l as u16,
         None => lm,
      };
      return lt-lm;
   }
   /* aninha o texto na matriz dada, partindo da 
    * direita da tela, ou no centro dela, o parâmetro
    * pedindo o tipo de 'aninhamento'. Não há está
    * opção a esquerda, pois a função que gera o 
    * texto já é, por padrão, aninhado a ela. */
   fn aninha_linha(mut matriz:Vec<Vec<char>>, 
   aninhar:Aninhamento) -> Vec<Vec<char>> {
      // dimensão(vertical) da matriz.
      //let altura = matriz.len();
      // espaços em brancos necessários.
      const RECUO:char = ' ';
      let qtd_espacos:usize = Texto::calculo_espaco_vago(&matriz) as usize;
      /* tratando todos tipos de aninhamento.
       * O tipo de algoritmo que fará isto é:
       * vamos computar, com uma função auxiliar,
       * a distãncia do texto á tela, posteriormente
       * vamos, no caso da "direita" preencher
       * o lado esquerdo com 'filetes branco' 
       * até que o texto "encoste" no limite da 
       * tela; já o centro o mesmo, porém divido
       * em partes metade desse acrescimo de "filetes"
       * ao lado esquerdo, o outro ao direito. */
      match aninhar {
         // primeiro à direita.
         Aninhamento::Direita => {
            for _ in 1..(qtd_espacos+1) {
               for linha in matriz.iter_mut() {
                  linha.insert(0, RECUO);
               }
            }
         },
         // ao centro.
         Aninhamento::Centro => {
            for _ in 1..(qtd_espacos/2+1) {
               for linha in matriz.iter_mut() {
                  // parte na direita.
                  linha.insert(0, RECUO);
                  // parte na esquerda.
                  linha.push(RECUO);
               }
            }
         },
         // à esquerda, então o padrão...
         _ => {},
      };
      // retornando a matriz, tendo sido modificada ou não.
      matriz
   }
   // cria texto, primariarimente aninhado à esquerda.
   pub fn cria(string:&str, posicao:Aninhamento) -> Texto { 
      let mut texto_inicio = aninha_str_em_texto(string); 
      // contador de ínicio e upper-bound.
      let mut p = 1;
      let k = texto_inicio.len();
      // alterando o aninhamento de cada linha do texto.
      while p <= k {
         /* tira do ínicio, e coloca no 
          * final pós modificação, pois 
          * depois de de 'k' voltas, a 
          * lista fica a mesma coisa. */
         match texto_inicio.pop_front() {
            Some(l) => { 
               let nova_linha = Texto::aninha_linha(l, posicao);
               texto_inicio.push_back(nova_linha);
            },
            None => (),
         }
         p += 1;
      }
      return Texto {
         linhas: texto_inicio,
         posicao,
      };
   }
}
impl Display for Texto {
   fn fmt(&self, formatador:&mut Formatter<'_>) -> String_fmt {
      let mut texto:String = String::new();
      for linha in self.linhas.iter() {
         // linha é uma matriz "Vec<Vec<char>>".
         for array in linha.iter() {
            // cada local da array da matriz.
            for ch in array.iter() 
               { texto.push(*ch); }
            // colocando quebra de linha na string.
            texto.push('\n');
         }
      }
      // "formata" string, não faço idéia do que 
      // isso significa, anyway....
      write!(formatador, "{}", texto)
   }
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
         match super::desenha_frase(frase) {
            Ok(fd) => fd,
            Err(erro_msg) => 
               { assert!(false); panic!("{}", erro_msg); }
         }
      };
      let frase:&str = "você percebeu isto!";
      let frase_desenho_ii:Vec<Vec<char>> = {
         super::desenha_frase(frase)
         .expect("erro com a frase dada") 
      };
      // visualizando...
      super::forma_strings::imprime(&frase_desenho);
      super::forma_strings::imprime(&frase_desenho_ii);
      assert!(true);
   }

   use super::{Texto, Aninhamento};
   #[test]
   fn mesmo_texto_alinhamentos_diferentes() {
      let string = "isso é um teste básico de um texto o suficiente longo!";
      let a_esquerda = Texto::cria(string, Aninhamento::Esquerda);
      let a_direita = Texto::cria(string, Aninhamento::Direita);
      let no_centro = Texto::cria(string, Aninhamento::Centro);
      println!(
         "ESQUERDA:\n{}\n\nCENTRO:\n{}\n\nDIREITA:\n{}\n\n",
         a_esquerda, no_centro, a_direita
      );
      assert!(true);
   }
}
