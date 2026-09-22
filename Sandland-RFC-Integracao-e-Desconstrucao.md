# Sandland — Plano de integração, desconstrução e desenvolvimento próprio

**Documento complementar ao RFC/PRD preliminar**  
**Data:** 22 de setembro de 2026  
**Status:** proposta de revisão arquitetural; decisões ainda sujeitas à aprovação e a provas de conceito  
**Base analisada:** `Documento de Arquitetura Técnica e Especificação de Produto.md`, fornecido pelo autor  
**Premissa de produto:** aplicação open source, conduzida sem fins comerciais

> **Síntese:** manter Tauri/Rust como núcleo de autoridade; PixiJS como cena espacial; BlockSuite como motor editorial; adaptar o núcleo estrutural do PageIndex ao contexto das células; escolher Qdrant ou HelixDB como índice de recuperação; integrar Jev como provedor de decisões; Docling e os motores web como trabalhadores especializados; LocalAI como infraestrutura de inferência. O Sandland desenvolve os contratos, a persistência canônica, a semântica do produto, a segurança e a coordenação entre esses motores.

Este plano não propõe substituir a seleção do autor por outra stack. Também não propõe incorporar as interfaces e os backends completos de todos os produtos. O objetivo é decompor responsabilidades e reaproveitar os mecanismos adequados.

**Limites desta análise:** foram lidos o RFC inteiro, documentação técnica, manifests e trechos relevantes de código upstream. Não foram compilados os componentes nem executados benchmarks ou testes de integração. As interfaces Sandland aqui nomeadas são propostas, não APIs já implementadas. O exame de licenças é arquitetural, não um parecer jurídico completo sobre todas as dependências transitivas.

---

## 1. Decisão estrutural antes de começar

O RFC atual não é integralmente compatível com a composição escolhida posteriormente.

Sua seção 1 exige backend, extração e inferência **estritamente em Rust, sem interpretadores externos**. Entretanto:

- PageIndex, Docling, Scrapling e SearxNG utilizam Python.
- LocalAI tem núcleo em Go e backends de diferentes tecnologias.
- Readability e Turndown são bibliotecas JavaScript.
- O próprio desenho original já prevê motores C/C++ através de llama.cpp e whisper.cpp; bindings Rust não transformam esses motores em Rust.

### Proposta recomendada de substituição do princípio

> **Núcleo nativo, extensões controladas:** o núcleo de dados, autorização, persistência e coordenação é Rust. Motores especializados podem usar outras linguagens, executados localmente em processos gerenciados, com versões fixadas, permissões mínimas, limites de recursos e desligamento quando ociosos. O usuário não precisa instalar ambientes de desenvolvimento, e nenhum serviço remoto é obrigatório para acessar e editar seus dados.

Isso preserva soberania e controle de recursos sem exigir reimplementar todos os motores escolhidos.

Empacotar um worker Python em executável não elimina o interpretador: apenas o distribui junto. Da mesma forma, «instalador único» pode conter vários processos auxiliares e pacotes opcionais; não deve ser confundido com «um único binário sem dependências».

**Se Rust estrito permanecer inegociável**, PageIndex, Docling e Scrapling passam a ser referências para portabilidade/reimplementação; SearxNG vira um serviço externo ao núcleo; LocalAI deixa de ser o runtime adotado. Esse caminho é diferente e demanda significativamente mais desenvolvimento próprio. Não é a hipótese principal deste plano.

A mudança de princípio é uma **ADR a aprovar**, não uma alteração já autorizada no documento original.

---

## 2. Mapa de reaproveitamento

Há quatro formas de integração:

1. **Biblioteca:** utilizar módulos upstream pela interface pública.
2. **Adaptador:** traduzir contratos Sandland para um motor existente.
3. **Extração/fork delimitado:** modificar um subconjunto, preservando origem, licença e testes.
4. **Desenvolvimento próprio:** implementar semântica específica do Sandland.

Desconstruir responsabilidades não exige copiar fisicamente cada repositório. Em bancos e runtimes, a fronteira pública costuma economizar mais manutenção que extrair estruturas internas.

| Motor | Forma principal | Reaproveitar | Desenvolver no Sandland | Evitar incorporar |
|---|---|---|---|---|
| BlockSuite | Bibliotecas + extensões | Edição de blocos, rich text, comandos, seleção, modelos e mecanismos de edição | Perfil Markdown, adaptador de persistência, blocos de citação, edição de células e Peças, ligação com PixiJS | Aplicação AFFiNE completa, autenticação/cloud e um segundo canvas concorrente |
| PageIndex | Extração/adaptação delimitada | Hierarquia, resumos por nó, navegação e leitura progressiva | IDs estáveis, árvore projetada do board, acesso às relações, escopo por workspace, invalidação e orçamento de contexto | Store como fonte de verdade, suposição de que célula é página de PDF, permissões baseadas apenas no prompt |
| Qdrant/HelixDB | Adaptador para um motor escolhido | Índices, consultas, filtros e persistência dos índices | Contrato de busca, proveniência, revisão dos resultados, reindexação e política de relevância | Autoridade sobre conteúdo; dois motores ativos por padrão |
| Jev | Cliente de API + DecisionProvider | Choice, Score, Noul e resultados probabilísticos | Taxonomia, critérios, candidatos, limiares, revisão humana, autorização de envio e fallback | Geração de texto, invenção de tags livres, suposição de execução local |
| Docling | Worker local | Parsing, OCR, estrutura, tabelas e exportação | Entrada segura, conversão para formato Sandland, mapeamento às fontes, fila e orçamento | Serviço sempre residente; acesso geral ao Vault |
| Readability/Turndown | Adaptador JS | Artigo principal e HTML → Markdown | Ambiente de execução isolado, sanitização, URLs/assets, metadados e formato canônico | HTML remoto na WebView privilegiada; execução de scripts da página |
| SearxNG | Serviço local gerenciado ou instância pessoal configurada | Metabusca e API de resultados | Widget, escopo da consulta, deduplicação, histórico e política de rede | Frontend do SearxNG dentro do produto; dependência silenciosa de instância pública |
| Scrapling | Worker local | Aquisição HTTP, páginas dinâmicas, seletores e sessões | Política de URLs, browser sob demanda, extração de evidências e lifecycle | Chromium por widget, crawl irrestrito, autoridade para escrever no Vault |
| LocalAI | Processo de inferência gerenciado | APIs, backends, carregamento, descarregamento e recursos dos modelos | Catálogo aprovado, perfis de hardware, política de uso, instalação verificada e integração | Agentes, memória/RAG e execução de ferramentas paralelos aos do Sandland |

**Complexidade própria mais alta:** persistência/revisões, adaptação PageIndex–topologia, integração PixiJS–editor, proveniência e sandbox multiplataforma. O trabalho não desaparece; muda de implementar motores para integrar invariantes.

---

## 3. Arquitetura de destino

