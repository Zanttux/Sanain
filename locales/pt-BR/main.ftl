# Machine translation by Claude (AI); review by a native speaker required before release.
app-name = Sanain
menu-file = Arquivo
file-open = Abrir…
file-save = Salvar
status-tabs = { $count ->
    [one] Uma aba
    [many] { $count } abas
   *[other] { $count } abas
    }
error-open = Não foi possível abrir { $name }.
menu-edit = Editar
file-new = Nova aba
file-save-as = Salvar como…
edit-undo = Desfazer
edit-redo = Refazer
edit-select-all = Selecionar tudo
untitled = Sem título
window-title = { $dirty ->
    [yes] ● { $name } — Sanain
   *[no] { $name } — Sanain
    }
status-position = Ln { $line }, Col { $column }
status-selections = { $count ->
    [one] 1 seleção
    [many] { $count } seleções
   *[other] { $count } seleções
    }
status-saved = { $name } salvo
encoding-utf8 = UTF-8
encoding-utf8-bom = UTF-8 com BOM
encoding-utf16le = UTF-16 LE
encoding-utf16be = UTF-16 BE
line-ending-lf = LF
line-ending-crlf = CRLF
line-ending-cr = CR
line-ending-mixed = Quebras de linha mistas
editor-accessible-name = Editor de texto
button-save = Salvar
button-dont-save = Não salvar
button-cancel = Cancelar
button-ok = OK
button-overwrite = Substituir
button-save-as = Salvar como…
dialog-unsaved-title = Salvar as alterações em { $name }?
dialog-unsaved-body = Suas alterações serão perdidas se você não salvá-las.
dialog-conflict-title = { $name } foi alterado no disco
dialog-conflict-body = Outro programa modificou este arquivo depois que você o abriu. Substituir descartará essas alterações.
dialog-readonly-title = { $name } é somente leitura
dialog-readonly-body = Em vez disso, salve uma cópia com outro nome.
error-open-encoding = { $name } não é um texto UTF-8 ou UTF-16. Ele não foi aberto, então seu conteúdo não foi alterado. Use Arquivo > Reabrir com codificação para escolher a codificação.
error-save = Não foi possível salvar { $name }: { $reason }
menu-window = Janela
edit-cut = Recortar
edit-copy = Copiar
edit-paste = Colar
app-quit = Encerrar Sanain
app-hide = Ocultar Sanain
app-hide-others = Ocultar outros
app-show-all = Mostrar tudo
window-minimize = Minimizar
window-zoom = Zoom
status-recovered = As alterações não salvas da última sessão foram restauradas
status-recovery-failed = Não foi possível salvar uma cópia de recuperação: { $reason }
edit-duplicate-line = Duplicar linha
edit-delete-line = Excluir linha
edit-move-line-up = Mover linha para cima
edit-move-line-down = Mover linha para baixo
file-close-tab = Fechar aba
file-reopen-tab = Reabrir aba fechada
window-next-tab = Próxima aba
window-prev-tab = Aba anterior
tab-close = Fechar aba
edit-find = Localizar…
edit-find-next = Localizar próxima
edit-find-prev = Localizar anterior
edit-replace = Substituir…
find-placeholder = Localizar
replace-placeholder = Substituir por
find-match-case = Diferenciar maiúsculas e minúsculas
find-whole-word = Palavra inteira
find-regex = Expressão regular
find-no-results = Nenhum resultado
find-count = { $total ->
    [one] 1 ocorrência
    [many] { $total } ocorrências
   *[other] { $total } ocorrências
    }
find-count-current = { $current } de { $total }
find-prev = Ocorrência anterior (Shift+Enter)
find-next = Próxima ocorrência (Enter)
find-close = Fechar (Esc)
replace-one = Substituir
replace-all = Substituir tudo
status-replaced = { $count ->
    [one] 1 ocorrência substituída
    [many] { $count } ocorrências substituídas
   *[other] { $count } ocorrências substituídas
    }
