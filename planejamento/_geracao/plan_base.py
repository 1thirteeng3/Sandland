from pathlib import Path

ROOT = Path('/home/user')
SPRINTS = []
RELEASES = [
 ('v0.0.1',1,3,'Arquitetura executável','Interna','Decisões, contratos de dados e inventário de origem aprovados; nenhuma funcionalidade de produto prometida.'),
 ('v0.0.2',4,6,'Bancada reproduzível','Interna','Repositório, CI, fixtures e shell desktop compiláveis sem importar aplicações completas.'),
 ('v0.1.0',7,11,'Vault recuperável','Alpha técnica','Criar, alterar, recuperar e reconstruir conhecimento local com ferramenta de diagnóstico.'),
 ('v0.2.0',12,15,'Editor local BlockSuite','Alpha de autoria','Editar notas no perfil Markdown suportado e reabrir sem depender do estado interno do editor.'),
 ('v0.3.0',16,19,'Mesa espacial','Alpha de autoria','Organizar células, grupos e relações com PixiJS e edição BlockSuite integrada.'),
 ('v0.4.0',20,22,'Peça e proveniência','Alpha funcional sem IA','Produzir texto editorial a partir da Mesa com fork-on-insert, referências e exportação.'),
 ('v0.5.0',23,30,'Ingestão segura','Alpha de acervo','Capturar arquivos/HTML, extrair texto e OCR em workers confinados e organizar o acervo.'),
 ('v0.6.0',31,36,'Enriquecimento e modelos','Alpha assistida','LocalAI gerenciado, resumos, Jev autorizado, fallback local e transcrição homologada.'),
 ('v0.7.0',37,41,'Busca híbrida','Alpha integrada','Escolher Qdrant OU HelixDB e recuperar materiais/Peças com índices descartáveis e revisões válidas.'),
 ('v0.8.0',42,46,'Contexto estrutural','Alpha de raciocínio','PageIndex adaptado às células, Skeleton Map e recuperação relacional limitada por escopo e orçamento.'),
 ('v0.9.0',47,53,'Ciclo analítico completo','Alpha de ciclo completo','Pesquisa web, widget, copiloto, chat e intenções conectam Captura → Mesa → Peça.'),
 ('v1.0.0-beta.1',54,57,'Homologação multiplataforma','Beta','Segurança, performance e pacotes validados nos cinco alvos, sem presumir paridade de aceleração.'),
 ('v1.0.0-rc.1',58,59,'Candidata à estabilidade','RC','Migrações, backup/Git, documentação, acessibilidade e instalação limpa avaliados.'),
 ('v1.0.0',60,60,'Escopo integral estável','Estável','Todos os requisitos do escopo aprovado, evidências e condições de distribuição satisfeitos.'),
 ('v1.1.0',61,63,'Confiabilidade operacional','Estável — maturação','Correções do uso real, recuperação assistida e manutenção upstream sem perda de dados.'),
 ('v1.2.0',64,66,'Escala e portabilidade','Estável — evolução proposta','Acervos maiores, pacotes offline e interoperabilidade ampliada, mediante ratificação de escopo.'),
 ('v2.0.0-beta.1',67,70,'Extensibilidade controlada','Beta — evolução proposta','Contratos de provedores, kit de conformidade e instalação de extensões com permissões explícitas.'),
 ('v2.0.0-rc.1',71,71,'Compatibilidade consolidada','RC','Migração de v1.x, rollback e matriz completa de regressão de motores/formatos.'),
 ('v2.0.0',72,72,'Plataforma estável e sustentável','Estável — fim do horizonte','Base madura, interoperável e extensível; continuidade de manutenção, não fim definitivo do projeto.'),
]

