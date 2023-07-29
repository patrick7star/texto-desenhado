
/*!
 Fazendo um objeto que cuida do desenho, como
 funções relacionadas a ele diretamente: 
 concatenação, aparação de espaços em brancos
 e etc.
*/


/* capacidade baseado na dimensão máxima
 * do laptop, que foi primeiramente 
 * codificado. Resolução da tela dele é
 * 1366x768 pixels. */
const MAX_LARGURA: usize = 42;
const MAX_ALTURA: usize = 151;
const FUNDO: char = {
   if cfg!(debug_assertions) { '.' } 
   else { ' ' }
};

// para codificação.
type MultiArray = Vec<Vec<char>>;
type MAS = [[char; MAX_LARGURA]; MAX_ALTURA];
type MultiArrayStack = MAS;

#[derive(Clone)]
pub struct MatrizTexto {
   /* alocação dinâmica, portanto mais 
    * lento para inicializar-lo. */
   grade_heap: MultiArray,
   /* memória é alocada na compilação 
    * do código, portanto operações serão
    * feitas mais rapida. */
   grade_stack: MultiArrayStack,
   // dimensão da Matriz.
   altura: u16, largura: u16,
   /* ativa o tipo de matriz a ser operar
    * ser for menor que o limite em ambas
    * dimensões, a estrutura usa a matriz
    * fixa na stack, caso contrário a matriz
    * com alocação dinâmica(na heap). */
   usando_a_heap: bool
}

impl MatrizTexto {
   // método construtor:
   pub fn cria(altura: u16, largura: u16) -> Self {
      let usando_a_heap: bool;
      let grade_stack: MultiArrayStack;
      let mut grade_heap: MultiArray;

      grade_heap = Vec::with_capacity(MAX_LARGURA);
      grade_stack = [
         [FUNDO; MAX_LARGURA]; 
         MAX_ALTURA
      ];

      if altura <= MAX_ALTURA as u16
      && largura <= MAX_LARGURA as u16 { 
         usando_a_heap = false; 
         grade_heap = Vec::new();
      } else { 
         usando_a_heap = true; 
         for _ in 0..altura {
            let mut linha: Vec<char>;
            linha = Vec::with_capacity(MAX_ALTURA);
            for _ in 0..largura
               { linha.push(FUNDO); }
            grade_heap.push(linha);
         }
      }

      MatrizTexto { 
         grade_stack, usando_a_heap, 
         grade_heap, altura, largura 
      }
   }

   // muda célula na matriz.
   pub fn set(&mut self, y: u16, x: u16, ch: char) { 
      if self.usando_a_heap
         { self.grade_heap[y as usize][x as usize] = ch;  }
      else
         { self.grade_stack[y as usize][x as usize] = ch;  }
   }

   // obtém célula na matriz.
   pub fn get(&self, y: u16, x: u16) -> char {
      if self.usando_a_heap
         { self.grade_heap[y as usize][x as usize]  }
      else
         { self.grade_stack[y as usize][x as usize]  }
   }

   /* tupla com dimensão da matriz: altura e 
    * largura respectivamente. */
   pub fn dimensao(&self) -> (u16, u16) 
      { (self.altura, self.largura) }
   
   /* redimensiona se necessário a grade com 
    * caractéres trocando pela 'heap'. */
   pub fn redimensiona(&mut self) {
      let transborda_y = self.altura >= MAX_ALTURA as u16;
      let transborda_x = self.altura >= MAX_ALTURA as u16;

      if transborda_x || transborda_y { 
         // muda para grade alocada na 'heap'.
         self.usando_a_heap = true; 
         // copiand todo conteúdo da stack para heap.
         for x in 0..(self.largura as usize) {
            for y in 0..(self.altura as usize) {
               let ch = self.grade_stack[y][x];
               self.grade_heap[y][x] = ch;
            }
         }
      }
   }
}

