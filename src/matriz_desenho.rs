
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
const FUNDO: char = ' ';

// para codificação.
type MultiArray = Vec<Vec<char>>;
// para encurtar os parâmetros.
pub type MatrizTexto = MT;


pub struct MatrizTexto {
   matriz: Vec<Vec<char>>,
   altura: u16,
   largura: u16,
}

impl MatrizTexto {
   // método construtor:
   pub fn cria(altura: u16, largura: u16) -> Self {
      let mut matriz: MultiArray;
      matriz = Vec::with_capacity(MAX_LARGURA);

      for _ in 0..altura {
         let mut linha: Vec<char>;
         linha = Vec::with_capacity(MAX_ALTURA);
         for _ in 0..largura
            { linha.push(FUNDO); }
         matriz.push(linha);
      }
      return MatrizTexto { matriz, altura, largura }
   }

   // muda célula na matriz.
   pub fn set(self, y: usize, x: usize, ch: char) 
      { self.matriz[y][x] = ch; }

   // obtém célula na matriz.
   pub fn get(self, y: usize, x: usize) -> char 
      { self.matriz[y][x] }

   /* tupla com dimensão da matriz: altura e 
    * largura respectivamente. */
   pub fn dimensao(self) -> (usize, usize) 
      { (self.altura, self.largura) }
}

// outros métodos mais excentricos.
impl MatrizTexto {
   // apara colunas em brancos à direita.
   pub fn trim_right(self) {
      let coluna_em_branco: bool = {
         let x = self.largura-1;
         for y in 0..self.altura {
            if !matriz[y][x].is_whitespace()
               { return false; }
         }
         return true;
      };
      for line in self.matriz.iter_mut() 
         { line.pop(); }
   }

   // apara colunas em brancos à esquerda.
   pub fn trim_left(self) {
      let coluna_em_branco: bool = {
         for y in 0..self.altura {
            if !matriz[y][0].is_whitespace()
               { return false; }
         }
         return true;
      };
      for line in self.matriz.iter_mut() 
         { line.remove(0); }
   }

   // acrescenta 'm' linhas na matriz.
   fn aumenta_altura(self, h: u16) {
      // registro aumento vertical da matriz.
      self.altura += h;

      // forma uma linha com devida largura.
      let linha_em_branco: Vec<char>;
      linha_em_branco = Vec::with_capacity(MAX_LARGURA);
      for _ in 0..self.largura 
         { linha_em_branco.push(' '); }
      // adiciona linha-em-branco no topo.
      for _ in 0..h
         { self.matriz.insert(0, linha_em_branco); }
   }
   
   // acrescenta 'n' colunas.
   fn aumenta_largura(self, l: u16) {
      // registro aumento vertical da matriz.
      self.largura += l;
      for linha in self.matriz.iter_mut() {
         for _ in 0..l 
            { linha.push(FUNDO); }
      }
   }

   // equalisa a menor matrix-texto com a maior.
   fn equaliza_matrizes(mt1:&mut MT, mt2:&mut MT) {
      // altura de ambos.
      let (h1, _) = mt1.dimensao();
      let (h2, _) = mt2.dimensao();

      if h1 > h2 {
         let diferenca = h1 - h2;
         mt1.aumenta_altura(diferenca);
      } else if h1 < h2 {
         let diferenca = h2 - h1;
         mt2.aumenta_altura(diferenca);
      } 
   }
}

/* operações das matriz nela própria, ou
 * geradora, que significa que tal operação
 * resulta numa matriz-texto nova. */
impl MatrizTexto {
   /* concatena matriz-texto passado com a própria
    * instância. O argumento(que é uma matriz-texto)
    * será "consumida" dentro do método. */
   pub fn concatena(&mut self, mut matriz: MT) {
      (_, l) = matriz.dimensao();
      MT::equaliza_matrizes(self, &mut matriz); 
      self.aumenta_largura(l);
      for y in 0..self.altura {
         for x in 0..l { 
            let value = matriz.get(y, x);
            self.set(y, x+l, value);
         }
      }
   }

   /* a mesma função que o método acima, porém com
    * resultante da nova matrix-texto formada. É 
    * um método estático. Os argumentos são referências,
    * e a ordem de concatenação é da esquerda à 
    * direita, mesma da codificação dos parâmetros. */
   pub fn concatena(mt1:&MT, mt2:&MT) -> MT {
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

   /* obtem a referência do objeto e, imprime
    * ele via saída padrão. */
   pub fn imprime(&self) {
      for row in self.matriz {
         for cell in row 
            { print!("{}", cell); }
         print!("\n");
      }
   }

   /* transforma uma string -- é necesário que
    * ela seja múltilinha -- numa matriz-texto.
    */
   pub fn to_matriz(string: &str) -> Self {
      let largura: usize = {
         string.lines()
         .map(|linha| len(linha))
         .max()
      };
      // mede baseado na quantia de quebra-de-linhas.
      let altura: usize = {
         string.chars()
         .map(|ch| ((ch == '\n') as u32) * 1)
         .sum()
      };

      // só faz em certas condições estabelecidas.
      if altura == 0 || altura == 1 || largura == 0 
         { panic!("dimensões inviáveis para tarefa!"); }

      let matriz = MatrizTexto::cria(
         altura as u16, 
         largura as u16
      );
      let mut iterador = string.chars();

      for y in 0..altura {
         for x in 0..largura {
            match iterador.next() {
               Some(_char) => 
                  { matriz.set(y, x, _char); }
               None => ()
            }
         }
      }

      return matriz;
   }
}

