# Sistema de Busca para Catálogo de Produtos - MegaStore

## Descrição do Projeto
O MegaStore Search é um sistema de busca desenvolvido em Rust para catálogos extensos de produtos. O sistema utiliza algoritmos avançados de indexação e recuperação de informações para fornecer resultados rápidos e relevantes, mesmo com milhões de produtos.

Principais funcionalidades:

Busca full-text por termos e frases

Indexação inversa para consultas rápidas

Sistema de cache para resultados frequentes

Métricas de desempenho em tempo real

Sistema de recomendação baseado em grafos

Tecnologias Utilizadas
Linguagem: Rust 2021 Edition

# Crates principais:

hashbrown - Tabelas hash de alta performance

rayon - Processamento paralelo

criterion - Benchmarking e testes de desempenho

serde - Serialização/deserialização

clap - Interface de linha de comando

# Ferramentas de desenvolvimento:

Cargo para gerenciamento de dependências

Rustfmt para formatação de código

Clippy para análise estática

# Como Executar o Sistema
## Pré-requisitos
Instale o Rust e Cargo
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Compile em modo release (otimizado)
cargo build --release

# Execute o sistema
cargo run --release -- [ARGS]

# Ou execute diretamente o binário
./target/release/megastore_search [ARGS]

# Exemplo de Uso
## Buscar produtos por termo
cargo run --release -- search "notebook gamer"

## Buscar com múltiplos termos
cargo run --release -- search "smartphone 128gb 5g"

# Ver métricas de desempenho
cargo run --release -- metrics

# Como Executar os Testes

## Testes Unitários
# Executar todos os testes
cargo test

# Testes específicos de um módulo
cargo test test_busca
cargo test test_indexacao

# Testes com output detalhado
cargo test -- --nocapture

# Testes de Integração
## Testes de integração na pasta tests/
cargo test --test integration_tests

# Testes de desempenho com criterion
cargo bench

# Testes com Cobertura
### Instalar ferramenta de cobertura
cargo install cargo-tarpaulin

# Executar testes com cobertura
cargo tarpaulin --ignore-tests

# Testes do Motor de Busca
cargo test busca::motor::tests::test_capacidade_maxima

# Testes de Indexação Inversa:
cargo test indexacao::inverso::tests::test_busca_inexistente
cargo test indexacao::inverso::tests::test_indexacao_busca

# Testes de Métricas:
cargo test metricas::tests::test_multiplas_metricas
cargo test metricas::tests::test_registro_metricas

# Testes de Produto:
cargo test modelos::produto::tests::test_clone_produto
cargo test modelos::produto::tests::test_criacao_produto

# Para executar grupos de testes:
### 1. Todos os testes de busca:
cargo test busca

### 2. Todos os testes de indexação:
cargo test indexacao

### 3. Todos os testes de métricas:
cargo test metricas

### 4. Todos os testes de modelos:
cargo test modelos

### 5. Todos os testes (recomendado):
cargo test

# Exemplo prático:

### Executa apenas os testes de indexação
cargo test indexacao

### Executa apenas os testes de produto
cargo test modelos::produto

# Exemplos de Uso

## Consultas Básicas
// Busca simples por termo
let resultados = motor_busca.buscar("notebook");

// Busca com múltiplos termos (AND)
let resultados = motor_busca.buscar("notebook dell i7");

// Busca por frase exata
let resultados = motor_busca.buscar("\"notebook gamer\"");

# Filtros Avançados
// Busca com filtro de categoria
let filtros = Filtros {
    categoria: Some("eletrônicos"),
    preco_min: Some(1000.0),
    preco_max: Some(5000.0),
    estoque: true
};

let resultados = motor_busca.buscar_com_filtros("smartphone", filtros);

Sistema de Recomendação
// Obter recomendações baseadas em produto
let recomendacoes = sistema_recomendacao.recomendar_por_produto(123);

// Obter recomendações baseadas em histórico
let recomendacoes = sistema_recomendacao.recomendar_por_usuario(456);

# Módulos Principais

1. src/main.rs - Ponto de entrada da aplicação

