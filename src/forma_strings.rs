/*
 * Forma uma palavras, sequências numéricas,
 * qualquer string no formato ascii passada,
 * que se queria representar na forma de 
 * de soupa de caractéres.
 */

// minha própria lib.
use crate::construtor::{
   carrega_caracteres_pontuacao,
   carrega_desenhos_numeros,
   carrega_desenhos_letras,
   Matriz
};


fn forma_palavra(palavra:&str) -> Matriz {
    /* forma uma palavra dada com slice string
     * por parâmetro, é retornado uma multiarray 
     * assim com os textos e números aplicados 
     * para forma-lô. */
    let mut iterador = palavra.chars();
    let letra:char = match iterador.next() {
        Some(o) => o.to_ascii_uppercase(), 
        _ => panic!("outro none"),
    };

    // todas as letras necessárias.
    let mut alfabeto = carrega_desenhos_letras();
    let mut matriz: Matriz;
    matriz = match alfabeto.get_mut(&letra) { 
        Some(obj) => obj.to_vec(), 
        _ => panic!("notok")
    };

    // primeira verificando se a string passada é 
    // válida em ser montada.
    for c in iterador {

        // se for válido o caractére...
        if c.is_ascii_alphabetic() {
            // matriz da letra-desenho do caractére.
            let chave:char = c.to_ascii_uppercase();
            let temp_matriz = match alfabeto.get_mut(&chave){
                Some(matriz) => matriz,
                _ => panic!("linha 49"),
            };

            // pecorre cada llinha da matriz atual.
            for linha in 0..matriz.len() {
                // transformando numa fatia atual linha.
                let fatia = temp_matriz[linha].as_slice();
                // colocando um espaço antes...
                matriz[linha].push(' ');
                // extendendo ela no vetor principal.
                matriz[linha].extend_from_slice(fatia);
            }
        }
    }
    return matriz;
}

pub fn concatena_matriz(m1:&mut Matriz, m2:Matriz) {
    /* terceira função de concatena duas matrizes, no caso
     * concatena uma segunda(m2) na primeira(m1), esta última
     * sendo uma referência.*/
    // pecorre cada linha da matriz atual.
    for linha in 0..m1.len() {
        // transformando numa fatia atual linha.
        let fatia = m2[linha].as_slice();
        // colocando um espaço antes...
        m1[linha].push(' ');
        // extendendo ela no vetor principal.
        m1[linha].extend_from_slice(fatia);
    }
}

fn forma_numero(numero:&str) -> Matriz {
   /* dado um número em forma de string, tendo ele
    * um ou mais dígitos, a função retorna uma 
    * matriz representando ele em forma de 
    * desenho-de-caractéres. */
    // carregando algarismos...
    let mut algarismos = carrega_desenhos_numeros();

    //usa-se da recursividade para acionar o 1º 'if'.
    if numero.is_empty() 
      { forma_numero("0") }

    else if numero.len() == 1 { 
        // chave para acessar dicionário.
        let chave:u8 = match numero.chars().next() {
            Some(ch) => match ch.to_digit(10) {
                Some(int) => int as u8,
                None => panic!("erro! -- var:'int'"),
            },
            None => panic!("erro! -- var:'chave'"),
        };
        // busca arquivo com símbolo-desenho-texto.
        let alg = match algarismos.get_mut(&chave) {
            Some(matriz) => (*matriz).to_vec(),
            None => panic!("erro! -- var:'matriz'"),
        };
        // já retorna, pois há só um algarismos passado.
        return alg;
    }

    // nos demais casos, é igual ao de formar palavra.
    else {
        let mut iterador = numero.chars();
        let chave:u8 = match iterador.next() {
            Some(option) => match option.to_digit(10) {
                Some(int) => int as u8,
                _ => panic!("var: 'int'"),
            }
            _ => panic!("var:'chave'"),
        };
        // matriz auxiliar para concatenação.
        let mut matriz = match algarismos.get_mut(&chave) {
            Some(m) => (*m).clone(),
            _ => panic!("var='matriz'"),
        };
        // iterando cada dígito...
        for alg in iterador {
            // apenas se for um dígito, então, digite apenas números.
            if alg.is_ascii_digit() {
                let chave = match alg.to_digit(10) {
                    Some(int) => int as u8,
                    _ => panic!("var:'var'"),
                };
                let matriz_alg = match algarismos.get(&chave) {
                    Some(x) => (*x).to_vec(),
                    _ => panic!("var: 'matriz_alg'"),
                };
                concatena_matriz(&mut matriz, matriz_alg);
            }
        }
        return matriz;
    }
}

