use std::collections::{HashMap, HashSet};

pub struct GrafoRecomendacao {
    relacoes: HashMap<u32, HashSet<u32>>,
}

impl GrafoRecomendacao {
    pub fn new() -> Self {
        Self {
            relacoes: HashMap::new(),
        }
    }

    pub fn adicionar_relacao(&mut self, produto_id: u32, relacionados: Vec<u32>) {
        let conjunto = self.relacoes.entry(produto_id).or_insert_with(HashSet::new);
        conjunto.extend(relacionados);
    }

    pub fn recomendar_baseado_carrinho(&self, carrinho: Vec<u32>) -> Vec<u32> {
        let mut recomendacoes = HashSet::new();
        let carrinho_set: HashSet<u32> = carrinho.iter().cloned().collect();
        
        // Usa referência para evitar mover o carrinho
        for produto_id in &carrinho {
            if let Some(relacionados) = self.relacoes.get(produto_id) {
                recomendacoes.extend(relacionados);
            }
        }
        
        // Remove produtos que já estão no carrinho
        for produto_id in &carrinho_set {
            recomendacoes.remove(produto_id);
        }
        
        recomendacoes.into_iter().collect()
    }
}
