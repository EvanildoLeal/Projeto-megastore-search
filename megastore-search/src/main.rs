mod busca;
mod indexacao;
mod metricas;
mod modelos;
mod recomendacao;

use busca::MotorBusca;
use metricas::MetricasBusca;
use modelos::Produto;
use std::time::Instant;

fn main() {
    println!("Inicializando Sistema de Busca da MegaStore");
    
    // Inicializa motor de busca
    let mut motor = MotorBusca::new(1000);
    let mut metricas = MetricasBusca::new();
    let mut grafo_recomendacao = recomendacao::GrafoRecomendacao::new();
    
    // Popula com dados de exemplo
    println!("Populando catalogo com produtos...");
    popular_catalogo(&mut motor);
    
    println!("Total de produtos: {}", motor.total_produtos());
    
    // Constrói grafo de recomendação simples
    construir_grafo_recomendacao(&motor, &mut grafo_recomendacao);
    
    // Testes de busca
    println!("\nTestando buscas...");
    testar_buscas(&motor, &mut metricas);
    
    // Exibe métricas
    println!("\nMétricas do sistema:");
    metricas.exibir_metricas();
    
    // Teste de recomendações
    println!("\nTestando recomendações...");
    let recomendacoes = grafo_recomendacao.recomendar_baseado_carrinho(vec![1, 2, 3]);
    println!("Recomendações para carrinho: {} produtos", recomendacoes.len());
    
    // Demonstra busca por ID
    println!("\nDetalhes do produto ID 1:");
    if let Some(produto) = motor.obter_produto_por_id(1) {
        println!("Nome: {}", produto.nome);
        println!("Marca: {}", produto.marca);
        println!("Categoria: {}", produto.categoria);
        println!("Preço: R$ {:.2}", produto.preco);
        println!("Descrição: {}", produto.descricao);
    }
}

fn popular_catalogo(motor: &mut MotorBusca) {
    let produtos = vec![
        Produto::new(1, "Smartphone Samsung".to_string(), "Samsung".to_string(), "Eletrônicos".to_string(), 899.99, "Smartphone Android com 128GB".to_string()),
        Produto::new(2, "iPhone 13".to_string(), "Apple".to_string(), "Eletrônicos".to_string(), 1299.99, "Smartphone Apple com 256GB".to_string()),
        Produto::new(3, "Notebook Dell".to_string(), "Dell".to_string(), "Informática".to_string(), 1999.99, "Notebook i7 16GB RAM".to_string()),
        Produto::new(4, "TV LED 55".to_string(), "LG".to_string(), "Eletrônicos".to_string(), 1499.99, "TV Smart 4K 55 polegadas".to_string()),
        Produto::new(5, "Fone Bluetooth".to_string(), "Sony".to_string(), "Áudio".to_string(), 199.99, "Fone sem fio com cancelamento de ruído".to_string()),
    ];
    
    for produto in produtos {
        motor.adicionar_produto(produto).unwrap();
    }
}

fn construir_grafo_recomendacao(motor: &MotorBusca, grafo: &mut recomendacao::GrafoRecomendacao) {
    // Relações simples: produtos da mesma categoria são relacionados
    for produto in motor.obter_todos_produtos() {
        let relacionados: Vec<u32> = motor.obter_todos_produtos()
            .iter()
            .filter(|p| p.categoria == produto.categoria && p.id != produto.id)
            .map(|p| p.id)
            .collect();
        
        grafo.adicionar_relacao(produto.id, relacionados);
    }
}

fn testar_buscas(motor: &MotorBusca, metricas: &mut MetricasBusca) {
    let consultas = vec!["samsung", "eletrônicos", "smartphone", "notebook"];
    
    for consulta in consultas {
        let inicio = Instant::now();
        let resultados = motor.buscar(consulta);
        let duracao = inicio.elapsed();
        
        metricas.registrar_consulta(resultados.len(), duracao);
        
        println!("Consulta: '{}' - {} resultados em {:?}", 
                 consulta, resultados.len(), duracao);
        
        for resultado in resultados.iter().take(3) {
            println!("  → {}: R$ {:.2}", resultado.nome, resultado.preco);
        }
    }
}
