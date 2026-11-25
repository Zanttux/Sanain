# Machine translation by Claude (AI); review by a native speaker required before release.
app-name = Sanain
menu-file = Ficheiro
file-open = Abrir…
file-save = Guardar
status-tabs = { $count ->
    [one] Um separador
    [many] { $count } separadores
   *[other] { $count } separadores
    }
error-open = Não foi possível abrir { $name }.
menu-edit = Editar
file-new = Novo separador
file-save-as = Guardar como…
edit-undo = Anular
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
status-saved = { $name } guardado
encoding-utf8 = UTF-8
encoding-utf8-bom = UTF-8 com BOM
encoding-utf16le = UTF-16 LE
encoding-utf16be = UTF-16 BE
line-ending-lf = LF
line-ending-crlf = CRLF
line-ending-cr = CR
line-ending-mixed = Fins de linha mistos
editor-accessible-name = Editor de texto
button-save = Guardar
button-dont-save = Não guardar
button-cancel = Cancelar
button-ok = OK
button-overwrite = Substituir
button-save-as = Guardar como…
dialog-unsaved-title = Guardar as alterações em { $name }?
dialog-unsaved-body = As alterações serão perdidas se não as guardar.
dialog-conflict-title = { $name } foi alterado no disco
dialog-conflict-body = Outro programa modificou este ficheiro depois de o ter aberto. Substituir elimina essas alterações.
dialog-readonly-title = { $name } é só de leitura
dialog-readonly-body = Em alternativa, guarde uma cópia com outro nome.
error-open-encoding = { $name } não é texto UTF-8 nem UTF-16. Não foi aberto, pelo que o conteúdo não foi alterado. Utilize Ficheiro > Reabrir com codificação para escolher a codificação.
error-save = Não foi possível guardar { $name }: { $reason }
menu-window = Janela
edit-cut = Cortar
edit-copy = Copiar
edit-paste = Colar
app-quit = Sair do Sanain
app-hide = Ocultar Sanain
app-hide-others = Ocultar outros
app-show-all = Mostrar tudo
window-minimize = Minimizar
window-zoom = Zoom
status-recovered = As alterações não guardadas da última sessão foram restauradas
status-recovery-failed = Não foi possível guardar uma cópia de recuperação: { $reason }
edit-duplicate-line = Duplicar linha
edit-delete-line = Eliminar linha
edit-move-line-up = Mover linha para cima
edit-move-line-down = Mover linha para baixo
file-close-tab = Fechar separador
file-reopen-tab = Reabrir separador fechado
window-next-tab = Separador seguinte
window-prev-tab = Separador anterior
tab-close = Fechar separador
edit-find = Procurar…
edit-find-next = Procurar seguinte
edit-find-prev = Procurar anterior
edit-replace = Substituir…
find-placeholder = Procurar
replace-placeholder = Substituir por
find-match-case = Respeitar maiúsculas/minúsculas
find-whole-word = Palavra inteira
find-regex = Expressão regular
find-no-results = Sem resultados
find-count = { $total ->
    [one] 1 correspondência
    [many] { $total } correspondências
   *[other] { $total } correspondências
    }
find-count-current = { $current } de { $total }
find-prev = Correspondência anterior (Shift+Enter)
find-next = Correspondência seguinte (Enter)
find-close = Fechar (Esc)
replace-one = Substituir
replace-all = Substituir tudo
status-replaced = { $count ->
    [one] 1 correspondência substituída
    [many] { $count } correspondências substituídas
   *[other] { $count } correspondências substituídas
    }