2. src/busca/ - Motor de busca principal

    motor.rs - Lógica principal de busca

    cache.rs - Sistema de cache de resultados

3. src/indexacao/ - Sistema de indexação

    inverso.rs - Índice inverso para busca rápida

    texto.rs - Processamento de texto

4. src/modelos/ - Estruturas de dados

    produto.rs - Modelo de produto

5. src/metricas/ - Coleta de métricas de desempenho

6. src/recomendacao/ - Sistema de recomendação

    grafo.rs - Grafos de relacionamento entre produtos

# Fluxo de Dados
Consulta → Processamento → Indexação → Busca → Cache → Resultados

# Algoritmos e Estruturas de Dados

## Tabelas Hash (HashMaps)
// Índice inverso principal
HashMap<String, HashSet<u32>> // termo → IDs de produtos

// Cache de resultados frequentes
HashMap<String, Vec<Produto>> // query → resultados

// Grafos de recomendação
HashMap<u32, Vec<(u32, f64)>> // produto → relacionamentos

## Algoritmos Implementados
1. Tokenização e Processamento de Texto

    Lowercasing e stemming
    Remoção de stopwords
    N-grams para buscas parciais
2. Indexação Inversa
    Construção eficiente de índices
    Compressão de postings lists
    Atualização incremental
3. Algoritmo de Busca
    Intersecção eficiente de sets
    Scoring por TF-IDF
    Ordenação por relevância
4. Sistema de Cache
    LRU (Least Recently Used) cache
    Time-based expiration
    Size-based eviction

# Considerações sobre Desempenho e Escalabilidade

## Resultados de Benchmark
- Indexação: 10,000 produtos/sec (single thread)
- Consultas: 50,000 queries/sec (com cache)
- Latência: <5ms para 95% das consultas
- Memória: ~2GB para 1 milhão de produtos

# Estratégias de Otimização
## Paralelismo: Uso intensivo de Rayon para processamento paralelo

## Cache: Sistema multi-camada (memória, disco)

## Compressão: Compactação de índices em memória

## Batch processing: Processamento em lote para grandes volumes

# Escalabilidade Horizontal
## Arquitetura stateless permite múltiplas instâncias

Sharding de índices por categorias ou letras iniciais

Balanceamento de carga com round-robin

Padrões de Código
Siga as convenções do Rustfmt

Use Clippy para verificar best practices

Escreva testes para novas funcionalidades

Documente APIs públicas

## Licença

Este projeto está licenciado sob a Licença MIT.

Licença MIT

Copyright (c) 2024 Evanildo Leal

É concedida permissão, gratuitamente, a qualquer pessoa que obtenha uma cópia
deste software e arquivos de documentação associados (o "Software"), para lidar
com o Software sem restrições, incluindo, sem limitação, os direitos
de usar, copiar, modificar, mesclar, publicar, distribuir, sublicenciar e/ou vender
cópias do Software, e para permitir que as pessoas a quem o Software é
fornecido o façam, sujeito às seguintes condições:

O aviso de direitos autorais acima e este aviso de permissão devem ser incluídos em todas as
cópias ou partes substanciais do Software.

O SOFTWARE É FORNECIDO "NO ESTADO EM QUE SE ENCONTRA", SEM GARANTIA DE QUALQUER TIPO, EXPRESSA OU
IMPLÍCITA, INCLUINDO, MAS NÃO SE LIMITANDO ÀS GARANTIAS DE COMERCIALIZAÇÃO,
ADEQUAÇÃO A UM DETERMINADO FIM E NÃO VIOLAÇÃO. EM NENHUMA HIPÓTESE OS
AUTORES OU TITULARES DOS DIREITOS AUTORAIS SERÃO RESPONSÁVEIS POR QUALQUER REIVINDICAÇÃO, DANOS OU OUTRA
RESPONSABILIDADE, SEJA EM UMA AÇÃO CONTRATUAL, ATO ILÍCITO OU DE OUTRA FORMA, DECORRENTE DE,
DE OU EM CONEXÃO COM O SOFTWARE OU O USO OU OUTRAS NEGOCIAÇÕES NO
SOFTWARE.