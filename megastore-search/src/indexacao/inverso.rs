use std::collections::{HashMap, HashSet};
use crate::modelos::Produto;
use super::texto::ProcessadorTexto;

pub struct IndiceInverso {
    pub indice_nome: HashMap<String, HashSet<u32>>,
    pub indice_marca: HashMap<String, HashSet<u32>>,
    pub indice_categoria: HashMap<String, HashSet<u32>>,
    pub indice_texto: HashMap<String, HashSet<u32>>,
    processador: ProcessadorTexto,
}

impl IndiceInverso {
    pub fn new() -> Self {
        Self {
            indice_nome: HashMap::new(),
            indice_marca: HashMap::new(),
            indice_categoria: HashMap::new(),
            indice_texto: HashMap::new(),
            processador: ProcessadorTexto::new(),
        }
    }

    pub fn indexar_produto(&mut self, produto: &Produto) {
        // Indexa cada campo separadamente sem problemas de borrow
        Self::indexar_texto(&mut self.indice_nome, &self.processador, &produto.nome, produto.id);
        Self::indexar_texto(&mut self.indice_marca, &self.processador, &produto.marca, produto.id);
        Self::indexar_texto(&mut self.indice_categoria, &self.processador, &produto.categoria, produto.id);
        
        // Indexa texto completo
        let texto_completo = format!("{} {} {}", produto.nome, produto.marca, produto.categoria);
        Self::indexar_texto(&mut self.indice_texto, &self.processador, &texto_completo, produto.id);
    }

    fn indexar_texto(
        indice: &mut HashMap<String, HashSet<u32>>,
        processador: &ProcessadorTexto,
        texto: &str,
        id: u32,
    ) {
        let tokens = processador.processar(texto);
        for token in tokens {
            indice.entry(token).or_insert_with(HashSet::new).insert(id);
        }
    }

    #[allow(dead_code)]  // ← ADICIONE ESTA LINHA
    pub fn buscar_no_indice(&self, termo: &str, indice: &HashMap<String, HashSet<u32>>) -> HashSet<u32>{
        let token = self.processador.processar_termo(termo);
        indice.get(&token).cloned().unwrap_or_else(HashSet::new)
    }

    pub fn buscar(&self, query: &str) -> HashSet<u32> {
        let tokens = self.processador.processar(query);
        let mut resultados = HashSet::new();
        
        for token in tokens {
            if let Some(ids) = self.indice_texto.get(&token) {
                resultados.extend(ids);
            }
        }
        
        resultados
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modelos::Produto;

    #[test]
    fn test_indexacao_busca() {
        let mut indice = IndiceInverso::new();
        let produto = Produto::new(
            1,
            "Smartphone Samsung".to_string(),
            "Samsung".to_string(),
            "Eletrônicos".to_string(),
            899.99,
            "Descrição".to_string()
        );

        indice.indexar_produto(&produto);
        
        let resultados = indice.buscar("samsung");
        assert!(resultados.contains(&1));
        
        let resultados = indice.buscar("eletrônicos");
        assert!(resultados.contains(&1));
    }

    #[test]
    fn test_busca_inexistente() {
        let indice = IndiceInverso::new();
        let resultados = indice.buscar("inexistente");
        assert!(resultados.is_empty());
    }
}