language-plain = Texto simples
menu-view = Exibir
app-settings = Configurações…
view-zoom-in = Ampliar
view-zoom-out = Reduzir
view-zoom-reset = Tamanho real
settings-title = Configurações
settings-font-size = Tamanho da fonte
settings-tab-width = Largura da tabulação
settings-insert-spaces = Inserir espaços ao pressionar Tab
settings-theme = Aparência
settings-theme-system = Seguir o sistema
settings-theme-light = Claro
settings-theme-dark = Escuro
settings-restore-session = Reabrir arquivos da última sessão
settings-language = Idioma
settings-language-system = Seguir o sistema
settings-language-note = As alterações de idioma serão aplicadas após reiniciar o Sanain.
settings-save-failed = Não foi possível salvar as configurações: { $reason }
view-command-palette = Paleta de comandos…
palette-placeholder = Digite um comando
view-markdown-source = Código-fonte Markdown
view-markdown-split = Código-fonte e visualização Markdown
view-markdown-preview = Visualização Markdown
preview-too-large = Este documento é grande demais para ser visualizado.
preview-mermaid-unsupported = Diagramas Mermaid ainda não são renderizados; exibindo o código-fonte.
menu-tools = Ferramentas
tools-json-validate = Validar JSON
tools-json-format = Formatar JSON
tools-json-minify = Minificar JSON
tools-sort-lines = Ordenar linhas
tools-sort-lines-desc = Ordenar linhas em ordem decrescente
tools-unique-lines = Remover linhas duplicadas
tools-trim-trailing = Remover espaços no fim das linhas
tools-upper-case = Maiúsculas
tools-lower-case = Minúsculas
tools-to-lf = Converter quebras de linha para LF
tools-to-crlf = Converter quebras de linha para CRLF
tools-json-valid = O JSON é válido
tools-done = Concluído. Desfazer reverte a alteração.
tools-no-change = Nada a alterar
tools-json-error = JSON inválido na linha { $line }, coluna { $column }: { $reason }
json-reason-unterminated-string = string não terminada
json-reason-invalid-u-escape = escape \u inválido
json-reason-invalid-escape = escape inválido
json-reason-control-character-in-string = caractere de controle na string
json-reason-invalid-number = número inválido
json-reason-unexpected-character = caractere inesperado
json-reason-unexpected-content-after-the-value = conteúdo inesperado após o valor
json-reason-unexpected-comma = vírgula inesperada
json-reason-expected-a-string-key = esperava-se uma chave do tipo string
json-reason-expected = esperava-se ':'
json-reason-expected-or-a-closing-bracket = esperava-se ',' ou um colchete de fechamento
json-reason-expected-a-value = esperava-se um valor
json-reason-empty-document = documento vazio
json-reason-unexpected-end-of-input = fim inesperado da entrada
json-reason-too-deeply-nested-to-format = aninhamento profundo demais para formatar
tools-compare = Comparar com…
compare-need-two = Abra outra aba para comparar.
compare-pick-title = Comparar { $name } com
compare-count = { $count ->
    [0] Nenhuma diferença
    [one] 1 alteração
    [many] { $count } alterações
   *[other] { $count } alterações
    }
compare-close = Fechar comparação
compare-to-left = Copiar esta alteração para a esquerda
compare-to-right = Copiar esta alteração para a direita
compare-stale = Um documento foi alterado durante a comparação. A comparação foi atualizada; tente novamente.
view-structured = Visualização estruturada (CSV, JSON, XML, YAML)
csv-delimiter = Delimitador
csv-rows = { $count ->
    [one] 1 linha
    [many] { $count } linhas
   *[other] { $count } linhas
    }
