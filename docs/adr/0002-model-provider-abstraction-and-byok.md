# ADR 0002: Abstração de Provedores de Modelos (`ModelProvider`) e Política BYOK

**Data:** 2026-09-21  
**Status:** Aceito  
**Contexto:** Camada de Inteligência Artificial do Sandland

---

## Contexto e Problema
O cenário de modelos de linguagem evolui velozmente. Usuários do Sandland variam entre pesquisadores que demandam soberania total e confidencialidade estrita (sem tráfego de dados para a internet) e profissionais que desejam alavancar os modelos mais avançados de raciocínio disponíveis em nuvem (ex: Claude 3.5 Sonnet, GPT-4o, DeepSeek).

Amarrar o software exclusivamente a um motor local rígido (como `llama.cpp` exclusivo) limitaria a versatilidade, enquanto depender de uma API de nuvem centralizada violaria o princípio de soberania e *Local-First*.

## Decisão Arquitetural
Decidimos implementar uma arquitetura agnóstica a modelos baseada na trait unificada Rust `ModelProvider`, aliada ao modelo operacional **BYOK (*Bring Your Own Key*)**:

1. **Trait Rust Unificada:** O backend define a interface assíncrona `ModelProvider`, desacoplando a lógica de negócio (classificação taxonômica, síntese de board, chat) dos detalhes específicos de cada provedor.
2. **Adaptador Local (`LocalLlamaProvider`):** Executa modelos quantizados GGUF de forma 100% offline via bindings nativos de C/C++ (`llama.cpp`), com suporte a GPU e descarregamento automático da memória após inatividade.
3. **Adaptador de Nuvem (`CloudApiProvider`):** Integra-se a provedores compatíveis (OpenAI, Anthropic, OpenRouter, Together AI) com suporte a streaming de tokens e Structured Outputs via JSON Schema.
4. **Política BYOK e Privacidade:** O Sandland não opera servidores de proxy nem intermedia faturamento de tokens. O usuário fornece suas próprias chaves de API, as quais são armazenadas estritamente no dispositivo local e nunca transmitidas a terceiros além do provedor escolhido.

## Consequências
### Positivas
- **Flexibilidade Total:** O usuário escolhe livremente entre privacidade absoluta (local) ou máxima capacidade de raciocínio (nuvem).
- **Sem Custos de Servidor:** Arquitetura sem backend central, eliminando custos de infraestrutura e dependência de serviços proprietários.
- **Extensibilidade:** Novos provedores ou servidores locais (como Ollama ou vLLM) podem ser adicionados como implementações simples da trait `ModelProvider`.

### Negativas / Desafios
- A UI precisa prover interfaces claras para testar chaves de API, alertar sobre erros de cota e orientar o download de modelos locais pesados.
