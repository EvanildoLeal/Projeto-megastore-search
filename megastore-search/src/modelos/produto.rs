#[derive(Debug, Clone)]
pub struct Produto {
    pub id: u32,
    pub nome: String,
    pub marca: String,
    pub categoria: String,
    pub preco: f64,
    pub descricao: String,
}

impl Produto {
    pub fn new(id: u32, nome: String, marca: String, categoria: String, preco: f64, descricao: String) -> Self {
        Self {
            id,
            nome,
            marca,
            categoria,
            preco,
            descricao,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_criacao_produto() {
        let produto = Produto::new(
            1,
            "Teste".to_string(),
            "Marca".to_string(),
            "Categoria".to_string(),
            100.0,
            "Descrição".to_string()
        );
        
        assert_eq!(produto.id, 1);
        assert_eq!(produto.nome, "Teste");
        assert_eq!(produto.preco, 100.0);
    }

    #[test]
    fn test_clone_produto() {
        let produto = Produto::new(1, "Teste".to_string(), "Marca".to_string(), "Categoria".to_string(), 100.0, "Descrição".to_string());
        let clone = produto.clone();
        
        assert_eq!(produto.id, clone.id);
        assert_eq!(produto.nome, clone.nome);
    }
}