csv-unterminated = Aspas não fechadas a partir da linha { $line }
csv-truncated = Exibindo as primeiras 200.000 linhas
json-more = … mais { $count }
dialog-open-link-title = Abrir este link no navegador?
button-open = Abrir
link-blocked = Por segurança, este tipo de link não é aberto.
link-not-found = Arquivo vinculado não encontrado: { $target }
enc-windows-1252 = Europeu ocidental
enc-iso-8859-15 = Europeu ocidental
enc-windows-1250 = Europeu central
enc-iso-8859-2 = Europeu central
enc-windows-1251 = Cirílico
enc-koi8-r = Cirílico
enc-windows-1253 = Grego
enc-windows-1254 = Turco
enc-windows-1255 = Hebraico
enc-windows-1256 = Árabe
enc-windows-1257 = Báltico
enc-windows-874 = Tailandês
enc-shift_jis = Japonês
enc-euc-jp = Japonês
enc-gb18030 = Chinês simplificado
enc-big5 = Chinês tradicional
enc-euc-kr = Coreano
file-reopen-encoding = Reabrir com codificação…
file-save-encoding = Salvar com codificação…
encoding-reopen-title = Reabrir com codificação
encoding-save-title = Salvar com codificação
encoding-not-exact = Esta codificação não consegue abrir o arquivo sem alterá-lo: { $reason }
encoding-unencodable = “{ $character }” não pode ser salvo como { $encoding }. Escolha outra codificação ou altere o texto; nada foi gravado.
encoding-save-first = Salve ou desfaça suas alterações antes de reabrir com outra codificação.
image-remote = Imagens remotas não são carregadas.
image-blocked = Somente imagens da pasta deste documento são exibidas.
image-unsupported = Formato de imagem não suportado.
image-too-large = A imagem é grande demais para ser exibida.
image-unreadable = Não foi possível ler a imagem.
settings-theme-high-contrast = Alto contraste
view-fold = Recolher
view-unfold = Expandir
view-unfold-all = Expandir tudo
settings-shortcuts = Atalhos de teclado
settings-keymap-preset = Predefinição
settings-keymap-default = Sanain
settings-keymap-notepadpp = Notepad++
settings-shortcut-change = Alterar
settings-shortcut-reset = Redefinir
settings-shortcut-press = Pressione um atalho… (Esc cancela)
settings-shortcut-moved = Atalho movido de:
edit-find-in-folder = Localizar na pasta…
folder-search-placeholder = Localizar na pasta
folder-search-run = Pesquisar
folder-search-stop = Parar
folder-search-no-folder = Escolher pasta
folder-search-summary = { $count ->
    [one] 1 ocorrência
    [many] { $count } ocorrências
   *[other] { $count } ocorrências
    } em { $files ->
    [one] 1 arquivo
    [many] { $files } arquivos
   *[other] { $files } arquivos
    }
app-check-updates = Verificar Atualizações…
update-checking = Verificando atualizações…
update-up-to-date = Sanain { $version } é a versão mais recente.
update-no-releases = Nenhuma versão do Sanain foi publicada ainda.
update-failed = Não foi possível verificar atualizações: { $reason }
update-invalid = resposta inesperada
update-available-title = Sanain { $version } está disponível
update-available-body = Abra a página de download para obtê-lo. O Sanain não baixa nem instala nada por conta própria.
update-open-page = Abrir Página de Download
update-later = Mais tarde
settings-check-updates = Verificar atualizações uma vez por dia
settings-check-updates-detail = Consulta o github.com sobre a versão mais recente. Nada mais é enviado.
file-local-history = Histórico local…
file-clear-history = Limpar histórico local…
history-title = Histórico local de { $name }
history-open = Abrir
history-restore = Restaurar
history-empty = Ainda não há versões salvas. Uma cópia é mantida sempre que você salva.
history-unreadable = Não foi possível ler esta versão.
history-just-now = Agora mesmo
history-minutes-ago = { $count ->
    [one] há 1 minuto
    [many] há { $count } minutos
   *[other] há { $count } minutos
    }
history-hours-ago = { $count ->
    [one] há 1 hora
    [many] há { $count } horas
   *[other] há { $count } horas
    }
history-days-ago = { $count ->
    [one] há 1 dia
    [many] há { $count } dias
   *[other] há { $count } dias
    }
history-purge-title = Excluir todo o histórico local?
history-purge-body = As cópias salvas de todos os arquivos são removidas deste computador. Os seus arquivos não são alterados.
history-purge-confirm = Excluir histórico
history-purged = Histórico local excluído.
settings-local-history = Manter histórico local dos arquivos salvos
settings-local-history-detail = Mantém até 50 versões por arquivo durante 30 dias, somente neste computador.
file-open-log = Abrir no visualizador de logs…
log-lines = { $count ->
    [one] { $count } linha
    [many] { $count } linhas
   *[other] { $count } linhas
    }
log-indexing = Lendo… { $percent }%
log-matches = { $count ->
    [one] { $count } linha correspondente
    [many] { $count } linhas correspondentes
   *[other] { $count } linhas correspondentes
    }
log-filter-hint = Filtrar linhas
log-bad-pattern = Padrão inválido
log-follow = Acompanhar
log-close = Fechar visualizador
log-opened-large = { $name } é grande, então foi aberto como somente leitura no visualizador de logs.
tab-pin = Fixar aba
tab-unpin = Desafixar aba
edit-add-cursor-above = Adicionar cursor acima
edit-add-cursor-below = Adicionar cursor abaixo
view-word-wrap = Quebra automática de linha
status-wrap-on = As linhas longas agora são quebradas na largura da janela.
status-wrap-off = As linhas longas não são mais quebradas.
settings-word-wrap = Quebrar linhas longas
edit-go-to-line = Ir para a linha…
edit-go-to-heading = Ir para o título…
goto-line-hint = Linha ou linha:coluna
goto-line-note = { $count ->
    [one] O documento tem 1 linha.
    [many] O documento tem { $count } de linhas.
   *[other] O documento tem { $count } linhas.
    }