```text
APRESENTAÇÃO — Tauri WebView
  PixiJS: câmera, seleção, conexões, grupos, culling e LOD
  BlockSuite: conteúdo editável das células e das Peças
  UI própria: Ingest, widget de pesquisa, intenções e copiloto
                       │ IPC tipado
                       ▼
SANDLAND CORE — Rust, autoridade do produto
  VaultStore / RevisionStore / ProvenanceStore
  BoardService / PieceService / TaxonomyService / IntentService
  JobScheduler / ResourceSupervisor
  ToolBroker / EgressPolicy / SandboxManager
  SearchService / ContextService / ModelGateway
                       │ contratos versionados
       ┌───────────────┼───────────────────────────────┐
       ▼               ▼                               ▼
ÍNDICES LOCAIS    WORKERS LOCAIS                  PROVEDORES DE REDE
SQLite*          Docling                         Jev, se autorizado
Qdrant OU Helix  PageIndex adaptado               BYOK, se autorizado
Context cache    Scrapling + browser eventual    SearxNG → buscadores
                 normalizador JS
                 LocalAI → backends/modelos
       │               │
       └──── leituras/propostas por escopo ─────────────┘
                       │
                       ▼
VAULT CANÔNICO
Markdown + frontmatter + topologia JSON + assets + histórico aberto
```

`*` SQLite pode continuar como projeção de inventário, fila reconstruível e índice léxico. Sua presença não obriga a manter sqlite-vec quando outro motor assumir os vetores.

### Regra de autoridade

- A UI propõe operações de produto.
- Workers produzem resultados, nunca mutações arbitrárias do Vault.
- O Core valida esquema, escopo, revisão de entrada e permissões antes de persistir.
- Índices recebem eventos posteriores ao registro durável da mudança.
- LLMs não recebem acesso a shell, caminhos arbitrários ou credenciais do processo principal.
- A ausência de IA, de rede ou de um índice não bloqueia a abertura e edição do conhecimento existente.

---

## 4. BlockSuite + PixiJS: reaproveitar edição, desenvolver a mesa

### 4.1 Recorte de upstream

Na árvore atual `AFFiNE/blocksuite`, existem componentes como:

- `framework/store` — `@blocksuite/store`;
- `framework/std` — `@blocksuite/std`;
- `affine/rich-text` — `@blocksuite/affine-rich-text`;
- modelos, blocos, inlines e componentes auxiliares sob `affine/`.

Os manifests inspecionados declaram dependências internas `workspace:*`. Portanto, não se deve presumir que copiar três diretórios basta para obter um pacote independente. Primeiro se fecha o grafo mínimo de dependências e se fixa uma revisão coerente.

**Estratégia:** preferir pacotes públicos adequados; se o recorte necessário só existir no monorepo, produzir um pacote de integração reproduzível, com origem/commit, patch set e testes. Não atualizar automaticamente contra `canary`.

### 4.2 Fronteira entre os dois motores visuais

- **PixiJS:** cena, posição, zoom, conectores, seleção de cartões e previews.
- **BlockSuite:** conteúdo do cartão em edição e documento da Peça.
- **Sandland:** foco, atalhos, transições, clipboard, arraste de conteúdo, permissões e persistência.

Um editor DOM/Web Components não é um sprite PixiJS. O plano deve prever um **overlay DOM sincronizado com a câmera** ou um painel editorial ao entrar em edição.

Não montar um editor completo em cada um dos 1.000 nós. Cartões passivos usam previews; o editor é ativado somente nos poucos elementos que realmente estão sendo editados. A seção 5 do RFC deve trocar «inputs ativos em todos os nós próximos» por uma política explícita de ativação.

Também são trabalho próprio: IME, acessibilidade, seleção de texto versus arraste do cartão, escala do editor, múltiplos monitores e captura de atalhos sem conflito com os comandos do canvas.

### 4.3 Perfil de documento Sandland

Definir os blocos admitidos e a representação de cada um em Markdown/frontmatter:

- parágrafos, títulos, listas, código, tabelas e citações;
- links e referências a assets;
- referências de origem e IDs de blocos quando necessários;
- metadados desconhecidos, que não podem ser descartados por regravação.

Blocos sem representação reversível devem ter extensão aberta documentada ou ser desabilitados. Não pode haver um «export Markdown» que silenciosamente perca conteúdo.

Yjs pode permanecer como estado de edição. O requisito é reconstruir **conteúdo e identidades relevantes** a partir dos arquivos canônicos, não preservar automaticamente todo o histórico interno CRDT. Se esse histórico se tornar requisito autoral, terá de ganhar representação durável própria.

### 4.4 Fork-on-insert

Manter o comportamento previsto na seção 7: inserir uma célula na Peça cria uma cópia independente. Porém, a citação precisa ser mais completa que `source_cell_id` + data:

```text
source_cell_id
source_revision
source_block_id ou seletor do trecho
source_asset_hash / referência ao material original
quote e quote_hash
piece_block_id de destino
inserted_at
```

Editar a célula depois não muda retroativamente o trecho já inserido. O usuário pode comparar com uma revisão nova, mas não receber uma substituição silenciosa.

---

## 5. PageIndex: desconstrução para contexto das células

### 5.1 O que o código atual permite reaproveitar

A inspeção encontrou pontos concretos de partida:

| Arquivo upstream | Conteúdo observado | Uso proposto |
|---|---|---|
| `page_index_md.py` | Extração de headings, construção de árvore e resumo de nós Markdown | Base para conteúdo textual estruturado; adaptar, não converter células em PDF |
| `agent_tools.py` | Contratos e implementações de ferramentas de navegação/leitura | Referência para ferramentas tipadas sobre o escopo Sandland |
| `local_store.py` | `DocStore`, arquivos `tree.json`, `pages.json`, `doc.json` e manifesto | Cache derivado ou referência; não substituir o VaultStore |
| `local_chat.py`, `integrations/` | Camadas de orquestração/integração presentes no repositório | Avaliar no spike; não foram integralmente auditadas nesta análise |

O `page_index_md.py` gera IDs sequenciais na construção da árvore e usa uma heurística própria de headings. Isso não corresponde automaticamente aos IDs persistentes nem ao perfil Markdown do Sandland. Sua sumarização paralela também precisa passar pelo scheduler e orçamento do produto.

O `DocStore` inspecionado possui um lock com `fcntl` e caminho best-effort quando esse mecanismo não está disponível no Windows. Não o tomaria como garantia de concorrência e durabilidade multiplataforma do Vault.

### 5.2 Adaptação proposta: Sandland Context Engine

Construir uma fachada que reaproveita o núcleo pertinente do PageIndex e substitui quatro acoplamentos:

1. **Identidade:** nome de documento/página → IDs de célula, bloco e revisão.
2. **Entrada:** PDF/Markdown genérico → snapshot estruturado do workspace.
3. **Leitura:** store global da biblioteca → ToolBroker com escopo autorizado.
4. **Modelo:** chamadas de provedor próprias → ModelGateway/LocalAI/BYOK sob política do Sandland.

O snapshot já contém grupos e blocos. Não é necessário pedir ao modelo que invente uma hierarquia que o usuário já definiu.

```text
WorkspaceSnapshot
 ├── hierarchy: workspace → grupos → células → blocos
 ├── relations: arestas explícitas, inclusive entre grupos
 ├── layout: posições, dimensões, ordem e agrupamentos
 ├── sources: referências às evidências
 └── revisions: versões dos conteúdos e da topologia
```

A hierarquia é uma **projeção para navegação**, não uma substituição do grafo completo. Arestas cruzadas e ciclos permanecem disponíveis em ferramentas específicas.

### 5.3 Ferramentas Sandland propostas

Estes nomes representam interfaces novas, não funções já fornecidas pelo PageIndex:

- `get_workspace_overview(snapshot_ref, budget)`;
- `get_group(group_id, revision)`;
- `read_cell(cell_id, revision, block_range)`;
- `read_relations(node_id, relation_filter)`;
- `read_source(evidence_ref)`.