language-plain = Texto simples
menu-view = Ver
app-settings = Definições…
view-zoom-in = Ampliar
view-zoom-out = Reduzir
view-zoom-reset = Tamanho real
settings-title = Definições
settings-font-size = Tamanho do tipo de letra
settings-tab-width = Largura da tabulação
settings-insert-spaces = Inserir espaços ao premir Tab
settings-theme = Aspeto
settings-theme-system = Igual ao sistema
settings-theme-light = Claro
settings-theme-dark = Escuro
settings-restore-session = Reabrir ficheiros da última sessão
settings-language = Idioma
settings-language-system = Igual ao sistema
settings-language-note = As alterações de idioma aplicam-se após reiniciar o Sanain.
settings-save-failed = Não foi possível guardar as definições: { $reason }
view-command-palette = Paleta de comandos…
palette-placeholder = Escreva um comando
view-markdown-source = Código-fonte Markdown
view-markdown-split = Código-fonte e pré-visualização Markdown
view-markdown-preview = Pré-visualização Markdown
preview-too-large = Este documento é demasiado grande para pré-visualizar.
preview-mermaid-unsupported = Os diagramas Mermaid ainda não são apresentados; a mostrar o código-fonte.
menu-tools = Ferramentas
tools-json-validate = Validar JSON
tools-json-format = Formatar JSON
tools-json-minify = Minificar JSON
tools-sort-lines = Ordenar linhas
tools-sort-lines-desc = Ordenar linhas por ordem decrescente
tools-unique-lines = Remover linhas duplicadas
tools-trim-trailing = Remover espaços no fim das linhas
tools-upper-case = Maiúsculas
tools-lower-case = Minúsculas
tools-to-lf = Converter fins de linha para LF
tools-to-crlf = Converter fins de linha para CRLF
tools-json-valid = O JSON é válido
tools-done = Concluído. Anular reverte a alteração.
tools-no-change = Nada a alterar
tools-json-error = JSON inválido na linha { $line }, coluna { $column }: { $reason }
json-reason-unterminated-string = cadeia não terminada
json-reason-invalid-u-escape = escape \u inválido
json-reason-invalid-escape = escape inválido
json-reason-control-character-in-string = carácter de controlo na cadeia
json-reason-invalid-number = número inválido
json-reason-unexpected-character = carácter inesperado
json-reason-unexpected-content-after-the-value = conteúdo inesperado após o valor
json-reason-unexpected-comma = vírgula inesperada
json-reason-expected-a-string-key = esperava-se uma chave do tipo cadeia
json-reason-expected = esperava-se ':'
json-reason-expected-or-a-closing-bracket = esperava-se ',' ou um parêntese de fecho
json-reason-expected-a-value = esperava-se um valor
json-reason-empty-document = documento vazio
json-reason-unexpected-end-of-input = fim inesperado da entrada
json-reason-too-deeply-nested-to-format = aninhamento demasiado profundo para formatar
tools-compare = Comparar com…
compare-need-two = Abra outro separador para comparar.
compare-pick-title = Comparar { $name } com
compare-count = { $count ->
    [0] Sem diferenças
    [one] 1 alteração
    [many] { $count } alterações
   *[other] { $count } alterações
    }
compare-close = Fechar comparação
compare-to-left = Copiar esta alteração para a esquerda
compare-to-right = Copiar esta alteração para a direita
compare-stale = Um documento foi alterado durante a comparação. A comparação foi atualizada; tente novamente.
view-structured = Vista estruturada (CSV, JSON, XML, YAML)
csv-delimiter = Delimitador
csv-rows = { $count ->
    [one] 1 linha
    [many] { $count } linhas
   *[other] { $count } linhas
    }
csv-unterminated = Aspas não fechadas a partir da linha { $line }
csv-truncated = A mostrar as primeiras 200 000 linhas
json-more = … mais { $count }
dialog-open-link-title = Abrir esta ligação no navegador?
button-open = Abrir
link-blocked = Por segurança, este tipo de ligação não é aberto.
link-not-found = Ficheiro ligado não encontrado: { $target }
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
file-save-encoding = Guardar com codificação…
encoding-reopen-title = Reabrir com codificação
encoding-save-title = Guardar com codificação
encoding-not-exact = Esta codificação não consegue abrir o ficheiro sem o alterar: { $reason }
encoding-unencodable = “{ $character }” não pode ser guardado como { $encoding }. Escolha outra codificação ou altere o texto; nada foi escrito.
encoding-save-first = Guarde ou anule as alterações antes de reabrir com outra codificação.
image-remote = As imagens remotas não são carregadas.
image-blocked = Só são mostradas imagens da pasta deste documento.
image-unsupported = Formato de imagem não suportado.
image-too-large = A imagem é demasiado grande para ser mostrada.
image-unreadable = Não foi possível ler a imagem.
settings-theme-high-contrast = Alto contraste
view-fold = Fechar secção
view-unfold = Abrir secção
view-unfold-all = Abrir todas as secções
settings-shortcuts = Atalhos de teclado
settings-keymap-preset = Predefinição
settings-keymap-default = Sanain
settings-keymap-notepadpp = Notepad++
settings-shortcut-change = Alterar
settings-shortcut-reset = Repor
settings-shortcut-press = Prima um atalho… (Esc cancela)
settings-shortcut-moved = Atalho movido de:
edit-find-in-folder = Procurar na pasta…
folder-search-placeholder = Procurar na pasta
folder-search-run = Pesquisar
folder-search-stop = Parar
folder-search-no-folder = Escolher pasta
folder-search-summary = { $count ->
    [one] 1 correspondência
    [many] { $count } correspondências
   *[other] { $count } correspondências
    } em { $files ->
    [one] 1 ficheiro
    [many] { $files } ficheiros
   *[other] { $files } ficheiros
    }
