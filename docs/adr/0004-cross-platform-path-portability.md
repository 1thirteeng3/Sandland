# ADR 0004: Portabilidade Multiplataforma e Proibição de Caminhos Hardcoded

**Data:** 2026-09-21  
**Status:** Aceito  
**Contexto:** Infraestrutura, Build System e Resolução de Cofre no Sandland

---

## Contexto e Problema
Projetos desktop em estágio inicial frequentemente introduzem referências absolutas codificadas estaticamente no código-fonte (ex: `PathBuf::from(r"C:\Users\nome\...")` ou `/home/usuario/...`), gerando quebras críticas de execução quando o binário é executado por outros usuários ou em outros sistemas operacionais (Linux, macOS).

Além disso, convenções de separadores de caminho (`\` no Windows vs `/` no Unix) e prefixos UNC estendidos do Windows (`\\?\`) exigem tratamento canônico estrito para evitar falhas em verificações de sandbox.

## Decisão Arquitetural
Estabelecemos como **diretriz arquitetural obrigatória e inegociável** a portabilidade multiplataforma estrita:

1. **Banimento de Caminhos Hardcoded:**
   - É terminantemente proibido o uso de caminhos de usuário estáticos em código de produção, testes ou scripts de build.
2. **Resolução Dinâmica via Sistema Operacional:**
   - A localização do diretório home do usuário e do cofre DEVE ser obtida em tempo de execução através das rotinas padrão do Tauri (`app.path().home_dir()`, `app.path().document_dir()`).
3. **Normalização de Caminhos e Sandboxing:**
   - Toda verificação de sandbox executada pelo `VaultGuard` deve utilizar `std::path::Path::canonicalize()` para ambos os lados da comparação (`base_path` e `target_path`), garantindo compatibilidade uniforme tanto com prefixos normais quanto com prefixos UNC estendidos do Windows.
4. **Separadores Canônicos:**
   - Caminhos relativos internos persistidos em arquivos de metadados ou bancos de dados devem utilizar barras normais (`/`) para garantir portabilidade completa em caso de sincronização do cofre entre máquinas com sistemas operacionais distintos.

## Consequências
### Positivas
- O aplicativo compila e executa de forma determinística e imediata em qualquer máquina de qualquer usuário sem exigir ajustes de ambiente manuais.
- Permite portabilidade transparente de cofres entre Windows, macOS e Linux via sincronização de pastas (ex: Syncthing, Git, Dropbox).

### Negativas / Desafios
- Requer disciplina rigorosa dos desenvolvedores e testes contínuos em pipelines multiplataforma (CI em Windows, Ubuntu e macOS).