Todas passam pelo ToolBroker. Informar um documento no prompt não concede nem limita acesso por si só: a própria documentação de integração do PageIndex alerta que o contexto selecionado não restringe automaticamente o conjunto de ferramentas àqueles documentos.

### 5.4 Skeleton Map e RLM

O RFC usa «RLM» como Recursive Language Model, enquanto a descrição conceitual anterior usava Relational Language Modeling. Escolher uma definição no glossário. O mecanismo aqui proposto é **navegação estrutural com expansão e síntese progressivas**, sem pressupor treinamento de um novo modelo.

O nível 0 não deve receber 100 palavras por célula de um board com 1.000 células: seriam aproximadamente 100 mil palavras antes da pergunta. Usar:

1. mapa global limitado por tokens;
2. resumos de grupos e IDs relevantes;
3. expansão sob demanda;
4. leitura de evidências quando necessária;
5. resposta com referências verificáveis.

Limitar profundidade, chamadas, tokens, tempo e tamanho das respostas das ferramentas. Subagentes logicamente independentes não precisam carregar várias instâncias de modelo em paralelo; o scheduler pode serializar a inferência num único backend.

A promessa de redução «em até 80%» da seção 8 deve virar hipótese a medir, não critério presumido. Resumos podem omitir exceções: a evidência original continua acessível.

### 5.5 Invalidação incremental

Separar revisões de conteúdo e layout:

- mover cartão: atualiza geometria/topologia, não recalcula embeddings e resumos textuais;
- alterar texto: invalida resumo da célula, evidências afetadas e resumos dependentes;
- alterar aresta/grupo: invalida o mapa relacional correspondente;
- alterar modelo/prompt de resumo: invalida somente as derivações que o utilizam.

O índice inclui hashes das entradas e identificação do pipeline. Um índice antigo nunca deve aparecer como contexto atualizado sem aviso.

**Aceite do spike:** a mesma pergunta deve distinguir uma aresta «apoia» de «contradiz», consultar fontes dentro do escopo e manter referências após renomear arquivos. A compatibilidade só estará demonstrada depois desse teste.

---

## 6. File-as-Truth: corrigir a autoridade dos arquivos

### 6.1 Classes de dados

| Classe | Exemplos | Pode ser apagada sem perda autoral? |
|---|---|---|
| Canônica | Markdown, topologia, intenções, fontes preservadas, taxonomia aprovada, citações | Não |
| Histórico canônico | Revisões e eventos necessários a rollback/auditoria | Não, salvo política explícita de retenção |
| Derivada | Embeddings, FTS, árvore de contexto, previews, MPK, projeções SQL | Sim |
| Efêmera | Arquivos temporários, scratchpad não promovido, logs de execução não autorais | Sim |

**Resultado aceito ou editado pelo usuário não é cache**, mesmo que tenha sido originado por IA.

### 6.2 Duas correções indispensáveis no RFC

**A. `board.canvas.mpk` versus `board.canvas.json`.** Hoje o MPK recebe mudanças antes do JSON, criando dois estados com precedência implícita. Recomenda-se:

- JSON como representação canônica da topologia;
- MPK como cache rápido, identificado pelo hash/revisão do JSON;
- movimentações em andamento na memória;
- operações duráveis no journal, com checkpoints JSON;
- recuperação por checkpoints + operações confirmadas, nunca escolhendo o arquivo de maior timestamp por conveniência.

Não realizar I/O síncrono de arquivo na thread de renderização a cada 350 ms. Usar fila de escrita e definir o significado de «salvo».

**B. `audit_log.db` como índice efêmero.** Se ele contém o único histórico necessário a desfazer/recuperar, não é descartável. Para conservar a premissa de bancos reconstruíveis, mover os registros autorais a um journal aberto e snapshots; SQLite passa a ser uma projeção desse histórico.

Uma alternativa seria declarar explicitamente o banco de auditoria como canônico. Isso é possível, mas muda a premissa original e não é a recomendação principal.

### 6.3 Estrutura proposta

```text
vault/
├── vault.json                         # versão do formato e identidade do Vault
├── assets/<hash>.<ext>                 # fontes/binários preservados
├── ingest/.../*.md                    # conteúdo textual de referência
├── taxonomy/terms.json                # IDs, termos, aliases e decisões aprovadas
├── intentions/intentions.json
├── workspaces/<id>/
│   ├── board.canvas.json              # topologia canônica
│   ├── cells/*.md
│   ├── pieces/*.md
│   └── scratchpad/                     # temporário até promoção explícita
├── .history/
│   ├── events/*.jsonl                  # eventos duráveis, versionados
│   └── objects/<hash>                  # revisões/snapshots necessários
└── .system/cache/
    ├── index.db                       # projeções reconstruíveis
    ├── search/                        # Qdrant OU HelixDB
    ├── context/                       # árvores/mapas derivados
    └── boards/*.mpk                   # caches por revisão
```

Caminhos físicos são escolhidos pelo usuário e resolvidos pelo Core. O esquema é relativo, não uma exigência de instalar em `/vault`.

### 6.4 Protocolo de persistência

O VaultStore precisa de um protocolo definido e testado, não de gravações independentes no SQLite e no Markdown:

1. validar a revisão esperada e a operação;
2. preparar conteúdo/objetos de revisão sem sobrescrever o estado confirmado;
3. registrar de forma durável a operação e referências recuperáveis;
4. publicar arquivos/checkpoints através de substituições atômicas apropriadas ao SO;
5. sinalizar confirmação e emitir evento para indexadores;
6. recuperar operações interrompidas de forma idempotente.

A ordenação detalhada de journal, commit markers, `fsync`/equivalentes e renames é trabalho de engenharia. Uma transação SQLite não torna múltiplos arquivos automaticamente transacionais.

A meta de perda máxima de 500 ms requer escrita durável dentro dessa janela, protocolo de recuperação e testes de falha. Atomicidade de rename, sozinha, não garante persistência após queda de energia.

### 6.5 Alterações externas e append-only

- File watcher acelera a detecção; varreduras de reconciliação corrigem eventos perdidos.
- Mudança externa gera nova revisão e invalida índices; conflito não pode sobrescrever a edição do usuário silenciosamente.
- «Ingest append-only» deve significar fonte preservada, não proibir todo enriquecimento posterior.
- Separar original imutável de metadados/anotações versionados. Uma reextração não sobrescreve uma correção manual sem política explícita.
- Células, peças e ocorrências no board têm identidades próprias; caminhos são localizações mutáveis, não chaves permanentes.

---

## 7. Ingest: Docling, Jev e resumo executivo

### 7.1 Pipeline

```text
CAPTURAR
  preservar original, gerar ID e registrar hash
       ↓
EXTRAIR
  arquivos → Docling
  HTML adquirido → Readability/Turndown
  áudio/vídeo → pipeline de transcrição homologado
       ↓
NORMALIZAR
  blocos + texto + páginas/tempos/seletores + assets
       ↓
ENRIQUECER, EM TAREFAS INDEPENDENTES
  Jev → decisões tipadas, quando autorizado
  modelo leve via LocalAI → resumo executivo de 2–5 frases
  modelo de embeddings → representação semântica
       ↓
PERSISTIR RESULTADOS VERSIONADOS
       ↓
ATUALIZAR ÍNDICES E INTENÇÕES
```

A captura não espera o enriquecimento. Um material já salvo pode estar «OCR pendente», «classificação pendente» ou «sem índice semântico», sem impedir leitura e organização manual.

### 7.2 Docling