app-check-updates = Procurar atualizações…
update-checking = A procurar atualizações…
update-up-to-date = Sanain { $version } é a versão mais recente.
update-no-releases = Ainda não foi publicada nenhuma versão do Sanain.
update-failed = Não foi possível procurar atualizações: { $reason }
update-invalid = resposta inesperada
update-available-title = Sanain { $version } está disponível
update-available-body = Abra a página de transferência para o obter. O Sanain não transfere nem instala nada por si só.
update-open-page = Abrir página de transferência
update-later = Mais tarde
settings-check-updates = Procurar atualizações uma vez por dia
settings-check-updates-detail = Consulta o github.com sobre a versão mais recente. Nada mais é enviado.
file-local-history = Histórico local…
file-clear-history = Limpar histórico local…
history-title = Histórico local de { $name }
history-open = Abrir
history-restore = Restaurar
history-empty = Ainda não há versões guardadas. É mantida uma cópia sempre que guarda.
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
history-purge-title = Eliminar todo o histórico local?
history-purge-body = As cópias guardadas de todos os ficheiros são removidas deste computador. Os seus ficheiros não são alterados.
history-purge-confirm = Eliminar histórico
history-purged = Histórico local eliminado.
settings-local-history = Manter histórico local dos ficheiros guardados
settings-local-history-detail = Mantém até 50 versões por ficheiro durante 30 dias, apenas neste computador.
file-open-log = Abrir no visualizador de registos…
log-lines = { $count ->
    [one] { $count } linha
    [many] { $count } linhas
   *[other] { $count } linhas
    }
log-indexing = A ler… { $percent }%
log-matches = { $count ->
    [one] { $count } linha correspondente
    [many] { $count } linhas correspondentes
   *[other] { $count } linhas correspondentes
    }
log-filter-hint = Filtrar linhas
log-bad-pattern = Padrão inválido
log-follow = Acompanhar
log-close = Fechar visualizador
log-opened-large = { $name } é grande, por isso foi aberto só de leitura no visualizador de registos.
tab-pin = Afixar separador
tab-unpin = Desafixar separador
edit-add-cursor-above = Adicionar cursor acima
edit-add-cursor-below = Adicionar cursor abaixo
view-word-wrap = Moldar texto
status-wrap-on = As linhas longas são agora moldadas à largura da janela.
status-wrap-off = As linhas longas já não são moldadas.
settings-word-wrap = Moldar linhas longas
edit-go-to-line = Ir para a linha…
edit-go-to-heading = Ir para o título…
goto-line-hint = Linha ou linha:coluna
goto-line-note = { $count ->
    [one] O documento tem 1 linha.
    [many] O documento tem { $count } de linhas.
   *[other] O documento tem { $count } linhas.
    }
goto-line-invalid = Introduza um número de linha, opcionalmente seguido de :coluna.
goto-heading-hint = Escreva para filtrar os títulos
goto-heading-none = Este documento não tem títulos.
format-xml = XML
format-yaml = YAML
tools-tree-error = { $format } inválido na linha { $line }, coluna { $column }: { $reason }
edit-select-next-occurrence = Selecionar ocorrência seguinte
tools-record-macro = Iniciar ou parar a gravação de macro
tools-play-macro = Reproduzir macro
macro-recording = A gravar macro
macro-recorded = { $count ->
    [one] Macro gravada: 1 passo.
    [many] Macro gravada: { $count } de passos.
   *[other] Macro gravada: { $count } passos.
    }
tools-compare-saved = Comparar com a versão guardada
tools-compare-clipboard = Comparar com a área de transferência
compare-saved-label = { $name } (no disco)
compare-clipboard-label = Área de transferência
compare-saved-failed = Não foi possível ler o ficheiro guardado: { $reason }
compare-clipboard-empty = A área de transferência não contém texto para comparar.
error-save-disk-full = { $name } não foi guardado: o disco está cheio. O ficheiro no disco não foi alterado e o texto continua aqui. Liberte espaço ou utilize Guardar como para o guardar noutro local.
error-save-permission = { $name } não foi guardado: não tem permissão para escrever nesse local. O ficheiro no disco não foi alterado. Utilize Guardar como para o guardar noutro local.
error-save-read-only-volume = { $name } não foi guardado: o disco é só de leitura. O ficheiro no disco não foi alterado. Utilize Guardar como para o guardar noutro local.
replace-preview-title = { $count ->
    [one] Substituir 1 correspondência?
    [many] Substituir { $count } correspondências?
   *[other] Substituir { $count } correspondências?
    }
replace-preview-confirm = { $count ->
    [one] Substituir 1
    [many] Substituir { $count }
   *[other] Substituir { $count }
    }
replace-preview-more = Só são mostradas as primeiras 200 linhas alteradas.
replace-preview-stale = O documento foi alterado; nada foi substituído. Execute novamente Substituir tudo.
view-split-editor = Dividir editor