// outros métodos mais excentricos.
impl MatrizTexto {
   // acrescenta 'm' linhas na matriz.
   fn aumenta_altura(&mut self, h: u16) {
      // registro aumento vertical da matriz.
      self.altura += h;
      // alterar estrutura da matriz se for preciso.
      self.redimensiona();

      if self.usando_a_heap {
         // forma uma linha com devida largura.
         let mut linha_em_branco: Vec<char>;
         linha_em_branco = Vec::with_capacity(MAX_LARGURA);
         for _ in 0..self.largura 
            { linha_em_branco.push(' '); }
         // adiciona linha-em-branco no topo.
         for _ in 0..h
            { self.grade_heap.insert(0, linha_em_branco.clone()); }
      }
   }
   
   // acrescenta 'n' colunas.
   fn aumenta_largura(&mut self, l: u16) {
      // registro aumento vertical da matriz.
      self.largura += l;
      // alterar estrutura da matriz se for preciso.
      self.redimensiona();

      if self.usando_a_heap {
         for linha in self.grade_heap.iter_mut() {
            for _ in 0..l 
               { linha.push(FUNDO); }
         }
      }
   }

   // equalisa a menor matrix-texto com a maior.
   fn equaliza_matrizes(mt1:&mut MT, mt2:&mut MT) {
      // altura de ambos.
      let (h1, _) = mt1.dimensao();
      let (h2, _) = mt2.dimensao();
      
      if h1 < h2
         { mt1.aumenta_altura(h2 - h1); }
      else if h1 > h2
         { mt2.aumenta_altura(h1 - h2); }
   }
}

pub type MT = MatrizTexto;
/* operações das matriz nela própria, ou
 * geradora, que significa que tal operação
 * resulta numa matriz-texto nova. */
impl MatrizTexto {
   /* concatena matriz-texto passado com a própria
    * instância. O argumento(que é uma matriz-texto)
    * será "consumida" dentro do método. */
   pub fn concatena(&mut self, mut matriz: MT) {
      let (_, l) = matriz.dimensao();
      let largura = self.largura;
      /* coloca algumas das matrizes à nível 
       * da outra. */
      MT::equaliza_matrizes(self, &mut matriz); 
      // redimensiona matriz em "mais 'l'".
      self.aumenta_largura(l);
      for y in 0..self.altura {
         for x in 0..l { 
            let value = matriz.get(y, x);
            self.set(y, largura + x, value);
         }
      }
   }

   /* obtem a referência do objeto e, imprime
    * ele via saída padrão. */
   pub fn imprime(&self) {
      let (a, l) = (self.altura as usize, self.largura as usize);
      for y in 0..a {
         for x in 0..l {
            let celula: char;
            if self.usando_a_heap 
               { celula = self.grade_heap[y][x]; }
            else
               { celula = self.grade_stack[y][x]; }
            print!("{}", celula);
         }
         println!("");
      }
   }
}

// apenas métodos estáticos do objeto.
impl MatrizTexto {
   /* transforma uma string -- é necesário que
    * ela seja múltilinha -- numa matriz-texto. */
   pub fn to_matriz(string: &str) -> Self {
      let largura: usize = {
         string.lines()
         .map(|linha| linha.chars().count())
         .max().unwrap()
      };
      let altura: u16 = string.lines().count() as u16;
      let mut matriz = MatrizTexto::cria(
         altura as u16, 
         largura as u16
      );

      for (y, linha) in string.lines().enumerate() {
         for (x, char) in linha.chars().enumerate() {
            matriz.set(y as u16, x as u16, char);
         }
      }

      return matriz
   }
   
