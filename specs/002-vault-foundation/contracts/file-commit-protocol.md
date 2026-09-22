# Contract: Protocolo de Persistência Atômica (`file-commit`)

**Feature**: `002-vault-foundation` | **Date**: 2026-09-22

Este documento estabelece o protocolo de escrita atômica em duas fases, a estrutura de logs de eventos duráveis (`.jsonl`) e os invariantes de recuperação contra quedas abruptas.

---

## 1. Máquina de Estados e Sequência de I/O

```text
Ator (UI / Engine)              Escritor Único (Writer)                Mídia Não-Volátil (Disco)
       │                                  │                                        │
       │── commit_file(path, data) ───────►│                                        │
       │                                  │── 1. Escrever payload em .tmp_<uuid> ─►│
       │                                  │◄── OK ─────────────────────────────────│
       │                                  │                                        │
       │                                  │── 2. Append evento no journal .jsonl ──►│
       │                                  │── 3. FlushFileBuffers / fdatasync ────►│
       │                                  │◄── OK (Committed!) ────────────────────│
       │                                  │                                        │
       │                                  │── 4. Atomic Rename (.tmp -> target) ──►│
       │                                  │◄── OK (Applied!) ──────────────────────│
       │                                  │                                        │
       │◄── Retorna NewRevision ──────────│                                        │
       │                                  │── 5. Despacha DomainEvent assíncrono ──► (Indexadores)
```

---

## 2. Invariantes de Falha

1. **Falha antes da Etapa 3 (antes do flush do journal)**:
   - O journal não contém a confirmação do novo evento.
   - O arquivo original em `target` permanece 100% inalterado.
   - O arquivo temporário `.tmp_<uuid>` é purgado no boot subsequente.
   - *Resultado*: Sem perda de integridade; estado prévio consistente preservado.

2. **Falha entre Etapa 3 e Etapa 4 (após flush do journal, antes do rename)**:
   - O journal contém o registro do evento confirmado com seu hash SHA-256 e o snapshot correspondente salvo em `.history/objects/`.
   - O arquivo `target` ainda aponta para a versão anterior.
   - Na reinicialização, o módulo de recuperação detecta a disparidade e executa o rename atômico imediatamente.
   - *Resultado*: Transação completada com sucesso no boot; perda de dados = 0 ms.

3. **Garantia Temporal de Perda**:
   - A thread de escrita executa flush do buffer de eventos a cada operação autoral confirmada ou após uma janela de inatividade máxima de 350-500 ms de digitação ininterrupta.

---

## 3. Formato do Journal (`.history/events/YYYY-MM-DD.jsonl`)

Cada linha é um objeto JSON delimitado por `\n`, contendo:
- `v`: Versão do formato do journal (inteiro `2`).
- `id`: UUID v7 do evento.
- `ts`: Timestamp Unix em microssegundos.
- `op`: `"create" | "update" | "delete" | "move"`.
- `target`: Caminho relativo ao cofre.
- `prev_rev`: Inteiro opcional com a revisão esperada.
- `new_rev`: Inteiro sequencial atribuído.
- `hash`: SHA-256 do payload.
- `snap`: Hash do snapshot armazenado em `.history/objects/<snap>`.
