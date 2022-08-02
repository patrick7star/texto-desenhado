/*
Imprime texto e retorna strings, sobre qualquer
texto passado, porém na forma gráfica de desenho.
*/

// bibliotecas importantes.
use std::collections::HashMap;
use std::fs::{read_dir, File,DirEntry};
use std::io::Read;
use std::path::Path;

// caminhos comuns:
const CAMINHO_ALFABETO:&'static str = "simbolos/alfabeto/";
const CAMINHO_NUMEROS:&'static str = "./simbolos/numeros/";
const CAMINHO_PONTUACAO:&'static str = "simbolos/pontuacao";

// apelidos para facilitar codificação.
pub type Matriz = Vec<Vec<char>>;
type TabelaInt =  HashMap<u8, Matriz>;
type TabelaChar = HashMap<char, Matriz>; 


fn matriciar_string(string:String) -> Matriz {
    /* pega uma string que tem quebra de linha 
     * num texto, representado aqui por uma matriz
     * onde cada linha do texto equivale a uma 
     * linha da matriz. */
    // cria uma matriz.
    let mut matriz:Vec<Vec<char>> = Vec::new();

    // iterador que dá várias strings, brotadas
    // da quebra-de-linha.
    for linha in string.lines() {
        // vetor auxiliar que representa linha da 
        // matriz.
        let mut row:Vec<char> = Vec::new();
        // põe cada caractére da string-linha.
        for c in linha.chars() { row.push(c); }
        // põe o vetor na array-de-vetores.
        matriz.push(row);
    }
    // antes do retorno equaliza colunas.
    equaliza_matriz(&mut matriz);
    return matriz;
}

fn arquivo_para_matriz(caminho:&str) -> Matriz {
   /* dado o caminho ao arquivo com texto estruturado
    * lê seus dados e, transforma tal string cuspida 
    * deste dados numa matriz, onde as quebras de linhas
    * delimitam a linha da matriz gerada. */
   let mensagem_erro = format!("erro ao abrir arquivo \"{}\"",caminho);
   let mut arq = File::open(caminho).expect(mensagem_erro.as_str());

   let mut conteudo:String = String::new();
   // lendo conteúdo do arquivo.
   match arq.read_to_string(&mut conteudo) {
    Ok(string) => string,
    Err(e) => panic!("não possível ler conteúdo do arquivo::{}",e)
   };

   return matriciar_string(conteudo);
}

fn extrai_caminho_str(obj:DirEntry) -> String {
   /* pega a estrutura 'DirEntry' e extrai o
    * caminho que ela guarda. */
   // OS string.
   let parser = obj.path().into_os_string();
   // extrai sua string.
   match parser.into_string() {
    Ok(string) => string,
    Err(_) => panic!("não possível converter caminho(estrutura) em string")
   }
}

/** retorna uma tabela contendo as matrizes com desenhos
 de texto estruturado... de todos os algarismos 
 numéricos conhecidos. */
pub fn carrega_desenhos_numeros() -> TabelaInt {
   /* tabela contendo o número inteiro e uma 
   * estrutura de dados para sua representação 
   * gráfica de texto.*/
   let mut algarismos: TabelaInt = HashMap::new();
   /* adicionando manualmente todos caminhos, com
   * seus respectivos índices: */
   // array agregando chaves e nome de arquivos.
   let pares:[(u8, &str); 10] = [
      (1,"um.txt"),   (2,"dois.txt"), 
      (3,"tres.txt"), (4,"quatro.txt"), 
      (5,"cinco.txt"),(6,"seis.txt"),
      (7,"sete.txt"), (8,"oito.txt"),
      (9,"nove.txt"), (0,"zero.txt")
   ];
   // iterando tais pares da array.
   for (i, s) in pares { 
      // caminho para concatenação de caminho com nome do arquivo.
      let caminho = Path::new(CAMINHO_NUMEROS).join(s);
      // forma a matriz dado co conteúdo do arquivo.
      let matriz = arquivo_para_matriz(caminho.to_str().unwrap());
      // insere no dicionário.
      algarismos.insert(i, matriz);
   }
   return algarismos;
}

pub fn carrega_desenhos_letras() ->TabelaChar {
    /* carrega todo o alfabeto num dicionário, tal alfabeto
     * é uma matriz contendo o a letra desenhada por caractéres
     * de forma estruturada. */
    // dicionário contendo tais desenhos.
    let mut alfabeto: TabelaChar;
    alfabeto = HashMap::new();

    // caminho para as letras do alfabeto:
    let arquivos = match std::fs::read_dir(CAMINHO_ALFABETO) {
        Ok(iterador) => iterador,
        Err(_) => panic!("não conseguiu ler o interno do diretório"),
    };

    // pecorrendo cada arquivo.
    for a in arquivos {
        // filtrando estrutura DirEntry do Result<_>.
        let a = match a {
            Ok(b) => b,
            Err(_) => panic!("erro ao ler DirEntry!"),
        };

        // processando o nome da chave.
        let nome_arq = match a.file_name().into_string() {
                        Ok(s) => s,
                        Err(_) => String::from("nadaI"),
                      };

        // agora processanddo caminho para uma estrutra Path.
        let caminho_str = extrai_caminho_str(a); 
        
        // obtem a primeira letra do arquivo, que será
        // usada como chave no dicionário.
        let letra:char = match nome_arq.chars().next() {
            Some(c) => c,
            None => panic!(" erro ao pegar letra do nome do arquivo."),
        };
        // adicionando na tabela hash.
        alfabeto.insert(letra, arquivo_para_matriz(caminho_str.as_str()));
    }
    // retorno do dicionário.
    return alfabeto;
}