REQUIREMENTS = [
 ('R01','Distribuição','Linux x86_64/aarch64; Windows x86_64; macOS Apple Silicon/Intel','RFC §§1,12,13','Instalação limpa e matriz de capacidades por alvo; build cruzado não equivale a execução homologada.'),
 ('R02','Soberania','Arquivos abertos são a autoridade; bancos e caches são reconstruíveis','RFC §§1,2,4','Apagar caches preserva conteúdo, geometria, taxonomia, citações e histórico canônico.'),
 ('R03','Composição','Núcleo Rust com motores gerenciados, condicionado à ADR','Plano §1','Alteração de Rust estrito aprovada; nenhum runtime é introduzido sem inventário e lifecycle.'),
 ('R04','Formato','Markdown/frontmatter, IDs e extensões abertas documentadas','RFC §§2,4,7','Round-trip sem perda semântica no perfil suportado; metadados desconhecidos preservados.'),
 ('R05','Durabilidade','Journal/snapshots, recuperação e conflitos externos','RFC §§4,11,13','Falhas em cada etapa não corrompem estado confirmado; janela de perda medida.'),
 ('R06','Segurança FS','I/O por handles/capabilities e contenção por sistema operacional','RFC §§1,4,8,13','Tentativas de traversal, troca de links e fuga de processos são negadas nos perfis homologados.'),
 ('R07','Offline','Autoria e leitura sem rede; inferência local após provisionamento','RFC §1; conceito','Bloqueio de rede não impede trabalho local nem dispara fallback externo silencioso.'),
 ('R08','Privacidade','Autorização de egress, segregação de segredos e escopos','RFC §§1,8','Nenhum envio sem política autorizada; ferramentas não podem ampliar seu próprio escopo.'),
 ('R09','Recursos','Idle 350 MB, indexing leve 650 MB, inference 2,8 GB e unload 5 min','RFC §§1,10,13','Medir árvore de processos e perfis aprovados; desvios bloqueiam release ou requerem revisão explícita.'),
 ('R10','Editor','BlockSuite como motor de texto/blocos','Escolha do autor; RFC §§5,7','Editor integrado ao Vault; Yjs não é a única cópia de conteúdo autoral.'),
 ('R11','Mesa','PixiJS, células, grupos, arestas e múltiplos workspaces','RFC §5','Topologia reaberta fielmente; conteúdo e ocorrência visual têm identidades separadas.'),
 ('R12','Renderização','Culling, LOD, overlay editorial e fluidez','RFC §§5,13','Cenas de 100/1.000 nós avaliadas com frame time, hardware e payload documentados.'),
 ('R13','Peça','Editor editorial, fork-on-insert e proveniência','RFC §7','Inserção cria cópia independente referenciando célula, revisão, trecho e bloco de destino.'),
 ('R14','Ingest','Captura sem formulário e enriquecimento assíncrono','RFC §3; conceito','Original salvo antes de OCR/IA; falha de enriquecimento não perde a captura.'),
 ('R15','Extração','Docling para documentos/OCR com mapeamento à fonte','Escolha do autor','Texto, tabelas e localizadores preservados; worker não escreve no Vault canônico.'),
 ('R16','Transcrição','Áudio/vídeo e transcrições locais','RFC §§1,2,10','Modelo homologado, timestamps e fonte rastreáveis; cancelamento e recursos controlados.'),
 ('R17','Clipping','Readability/Turndown em ambiente seguro','Escolha do autor','HTML remoto não ganha scripts, rede ou IPC privilegiado durante normalização.'),
 ('R18','Decisões','Jev nativo, taxonomia e tratamento de incerteza','Escolha do autor; RFC §3 revisado','Choice/Score/Noul respeitados; novos termos e merges não são inferidos como certeza.'),
 ('R19','Resumo','Resumo executivo denso de 2–5 frases por modelo leve','Escolha do autor','Texto separado da decisão Jev; fatos e limitações rastreáveis; edição autoral preservada.'),
 ('R20','Embeddings','Embeddings locais e adequados ao português','RFC §§3,4,10','Manifesto de modelo, normalização e versão; espaços vetoriais incompatíveis não se misturam.'),
 ('R21','Busca','BM25 + semântica para Ingest e Peças; Qdrant OU HelixDB','Escolha do autor; RFC §4','Busca filtrada, fusão e exclusões/revisões corretas, com reconstrução a partir dos arquivos.'),
 ('R22','Contexto','PageIndex adaptado a células e topologia','Escolha do autor; RFC §8','Hierarquia navega sem apagar ciclos, relações cruzadas, geometria e identidade dos nós.'),
 ('R23','Orquestração','Skeleton Map, expansão e síntese progressivas','RFC §8','Orçamento de tokens/chamadas/tempo/recursão imposto em código; resposta cita evidências.'),
 ('R24','Descoberta','SearxNG com consultas autorizadas','Escolha do autor; RFC §6','API de resultados configurada; rede externa explícita; ausência de dependência pública oculta.'),
 ('R25','Aquisição web','Scrapling HTTP/browser sob demanda','Escolha do autor; RFC §6','SSRF e recursos controlados; não prometer bypass universal; browser pode encerrar sem órfãos.'),
 ('R26','Widget','Pesquisa, snapshots e promoção para células','RFC §§5,6','Atualizar resultados não sobrescreve síntese editada; snippets não viram prova sem fonte.'),
 ('R27','IA gerativa','LocalAI e BYOK sob política comum','Escolha do autor; RFC §§7,10','Capabilities verificadas, streaming/cancelamento e identidade de provedor preservados.'),
 ('R28','Intenções','Metas, hipóteses, chat global e sugestões proativas','RFC §9','Escopo e finalidade explícitos; sugestão não altera conhecimento sem confirmação.'),
 ('R29','Portabilidade','Backup, restauração e integração Git opcional','RFC §11','Exportação/restauração verificadas; push remoto exige configuração e autorização.'),
 ('R30','Supply chain','Pacotes, hashes, SBOM, atualização e rollback','RFC §10; Plano §14','Instalação íntegra por arquitetura, cadeia de origem e fontes/avisos disponíveis.'),
 ('R31','Usabilidade','Acessibilidade, teclado, erros e documentação','Derivado do produto desktop','Fluxos essenciais acessíveis e compreensíveis; estados de falha/pendência claros.'),
 ('R32','Compatibilidade','Migrações e estabilidade de contratos/formatos','Plano §§11,16','Matriz de versões e migrações sem perda; export legível independente do app.'),
 ('R33','Open source','Projeto não comercial com licença open source e conformidade','Orientação do autor','Licença ratificada, direitos upstream preservados e integrações pagas opcionais explicadas.'),
 ('R34','Qualidade','Testes, evidências e revisão humana de código produzido com IA','Estratégia solo + agentes','Nenhum gate fechado apenas por relato do agente; execução/resultado e revisão registrados.'),
 ('R35','Extensibilidade','Contratos/kit de provedores e extensões controladas','Proposta pós-v1.0','Requer ratificação; extensão não amplia permissões nem introduz leitura/escrita irrestrita.'),
 ('R36','Escala','Acervos maiores e pacotes offline portáveis','Proposta pós-v1.0','Metas aprovadas após baseline; sem reduzir fidelidade, escopo ou budgets silenciosamente.'),
 ('R37','Auditoria','Rastreabilidade criptográfica com limites explícitos','RFC §§1,11','Encadeamento/checagem de eventos e limites de adulteração/ancoragem documentados.'),
]