/* parte da tela/quadro/qualquer retângulo...
 * se quer almolfar com espaços em branco.
 */
pub enum Lado { Superior, Inferior }

pub fn preenche_linhas_em_branco(matriz:&mut Matriz, 
parte:Lado, qtd:u8) {
   // dimensão da matriz.
   let colunas = matriz[0].len();
   let linha = vec![' '; colunas];

   // adiciona a quantia de linhas demandada.
   for _ in 1..(qtd+1) {
      // adicionando à partir do lado dado...
      match parte {
         // ... de cima.
         Lado::Superior => 
            {matriz.insert(0, linha.clone())},
         // ... de baixo.
         Lado::Inferior => 
            {matriz.push(linha.clone())},
      };
   }
}

fn fatia_em_classes(string:&str) -> Vec<String> {
   let mut fatias:Vec<String> = Vec::new();
   let mut aux:String = String::from("");
   let mut aux_num: String = String::from("");

   for ch in string.chars() {
      /* concatena cada caractére do alfabeto e,
       * ao achar um, provalvemente, numérico, 
       * adiciona a variável auxiliar de letras,
       * e limpa-a para uma possível concatenação
       * numérica. */
      if ch.is_ascii_alphabetic() 
         { aux.push(ch); }
      else if !aux.is_empty() {
         fatias.push(aux.clone());
         aux.clear();
      }
      // o mesmo que o acima, porém apenas com dígitos.
      if ch.is_ascii_digit() 
         { aux_num.push(ch); }
      else if !aux_num.is_empty(){
         fatias.push(aux_num.clone());
         aux_num.clear();
      }

      /* se for uma das pontuações já criadas, então
       * fazer o mesmo. */
      let implementada = "\"{}[]()\\$!?@><%~+-/*=#,;:.^";
      if ch.is_ascii_punctuation() && 
         implementada.contains(&ch.to_string()) 
            { aux_num.push(ch); }
      else if !aux_num.is_empty(){
         fatias.push(aux_num.clone());
         aux_num.clear();
      }
   } 

   // adiciona o restante...
   if !(aux_num.is_empty() && aux.is_empty()) {
      if !aux_num.is_empty() { fatias.push(aux_num);}
      if !aux.is_empty() { fatias.push(aux);}
   }
   /* retorna uma array contendo todas fatias, 
    * sendo elas numéricas, alfabéticas e demais 
    * posteriormente adicionandas ao grupo. */
   fatias
}

pub fn aninha_matrizes(matriz:&mut Matriz, outra_matriz:&mut Matriz) {
   // quantia de linhas de cada matriz.
   let qtd_m = matriz.len();
   let qtd_om = outra_matriz.len();
   let dif: u8;

   // sua diferença, baseada na maior.
   if qtd_m > qtd_om 
      { dif = (qtd_m - qtd_om) as u8; }
   else 
      { dif = (qtd_om - qtd_m) as u8; }

   if dif != 0 {
      // achando o maior e, ajustando ela.
      let lado: Lado = Lado::Superior;
      if qtd_m > qtd_om 
         { preenche_linhas_em_branco(outra_matriz, lado, dif); }
      else 
         { preenche_linhas_em_branco(matriz, lado, dif); }
   }
}

// o tipo de dado que pode ser.
pub enum TipoStr {
   Alfabeto,
   Numerico,
   Pontuacao
}

fn string_tipo(string:&str) -> Option<TipoStr> {
   // verificando primeiramente se, há apenas letras.
   let mut e_alfabetica:bool = true;
   let mut e_numerico:bool = false;

   /* se houver apenas um caractére e for
    * uma pontuação, então é deste tipo. */
   if string.len() == 1 {
      let crtr = string.chars().next().unwrap();
      if crtr.is_ascii_punctuation() 
         { return Some(TipoStr::Pontuacao); }
   }

   // partindo do presupostos que é inteiramente alfabético.
   for ch in string.chars() {
      // se char algum não assim, anula presuposto.
      if !ch.is_ascii_alphabetic() { 
         e_alfabetica = false; 
         // e valida um presupostos que é numérico.
         e_numerico = true;
      }
   }

   // se chegar aqui sem alteração do valor, então é alfabética.
   if e_alfabetica { Some(TipoStr::Alfabeto) }
   else {
      // do contrário começa a varredura por não numéricos...
      for ch in string.chars() {
         if !ch.is_ascii_digit() { e_numerico = false; }
      }
      
      // se o presupostos não ter sido inválidado, então é este.
      if e_numerico { Some(TipoStr::Numerico) }
      // caso contrário retorna um valor "null" de inválido.
      else { None }
   }
}