   /* a mesma função que o método acima, porém com
    * resultante da nova matrix-texto formada. É 
    * um método estático. Os argumentos são referências,
    * e a ordem de concatenação é da esquerda à 
    * direita, mesma da codificação dos parâmetros. */
   pub fn concatena_matrizes(mt1:&MT, mt2:&MT) -> MT {
      // clona ambos.
      let mut mt1 = mt1.clone();
      let mut mt2 = mt2.clone();
      MT::equaliza_matrizes(&mut mt1, &mut mt2);
      // dimensão após redimensionamento de alguma.
      let ((a, l1), (_, l2)) = (
         mt1.dimensao(), 
         mt2.dimensao()
      );
      // matriz-texto resultante.
      let mut mtr: MatrizTexto;
      mtr = MT::cria(a, l1 + l2);
      for y in 0..a {
         // primeiro parâmetro paran nova matriz-texto.
         for x in 0..l1 
            { mtr.set(y, x, mt1.get(y, x)); }
         // agora, copiando o segundo.
         for x in 0..l2
            { mtr.set(y, x+l1, mt2.get(y, x)); }
      }
      return mtr;
   }
}

use std::ops::{IndexMut, Index};

impl Index<usize> for MatrizTexto 
{
   type Output = [char];
   
   fn index(&self, linha: usize) -> &Self::Output { 
      let l: usize = linha;
      if self.usando_a_heap 
         { &self.grade_heap[l] }
      else
         { &self.grade_stack[l] }
   }
}

impl IndexMut<usize> for MatrizTexto {
   fn index_mut(&mut self, coluna: usize)
     -> &mut Self::Output 
   { 
      let c: usize = coluna;
      if self.usando_a_heap 
         { &mut self.grade_heap[c] }
      else
         { &mut self.grade_stack[c] }
   }
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn indexacao_simples() {
      let mut m = MT::cria(10, 60);      
      for l in 0..10 {
         for c in 0..60 
            { m.set(l, c, '.'); }
      }
      m.imprime();
      for a in 1..=6
         { m[5][30 + a] = 'K'; }
      m.imprime();
   }

   use utilitarios::aleatorio::sortear;
   #[test]
   fn indexacoes_iguais() {
      let mut m = MT::cria(10, 60);      
      for l in 0..10 {
         for c in 0..60 { 
            let char = sortear::u32(65..=127);
            m.set(l, c, char::from_u32(char).unwrap()); 
         }
      }
      for y in 0..10 {
         for x in 0..60 
            { assert_eq!(m[y][x], m.get(y as u16, x as u16)); }
      }
      m.imprime();
   }

   #[test]
   fn limpa_alteracoes_feitas() {
      let mut m = MT::cria(10, 60);      
      for l in 0..10 {
         for c in 0..60 { 
            let char = sortear::u32(65..=127);
            m.set(l, c, char::from_u32(char).unwrap()); 
         }
      }
      m.imprime();
      for y in 0..10 {
         for x in 0..60 
            { m[y][x] = '_'; }
      }
      m.imprime();
   }

   #[test]
   #[should_panic]
   fn erro_das_estruturas_internas() {
      let mut m = MT::cria(10, 5);      
      // seus cantos.
      drop(m[9][4]);
      drop(m[0][0]);
      drop(m[0][4]);
      drop(m[9][0]);
      let fora_dos_cantos = [
         (10, 4), (9, 5), (10, 5),
         (0, 5), (15, 15)
      ];
      // aqui quebra.
      let s = sortear::usize(0..=4);
      let (y, x) = fora_dos_cantos[s];
      drop(m[y][x]);
   }

   fn imprime_multiarray(ma: &[[char; 10]; 10]) {
      let comprimento = 3 * 10 + 2;
      println!("{}", &"+".repeat(comprimento));
      for linha in ma {
         print!("+");
         for coluna in linha {
            print!(" {} ", coluna);
         }
         println!("+");
      }
      println!("{}", &"+".repeat(comprimento));
   }
   #[test]
   fn multiarray_fixa() {
      let mut multiarray = [['.'; 10]; 10];
      imprime_multiarray(&multiarray);
   }

   #[test]
   fn redimensionamento_da_matriz_texto() {
      let mut m = MatrizTexto::cria(40, 60);
      m.imprime();
   }
}
