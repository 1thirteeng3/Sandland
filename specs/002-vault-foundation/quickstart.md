# Quickstart & Validation Guide: Fundação do Vault e Protocolo de Persistência

**Feature**: `002-vault-foundation` | **Date**: 2026-09-22

Este guia descreve os cenários de validação executáveis para comprovar que a camada de persistência durável, ciclo de vida do cofre, deduplicação de ativos e proteção anti-TOCTOU atendem integralmente aos requisitos da especificação (`spec.md`).

---

## 1. Pré-Requisitos

- Ambiente de compilação Rust configurado (`cargo` 1.80+).
- Terminal PowerShell ou Bash no Windows/Linux/macOS.
- Repositório clonado e dependências locais resolvidas.

---

## 2. Cenários de Validação Automatizada

### Cenário 1: Ciclo de Vida do Vault e Recusa Segura de Pastas Inválidas
**Objetivo**: Validar que pastas vazias recebem a estrutura canônica e que pastas arbitrárias sem manifesto são rejeitadas sem sofrer mutações.

- **Comando de Teste**:
  ```bash
  cargo test --test vault_lifecycle_tests
  ```
- **Fluxo Executado**:
  1. Cria uma pasta temporária vazia e invoca `init_vault(&path, "Cofre Teste")`.
  2. Verifica que `vault.json` foi gerado com `schema_version: "2.0.0"` e UUID válido.
  3. Verifica que as pastas canônicas (`assets/`, `workspaces/`, `.history/`, etc.) foram criadas.
  4. Aponta `open_vault` para uma pasta arbitrária sem manifesto.
  5. Assegura que o retorno é `Err(VaultError::InvalidRootDirectory)` e que nenhum arquivo foi criado na pasta arbitrária.
- **Resultado Esperado**: Todos os testes do módulo aprovados com sucesso (`test result: ok`).

---

### Cenário 2: Persistência Atômica em Duas Fases (`file-commit`) e Deduplicação CAS
**Objetivo**: Comprovar a integridade dos dados, o registro sequencial no journal e a deduplicação de mídias por hash SHA-256.

- **Comando de Teste**:
  ```bash
  cargo test --test atomic_persistence_tests
  cargo test --test cas_deduplication_tests
  ```
- **Fluxo Executado**:
  1. Abre um cofre de teste e executa `commit_file("workspaces/ws1/board.canvas.json", payload, None)`.
  2. Verifica que o arquivo canônico foi atualizado no disco e que o journal `.jsonl` contém o evento confirmado correspondente.
  3. Adiciona um arquivo binário de 1 MB via `store_asset`.
  4. Adiciona o mesmo arquivo novamente sob outro nome; atesta que apenas 1 arquivo físico reside em `assets/<sha256>.<ext>` e que a chamada retorna o mesmo identificador.
- **Resultado Esperado**: Zero duplicação física no disco; todas as revisões persistidas de forma durável.

---

### Cenário 3: Simulação de Queda Brusca e Recuperação Idempotente no Boot
**Objetivo**: Validar a garantia de recuperação sem perda de integridade após encerramento forçado do processo.

- **Comando de Teste**:
  ```bash
  cargo test --test crash_recovery_tests
  ```
- **Fluxo Executado**:
  1. Inicia uma transação de escrita, grava o registro no journal e simula a interrupção abrupta do processo antes do `rename` atômico.
  2. Inicializa uma nova instância do `VaultStore` apontando para o mesmo cofre.
  3. O `recover_vault` é acionado automaticamente no boot: detecta a operação confirmada no journal, localiza o snapshot correspondente e conclui a publicação do arquivo canônico.
  4. Verifica que arquivos temporários órfãos (`.tmp_*`) foram expurgados.
- **Resultado Esperado**: O estado final do arquivo de destino reflete com exatidão a revisão confirmada no journal; integridade 100% preservada.

---

### Cenário 4: Defesa Contra Ataques TOCTOU e Travessia de Sandbox
**Objetivo**: Comprovar que atalhos maliciosos (symlinks), caminhos relativos fraudulentos (`../..`) e manipulações concorrentes de diretórios são interceptados.

- **Comando de Teste**:
  ```bash
  cargo test --test anti_toctou_tests
  ```
- **Fluxo Executado**:
  1. Tenta invocar leitura/escrita com caminhos contendo `../../etc/passwd` ou `C:\Windows\System32`.
  2. Cria um symlink ou junction dentro do cofre apontando para uma pasta externa sensível e tenta ler através dele.
  3. Verifica que o `VaultGuard` intercepta todas as tentativas antes de qualquer abertura no sistema de arquivos, disparando `SecurityError::SandboxEscapeAttempt`.
- **Resultado Esperado**: 100% das tentativas de fuga bloqueadas; evento registrado em auditoria.
