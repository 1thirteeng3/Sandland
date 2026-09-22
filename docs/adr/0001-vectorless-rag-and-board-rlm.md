# ADR 0001: Adoção do Paradigma Vectorless RAG e RLM para a Mesa de Pesquisa

**Data:** 2026-09-21  
**Status:** Aceito  
**Contexto:** Mesa de Pesquisa / Canvas do Sandland

---

## Contexto e Problema
A abordagem padrão de RAG (*Retrieval-Augmented Generation*) em aplicações modernas baseia-se em fatiar todo texto em blocos fixos (*chunking* arbitrário de 256 a 512 tokens), gerar embeddings densos através de redes neurais e realizar busca por similaridade de cosseno.

Ao aplicar essa abordagem ingênua sobre a mesa de trabalho espacial (Canvas):
1. **Perda de Estrutura Relacional:** A posição relativa das células, os agrupamentos visuais criados intencionalmente pelo usuário e as arestas direcionadas ("A apoia B", "C refuta D") são completamente ignorados pelos vetores densos.
2. **Poluição e Latência:** Vetorizar micro-post-its e rascunhos voláteis consome ciclos desnecessários de CPU/GPU e polui o banco vetorial com fragmentos descartáveis.
3. **Alucinação Topológica:** O modelo de linguagem recebe trechos sem saber quais ideias pertencem ao mesmo agrupamento conceitual.

## Decisão Arquitetural
Decidimos que **as células, grupos e arestas da Mesa de Pesquisa NÃO serão vetorizados nem fragmentados em chunks**. Em vez disso, adotamos o paradigma **Vectorless RAG** baseado em *Relational Language Modeling* (RLM) e no padrão *PageIndex*:

1. **Topologia Canônica:** A mesa de trabalho é serializada de forma estruturada em `topology.json`.
2. **Skeleton Map em Runtime:** Quando o usuário aciona o copiloto de IA na mesa, o backend gera um índice hierárquico em árvore sintetizando seções, grupos, nós, resumos e conexões.
3. **Injeção Direta na Janela de Contexto:** Esse mapa conciso (~1.000 a 2.000 tokens) é injetado diretamente no prompt de sistema do LLM, preservando 100% da integridade relacional com baixo consumo de tokens.
4. **Resolução Sob Demanda:** Se o modelo necessita do conteúdo detalhado de uma célula específica, utiliza chamada de ferramenta (*tool call*) para ler o corpo daquele nó pontualmente.

## Consequências
### Positivas
- **Fidelidade Cognitiva Total:** A IA compreende a estrutura visual e os agrupamentos conceituais exatamente como organizados pelo usuário.
- **Zero Overhead Vetorial na Mesa:** Não há custo computacional de inferência de embeddings para edições, movimentações ou criação de células.
- **Compatibilidade com Modelos Menores:** O Skeleton Map reduz drasticamente a contagem de tokens, viabilizando execução eficiente em modelos locais de 3B a 8B parâmetros.

### Negativas / Desafios
- Exige que o backend mantenha o serializador e o gerador de Skeleton Map rigorosamente sincronizados com as mutações de nós e arestas no canvas.