ADRS = [
 ('ADR-001','Rust estrito versus núcleo Rust + workers','S01','Aprovar composição poliglota controlada; se rejeitada, replanejar ports antes de avançar.'),
 ('ADR-002','Autoridade JSON/journal e MPK derivado','S02','Escolher precedência, fronteira de confirmação e formato aberto de histórico.'),
 ('ADR-003','Licença do código próprio e recorte upstream','S03','Ratificar licença; examinar por origem/arquivo, incluindo SearxNG e BlockSuite.'),
 ('ADR-004','Política Jev, BYOK e modo offline','S01','Jev de primeira classe quando autorizado; fallback local/pendência sem equivalente probabilístico presumido.'),
 ('ADR-005','Perfil Markdown e representação de IDs','S02','Fixar blocos suportados, campos preservados e extensões de proveniência.'),
 ('ADR-006','OS de referência e matriz de distribuição','S01','Escolher host principal sem reduzir os cinco alvos finais; obter acesso aos ambientes de teste.'),
 ('ADR-007','Budgets e perfis de hardware','S05','Fixar método de medição e perfis; OCR/STT pesado não recebe teto maior implicitamente.'),
 ('ADR-008','Recorte BlockSuite e integração DOM/PixiJS','S12','Fechar grafo de pacotes e licença/revisão; manter PixiJS como cena principal.'),
 ('ADR-009','Protocolos de workers e políticas por SO','S23','Fixar IPC, capabilities e fail-closed; tratar GPU/browser separadamente.'),
 ('ADR-010','Runtime LocalAI e backends/modelos permitidos','S31','Selecionar pacotes e desabilitar superfícies concorrentes de agentes/RAG/tools.'),
 ('ADR-011','Modelos de resumo, embeddings e transcrição','S32','Homologar por função/idioma/hardware; licença e hashes completos.'),
 ('ADR-012','Taxonomia, thresholds e abstenção','S35','Aprovar política calibrada; retirar merges silenciosos por distância/similaridade.'),
 ('ADR-013','Backend de busca inicial','S40','Escolher Qdrant ou HelixDB por evidência; não distribuir ambos por hábito.'),
 ('ADR-014','Recorte PageIndex e contrato de Context Engine','S42','Definir upstream mínimo e fronteiras substituídas; árvore é projeção do grafo.'),
 ('ADR-015','Instância SearxNG e normalização web','S47','Aprovar execução local/instância pessoal e política de consultas; sem instância pública implícita.'),
 ('ADR-016','Git, retenção e evidências criptográficas','S58','Aprovar dados incluídos/excluídos, purga, assinatura/ancoragem e alcance de rollback.'),
 ('ADR-017','Escopo pós-v1.0','S61','Ratificar evolução sem bloquear correções; distinguir obrigatório, experimental e não escopo.'),
 ('ADR-018','SDK/extensões da v2.0','S67','Aprovar superfície restrita, distribuição e permissões; não abrir execução arbitrária de plugins.'),
]


