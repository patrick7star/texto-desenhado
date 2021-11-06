/*
 * Forma uma palavras, sequências numéricas,
 * qualquer string no formato ascii passada,
 * que se queria representar na forma de 
 * de soupa de caractéres.
 */

// módulo externo.
mod constroi_simbolos;
pub use constroi_simbolos::*;


fn forma_palavra(palavra:&str) -> Vec<Vec<char>> {
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
    let mut matriz:Vec<Vec<char>>;
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


fn concatena_matriz(m1:&mut Vec<Vec<char>>, m2:Vec<Vec<char>>) {
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


fn forma_numero(numero:&str) -> Vec<Vec<char>> {
   /* dado um número em forma de string, tendo ele
    * um ou mais dígitos, a função retorna uma 
    * matriz representando ele em forma de 
    * desenho-de-caractéres. */
    // carregando algarismos...
    let mut algarismos = carrega_desenhos_numeros();

    //usa-se da recursividade para acionar o 1º 'if'.
    if numero.len() == 0 { forma_numero("0") }

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

fn preenche_linhas_em_branco(matriz:&mut Vec<Vec<char>>, 
                                 parte:Lado, qtd:u8) {
   // dimensão da matriz.
   let colunas = matriz[0].len();
   let linha = vec![' '; colunas];

   // adiciona a quantia de linhas demandada.
   for _i in 1..(qtd+1) {
      // adicionando à partir do lado dado...
      match parte {
         // ... de cima.
         Lado::Superior => {matriz.insert(0, linha.clone())},
         // ... de baixo.
         Lado::Inferior => {matriz.push(linha.clone())},
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
      if ch.is_ascii_alphabetic() { aux.push(ch); }
      else {
         if !aux.is_empty() {
            fatias.push(aux.clone());
            aux.clear();
         }
      }
      // o mesmo que o acima, porém apenas com dígitos.
      if ch.is_ascii_digit() { aux_num.push(ch); }
      else {
         if !aux_num.is_empty() {
            fatias.push(aux_num.clone());
            aux_num.clear();
         }
      }

      /* se for uma das pontuações já criadas, então
       * fazer o mesmo. */
      let implementada = "\"{}[]()@\\$:!+-=*/?><%#~.,;:^";
      if ch.is_ascii_punctuation() && 
         implementada.contains(&ch.to_string()) 
            { aux_num.push(ch); }
      else {
         if !aux_num.is_empty() {
            fatias.push(aux_num.clone());
            aux_num.clear();
         }
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
   return fatias;
}

fn aninha_matrizes(matriz:&mut Vec<Vec<char>>,
                       outra_matriz:&mut Vec<Vec<char>>) {
   // quantia de linhas de cada matriz.
   let qtd_m = matriz.len();
   let qtd_om = outra_matriz.len();
   let dif:u8;

   // sua diferença, baseada na maior.
   if qtd_m > qtd_om {
      dif = (qtd_m - qtd_om) as u8;
   }
   else { dif = (qtd_om - qtd_m) as u8; }

   if dif != 0 {
      // achando o maior e, ajustando ela.
      if qtd_m > qtd_om {
         preenche_linhas_em_branco(outra_matriz, Lado::Superior, dif);
      }
      else {
         preenche_linhas_em_branco(matriz, Lado::Superior, dif);
      }
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

pub fn desenha_string(string:&str) -> Vec<Vec<char>>{
   /* desenha qualquer string com símbolos ascii
    * e retorna a estrutura de dados(multiarray)
    * representando-o. */
   // fatia partes inteiramente numéricas e alfabéticas.
   let substrs = fatia_em_classes(string);
   let mut matriz:Vec<Vec<char>>;
   let mut outra:Vec<Vec<char>>;
   let mut iterador = substrs.into_iter();
   let pontuacao = carrega_caracteres_pontuacao();
   
   // primeira concatenação manualmente...
   let s = iterador.next().expect("268º linha");
   match string_tipo(s.clone().as_str()) {
      Some(TipoStr::Alfabeto) => 
         matriz = forma_palavra(s.clone().as_str()),
      Some(TipoStr::Numerico) => 
         matriz = forma_numero(s.clone().as_str()),
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
   
   #[test]
   #[ignore]
   fn testa_fec() {
      let vetor = super::fatia_em_classes("numero3casa08quadra46");
      println!("vetor={:#?}", vetor);
      assert_eq!(vec!["numero","3","casa","08","quadra","46"], vetor);
   }

   #[test]
   #[ignore]
   fn igualiza_qtd_linhas() {
      let mut letraH = super::forma_palavra("H");
      let num8 = super::forma_numero("8");
      let diferenca = (num8.len()-letraH.len()) as u8;
      super::preenche_linhas_em_branco(&mut letraH,
                                       super::Lado::Inferior,
                                       diferenca);

      assert_eq!(letraH.len(), num8.len());
   }

   use crate::forma_strings::constroi_simbolos::imprime;

   #[test]
   #[ignore]
   fn diferencas_preenchimento() {
      let mut letraR = super::forma_palavra("R");
      let mut letraH = super::forma_palavra("H");

      super::preenche_linhas_em_branco(&mut letraR,
                                super::Lado::Superior,6);
      super::preenche_linhas_em_branco(&mut letraH,
                                super::Lado::Inferior, 6);
      println!("mostrando resultados:");
      imprime(&letraR);
      println!("\n\n");
      imprime(&letraH);
   } 

   #[test]
   #[ignore]
   fn diferenca_pos_aninhamento() {
      let mut letraH = super::forma_palavra("A");
      let mut num8 = super::forma_numero("8");

      super::aninha_matrizes(&mut letraH, &mut num8);

      println!("como ficou:");
      super::imprime(&letraH.clone());
      super::imprime(&num8.clone());

      assert_eq!(0, letraH.len()-num8.len());
   }

   #[test]
   #[ignore]
   fn concatenacao_aninhamento_resultado() {
      let mut palavra = super::forma_palavra("resultado");
      let mut num = super::forma_numero("42");

      super::aninha_matrizes(&mut palavra, &mut num);

      super::concatena_matriz(&mut palavra, num.clone());
      super::imprime(&palavra);
      assert!(true);
   }


   #[test]
   #[ignore]
   fn testa_DS() {
      let string = super::desenha_string("jabuti1234");
      super::imprime(&string);
      assert!(true);
   }

   #[test]
   fn fatias_agora_com_pontuacao() {
      let string = super::fatia_em_classes("[algo, ok!]");
      println!("vetor com fatias: {:?}", string);
      assert_eq!(string, vec!["[","algo",",",
                              "ok","!","]"]);
   }

   #[test]
   fn string_formador_incrementado() {
      println!("fatiamento: {:?}",super::fatia_em_classes("(entao, entre"));
      let s = super::desenha_string("(entao, entre");
      super::imprime(&s);

      println!("fatiamento: {:?}",super::fatia_em_classes("voce!]"));
      let r = super::desenha_string("voce!]");
      super::imprime(&r);

      println!("fatiamento: {:?}",super::fatia_em_classes("10% de viva?"));
      let v = super::desenha_string("10% de viva?");
      super::imprime(&v);
   }
}