fn equaliza_matriz(matriz:&mut Matriz) {
    /* obtem a referência de uma matriz, então preenche
     * com espaços em branco até atinger a linha da matriz
     * com maior números de colunas. */
    let qtd_linhas = (*matriz).len();

    // acha linha com mais colunas e, esta quantia.
    let mut max_cols = matriz[0].len();
    for indice in 1..qtd_linhas {
        // contabiliza a quantia de colunas da linha atual.
        let qtd_cols = matriz[indice].len();
        if  max_cols < qtd_cols { max_cols = qtd_cols; }
    }

    /* equaliza todas as "linhas" da matriz baseado
     * na maior, ou seja, a com mais colunas. Serão
     * preenchidas com espaço em branco.
     */
    for i in 0..qtd_linhas {
        while matriz[i].len() < max_cols {
            matriz[i].push(' ');
        }
    }
}

pub fn carrega_caracteres_pontuacao() -> TabelaChar {
   /* todos os caractéres não alfabéticos e 
    * numéricos serão carregados aqui e 
    * colocado num dicionário. */
   // dicionário que conterá todos símbolos 
   // carregados na memória.
   let mut simbolos: TabelaChar = HashMap::new();
   // caminho para todos símbolos.
   let caminho = Path::new(CAMINHO_PONTUACAO);
   // tabela de equivalência.
   let equivalente = [ 
      ("abre_aspas_duplas",'\"'), ("abre_chaves",'{'), 
      ("abre_colchetes", '['), ("abre_parenteses",'('), 
      ("arroba", '@'), ("asterisco",'*'), ("backslash",'\\'),
      ("slash",'/'), ("cifrao",'$'), ("dois_pontos",':'),
      ("exclamacao",'!'), ("fecha_aspas_duplas",'\"'), 
      ("fecha_colchetes",']'), ("fecha_parenteses",')'), 
      ("igual",'='), ("interrogacao",'?'), ("maior_que",'>'), 
      ("mais",'+'), ("menor_que",'<'), ("porcentagem",'%'),
      ("til", '~'), ("traco",'-'), ("velha",'#'), 
      ("virgula", ','), ("fecha_chaves",'}'), ("ponto_virgula",';'), 
      ("ponto", '.'),("circunflexo",'^'),
   ];

   // iterando tudo dentro do diretório...
   for obj in read_dir(caminho).unwrap() {
      // transforma num Path.
      let aux = obj.unwrap().path();
      let pth = aux.as_path();
      let nome_arq:&str;

      // se for um arquivo.
      if pth.is_file() {
         nome_arq = {
            pth.file_name()
             .unwrap()
             .to_str()
             .unwrap()
         };
         // achando caractére correspondente...
         let comp_str = nome_arq.split_once(".txt").unwrap().0;
         for t in equivalente {
            if t.0 == comp_str {
               let caracter = t.1;
               // lendo e transformando conteúdo do arquivo.
               let matriz = arquivo_para_matriz(pth.to_str().unwrap());

               // registrando no dicionário.
               simbolos.insert(caracter, matriz);
               // achou o símbolo,... sai.
               break;
            }
         }
      }
   }
   // tabela com dados.
   return simbolos;
}

/* obtem a referência do objeto e, imprime
 * ele via saída padrão. */
pub fn imprime(matriz:&Matriz) {
   for row in matriz {
      for cell in row 
         { print!("{}", cell); }
      print!("\n");
   }
}

#[cfg(test)]
mod tests {
   // usando módulos acima...
   use crate::construtor::*;

   #[test]
   fn carrega_simbolos() {
      let outros = carrega_caracteres_pontuacao();
      let chaves:Vec<char> = outros.into_keys().collect();
      println!("{:?}", chaves);
      // verifica se foi carregado com sucesso.
      assert!(!chaves.is_empty());
   }
   
   #[test]
   fn matematica_simbolos() {
      let simbolos = carrega_caracteres_pontuacao();

      let adicao = simbolos.get(&'+').unwrap();
      let igual = simbolos.get(&'=').unwrap();
      let subtracao = simbolos.get(&'-').unwrap();
      let multiplicacao = simbolos.get(&'*').unwrap();

      super::imprime(adicao);
      super::imprime(igual);
      super::imprime(multiplicacao);
      super::imprime(subtracao);
   }

   #[test]
   fn qual_e_as_aspas() {
      let simbolos = carrega_caracteres_pontuacao();
      let aspas = simbolos.get(&'"').unwrap();
      imprime(aspas);
   }

   #[test]
   #[ignore]
   fn todos_simbolos_carregados() {
      println!("todos símbolos adicionados:");
      let simbolos = carrega_caracteres_pontuacao();
      for chave in simbolos.keys() {
         let conteudo = simbolos.get(chave).unwrap();
         imprime(conteudo);
         print!("\n");
      }
   }
}
