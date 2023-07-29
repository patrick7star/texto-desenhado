
/*!
 Vamos refazer o código de concatenação sem
 mexer no que já funciona. Com o tempo quando
 chegar ao mesmo nível, ou ficar ainda melhor
 substituímos pelo código atual, deletando
 os códigos antigos.
*/

// biblioteca 
use crate::matriz_texto::{MT, MatrizTexto};
use std::path::Path;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::str::FromStr;

// apelidos para facilitar codificação e legibilidade.
type TabelaASCII = HashMap<String, MT>;
// caminhos para os arquivos contendo os símbolos desenhados.
const ALFABETO:&str = r"simbolos/alfabeto";
const ALGARISMOS:&str = r"simbolos/numeros";
const PONTUACAO:&str = r"simbolos/pontuacao";
// equivalência de um valor ao nome do arquivo.
const PARES:[(u8, &str); 10] = [
   (1,"um.txt"),   (2,"dois.txt"), 
   (3,"tres.txt"), (4,"quatro.txt"), 
   (5,"cinco.txt"),(6,"seis.txt"),
   (7,"sete.txt"), (8,"oito.txt"),
   (9,"nove.txt"), (0,"zero.txt")
];
// tabela de equivalência.
const EQUIVALENTE: [(&str, char); 28] = [ 
   ("abre_aspas_duplas",'\"'), ("abre_chaves",'{'), 
   ("abre_colchetes", '['), ("abre_parenteses",'('), 
   ("arroba", '@'), ("asterisco",'*'), ("backslash",'\\'),
   ("slash",'/'), ("cifrao",'$'), ("dois_pontos",':'),
   ("exclamacao",'!'), ("fecha_aspas_duplas",'\"'), 
   ("fecha_colchetes",']'), ("fecha_parenteses",')'), 
   ("igual",'='), ("interrogacao",'?'), ("maior_que",'>'), 
   ("mais",'+'), ("menor_que",'<'), ("porcentagem",'%'),
   ("til", '~'), ("traco",'-'), ("velha",'#'), 
   ("virgula", ','), ("fecha_chaves",'}'), 
   ("ponto_virgula",';'), ("ponto", '.'),
   ("circunflexo",'^'),
];


// função inicializa todos símbolos desenhados.
fn inicializa() -> TabelaASCII {
   // tabela com todos símbolos carreagdos.
   let mut tabela: TabelaASCII = HashMap::new();

   // primeiro carrega alfabeto.
   let caminhos = [
      Path::new(ALGARISMOS), 
      Path::new(ALFABETO),
      Path::new(PONTUACAO)
   ];
   for caminho in caminhos.iter() {
      for entrada in caminho.read_dir().unwrap() {
         let caminho_arquivo = entrada.unwrap().path();
         let conteudo = read_to_string(
            caminho_arquivo
            .as_path()
         ).unwrap();
         let nome_arquivo: String = {
            caminho_arquivo
            .as_path()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
         };

         let mt = MT::to_matriz(conteudo.as_str());
         tabela.insert(nome_arquivo, mt);
      }
   }
   return tabela;
}

// interpleta o nome do arquivo e entrega
// uma 'chave' correspondente da tabela.
fn traduz_chave(string:&str) -> String {
   // converte string-caractére para caractére.
   let mut iterador = string.chars();
   let caractere = iterador.next().unwrap();
   let ascii_code = caractere as u32;

   let e_alfabetico: bool = {
      (97..=122).contains(&ascii_code) ||
      (65..=90).contains(&ascii_code)
   };
   let e_numerico = (48..=57).contains(&ascii_code);
   let e_pontuacao: bool = {
      (33..=47).contains(&ascii_code) ||
      (123..=126).contains(&ascii_code) ||
      (58..=64).contains(&ascii_code) ||
      (91..=94).contains(&ascii_code)
   };

   for (chave, nome_arquivo) in PARES.iter() {
      //quebra se inadequado.
      if !e_numerico
         { break; }
      let alg = u8::from_str(string).unwrap();
      if *chave == alg
         { return nome_arquivo.to_string(); }
   }

   for (nome_arquivo, chave) in EQUIVALENTE.iter() {
      if !e_pontuacao
         { break; }
      if caractere == *chave
         { return format!("{}.txt", nome_arquivo); }
   }

   if e_alfabetico
      {  format!("{}.txt", string.to_ascii_uppercase()) }
   else
      { panic!("não implementado para tal caractére."); }
}

/* faz um desenho da string passada, caíba ela
 * na tela do terminal, ou não. */
pub fn desenha_str(string:&str) -> MT {
   let tabela = inicializa();
   let espaco = MatrizTexto::cria(7, 1);

   // primeiro caractére.
   let chave = string.get(0..1).unwrap();
   let chave = traduz_chave(chave);
   let mut base = tabela.get(&chave).unwrap().clone();
   base.concatena(espaco.clone());

   // pegando um caractére por vez, mas com slice-str.
   let mut i = 1;
   while let Some(chave) = string.get(i..i+1) {
      let chave = traduz_chave(chave);
      let mt = tabela.get(&chave).unwrap().clone();
      base.concatena(mt);
      base.concatena(espaco.clone());
      i += 1;
   }
   base
}

#[cfg(test)]
mod tests {
   use super::{inicializa, desenha_str, MT};

   #[test]
   fn testa_funcao_inicializa() {
      let tabela = inicializa();
      for chave in tabela.keys() {
         println!("\n\nchave: {}", chave);
         let mt = tabela.get(chave).unwrap();
         MT::imprime(mt);
      }
      // funcionou?
      assert!(true);
   }

   #[test]
   fn testa_funcao_desenha_str() {
      let string = "livros";
      let desenho = desenha_str(string);
      desenho.imprime();
      let desenho = desenha_str("1123581321");
      desenho.imprime();
      let desenho = desenha_str("casa-de-queijo");
      desenho.imprime();
      let desenho = desenha_str("(){}[]+-~");
      desenho.imprime();
      desenha_str("senha=09210").imprime();
      assert!(true);
   }
}
