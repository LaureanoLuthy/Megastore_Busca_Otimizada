// src/models.rs

use petgraph::graph::NodeIndex;
use serde::{Serialize, Deserialize};

/// Define os tipos de NÓS (Vértices) que o Grafo da MegaStore pode conter.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NodeType { 
    Product, // Produto no catálogo
    User,    // Cliente/Usuário
    Term,    // Palavra-chave ou Tag para indexação
}

/// Define os tipos de ARESTAS (Relacionamentos) entre os nós.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EdgeType { 
    BOUGHT,              // User -> Product (Cliente comprou)
    VIEWED,              // User -> Product (Cliente visualizou)
    RELATED_BY_CATEGORY, // Product <-> Product (Similaridade para recomendação Item-Item)
    RELATED_BY_TAG,      // Product <-> Product (Similaridade por Tags)
    CONTAINS_TERM,       // Term -> Product (Link de indexação da busca)
}

/// STRUCT: Produto. Representa um item no catálogo.
/// Os derives Hash/Eq são necessários para usar Product em HashSets (para busca).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub brand: String,
    pub category: String,
    pub tags: Vec<String>,
}

/// STRUCT: Cliente (User). Representa um usuário ou cliente da MegaStore.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
}

/// STRUCT: Termo. Representa uma palavra-chave usada para indexar produtos.
#[derive(Debug, Clone)]
pub struct Term { 
    pub term: String,
}

/// Enum principal que define o CONTEÚDO de cada nó no grafo.
/// Permite que o petgraph::DiGraph seja heterogêneo.
#[derive(Debug, Clone)]
pub enum GraphNode {
    Product(Product),
    User(User),
    Term(Term),
}

impl GraphNode {
    /// Retorna o tipo do nó para facilitar a filtragem nos algoritmos.
    pub fn get_type(&self) -> NodeType {
        match self {
            GraphNode::Product(_) => NodeType::Product,
            GraphNode::User(_) => NodeType::User,
            GraphNode::Term(_) => NodeType::Term,
        }
    }
}

/// Alias de tipo para o Grafo Direcionado Principal.
/// Define o Grafo como: DiGraph<Conteúdo do Nó, Peso da Aresta>
pub type MegaStoreGraph = petgraph::graph::DiGraph<GraphNode, EdgeType>;

/// Estrutura principal que encapsula o Grafo e os índices de acesso rápido (Tabelas Hash).
#[derive(Debug)]
pub struct SearchSystem { 
    /// O grafo principal com todos os dados e relacionamentos.
    pub graph: MegaStoreGraph,
    /// Tabela Hash (HashMap) para mapear o ID de Produto (externo) para o índice interno (NodeIndex). O(1) Access.
    pub product_id_to_index: std::collections::HashMap<u32, NodeIndex>,
    /// Tabela Hash (HashMap) para mapear o ID de Cliente (externo) para o índice interno (NodeIndex). O(1) Access.
    pub user_id_to_index: std::collections::HashMap<u32, NodeIndex>,
}