goto-line-invalid = Digite um número de linha, opcionalmente seguido de :coluna.
goto-heading-hint = Digite para filtrar os títulos
goto-heading-none = Este documento não tem títulos.
format-xml = XML
format-yaml = YAML
tools-tree-error = { $format } inválido na linha { $line }, coluna { $column }: { $reason }
edit-select-next-occurrence = Selecionar próxima ocorrência
tools-record-macro = Iniciar ou parar gravação de macro
tools-play-macro = Reproduzir macro
macro-recording = Gravando macro
macro-recorded = { $count ->
    [one] Macro gravada: 1 etapa.
    [many] Macro gravada: { $count } de etapas.
   *[other] Macro gravada: { $count } etapas.
    }
tools-compare-saved = Comparar com a versão salva
tools-compare-clipboard = Comparar com a área de transferência
compare-saved-label = { $name } (no disco)
compare-clipboard-label = Área de transferência
compare-saved-failed = Não foi possível ler o arquivo salvo: { $reason }
compare-clipboard-empty = A área de transferência não contém texto para comparar.
error-save-disk-full = { $name } não foi salvo: o disco está cheio. O arquivo no disco não foi alterado e seu texto continua aqui. Libere espaço ou use Salvar como para salvá-lo em outro local.
error-save-permission = { $name } não foi salvo: você não tem permissão para gravar nesse local. O arquivo no disco não foi alterado. Use Salvar como para salvá-lo em outro local.
error-save-read-only-volume = { $name } não foi salvo: o disco é somente leitura. O arquivo no disco não foi alterado. Use Salvar como para salvá-lo em outro local.
replace-preview-title = { $count ->
    [one] Substituir { $count } ocorrência?
    [many] Substituir { $count } ocorrências?
   *[other] Substituir { $count } ocorrências?
    }
replace-preview-confirm = { $count ->
    [one] Substituir { $count }
    [many] Substituir { $count }
   *[other] Substituir { $count }
    }
replace-preview-more = Apenas as primeiras 200 linhas alteradas são exibidas.
replace-preview-stale = O documento foi alterado; nada foi substituído. Execute Substituir tudo novamente.
view-split-editor = Dividir editor
encoding-detected = Aberto como { $encoding } (detectado a partir do conteúdo do arquivo).
encoding-suggested = Sugerida com base no conteúdo do arquivo: { $encoding }. Verifique a pré-visualização antes de abrir.
app-about = Sobre o Sanain
menu-help = Ajuda
about-tagline = O poder das palavras.
about-version = Versão { $version }
about-license = Livre e de código aberto sob a Mozilla Public License 2.0.
about-ai = Criado por IA. Liderado por pessoas. Movido pela comunidade.
about-notices = Os componentes de terceiros e suas licenças estão listados em THIRD_PARTY_NOTICES, incluído em cada versão.
about-source = Código-fonte e issues no GitHub
tools-xml-validate = Validar XML
tools-yaml-validate = Validar YAML
tools-markup-valid = O { $format } é válido
file-open-recent = Abrir recente…
file-save-all = Salvar tudo
file-revert = Reverter para a versão salva
recent-hint = Digite para filtrar os arquivos recentes
recent-none = Ainda não há arquivos recentes.
revert-title = Descartar as alterações em { $name }?
revert-body = O arquivo é recarregado do disco. As alterações não salvas são perdidas e não podem ser desfeitas.
revert-confirm = Descartar e recarregar
revert-done = { $name } foi recarregado do disco.
status-saved-all = { $count ->
    [0] Nada para salvar.
    [one] { $count } arquivo salvo.
    [many] { $count } de arquivos salvos.
   *[other] { $count } arquivos salvos.
    }