Reaproveitar parsing/OCR/estrutura. O worker recebe somente entradas autorizadas e escreve em área temporária. O adaptador devolve um documento normalizado ao Core.

Preservar a relação entre texto extraído e fonte: página, bloco/tabela, coordenadas quando disponíveis, timestamp de transcrição e hash do original. Achatar tudo em Markdown e perder esse mapa prejudica citações futuras.

Não exigir OCR/VLM para um arquivo textual simples. Pipelines pesados têm perfil de execução separado e devem falhar de maneira recuperável quando o orçamento do hardware não permitir a tarefa.

### 7.3 Jev como implementação de primeira classe

A taxonomia deixa de depender de uma resposta generativa contendo categoria, tags e resumo no mesmo JSON.

| Operação | Mecanismo |
|---|---|
| Categoria entre opções | Choice, incluindo opção de insuficiência/nenhuma apropriada |
| Aplicação de tags candidatas | Noul por tag, sem forçar exclusividade entre tags |
| Relevância ou outra rubrica ordenada | Score |
| Novo termo textual | Extração determinística ou proposta do modelo generativo, seguida de validação |
| Resumo executivo | Modelo generativo, não Jev |

Jev não gera texto livre. Confidence de Choice/Score é derivada da distribuição, não uma garantia de correção. Noul não fornece uma confidence separada.

O Core registra modelo/versionamento da taxonomia, critérios, probabilidades e decisão aplicada. Limiares devem ser calibrados com materiais representativos; scores de Jev não são automaticamente intercambiáveis com números produzidos por um LLM local.

**Integração nativa não exige dependência obrigatória:** Jev pode ser o provedor preferencial quando autorizado. Sem rede, uma implementação local usa o mesmo contrato de tarefa, mas identifica suas próprias limitações; outra opção é manter a tarefa pendente ou usar regras léxicas. Não há fallback silencioso para nuvem.

Na oferta pública consultada, Jev é acessado pela API TypeSafe. Não foi identificada distribuição pública de pesos para executá-lo no LocalAI. Usá-lo obrigatoriamente contradiz a promessa de classificação totalmente offline.

### 7.4 Correções à taxonomia do RFC

- `Levenshtein <= 2` não é prova de identidade: remover merge silencioso.
- Similaridade vetorial alta não comprova sinonímia: relações/aliases relevantes precisam de política explícita e reversão.
- Guardar IDs estáveis dos termos, rótulos originais e idioma; kebab-case não deve ser a identidade semântica.
- Recuperar candidatos antes de chamar Jev é útil, mas o conjunto deve admitir «nenhum» e expansão controlada se a categoria correta não foi recuperada.
- `bge-small-en-v1.5` e o modelo multilíngue citados em seções diferentes não devem compartilhar um índice como se gerassem vetores equivalentes. Cada índice tem um manifesto de modelo/preprocessamento.
- Um fallback para OOM não pode depender obrigatoriamente de carregar outro modelo de embeddings. Incluir um caminho realmente sem modelo.
- GBNF/JSON Schema controlam formato, não verdade, pertinência ou qualidade das decisões. A validação semântica continua no produto.

### 7.5 Resumo executivo

Salvar o texto curto solicitado, com revisão de entrada, modelo e origem automática. Para documentos longos, gerar resumos intermediários por seções antes do resumo final quando necessário.

O resumo não substitui o corpo do material como evidência nem deve ser a única entrada obrigatória para classificação. Registrar cobertura/limitações quando a extração ou leitura tiver sido parcial.

---

## 8. Busca: Qdrant ou HelixDB, sem duplicar autoridade

### 8.1 Contrato de recuperação

O `SearchService` recebe consulta, escopo, filtros e revisão esperada; devolve IDs/referências e evidências, não caminhos arbitrários para o modelo abrir.

Cada unidade indexada deve carregar, no mínimo:

```text
item_id / source_id
item_kind: ingest | piece
workspace_scope
content_revision
chunk_or_block_id e localizador de origem
embedding_model_id + preprocessing_version
editorial_state / generated_or_authored
```

Indexar Peças com estado editorial e proveniência impede tratar uma síntese anterior da IA como confirmação externa independente.

### 8.2 Caminho Qdrant

Avaliar Qdrant Edge para integração embutida em Rust ou Qdrant Server gerenciado como processo separado, conforme os recursos requeridos.

- Reaproveitar os mecanismos de indexação e recuperação, sem extrair HNSW/storage do banco.
- Confirmar a API e as features da versão de Edge escolhida; não presumir paridade integral com Server.
- Manter FTS5 para léxico na primeira integração, se isso reduzir mudanças, ou adotar uma implementação léxica/esparsa no motor com avaliação equivalente.
- Se FTS5 e Qdrant forem separados, a fusão RRF ocorre no coordenador Rust. A query SQL única da seção 4 deixa de representar a arquitetura.

### 8.3 Caminho HelixDB

Avaliar o modo embutido com disco local e caches limitados.

- Pode reunir vetores, BM25 e projeções de relações.
- Essas relações são índices reconstruíveis da topologia/proveniência canônicas.
- Se o léxico migrar para HelixDB, evitar manter também uma busca FTS equivalente sem justificativa.
- Confirmar recuperação, exclusão, concorrência e empacotamento para os sistemas/arquiteturas-alvo.

### 8.4 Decisão

- **Qdrant:** recorte especializado de recuperação vetorial/híbrida.
- **HelixDB:** oportunidade de unificar busca e consultas relacionais de grafo.

PageIndex não exige que se escolha um banco de grafos. Escolher **um backend inicialmente**, usando uma interface estreita o suficiente para permitir troca sem reescrever o domínio.

A seleção final depende de uma prova comparativa com o acervo real, não de um vencedor presumido. Medir qualidade em português, atualização/exclusão, reinicialização, memória, consultas filtradas e reconstrução.

---

## 9. Widget de pesquisa: SearxNG + Scrapling

### 9.1 Fluxo de aquisição

```text
Consulta explícita do usuário ou proposta autorizada
            ↓
SearxNG: descoberta de resultados
            ↓
Deduplicação e seleção de fontes no Sandland
            ↓
Scrapling: HTTP primeiro; navegador quando necessário
            ↓
Normalização: Readability/Turndown ou Docling, conforme formato
            ↓
Snapshot + origem + data + hash
            ↓
Ingest e referências no widget/células
```

Os três papéis são diferentes: SearxNG encontra; Scrapling adquire; o normalizador prepara conteúdo. A síntese e a interface do widget são próprias.

A API JSON do SearxNG deve estar habilitada. Não depender implicitamente de instâncias públicas que podem bloquear esse formato, registrar consultas ou desaparecer.

Scrapling oferece fetchers HTTP e com browser. As sessões/browser workers são compartilhados sob controle do supervisor, não um Chromium por widget. O projeto também oferece conversão Markdown, que pode ser avaliada como implementação alternativa do normalizador; não aplicar duas conversões consecutivas desnecessárias.

### 9.2 Normalização JS

Readability requer um DOM; Turndown converte HTML para Markdown. Definir explicitamente seu ambiente de execução, por exemplo um renderer/parser isolado sem IPC privilegiado e sem carregamento de recursos remotos.

Não presumir que essas bibliotecas rodem diretamente em um Web Worker comum com DOM completo. Também não inserir o HTML adquirido na janela principal para depois limpá-lo.

Readability não substitui sanitização. Scripts, handlers, URLs perigosas, recursos remotos e conteúdo ativo devem ser tratados antes da apresentação e da persistência das referências.

### 9.3 O que o widget persiste

