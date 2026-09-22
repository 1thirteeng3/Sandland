# Pipeline de Inteligência Artificial e Orquestração RLM

**Status:** Aprovado  
**Escopo:** Abstração de Modelos (Model-Agnostic), BYOK, Orquestração RLM por Board e Skeleton Map

---

## 1. Visão Geral da Camada de IA

O Sandland adota uma postura **pragmática, soberana e agnóstica** quanto à inteligência artificial:
- Não amarra o usuário a uma única família de modelos ou fornecedor de nuvem.
- Garante operação 100% offline e privada através de modelos locais leves executados em hardware de consumidor.
- Permite conectar modelos de fronteira via nuvem através da política **BYOK (*Bring Your Own Key*)**.
- Rejeita o RAG ingênuo na mesa de trabalho em favor da modelagem relacional de contexto (**RLM — Relational Language Modeling**).

---

## 2. A Abstração `ModelProvider` (Backend Rust)

Toda interação de geração de texto, extração de entidades e classificação no backend é mediada pela trait unificada assíncrona `ModelProvider`:

```rust
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use tokio_stream::Stream;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String, // "system", "user", "assistant"
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationConfig {
    pub temperature: f32,
    pub max_tokens: usize,
    pub response_schema: Option<serde_json::Value>, // Structured Output (JSON Schema / GBNF)
}

pub type TokenStream = Pin<Box<dyn Stream<Item = Result<String, String>> + Send>>;

#[async_trait]
pub trait ModelProvider: Send + Sync {
    /// Identificador do provedor (ex: "local-llama", "openai", "anthropic", "openrouter")
    fn provider_id(&self) -> &'static str;

    /// Verifica se o provedor está pronto para inferência (pesos carregados ou API key presente)
    async fn is_available(&self) -> bool;

    /// Execução completa (One-Shot) com suporte a Structured Output
    async fn generate(
        &self,
        messages: &[ChatMessage],
        config: &GenerationConfig,
    ) -> Result<String, String>;

    /// Execução com streaming em tempo real (Server-Sent Events / Token a Token)
    async fn generate_stream(
        &self,
        messages: &[ChatMessage],
        config: &GenerationConfig,
    ) -> Result<TokenStream, String>;
}
```

### 2.1. Adaptadores Suportados

#### 1. `LocalLlamaProvider` (Offline / Soberano)
- **Engine:** `llama.cpp` nativo compilado via FFI / bindings Rust (`llama-cpp-2`).
- **Formato:** Pesos quantizados em `.gguf` (Q4_K_M, Q5_K_M).
- **Aceleração:** Detecção automática de GPU via Vulkan / Metal / CUDA com fallback transparente para CPU multithread (AVX2/AVX-512).
- **Estruturação:** Amostragem guiada por gramáticas GBNF geradas a partir de esquemas JSON.
- **Gestão Térmica e de Memória:** Mecanismo de *unload* automático após 5 minutos de inatividade para liberar a memória RAM/VRAM.

#### 2. `CloudApiProvider` (Modelos de Fronteira & BYOK)
- **Integrações:** OpenAI, Anthropic (Claude), OpenRouter, Together AI, Groq e endpoints compatíveis com a especificação OpenAI.
- **Protocolo:** Cliente HTTP Rust nativo (`reqwest`) com suporte a streaming assíncrono SSE e Structured Outputs nativos (`response_format: { type: "json_schema" }`).
- **Segurança:** As chaves de API nunca são enviadas a servidores do Sandland; são salvas localmente no cofre do usuário sob o arquivo protegido `.system/api_keys.json` ou no Keyring do SO.

---

## 3. RLM (Relational Language Modeling) e o Skeleton Map

Na Mesa de Pesquisa (Canvas), o Sandland não realiza busca semântica fragmentada por chunks. O contexto analítico é isolado pelo **workspace/board ativo** e reconstruído na forma de um **Skeleton Map**.

### 3.1. O Conceito de Skeleton Map
O Skeleton Map é uma representação textual hierárquica e concisa de todo o estado topológico do board gerada em milissegundos pelo backend Rust antes de despachar a solicitação para o modelo de linguagem:

```
[WORKSPACE: deep-research-ai]
=== GRUPO: "Segurança de Memória" (ID: grp-01) ===
  • [NÓ: cell-01] "Borrow Checker em Rust"
    Resumo: Previne data races sem garbage collector via rastreamento de tempos de vida.
    Conexões:
      ──(suporta)──► [cell-02] "Adoção no Linux Kernel"
      ──(contrasta)──► [cell-03] "Coletor de Lixo em Go"

  • [NÓ: cell-02] "Adoção no Linux Kernel"
    Resumo: Drivers Rust integrados na árvore oficial a partir do kernel 6.1.

=== CÉLULAS ISOLADAS (Rascunhos em Elaboração) ===
  • [NÓ: cell-04] "Ideia: Comparativo de Performance C++ vs Rust"
```

### 3.2. Vantagens do Skeleton Map sobre o RAG Vetorial
1. **Preservação de Contexto Global:** O modelo compreende quais ideias pertencem ao mesmo agrupamento e quais estão conectadas por arestas de causa ou oposição.
2. **Eficiência de Tokens:** Em vez de injetar o conteúdo de 100 páginas de texto bruto, o Skeleton Map injeta apenas títulos, sínteses e o grafo relacional (~800 a 2000 tokens), cabendo com folga até nos menores modelos locais (Phi-3, Gemma-2, Llama-3-8B).
3. **Leitura Paginada Sob Demanda (Page-Index):** Caso o modelo precise detalhar uma célula específica mencionada no Skeleton Map, ele pode emitir uma chamada de ferramenta (*Tool Call*) `read_cell_content(cell_id)` para inspecionar o corpo completo apenas daquele nó.

---

## 4. Gestão de Chaves e Configuração no Frontend (BYOK)

No frontend Svelte 5, um painel modal de configurações permite ao usuário:
1. **Selecionar o Provedor Ativo:** Alternar dinamicamente entre *Local (llama.cpp)*, *OpenAI*, *Anthropic*, *OpenRouter* ou *Endpoint Customizado*.
2. **Gerenciar Chaves Locais (BYOK):** Inserir chaves de API sem telemetria, testar conectividade e verificar cotas.
3. **Download Opt-in de Modelos GGUF:** Baixar sob demanda pesos recomendados para execução local com barra de progresso e verificação de integridade de hash SHA-256.