edit-toggle-comment = Alternar comentário de linha
edit-indent = Recuar linhas
edit-outdent = Remover recuo das linhas
comment-unsupported = Este tipo de arquivo não tem comentários de linha.
tab-close-others = Fechar outras abas
tab-close-all = Fechar todas as abas
file-copy-path = Copiar caminho do arquivo
file-reveal-macos = Mostrar no Finder
file-reveal-windows = Mostrar no Explorador de Arquivos
file-reveal-other = Mostrar no gerenciador de arquivos
status-path-copied = Caminho do arquivo copiado.
tabs-kept-unsaved = { $count ->
    [one] 1 aba com alterações não salvas foi mantida aberta.
    [many] { $count } abas com alterações não salvas foram mantidas abertas.
   *[other] { $count } abas com alterações não salvas foram mantidas abertas.
    }
status-indent-tabs = Tabulações
status-indent-spaces = Espaços: { $count }
find-searching = Pesquisando…
compare-ignore-whitespace = Ignorar espaços em branco
tools-compare-file = Comparar com um arquivo…
compare-file-failed = Não foi possível comparar com { $name }: { $reason }
cli-help =
    Uso:
      sanain [ARQUIVO…]                 abrir arquivos
      sanain --diff ESQUERDA DIREITA    comparar dois arquivos lado a lado
      sanain --version                  mostrar a versão
      sanain --help                     mostrar esta ajuda
    Use -- antes de nomes de arquivo que começam com hífen.
cli-unknown-option = Opção desconhecida { $option }.
cli-diff-needs-two = --diff precisa de dois arquivos: sanain --diff ESQUERDA DIREITA
window-move-tab-left = Mover aba para a esquerda
window-move-tab-right = Mover aba para a direita
edit-skip-occurrence = Pular para a próxima ocorrência
tools-play-macro-times = Reproduzir macro várias vezes…
macro-times-hint = Número de vezes
macro-times-note = De 1 a { $max }
macro-times-invalid = Digite um número de 1 a { $max }.
view-outline = Estrutura de tópicos
outline-empty = Não há títulos neste documento.
outline-close = Fechar estrutura de tópicos
folder-replace-placeholder = Substituir por
folder-replace-run = Substituir nos arquivos…
replace-files-title = Substituir { $count ->
    [one] { $count } ocorrência
    [many] { $count } ocorrências
   *[other] { $count } ocorrências
    } em { $files ->
    [one] { $files } arquivo
    [many] { $files } arquivos
   *[other] { $files } arquivos
    }?
replace-files-body = Os arquivos abertos em uma aba são alterados nela e a alteração pode ser desfeita; salve-os você mesmo. Os outros arquivos são salvos agora, e o conteúdo anterior é mantido no histórico local quando ele está ativado. Um arquivo que foi alterado no disco nesse meio-tempo é ignorado.
replace-files-confirm = Substituir
replace-files-done = { $count ->
    [one] { $count } ocorrência substituída
    [many] { $count } ocorrências substituídas
   *[other] { $count } ocorrências substituídas
    } em { $files ->
    [one] { $files } arquivo
    [many] { $files } arquivos
   *[other] { $files } arquivos
    } (abertos em abas: { $open }, não salvos).
replace-files-failed = { $failed ->
    [one] { $failed } arquivo não foi alterado:
    [many] { $failed } arquivos não foram alterados; primeiro:
   *[other] { $failed } arquivos não foram alterados; primeiro:
    } { $first }
image-outside = Esta imagem está fora da pasta do documento.
image-allow = Exibir imagens desta pasta
cli-help-integration =
    Integração com o gerenciador de arquivos (somente o usuário atual):
      sanain --register           adiciona o Sanain a "Abrir com" (Linux, Windows)
      sanain --unregister         remove-o novamente
cli-registered = O Sanain foi adicionado a "Abrir com". { $place }
cli-unregistered = O Sanain foi removido de "Abrir com".
cli-register-failed = Não foi possível alterar a integração com o gerenciador de arquivos: { $reason }
cli-register-macos = A integração com o Finder no macOS ainda não está disponível. Abra arquivos pelo Sanain (Arquivo › Abrir, arrastar e soltar) ou pela linha de comando.
settings-tab-general = Geral
settings-tab-editor = Editor
settings-tab-appearance = Aparência
settings-shortcut-search = Pesquisar comandos ou teclas
settings-font = Fonte
settings-font-default = Monoespaçada integrada
settings-font-loading = Procurando fontes instaladas…
settings-font-missing = A fonte { $font } não foi encontrada; usando a fonte integrada.
settings-theme-more = Mais temas
settings-theme-user-note = Seus próprios temas: arquivos .theme na pasta { $folder } (consulte docs/THEMES.md).
language-auto = Detectar automaticamente
language-choose = Escolher a linguagem desta aba
