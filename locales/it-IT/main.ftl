# Machine translation by Claude (AI); review by a native speaker required before release.
app-name = Sanain
menu-file = File
file-open = Apri…
file-save = Salva
status-tabs = { $count ->
    [one] Una scheda
   *[other] { $count } schede
    }
error-open = Impossibile aprire { $name }.
menu-edit = Modifica
file-new = Nuova scheda
file-save-as = Salva con nome…
edit-undo = Annulla
edit-redo = Ripeti
edit-select-all = Seleziona tutto
untitled = Senza titolo
window-title = { $dirty ->
    [yes] ● { $name } — Sanain
   *[no] { $name } — Sanain
    }
status-position = Riga { $line }, col. { $column }
status-selections = { $count ->
    [one] 1 selezione
   *[other] { $count } selezioni
    }
status-saved = { $name } salvato
encoding-utf8 = UTF-8
encoding-utf8-bom = UTF-8 con BOM
encoding-utf16le = UTF-16 LE
encoding-utf16be = UTF-16 BE
line-ending-lf = LF
line-ending-crlf = CRLF
line-ending-cr = CR
line-ending-mixed = Fine riga miste
editor-accessible-name = Editor di testo
button-save = Salva
button-dont-save = Non salvare
button-cancel = Annulla
button-ok = OK
button-overwrite = Sovrascrivi
button-save-as = Salva con nome…
dialog-unsaved-title = Salvare le modifiche a { $name }?
dialog-unsaved-body = Se non le salvi, le modifiche andranno perse.
dialog-conflict-title = { $name } è stato modificato sul disco
dialog-conflict-body = Un altro programma ha modificato questo file dopo che lo hai aperto. Sovrascrivendolo, quelle modifiche verranno sostituite.
dialog-readonly-title = { $name } è di sola lettura
dialog-readonly-body = Salva invece una copia con un altro nome.
error-open-encoding = { $name } non è testo UTF-8 o UTF-16. Non è stato aperto, quindi il contenuto è invariato. Usa File > Riapri con codifica per sceglierne la codifica.
error-save = Impossibile salvare { $name }: { $reason }
menu-window = Finestra
edit-cut = Taglia
edit-copy = Copia
edit-paste = Incolla
app-quit = Esci da Sanain
app-hide = Nascondi Sanain
app-hide-others = Nascondi altre
app-show-all = Mostra tutte
window-minimize = Contrai
window-zoom = Zoom
status-recovered = Ripristinate le modifiche non salvate dell’ultima sessione
status-recovery-failed = Impossibile salvare una copia di ripristino: { $reason }
edit-duplicate-line = Duplica riga
edit-delete-line = Elimina riga
edit-move-line-up = Sposta riga in alto
edit-move-line-down = Sposta riga in basso
file-close-tab = Chiudi scheda
file-reopen-tab = Riapri scheda chiusa
window-next-tab = Scheda successiva
window-prev-tab = Scheda precedente
tab-close = Chiudi scheda
edit-find = Trova…
edit-find-next = Trova successivo
edit-find-prev = Trova precedente
edit-replace = Sostituisci…
find-placeholder = Trova
replace-placeholder = Sostituisci con
find-match-case = Maiuscole/minuscole
find-whole-word = Parola intera
find-regex = Espressione regolare
find-no-results = Nessun risultato
find-count = { $total ->
    [one] 1 corrispondenza
   *[other] { $total } corrispondenze
    }
find-count-current = { $current } di { $total }
find-prev = Corrispondenza precedente (Shift+Enter)
find-next = Corrispondenza successiva (Enter)
find-close = Chiudi (Esc)
replace-one = Sostituisci
replace-all = Sostituisci tutto
status-replaced = { $count ->
    [one] 1 corrispondenza sostituita
   *[other] { $count } corrispondenze sostituite
    }