def add(n, title, component, nature, reqs, deps, objective, deliverable, demo, rows, note=''):
    sid=f'S{n:02d}'
    rel=next(r[0] for r in RELEASES if r[1]<=n<=r[2])
    tasks=[]
    for ln in rows.strip().splitlines():
        ln=ln.strip()
        if not ln or ln.startswith('#'): continue
        parts=[p.strip() for p in ln.split(' | ')]
        if len(parts)!=3: raise ValueError((sid,ln,len(parts)))
        text,path,accept=parts
        typ='Implementação'
        if text.startswith('['):
            typ,text=text[1:].split('] ',1)
        title_short,sep,steps=text.partition(' :: ')
        if not sep: steps=title_short
        tasks.append(dict(title=title_short, execution=steps, output=path, acceptance=accept, kind=typ))
    if len(tasks)<8: raise ValueError(f'{sid}: pouca granularidade: {len(tasks)}')
    risk='Crítico' if any(x in reqs for x in ['R05','R06','R08','R30']) else 'Alto' if nature!='Próprio' or 'R09' in reqs else 'Médio'
    SPRINTS.append(dict(n=n,id=sid,release=rel,title=title,component=component,nature=nature,requirements=reqs,deps=deps,objective=objective,deliverable=deliverable,demo=demo,note=note,risk=risk,tasks=tasks))