- consulta, filtros e configuração da pesquisa;
- resultados selecionados e snapshots referenciados;
- execuções com data e estado de atualização;
- síntese e edições autorais;
- posições/conexões na mesa.

Atualizar resultados não pode sobrescrever uma síntese editada. Snippet de buscador é pista de descoberta, não evidência equivalente à página lida.

### 9.4 Política de rede

SearxNG consulta serviços externos. Operação auto-hospedada não significa busca offline nem ausência de divulgação da consulta.

- Gerar consultas a partir de trechos autorizados, não do board inteiro automaticamente.
- Revalidar destinos e redirecionamentos; bloquear acessos indevidos a localhost, redes privadas, metadados e serviços locais do Sandland.
- Limitar bytes, duração, concorrência, profundidade e downloads.
- Não prometer superar todo bloqueio ou paywall: oferecer falha legível, captura pelo navegador do usuário ou importação manual.

O «fast path resolve +85%» do RFC é uma hipótese para o corpus-alvo, não uma propriedade garantida de Scrapling ou de um cliente TLS.

---

## 10. LocalAI e gestão dos trabalhadores

### 10.1 Separar política de execução

**LocalAI controla mecanismos de inferência:** APIs, backends e carga/descarga dos modelos.

**Sandland controla política de produto:** qual modelo pode rodar, para qual tarefa, com quais dados, orçamento, permissões e comportamento em falha.

Não criar um segundo Model Lifecycle Manager que brigue com o LocalAI. O gerenciador Sandland é a camada de catálogo, aprovação, distribuição e supervisão sobre o runtime.

Não utilizar por padrão os agentes, memória/RAG, ferramentas de shell ou mecanismos de aquisição de dados do LocalAI. Eles duplicariam responsabilidades do Core e ampliariam a superfície de acesso.

### 10.2 Backends e capacidades

- Resumo, copiloto e embeddings são papéis distintos; validar o suporte do backend escolhido a cada um.
- Mesmo endpoint «OpenAI-compatible» não garante equivalência de ferramentas, formatos estruturados ou tokenização.
- Jev usa um adaptador próprio de decisões; não é um modelo a baixar via LocalAI.
- Docling não é substituído automaticamente pelo LocalAI: parsing/OCR continuam tendo seu próprio contrato e recursos.

### 10.3 Descarregamento e orçamento

O LocalAI documenta mecanismos como `max-active-backends`, watchdog de ociosidade e configurações de VRAM. A política de cinco minutos pode usar esses mecanismos.

Ainda são responsabilidade Sandland:

- encerrar workers e sessões de navegador ociosos;
- verificar consumo agregado, incluindo backends filhos;
- evitar sobreposição de OCR, transcrição e geração quando não couber;
- não interromper trabalhos ativos como se fossem ociosos;
- restaurar tarefas de forma idempotente após falha;
- validar que o descarregamento efetivamente libera recursos no SO/hardware-alvo.

Planejar batches evita alternar o tempo todo entre modelo de embeddings e modelo gerativo quando apenas um pode permanecer carregado.

### 10.4 Instalação e atualizações

Separar o instalador leve dos pacotes de capacidade opcionais, com artefatos específicos por SO/arquitetura. Não exigir Python/Go/Docker de desenvolvimento no computador do usuário como condição silenciosa de uso.

O pacote pode conter runtimes privados; sua disponibilidade em todas as plataformas deve ser demonstrada. Não se infere suporte a Linux aarch64, Windows ou macOS Intel apenas porque o núcleo Tauri compila ali.

- Pins de versão/commit e lockfiles;
- manifests com hashes completos e origem confiável;
- download consentido, retomável e com rollback;
- provisionamento offline possível;
- migração e rollback de versões de índices;
- nenhuma instalação dinâmica de código sugerida pelo modelo.

Os checksums truncados da seção 10 são placeholders e não servem para verificação. Hash de um arquivo precisa ser comparado a um manifesto confiável; obter ambos de uma origem comprometida não oferece autenticidade.

---

## 11. Contratos que o Sandland precisa possuir

| Contrato proposto | Entrada | Saída | Invariante |
|---|---|---|---|
| `VaultStore` | Operação + revisão esperada | Nova revisão/evento | Nenhum motor escreve canonicamente por fora |
| `EditorBridge` | Snapshot/edição BlockSuite | Patch de domínio | Round-trip do perfil suportado |
| `ExtractProvider` | Referência de entrada autorizada | Documento normalizado + mapa de fontes | Worker sem acesso geral ao Vault |
| `DecisionProvider` | Estado, candidatos, perguntas, política | Resultados tipados + proveniência do provedor | Resultado não equivale a permissão de agir |
| `GenerationProvider` | Contexto aprovado + limites | Texto/patch proposto + referências | Não pode executar mutações por conta própria |
| `EmbeddingProvider` | Texto + manifesto do modelo | Vetor + identidade do espaço | Dimensões iguais não tornam modelos compatíveis |
| `SearchProvider` | Consulta + filtros + escopo | Referências com revisão e scores | Excluir itens apagados/fora do escopo |
| `ContextProvider` | Snapshot do board + pergunta | Contexto/evidências limitados | Não perder relações explícitas nem contornar escopo |
| `WebDiscoveryProvider` | Consulta autorizada | Resultados com origem | Política de saída aplicada antes da rede |
| `WebFetchProvider` | URL aprovada + limites | Snapshot/bytes e cadeia de origem | Proteção contra destinos e redirecionamentos indevidos |
| `RuntimeProvider` | Modelo/capacidade + orçamento | Sessão de execução controlada | Cancelamento, accounting e encerramento |

### Envelope comum de tarefas

Proposta de campos, não uma API existente:

```text
protocol_version
job_id / task_kind
workspace_id / snapshot_revision
input_refs e seus hashes
capability_id de escopo limitado
model_or_pipeline_manifest
deadline / token_budget / memory_profile
cancellation_id
```

Resultados incluem status, referências produzidas, revisão de entrada, métricas e erro tipado. Não aceitar um resultado atrasado de uma revisão antiga como se tivesse processado o texto atual.

Para workers, usar IPC local versionado, por exemplo JSON-RPC sobre pipes ou sockets apropriados ao SO. Para APIs HTTP existentes, a comunicação parte do Core, com autenticação local e escopo restrito. Chaves não ficam no frontend nem no Vault versionado em Git.

### Eventos de domínio

Exemplos: `SourceCaptured`, `ExtractionCompleted`, `ClassificationProposed`, `TaxonomyDecisionAccepted`, `CellEdited`, `BoardRelationChanged`, `CellForkedIntoPiece`, `IndexInvalidated`.

Cada evento contém IDs/revisões, não dependência de um caminho hardcoded. Operações repetidas precisam ser idempotentes ou detectar duplicação.

---

## 12. Segurança: correções necessárias antes de chamar o RFC de aprovado

### 12.1 A função de TOCTOU ainda contém TOCTOU

Na seção 4, a sequência é:

```text
canonicalize → starts_with → File::open(canonical_target)
```

Entre a canonicalização/validação e o `open`, um componente do caminho pode ser substituído. Abrir o caminho já canonicalizado não vincula a abertura ao mesmo objeto que foi verificado.

A correção deve usar primitivas de abertura relativas a diretórios/handles com restrições durante a resolução e tratamento por plataforma. No Linux, `openat2` com um diretório-base e flags como `RESOLVE_BENEATH` e políticas apropriadas de symlinks é uma referência útil. Isso não é uma receita automaticamente portátil para os demais sistemas.

