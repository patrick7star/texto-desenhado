
// biblioteca padrão do Rust.
use std::fmt::{Formatter, Display, Result as String_fmt};

// própria biblioteca do caixote.
use crate::{Matriz, aninha_str_em_texto, Linhas};

// biblioteca externa:
extern crate utilitarios;
use utilitarios::terminal_dimensao::{Altura, Largura, dimensao};

#[derive(Clone, Copy, PartialEq)]
pub enum Aninhamento {
   Centro,
   Direita,
   Esquerda,
}

// recuo padrão para formatar textos.
const RECUO:char = ' ';

// appelido para melhorar legibilidade.
pub struct Texto {
   // lista-ligada contendo cada matriz-de-caractéres.
   linhas: Linhas,
   // em que parte da tela o texto "parte".
   posicao: Aninhamento
}

impl Texto {
   /* computa o espaço em branco entre o texto 
    * impresso e o limite da tela. */
   fn calculo_espaco_vago(matriz:&Matriz) -> u16 {
      let lm:u16 = matriz[1].len() as u16;
      let lt:u16 = match dimensao() {
         Some((Largura(l), Altura(_))) => l,
         None => lm,
      };
      return lt-lm;
   }

   /* aninha o texto na matriz dada, partindo da 
    * direita da tela, ou no centro dela, o parâmetro
    * pedindo o tipo de 'aninhamento'. Não há está
    * opção a esquerda, pois a função que gera o 
    * texto já é, por padrão, aninhado a ela. */
   fn aninha_linha(mut matriz:Matriz, aninhar:Aninhamento) -> Matriz {
      // espaços em brancos necessários.
      let qe:usize = Texto::calculo_espaco_vago(&matriz) as usize;
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
            for _ in 1..=qe {
               for linha in matriz.iter_mut() 
                  { linha.insert(0, RECUO); }
            }
         },
         // ao centro.
         Aninhamento::Centro => {
            for _ in 1..(qe/2 + 1) {
               for linha in matriz.iter_mut() {
                  // parte na direita.
                  linha.insert(0, RECUO);
                  // parte na esquerda.
                  linha.push(RECUO);
               }
            }
         },
         // à esquerda, então o padrão...
         Aninhamento::Esquerda => {},
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
      // retornando instância criada.
      Texto { linhas: texto_inicio, posicao }
   }

   /* remove espaços de vários aninhamentos, para
    * que se transforme num aninhamento de 
    * Esquerda novamente, então daí é possível ir
    * para qualquer outro. */
   fn remove_aninhamento(matriz:&mut Matriz) {
      // localizando "concreto" da letra mais próximo.
      let mut mais_proximo = matriz[0].len();
      for linha in matriz.iter() {
         for (p, ch) in linha.iter().enumerate() { 
            if p < mais_proximo && *ch != RECUO
               { mais_proximo = p; }
         }
      }
      /* Agora removendo vácuo entre texto-desenhado.
       * Serão removidos os primeiros elementos, que 
       * são provalvemente, espaços vázios. */
      for _ in 1..=mais_proximo {
         // interando cada "retalho" que compõe tal objeto.
         for linha in matriz.iter_mut() 
            { linha.remove(0); }
      }
   }

   // realinha texto para uma nova posição.
   pub fn reaninha(&mut self, nova_posicao:Aninhamento) {
      /* primeiro checar para ver se novo alinhamento
       * demandado já é o mesmo atualmente. Se
       * for o atual, então já encerra o algoritmo
       * neste instante. 
       */
      // se tem parâmetros inválidos, acabar aqui. 
      if nova_posicao == self.posicao
         { return (); }

      match nova_posicao {
         /* neste caso da Esquerda, apenas remover aninhamento
          * já produzindo anteriormente, seja na criação
          * ou um reaninhamento anterior produzido. */
         Aninhamento::Esquerda => { 
            /* Removendo no fim, trabalhando no texto
             * da linha, e inserindo novamente como 
             * linha um, chega ao mesmo texto antigo, 
             * porém reprocessado. O processo inverso
             * também pode ser feito, mas vamos deste 
             * mesmo. A quantia que será feito isto é,
             * o número inicial de linhas.*/
            let mut ql = self.linhas.len();
            // realiza o processa acima 'ql' vezes.
            while ql > 0 {
               let mut texto = {
                  self.linhas
                  .pop_back()
                  .unwrap()
               };
               Texto::remove_aninhamento(&mut texto); 
               // reinserindo na lista-ligada.
               self.linhas.push_front(texto);
               // contando ...
               ql -= 1;
            }
         },
         /* Os outros o mesmo processo, remove aninhamento
          * e usa mesma função de aninhamento que foi feito
          * na criação. */
         Aninhamento::Direita | Aninhamento::Centro => {
            /* O mesmo processo do outro braço do 'match'
             * porém inverso... apenas porque quero resultados
             * iguais, baseado em manobras diferentes. */
            let mut ql = self.linhas.len();
            // executando tal processo 'ql' vezes.
            while ql > 0 {
               let mut texto = {
                  self.linhas
                  .pop_front()
                  .unwrap()
               };
               Texto::remove_aninhamento(&mut texto);
               let novo_texto = Texto::aninha_linha(texto, nova_posicao);
               self.linhas.push_back(novo_texto);
               // contando ...
               ql -= 1;
            }
         },
      }
      // marca o novo posicionamento.
      self.posicao = nova_posicao;
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
         // espaçamento vertical para melhor visualização.
         texto.push('\n');
      }
      // duas linhas no fim do texto para sintetizar um fim.
      texto.push_str("\n\n");
      // "formata" string, não faço idéia do que 
      // isso significa, anyway....
      write!(formatador, "{}", texto)
   }
}

