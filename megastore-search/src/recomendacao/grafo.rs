use std::collections::{HashMap, HashSet};
use crate::modelos::produto::Produto;

pub struct GrafoRecomendacao {
    grafo: HashMap<u32, HashSet<u32>>, // produto_id -> produtos_similares
}

impl GrafoRecomendacao {
    pub fn new() -> Self {
        Self {
            grafo: HashMap::new(),
        }
    }

    pub fn adicionar_produto(&mut self, produto: &Produto, produtos: &HashMap<u32, Produto>) {
        let mut similares = HashSet::new();

        for outro in produtos.values() {
            if produto.id != outro.id && produto.similaridade(outro) > 0.4 {
                similares.insert(outro.id);
            }
        }

        self.grafo.insert(produto.id, similares);
    }

    pub fn recomendar(&self, produto_id: u32, limite: usize) -> Vec<u32> {
        self.grafo
            .get(&produto_id)
            .map(|similares| {
                similares
                    .iter()
                    .take(limite)
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn recomendar_para_carrinho(&self, produtos_ids: &[u32], limite: usize) -> Vec<u32> {
        let mut recomendacoes = HashSet::new();
        
        for &produto_id in produtos_ids {
            if let Some(similares) = self.grafo.get(&produto_id) {
                for &similar_id in similares {
                    if !produtos_ids.contains(&similar_id) {
                        recomendacoes.insert(similar_id);
                    }
                }
            }
        }

        recomendacoes.into_iter().take(limite).collect()
    }
}