Além do mecanismo de SO, o modelo solicita IDs e referências; o ToolBroker decide quais objetos podem ser abertos. Testes precisam cobrir troca de symlinks, renomeação de diretórios, reparse points/junctions e a política de hard links.

Fonte: [manual de openat2](https://man7.org/linux/man-pages/man2/openat2.2.html).

### 12.2 Sandbox é por processo e perfil, não apenas por linguagem

| Plataforma | Ajuste necessário |
|---|---|
| Linux | Landlock restringe direitos sobre recursos; seccomp restringe chamadas. Negociar suporte do kernel e tratar rede/descendentes separadamente. Não dizer que uma regra de filesystem bloqueia toda syscall ou todo tráfego. |
| Windows | Job Objects agrupam e limitam processos; não são, sozinhos, uma fronteira de arquivos/rede. Investigar AppContainer/capabilities, tokens e ACLs efetivas, com broker de I/O. Remover privilégios administrativos não remove automaticamente todos os acessos do usuário fora do workspace. |
| macOS | `sandbox-exec` está deprecated. Não apresentá-lo como fundação futura garantida; avaliar helpers assinados, App Sandbox/XPC e limites reais de distribuição. App Sandbox também não é uma substituição automática e trivial para todo perfil dinâmico de processo. |

A documentação Microsoft distingue explicitamente limites de jobs e segurança individual dos processos. O manual de `sandbox-exec` registra sua depreciação [1](https://manp.gs/mac/1/sandbox-exec).

Um `ActiveProcessLimit = 1` universal conflita com Chromium multiprocesso e com runtimes que criam backends. Definir perfis de processo próprios para parser, browser e inferência, mantendo o controle de toda a árvore de descendentes.

### 12.3 Escopos de conteúdo e recursos

O RFC proíbe acesso fora do workspace, mas mantém fontes em `/ingest`, assets globais e modelos fora do Vault. É necessário distinguir:

- conteúdo privado de outros workspaces: negado;
- fontes compartilhadas explicitamente autorizadas: somente leitura, por referência;
- modelos, bibliotecas e recursos técnicos: acesso mínimo necessário;
- saída temporária: pasta por tarefa;
- escrita canônica: somente Core.

Workers podem receber snapshots ou handles limitados em vez de acesso ao diretório inteiro. Um LLM que só processa texto não precisa percorrer o filesystem.

### 12.4 Rede e prompt injection

- Inferência local e parsing: sem saída externa, salvo capacidades específicas aprovadas.
- Aquisição web: rede habilitada com política de destinos; sem acesso aos segredos do app.
- Jev/BYOK: envio explicitamente autorizado antes da chamada, inclusive para resumos e metadados sensíveis.
- LocalAI/SearxNG locais: acesso apenas pelas interfaces previstas; não expor serviços desnecessariamente à LAN.
- Documentos, páginas e saídas de modelos são dados não confiáveis.
- Leitura por ferramenta não concede autorização para apagar, exportar ou executar código.
- Falha de confinamento obrigatório deve impedir a tarefa ou oferecer alternativa segura; não reduzir silenciosamente o isolamento.

---

## 13. Desenvolvimento próprio: backlog de responsabilidade

| Pacote de trabalho | Entrega própria | Motor que evita reimplementar |
|---|---|---|
| Formato Sandland | Esquemas, IDs, revisões, migrações e validadores | Nenhum projeto define esse domínio pelo Sandland |
| Núcleo do Vault | Persistência durável, reconciliação externa, conflitos, CAS e reconstrução | SQLite pode acelerar projeções; não resolve o protocolo de arquivos |
| EditorBridge | Serialização, extensão de blocos, undo de produto e integração com Peça | BlockSuite evita criar o motor de edição |
| Mesa | Interação espacial, grupos/arestas, foco, overlays, LOD, galeria de workspaces | PixiJS evita criar um renderizador gráfico |
| Context Engine | Projeção de topologia, IDs estáveis, expansão relacional, budgets, invalidação | PageIndex fornece estrutura e padrões de navegação |
| Taxonomia | Termos/aliases, candidatos, políticas de confiança, revisão e reversão | Jev evita implementar o modelo de decisões |
| Ingest Coordinator | Fila, snapshots, deduplicação, status, cancelamento e normalização | Docling/Readability/Turndown evitam criar parsers completos |
| SearchService | Escopo, revisão, fusão e gestão de reindexação | Qdrant/HelixDB evitam construir índices e motores de consulta |
| Widget de pesquisa | UX, histórico de consultas, seleção, promoção a células e referências | SearxNG/Scrapling evitam criar toda a descoberta/aquisição |
| Peça/Copiloto | Fork-on-insert, propostas de edição, revisão humana e evidências | BlockSuite + modelos gerativos |
| Intenções | Objetivos, alertas, explicação da afinidade e propostas não intrusivas | Busca e Jev podem ajudar a selecionar candidatos |
| Runtime Supervisor | Permissões, instalação, budgets, accounting e lifecycle global | LocalAI evita implementar cada backend de inferência |
| Segurança | ToolBroker, VaultGuard, perfis por SO, autorização de rede e proteção de segredos | Primitivas do SO/Tauri são mecanismos, não a política pronta |
| Governança | Histórico, reversão, exportação, retenção e testes de reconstrução | Git é integração complementar, não substituto do modelo de histórico |
| Distribuição | Builds por plataforma, atualizações, SBOM, licenças e suporte offline | Reutilização não elimina empacotamento e manutenção |

**Não desenvolver novamente:** algoritmos de busca vetorial, renderizador WebGL, edição rica básica, OCR geral, metabusca, browser automation e kernels de inferência — salvo uma lacuna concreta demonstrada.

---

## 14. Open source e ausência de fins comerciais

A ausência de finalidade comercial do projeto não dispensa obrigações de licença, nem torna APIs, hospedagem, assinatura de binários e modelos automaticamente gratuitos.

Também há uma distinção importante: **o projeto pode operar sem fins comerciais, mas uma licença open source reconhecida permite uso comercial por terceiros**. Acrescentar uma cláusula geral «proibido uso comercial» deixaria de atender à definição OSI. Isso não obriga o autor a comercializar o Sandland.

### 14.1 Licenças observadas

| Componente | Situação observada | Consequência prática |
|---|---|---|
| BlockSuite standalone | MPL-2.0 na raiz | Preservar obrigações dos arquivos cobertos quando reaproveitados/modificados |
| Componentes BlockSuite examinados no AFFiNE atual | `store`, `std`, `affine-rich-text` declaram MIT; licença raiz do AFFiNE prevê MIT fora das exceções indicadas | Escolher a árvore/revisão e conferir os arquivos/dependências efetivos; não presumir uma licença única para toda origem possível |
| PageIndex | MIT | Manter avisos e identificar adaptações |
| Qdrant/Edge inspecionado | Código core/manifest de `lib/edge` sob Apache-2.0 | Conferir artefato distribuído e dependências; preservar avisos aplicáveis |
| HelixDB | Apache-2.0 | Preservar licença/avisos e mudanças conforme os termos |
| Docling | MIT no código | Modelos/dependências têm licenças próprias |
| Readability | Apache-2.0 | Preservar avisos aplicáveis |
| Turndown | MIT | Preservar avisos |
| Scrapling | BSD-3-Clause | Preservar avisos e não sugerir endosso dos autores |
| SearxNG | AGPL-3.0, com indicação or-later no README | Cumprir copyleft e fornecimento de fonte correspondente nas condições aplicáveis |
| LocalAI | MIT | Backends/modelos devem ser inventariados separadamente |
| Jev | Serviço API da TypeSafe na oferta pública consultada | Cliente aberto não torna o modelo aberto; considerar termos, custo e autorização de dados |

### 14.2 Sugestão de governança de licença

**AGPL-3.0-or-later é uma candidata coerente para o código próprio**, se a intenção for reciprocidade inclusive em versões oferecidas como serviço. Não é uma escolha automaticamente imposta apenas por consumir uma API do SearxNG.

Outra licença open source pode ser escolhida para os módulos próprios, desde que a distribuição e as obras derivadas respeitem as licenças dos componentes. Extrair código de um módulo AGPL é diferente de interoperar com um serviço separado. A fronteira deve ser examinada no desenho real, não inferida somente do nome do protocolo.

Não rebatizar todos os arquivos de terceiros como se tivessem sido originalmente escritos sob a licença Sandland.

Manter:

- `LICENSES/` e `THIRD_PARTY_NOTICES`;
- SBOM por release;
- origem, commit e patches dos módulos extraídos;
- oferta de fonte correspondente quando exigida;
- scripts reproduzíveis de build/empacotamento;
- licenças e hashes de cada modelo distribuído;
- integração Jev desativável e um fluxo utilizável sem credencial comercial.

Fontes: [definição OSI](https://opensource.org/osd), [AGPL](https://www.gnu.org/licenses/agpl-3.0.html), licenses/manifests listados ao final.

---

## 15. Metas de desempenho: conservar objetivos, retirar garantias presumidas

As metas atuais são úteis, mas não foram demonstradas para essa composição.

| Meta do RFC | Como transformá-la em teste |
|---|---|
| Idle ≤350 MB, 100 nós | Contar Core, WebView, GPU/processos auxiliares relevantes e serviços gerenciados; modelos descarregados; dataset e metodologia fixos |
| Indexing ≤650 MB | Definir um perfil leve de indexação e medir separadamente OCR, transcrição e pipelines multimodais; não prometer que qualquer documento caiba no mesmo teto |
| Inferência ≤2,8 GB | Homologar modelo, contexto, quantização e concorrência por hardware; RAM e VRAM devem ser discriminadas |
| Startup <2,2 s em NVMe | Abrir UI e snapshot sem aguardar modelos, rede ou reconstrução integral de índices |
| Comando <150 ms | Esclarecer se mede aceitação/acknowledgement do comando; não confundir com conclusão de inferência |
| 1.000 nós em 60 FPS | Medir cena representativa, textos/mídias, zoom e overlay de edição; não inferir o resultado apenas pela existência de culling/LOD |
| Unload após 5 min | Verificar memória e processos depois do timeout, sem interromper tarefas ativas |

Não é recomendável afrouxar metas silenciosamente. Se um pipeline de OCR exceder o teto, há uma decisão explícita: perfil separado aprovado, processamento por unidades menores, pacote alternativo homologado ou indisponibilidade daquela operação no hardware em questão.

Os valores «1,8 segundos», «+85%» e «até 80%» do RFC precisam de ambiente, corpus e método de medição. Tratá-los como hipóteses até lá.

### Auditoria criptográfica

Hashes de assets e modelos detectam mudanças relativas a uma referência confiável; não tornam todo o histórico inviolável. Para auditoria, especificar serialização canônica de eventos, encadeamento de hashes e, se necessário, assinaturas/ancoragem externa. Um atacante que pode reescrever o Vault inteiro pode também reescrever uma cadeia não ancorada.

«Criptograficamente auditável» também não significa «criptografado em repouso». O RFC precisa declarar separadamente se depende da criptografia do sistema operacional ou de uma função própria de Vault cifrado.

---

## 16. Ordem de implementação e critérios de passagem

Sem calendário artificial: as etapas têm gates verificáveis, e segurança acompanha cada integração.

### G0 — Contratos e decisões de arquitetura

Aprovar: núcleo Rust + workers, autoridade JSON/journal, política Jev/offline, licença, escopos e perfil Markdown. Definir protocolo de jobs, manifest de modelos e matriz de alvos. Retirar os trechos de segurança incorretamente apresentados como garantidos.

**Saída:** RFC revisado, esquemas iniciais e lista de invariantes testáveis.

### G1 — Fatia vertical de autoria

Tauri + VaultStore + PixiJS + BlockSuite. Criar célula, editar, mover, ligar, inserir na Peça por fork, fechar e reconstruir a partir dos arquivos. Implementar broker de arquivos e testes de concorrência/falha desde aqui.

**Gate:** excluir caches não perde conteúdo, geometria, vínculos ou citações; edição externa não é sobrescrita silenciosamente.

### G2 — Ingest local seguro

CAS, jobs, worker Docling e normalização de clipping. Implementar perfis de processo antes de habilitar arquivos não confiáveis. Primeiro persistir e ler; enriquecimento vem depois.

**Gate:** falha, cancelamento ou documento malformado não corrompe o Vault; fontes e localizadores são preservados.

### G3 — Inferência, decisões e busca

LocalAI gerenciado, resumo de 2–5 frases, embeddings, integração Jev com consentimento e fallback explícito. Comparar Qdrant e HelixDB num spike curto e escolher um para a implementação inicial.

**Gate:** operação offline utilizável; consulta filtrada não mistura workspaces/revisões; classificação incerta não faz merges destrutivos.

### G4 — Contexto estrutural

Adaptar PageIndex, integrar topologia, summaries incrementais e ferramentas delimitadas. Testar relações contraditórias, múltiplas referências, ausência de evidência e alterações de revisão.

**Gate:** escopo é aplicado em código; evidências apontam a revisões válidas; contexto respeita orçamento sem achatar a mesa em chunks cegos.

### G5 — Pesquisa web integrada

SearxNG, Scrapling HTTP/browser, widget próprio, snapshots e promoção ao Ingest. Aplicar proteção de rede e separação dos segredos locais.

**Gate:** páginas hostis e redirecionamentos não alcançam serviços privados; browser e workers encerram; updates não apagam edição autoral.

### G6 — Homologação e distribuição

Perfis de hardware, todos os SOs/arquiteturas declarados, assinatura/manifest de pacotes, SBOM, atualização/rollback e testes de recuperação durável.

**Gate:** publicar apenas capacidades efetivamente homologadas por plataforma. Uma edição Linux x86_64 funcional não comprova automaticamente a matriz inteira.

---

## 17. Alterações propostas por seção do RFC original

| Seção | Manter | Alterar/complementar |
|---|---|---|
| 1 — Filosofia | Soberania, separação cognitiva e budgets por estado | Rust estrito → Core nativo + workers; precisar offline versus rede consentida |
| 2 — Topologia/Vault | Tauri, assets CAS e workspaces | Adicionar adapters/ToolBroker; separar histórico canônico de caches; JSON autoritativo |
| 3 — Taxonomia | Candidatos, normalização e fallback | Jev para decisões; resumo separado; retirar merges silenciosos; versionar taxonomia |
| 4 — Busca/FS | Índices reconstruíveis e híbrido | Qdrant/HelixDB; RRF fora do SQL quando necessário; corrigir TOCTOU |
| 5 — Whiteboard | PixiJS, pílulas, culling e LOD | Overlay BlockSuite, política de editores ativos, precedência JSON/MPK |
| 6 — Web | Aquisição em duas vias e browser sob demanda | SearxNG + Scrapling + normalizadores; retirar exigência Rust puro e promessa de bypass universal |
| 7 — Peça | Fork-on-insert e copiloto | BlockSuite, revisão da origem, ID do bloco de destino e propostas de edição |
| 8 — RLM/Sandbox | Expansão progressiva e menor privilégio | Núcleo PageIndex adaptado, topologia preservada, contexto limitado; revisar OS sandbox |
| 9 — Intenções | Metas, hipóteses e sugestões | Limiar calibrado, origem da sugestão, escopo, feedback e nenhuma criação autoral silenciosa |
| 10 — Modelos | Integridade, catálogo e cache compartilhado | LocalAI como executor; manifests reais; licenças; pacotes offline e lifecycle unificado |
| 11 — Auditoria | Histórico semântico e Git opcional | Journal/snapshots canônicos; SQLite como projeção; política de retenção/ancoragem |
| 12 — Milestones | Entregas incrementais | Gates verticais, segurança desde o início e homologação por plataforma |
| 13 — Aceite | Testes de recursos, durabilidade e isolamento | Metodologia mensurável; recuperação de caches; egress; IDs/revisões e fidelidade editorial |

---

## 18. Testes mínimos que decidem se a composição funciona

1. **Reconstrução:** apagar `.system/cache/`; recuperar conteúdo, mesa, peças, taxonomia aprovada, referências e histórico a partir dos arquivos canônicos.
2. **Round-trip editorial:** importar/exportar o perfil Markdown sem perder blocos suportados, frontmatter desconhecido, citações e IDs necessários.
3. **Crash recovery:** matar processos e simular interrupções em cada etapa de persistência; avaliar separadamente falha de processo e queda de energia.
4. **Edição externa:** modificar/renomear arquivos enquanto o app está aberto; detectar conflito e impedir sobrescrita silenciosa.
5. **Contexto:** mudar arestas/grupos e verificar que as respostas usam o novo snapshot; ausência de fonte deve ser reconhecida, não preenchida por inferência.
6. **Escopo:** referência maliciosa em PDF/HTML não autoriza leitura de outro workspace, modelos secretos, chaves ou caminhos arbitrários.
7. **Rede:** offline com artefatos já provisionados não gera chamadas Jev/BYOK, downloads ou telemetria inesperada; web desativada mantém snapshots legíveis.
8. **Índices:** excluir/alterar material impede recuperação de sua revisão obsoleta como atual; vetores de modelos diferentes nunca são misturados silenciosamente.
9. **Taxonomia:** avaliar tags/categorias em português, out-of-taxonomy e confiança; manter rollback de decisão automática.
10. **Recursos:** contabilizar árvore de processos, browser, modelos, caches e VRAM; testar retorno a idle após OCR e inferência.
11. **Proveniência:** fork-on-insert continua apontando ao trecho e revisão corretos depois de editar ou renomear a célula-fonte.
12. **Distribuição:** instalação limpa sem ferramentas de desenvolvimento prévias; modelos/pacotes íntegros; avisos e fontes correspondentes disponíveis.

**Definição de sucesso:** é possível substituir um motor, apagar seus índices ou ficar sem rede sem perder o conhecimento autoral. Nenhuma reutilização vale a pena se romper essa propriedade.

---

## 19. Referências técnicas verificadas

As URLs de branches móveis servem à análise. Na implementação, fixar versões/commits e arquivar o inventário de origem.

### Documento do autor

- `Documento de Arquitetura Técnica e Especificação de Produto.md`, 13 seções, lido integralmente; preservado sem alteração.

### Edição e licenças BlockSuite

- [Store e Yjs](https://blocksuite.io/guide/store).
- [Árvore BlockSuite dentro de AFFiNE](https://github.com/toeverything/AFFiNE/tree/canary/blocksuite).
- [Manifest store](https://raw.githubusercontent.com/toeverything/AFFiNE/canary/blocksuite/framework/store/package.json).
- [Manifest std](https://raw.githubusercontent.com/toeverything/AFFiNE/canary/blocksuite/framework/std/package.json).
- [Manifest rich-text](https://raw.githubusercontent.com/toeverything/AFFiNE/canary/blocksuite/affine/rich-text/package.json).
- [Licença AFFiNE e exceções](https://raw.githubusercontent.com/toeverything/AFFiNE/canary/LICENSE).
- [Licença do repositório standalone](https://raw.githubusercontent.com/toeverything/blocksuite/main/LICENSE).

### PageIndex

- [SDK, licença e dependências](https://raw.githubusercontent.com/VectifyAI/PageIndex/main/pyproject.toml).
- [Markdown → árvore](https://raw.githubusercontent.com/VectifyAI/PageIndex/main/pageindex/page_index_md.py).
- [Contratos de ferramentas](https://raw.githubusercontent.com/VectifyAI/PageIndex/main/pageindex/agent_tools.py).
- [DocStore local](https://raw.githubusercontent.com/VectifyAI/PageIndex/main/pageindex/local_store.py).
- [Integração com agentes e observação sobre escopo](https://docs.pageindex.ai/sdk/agents).
- [Configuração de modelos/endpoints](https://docs.pageindex.ai/sdk/client).

### Busca, ingestão e inferência

- [Qdrant Edge](https://qdrant.tech/documentation/edge/).
- [Manifest do código Edge — Apache-2.0](https://raw.githubusercontent.com/qdrant/qdrant/dev/lib/edge/Cargo.toml).
- [HelixDB Embedded](https://docs.helix-db.com/database/helix-db/start-here/local-development/embedded-database).
- [Licença HelixDB](https://raw.githubusercontent.com/HelixDB/helix-db/main/LICENSE).
- [Jev e primitivas](https://docs.typesafe.ai/introduction.md).
- [API Jev](https://docs.typesafe.ai/introduction/quickstart.md).
- [Confidence Jev](https://docs.typesafe.ai/confidence.md).
- [Docling](https://github.com/docling-project/docling).
- [Readability](https://github.com/mozilla/readability).
- [Turndown](https://github.com/mixmark-io/turndown) e [licença](https://raw.githubusercontent.com/mixmark-io/turndown/master/LICENSE).
- [API SearxNG](https://docs.searxng.org/dev/search_api.html) e [README/licença](https://raw.githubusercontent.com/searxng/searxng/master/README.rst).
- [Fetchers Scrapling](https://scrapling.readthedocs.io/en/latest/fetching/choosing.html) e [licença](https://raw.githubusercontent.com/D4Vinci/Scrapling/main/LICENSE).
- [LocalAI](https://github.com/mudler/LocalAI), [módulo Go](https://raw.githubusercontent.com/mudler/LocalAI/master/go.mod), [gestão de memória](https://localai.io/docs/advanced/vram-management/) e [licença](https://raw.githubusercontent.com/mudler/LocalAI/master/LICENSE).

### Segurança e governança

- [openat2](https://man7.org/linux/man-pages/man2/openat2.2.html).
- [Landlock](https://landlock.io/).
- [Windows Job Objects](https://learn.microsoft.com/en-us/windows/win32/procthread/job-objects).
- [Windows AppContainer](https://learn.microsoft.com/en-us/windows/win32/secauthz/appcontainer-isolation).
- [Apple App Sandbox](https://developer.apple.com/documentation/security/app-sandbox).
- Depreciação de sandbox-exec: [1](https://manp.gs/mac/1/sandbox-exec).
- [Open Source Definition](https://opensource.org/osd).
- [GNU AGPL-3.0](https://www.gnu.org/licenses/agpl-3.0.html).