#[cfg(test)]
mod tests {
   use super::*;

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

   #[test]
   fn testa_reaninhamento_do_texto() {
      let string = "rose are red violet are blue you want me i want you.";
      // texto-desenhados.
      let mut a_esquerda = Texto::cria(string, Aninhamento::Esquerda);
      println!("ESQUERDA >> CENTRO >> DIREITA:");
      println!("{}", a_esquerda);
      a_esquerda.reaninha(Aninhamento::Centro);
      println!("{}", a_esquerda);
      //assert_eq!(a_esquerda.posicao, Aninhamento::Centro);
      a_esquerda.reaninha(Aninhamento::Direita);
      println!("{}", a_esquerda);
      //assert_eq!(a_esquerda.posicao, Aninhamento::Direita);

      println!("DIREITA >> CENTRO >> ESQUERDA:");
      let mut a_direita = Texto::cria(string, Aninhamento::Direita);
      println!("{}", a_direita);
      a_direita.reaninha(Aninhamento::Centro);
      //assert_eq!(a_direita.posicao, Aninhamento::Centro);
      println!("{}", a_direita);
      a_direita.reaninha(Aninhamento::Esquerda);
      //assert_eq!(a_direita.posicao, Aninhamento::Esquerda);
      println!("{}", a_direita);

      println!("CENTRO >> ESQUERDA >> CENTRO >> DIREITA");
      let mut no_centro = Texto::cria(string, Aninhamento::Centro);
      println!("{}", no_centro);
      no_centro.reaninha(Aninhamento::Esquerda);
      //assert_eq!(no_centro.posicao, Aninhamento::Esquerda);
      println!("{}", no_centro);
      // volta para o original.
      no_centro.reaninha(Aninhamento::Centro);
      //assert_eq!(no_centro.posicao, Aninhamento::Centro);
      println!("{}", no_centro);
      // agora o oposto do primeiro.
      no_centro.reaninha(Aninhamento::Direita);
      //assert_eq!(no_centro.posicao, Aninhamento::Direita);
      println!("{}", no_centro);
      
      // se tudo correr bem?
      assert!(false);
   }
}
