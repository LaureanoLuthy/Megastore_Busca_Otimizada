// tests/integration_tests.rs

// Importa as funções e estruturas necessárias usando o nome da crate.
// Se o nome no seu Cargo.toml for diferente, ajuste a primeira parte.
use megastore_busca_otimizada::{graph_builder, search, recommender}; 

// Define uma função de teste básica para inicialização do sistema
#[test]
fn test_system_initialization() {
    // Ação: Constrói o sistema usando os dados mockados
    let system = graph_builder::build_system_from_data();
    
    // Asserções: Verifica se o grafo foi construído com os dados esperados.
    // Produtos (5) + Usuários (3) = 8 nós fixos. O restante são Termos.
    assert!(system.graph.node_count() >= 8, "O grafo deve ter pelo menos 8 nós fixos.");
    assert_eq!(system.user_id_to_index.len(), 3, "Deve haver 3 usuários indexados (Tabelas Hash).");
    assert_eq!(system.product_id_to_index.len(), 5, "Deve haver 5 produtos indexados (Tabelas Hash).");
}

#[test]
fn test_search_accuracy() {
    let system = graph_builder::build_system_from_data();
    
    // 1. Teste de Busca por Termo (Busca Otimizada via Grafo BFS)
    // Busca por 'TV' deve retornar 2 produtos (101 e 104)
    let results_tv = search::search_by_term(&system, "TV");
    
    assert_eq!(results_tv.len(), 2, "A busca por 'TV' deveria retornar 2 produtos indexados.");
    assert!(results_tv.iter().any(|p| p.id == 101), "Deve conter Smart TV 4K LG (101).");
    assert!(results_tv.iter().any(|p| p.id == 104), "Deve conter Smart TV QLED Samsung (104).");

    // 2. Teste de Busca por Tag (Busca por 'verao')
    // Deve retornar 1 produto (Vestido ID 103)
    let results_summer = search::search_by_term(&system, "verao");
    assert_eq!(results_summer.len(), 1, "A busca por 'verao' (tag) deveria retornar 1 produto.");
    assert!(results_summer.iter().any(|p| p.id == 103), "Deve conter Vestido Floral Verão (103).");

    // 3. Teste de Busca sem Resultados
    let results_none = search::search_by_term(&system, "banana");
    assert_eq!(results_none.len(), 0, "A busca por termo irrelevante não deveria retornar resultados.");
}

#[test]
fn test_recommendation_collaborative_filtering() {
    let system = graph_builder::build_system_from_data();

    // Cenário: Alice (ID 1)
    // Histórico: Comprou 101, Viu 102.
    // Vizinho Bob (ID 2): Comprou 104, Comprou 102.
    // Recomendação esperada: 104 (TV Samsung), pois Bob comprou e Alice não comprou/viu.
    let user_id_alice = 1;
    let recs_alice = recommender::recommend_for_user(&system, user_id_alice);
    
    assert_eq!(recs_alice.len(), 1, "Alice deve receber exatamente 1 recomendação relevante do vizinho.");
    assert_eq!(recs_alice[0].id, 104, "A melhor recomendação deve ser a Smart TV QLED Samsung (104).");
    assert!(!recs_alice.iter().any(|p| p.id == 101), "Produtos já interagidos não devem ser recomendados.");
    
    // Cenário: Charlie (ID 3)
    // Histórico: Comprou 103, Viu 105. Sem vizinhos com histórico relevante.
    let user_id_charlie = 3;
    let recs_charlie = recommender::recommend_for_user(&system, user_id_charlie);
    assert_eq!(recs_charlie.len(), 0, "Charlie não deve receber recomendações fortes no mock.");

    // Cenário: Usuário inexistente
    let recs_invalid = recommender::recommend_for_user(&system, 999);
    assert_eq!(recs_invalid.len(), 0, "Usuário inexistente não deve gerar recomendações.");
}