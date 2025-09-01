use std::collections::HashMap;
use crate::modelos::Produto;
use crate::indexacao::IndiceInverso;

pub struct MotorBusca {
    produtos: HashMap<u32, Produto>,
    indice: IndiceInverso,
    capacidade_maxima: usize,
}

impl MotorBusca {
    pub fn new(capacidade_maxima: usize) -> Self {
        Self {
            produtos: HashMap::new(),
            indice: IndiceInverso::new(),
            capacidade_maxima,
        }
    }

    pub fn adicionar_produto(&mut self, produto: Produto) -> Result<(), String> {
        if self.produtos.len() >= self.capacidade_maxima {
            return Err("Capacidade máxima atingida".to_string());
        }
        
        let id = produto.id;
        self.indice.indexar_produto(&produto);
        self.produtos.insert(id, produto);
        
        Ok(())
    }

    pub fn buscar(&self, query: &str) -> Vec<&Produto> {
        let resultados = self.indice.buscar(query);
        resultados.iter()
            .filter_map(|id| self.produtos.get(id))
            .collect()
    }

    pub fn obter_produto_por_id(&self, id: u32) -> Option<&Produto> {
        self.produtos.get(&id)
    }

    pub fn obter_todos_produtos(&self) -> Vec<&Produto> {
        self.produtos.values().collect()
    }

    pub fn total_produtos(&self) -> usize {
        self.produtos.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modelos::Produto;

    #[test]
    fn test_adicionar_buscar_produto() {
        let mut motor = MotorBusca::new(100);
        let produto = Produto::new(
            1,
            "Test Product".to_string(),
            "Test Brand".to_string(),
            "Test Category".to_string(),
            100.0,
            "Test Description".to_string()
        );

        assert!(motor.adicionar_produto(produto).is_ok());
        
        let resultados = motor.buscar("test");
        assert_eq!(resultados.len(), 1);
        assert_eq!(resultados[0].nome, "Test Product");
    }

    #[test]
    fn test_capacidade_maxima() {
        let mut motor = MotorBusca::new(1);
        let produto1 = Produto::new(1, "P1".to_string(), "B1".to_string(), "C1".to_string(), 100.0, "D1".to_string());
        let produto2 = Produto::new(2, "P2".to_string(), "B2".to_string(), "C2".to_string(), 200.0, "D2".to_string());

        assert!(motor.adicionar_produto(produto1).is_ok());
        assert!(motor.adicionar_produto(produto2).is_err());
    }
}