language-plain = Testo semplice
menu-view = Vista
app-settings = Impostazioni…
view-zoom-in = Ingrandisci
view-zoom-out = Riduci
view-zoom-reset = Dimensioni effettive
settings-title = Impostazioni
settings-font-size = Dimensione carattere
settings-tab-width = Larghezza tabulazione
settings-insert-spaces = Inserisci spazi premendo Tab
settings-theme = Aspetto
settings-theme-system = Come il sistema
settings-theme-light = Chiaro
settings-theme-dark = Scuro
settings-restore-session = Riapri i file dell’ultima sessione
settings-language = Lingua
settings-language-system = Come il sistema
settings-language-note = Le modifiche alla lingua hanno effetto dopo il riavvio di Sanain.
settings-save-failed = Impossibile salvare le impostazioni: { $reason }
view-command-palette = Tavolozza comandi…
palette-placeholder = Digita un comando
view-markdown-source = Sorgente Markdown
view-markdown-split = Sorgente e anteprima Markdown
view-markdown-preview = Anteprima Markdown
preview-too-large = Questo documento è troppo grande per l’anteprima.
preview-mermaid-unsupported = I diagrammi Mermaid non sono ancora visualizzati; viene mostrato il sorgente.
menu-tools = Strumenti
tools-json-validate = Convalida JSON
tools-json-format = Formatta JSON
tools-json-minify = Minimizza JSON
tools-sort-lines = Ordina righe
tools-sort-lines-desc = Ordina righe in ordine decrescente
tools-unique-lines = Rimuovi righe duplicate
tools-trim-trailing = Rimuovi spazi a fine riga
tools-upper-case = Maiuscole
tools-lower-case = Minuscole
tools-to-lf = Converti fine riga in LF
tools-to-crlf = Converti fine riga in CRLF
tools-json-valid = Il JSON è valido
tools-done = Fatto. Annulla ripristina lo stato precedente.
tools-no-change = Nulla da modificare
tools-json-error = JSON non valido alla riga { $line }, colonna { $column }: { $reason }
json-reason-unterminated-string = stringa non terminata
json-reason-invalid-u-escape = sequenza di escape \u non valida
json-reason-invalid-escape = sequenza di escape non valida
json-reason-control-character-in-string = carattere di controllo nella stringa
json-reason-invalid-number = numero non valido
json-reason-unexpected-character = carattere imprevisto
json-reason-unexpected-content-after-the-value = contenuto imprevisto dopo il valore
json-reason-unexpected-comma = virgola imprevista
json-reason-expected-a-string-key = prevista una chiave di tipo stringa
json-reason-expected = previsto ':'
json-reason-expected-or-a-closing-bracket = previsto ',' o una parentesi di chiusura
json-reason-expected-a-value = previsto un valore
json-reason-empty-document = documento vuoto
json-reason-unexpected-end-of-input = fine dell’input imprevista
json-reason-too-deeply-nested-to-format = annidamento troppo profondo per la formattazione
tools-compare = Confronta con…
compare-need-two = Apri un’altra scheda per il confronto.
compare-pick-title = Confronta { $name } con
compare-count = { $count ->
    [0] Nessuna differenza
    [one] 1 modifica
   *[other] { $count } modifiche
    }
compare-close = Chiudi confronto
compare-to-left = Copia questa modifica a sinistra
compare-to-right = Copia questa modifica a destra
compare-stale = Un documento è cambiato durante il confronto. Il confronto è stato aggiornato; riprova.
view-structured = Vista strutturata (CSV, JSON)
csv-delimiter = Delimitatore
csv-rows = { $count ->
    [one] 1 riga
   *[other] { $count } righe
    }
csv-unterminated = Virgolette non chiuse a partire dalla riga { $line }
csv-truncated = Vengono mostrate le prime 200.000 righe
json-more = … altri { $count }
dialog-open-link-title = Aprire questo link nel browser web?
button-open = Apri
link-blocked = Per sicurezza, questo tipo di link non viene aperto.
link-not-found = File collegato non trovato: { $target }
enc-windows-1252 = Europa occidentale
enc-iso-8859-15 = Europa occidentale
enc-windows-1250 = Europa centrale
enc-iso-8859-2 = Europa centrale
enc-windows-1251 = Cirillico
enc-koi8-r = Cirillico
enc-windows-1253 = Greco
enc-windows-1254 = Turco
enc-windows-1255 = Ebraico
enc-windows-1256 = Arabo
enc-windows-1257 = Baltico
enc-windows-874 = Thailandese
enc-shift_jis = Giapponese
enc-euc-jp = Giapponese
enc-gb18030 = Cinese semplificato
enc-big5 = Cinese tradizionale
enc-euc-kr = Coreano
file-reopen-encoding = Riapri con codifica…
file-save-encoding = Salva con codifica…
encoding-reopen-title = Riapri con codifica
encoding-save-title = Salva con codifica
encoding-not-exact = Questa codifica non può aprire il file senza modificarlo: { $reason }
encoding-unencodable = «{ $character }» non può essere salvato come { $encoding }. Scegli un’altra codifica o modifica il testo; non è stato scritto nulla.
encoding-save-first = Salva o annulla le modifiche prima di riaprire con un’altra codifica.
image-remote = Le immagini remote non vengono caricate.
image-blocked = Vengono mostrate solo le immagini nella cartella di questo documento.
image-unsupported = Formato immagine non supportato.
image-too-large = L’immagine è troppo grande per essere mostrata.
image-unreadable = Impossibile leggere l’immagine.
settings-theme-high-contrast = Contrasto elevato
view-fold = Comprimi
view-unfold = Espandi
view-unfold-all = Espandi tutto
settings-shortcuts = Scorciatoie da tastiera
settings-keymap-preset = Preimpostazione
settings-keymap-default = Sanain
settings-keymap-notepadpp = Notepad++
settings-shortcut-change = Cambia
settings-shortcut-reset = Ripristina
settings-shortcut-press = Premi una scorciatoia… (Esc per annullare)
settings-shortcut-moved = Scorciatoia spostata da:
edit-find-in-folder = Trova nella cartella…
folder-search-placeholder = Trova nella cartella
folder-search-run = Cerca
folder-search-stop = Interrompi
folder-search-no-folder = Scegli cartella
folder-search-summary = { $count ->
    [one] 1 corrispondenza
   *[other] { $count } corrispondenze
    } in { $files ->
    [one] 1 file
   *[other] { $files } file
    }