pub fn desenha_string(string: &str) -> Matriz {
   /* desenha qualquer string com símbolos ascii
    * e retorna a estrutura de dados(multiarray)
    * representando-o. */
   // fatia partes inteiramente numéricas e alfabéticas.
   let substrs = fatia_em_classes(string);
   let mut matriz: Matriz;
   let mut outra: Matriz;
   let mut iterador = substrs.into_iter();
   let pontuacao = carrega_caracteres_pontuacao();
   
   // primeira concatenação manualmente...
   let s = iterador.next().expect("268º linha");
   match string_tipo(s.as_str()) {
      Some(TipoStr::Alfabeto) => 
         matriz = forma_palavra(s.as_str()),
      Some(TipoStr::Numerico) => 
         matriz = forma_numero(s.as_str()),
      Some(TipoStr::Pontuacao) => {
         let ch = s.chars().next().unwrap();
         matriz = pontuacao.get(&ch).unwrap().to_vec();
      },
      None => 
         panic!("string \"{}\" incompatível", s)
   }

   // iterando os demais...
   for _str in iterador {
      let si = _str.as_str();
      match string_tipo(si) {
       Some(TipoStr::Alfabeto) => 
         { outra = forma_palavra(si); },
       Some(TipoStr::Numerico) => 
         { outra = forma_numero(si); },
      Some(TipoStr::Pontuacao) => {
         let ch = si.chars().next().unwrap();
         outra = pontuacao.get(&ch).unwrap().to_vec();
      },
       None => 
         { panic!("string \"{}\" incompatível", s); }
      }

      // o menor será aninhado com base no maior.
      aninha_matrizes(&mut matriz, &mut outra);
      // daí será concatenado.
      concatena_matriz(&mut matriz, outra);
   }

   // retorna matriz combinada.
   return matriz;
}

#[cfg(test)]
mod tests {
   // importando tudo acima...
   use crate::forma_strings::*;
   use crate::construtor::imprime;

   #[test]
   fn testa_fec() {
      let vetor = fatia_em_classes("numero3casa08quadra46");
      println!("vetor={:#?}", vetor);
      assert_eq!(vec!["numero","3","casa","08","quadra","46"], vetor);
   }

   #[test]
   fn igualiza_qtd_linhas() {
      let mut letra_h = super::forma_palavra("H");
      let num8 = super::forma_numero("8");
      let diferenca = (num8.len()-letra_h.len()) as u8;
      preenche_linhas_em_branco(
         &mut letra_h,
         Lado::Inferior,
         diferenca
      );
      assert_eq!(letra_h.len(), num8.len());
   }

   #[test]
   fn diferencas_preenchimento() {
      let mut letra_r = forma_palavra("R");
      let mut letra_h = forma_palavra("h");

      preenche_linhas_em_branco(&mut letra_r, Lado::Superior,6);
      preenche_linhas_em_branco(&mut letra_h, Lado::Inferior, 6);
      println!("mostrando resultados:");
      imprime(&letra_r);
      println!("\n\n");
      imprime(&letra_h);
   } 

   #[test]
   fn diferenca_pos_aninhamento() {
      let mut letra_h = forma_palavra("A");
      let mut num8 = forma_numero("8");

      aninha_matrizes(&mut letra_h, &mut num8);

      println!("como ficou:");
      imprime(&letra_h.clone());
      imprime(&num8.clone());

      assert_eq!(0, letra_h.len()-num8.len());
   }

   #[test]
   fn concatenacao_aninhamento_resultado() {
      let mut palavra = forma_palavra("resultado");
      let mut num = forma_numero("42");

      aninha_matrizes(&mut palavra, &mut num);

      concatena_matriz(&mut palavra, num.clone());
      imprime(&palavra);
      assert!(true);
   }

   #[test]
   fn testa_ds() {
      let string = desenha_string("jabuti1234");
      imprime(&string);
      assert!(true);
   }

   #[test]
   fn fatias_agora_com_pontuacao() {
      let string = fatia_em_classes("[algo, ok!]");
      println!("vetor com fatias: {:?}", string);
      assert_eq!(
         string, 
         vec!["[","algo",",","ok","!","]"]
      );
   }

   #[test]
   fn string_formador_incrementado() {
      println!("fatiamento: {:?}",fatia_em_classes("(entao, entre"));
      let s = desenha_string("(entao, entre");
      imprime(&s);

      println!("fatiamento: {:?}",fatia_em_classes("voce!]"));
      let r = desenha_string("voce!]");
      imprime(&r);

      println!("fatiamento: {:?}",fatia_em_classes("10% de viva?"));
      let v = desenha_string("10% de viva?");
      imprime(&v);
   }
}

