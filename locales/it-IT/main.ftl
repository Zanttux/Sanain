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
preview-mermaid-unsupported-type = Tipo di diagramma non supportato; viene mostrato il sorgente.
preview-mermaid-unsupported-syntax = Il diagramma usa una sintassi che non può essere disegnata; viene mostrato il sorgente.
preview-mermaid-too-large = Il diagramma è troppo grande da disegnare; viene mostrato il sorgente.
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
view-structured = Vista strutturata (CSV, JSON, XML, YAML)
csv-delimiter = Delimitatore
csv-rows = { $count ->
    [one] 1 riga
   *[other] { $count } righe
    }
csv-unterminated = Virgolette non chiuse a partire dalla riga { $line }
csv-truncated = Vengono mostrate le prime 200.000 righe
csv-edit-cell = Modifica cella
csv-insert-row-below = Inserisci riga sotto
csv-delete-row = Elimina riga
csv-read-only = File di sola lettura: la griglia non può essere modificata
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
app-check-updates = Verifica aggiornamenti…
update-checking = Verifica degli aggiornamenti in corso…
update-up-to-date = Sanain { $version } è la versione più recente.
update-no-releases = Non è ancora stata pubblicata alcuna versione di Sanain.
update-failed = Impossibile verificare gli aggiornamenti: { $reason }
update-invalid = risposta imprevista
update-available-title = Sanain { $version } è disponibile
update-available-body = Apri la pagina di download per ottenerlo. Sanain non scarica né installa nulla da solo.
update-open-page = Apri pagina di download
update-later = Più tardi
settings-check-updates = Verifica aggiornamenti una volta al giorno
settings-check-updates-detail = Chiede a github.com l’ultima versione pubblicata. Non viene inviato nient’altro.
file-local-history = Cronologia locale…
file-clear-history = Cancella cronologia locale…
history-title = Cronologia locale di { $name }
history-open = Apri
history-restore = Ripristina
history-empty = Nessuna versione salvata. Viene conservata una copia a ogni salvataggio.
history-unreadable = Impossibile leggere questa versione.
history-just-now = Proprio ora
history-minutes-ago = { $count ->
    [one] 1 minuto fa
   *[other] { $count } minuti fa
    }
history-hours-ago = { $count ->
    [one] 1 ora fa
   *[other] { $count } ore fa
    }
history-days-ago = { $count ->
    [one] 1 giorno fa
   *[other] { $count } giorni fa
    }
history-purge-title = Eliminare tutta la cronologia locale?
history-purge-body = Le copie salvate di tutti i file vengono rimosse da questo computer. I file stessi non vengono modificati.
history-purge-confirm = Elimina cronologia
history-purged = Cronologia locale eliminata.
settings-local-history = Conserva la cronologia locale dei file salvati
settings-local-history-detail = Conserva fino a 50 versioni per file per 30 giorni, solo su questo computer.
file-open-log = Apri nel visualizzatore di log…
log-lines = { $count ->
    [one] 1 riga
   *[other] { $count } righe
    }
log-indexing = Lettura… { $percent }%
log-matches = { $count ->
    [one] 1 riga corrispondente
   *[other] { $count } righe corrispondenti
    }
log-filter-hint = Filtra righe
log-bad-pattern = Schema non valido
log-follow = Segui
log-close = Chiudi visualizzatore
log-opened-large = { $name } è grande, quindi è stato aperto in sola lettura nel visualizzatore di log.
tab-pin = Blocca scheda
tab-unpin = Sblocca scheda
edit-add-cursor-above = Aggiungi cursore sopra
edit-add-cursor-below = Aggiungi cursore sotto
view-word-wrap = A capo automatico
status-wrap-on = Le righe lunghe ora vanno a capo alla larghezza della finestra.
status-wrap-off = Le righe lunghe non vanno più a capo.
settings-word-wrap = Manda a capo le righe lunghe
edit-go-to-line = Vai alla riga…
edit-go-to-heading = Vai al titolo…
goto-line-hint = Riga o riga:colonna
goto-line-note = { $count ->
    [one] Il documento ha 1 riga.
   *[other] Il documento ha { $count } righe.
    }
goto-line-invalid = Inserisci un numero di riga, eventualmente seguito da :colonna.
goto-heading-hint = Digita per filtrare i titoli
goto-heading-none = Questo documento non ha titoli.
format-xml = XML
format-yaml = YAML
tools-tree-error = { $format } non valido alla riga { $line }, colonna { $column }: { $reason }
edit-select-next-occurrence = Seleziona occorrenza successiva
tools-record-macro = Avvia o interrompi registrazione macro
tools-play-macro = Esegui macro
macro-recording = Registrazione macro in corso
macro-recorded = { $count ->
    [one] Macro registrata: 1 passaggio.
   *[other] Macro registrata: { $count } passaggi.
    }
tools-compare-saved = Confronta con la versione salvata
tools-compare-clipboard = Confronta con gli appunti
compare-saved-label = { $name } (su disco)
compare-clipboard-label = Appunti
compare-saved-failed = Impossibile leggere il file salvato: { $reason }
compare-clipboard-empty = Gli appunti non contengono testo da confrontare.
error-save-disk-full = { $name } non è stato salvato: il disco è pieno. Il file sul disco non è cambiato e il tuo testo è ancora qui. Libera spazio oppure usa Salva con nome per salvarlo altrove.
error-save-permission = { $name } non è stato salvato: non hai l’autorizzazione per scrivere in quella posizione. Il file sul disco non è cambiato. Usa Salva con nome per salvarlo altrove.
error-save-read-only-volume = { $name } non è stato salvato: il disco è di sola lettura. Il file sul disco non è cambiato. Usa Salva con nome per salvarlo altrove.
replace-preview-title = { $count ->
    [one] Sostituire 1 corrispondenza?
   *[other] Sostituire { $count } corrispondenze?
    }
replace-preview-confirm = { $count ->
    [one] Sostituisci 1
   *[other] Sostituisci { $count }
    }
replace-preview-more = Vengono mostrate solo le prime 200 righe modificate.
replace-preview-stale = Il documento è cambiato; non è stato sostituito nulla. Esegui di nuovo Sostituisci tutto.
encoding-detected = Aperto come { $encoding } (rilevato dal contenuto del file).
encoding-suggested = Suggerita in base al contenuto del file: { $encoding }. Controlla l’anteprima prima di aprire.
app-about = Informazioni su Sanain
menu-help = Aiuto
about-tagline = Il potere delle parole.
about-version = Versione { $version }
about-license = Libero e open source con licenza Mozilla Public License 2.0.
about-ai = Creato dall’IA. Guidato da persone. Sostenuto dalla comunità.
about-notices = I componenti di terze parti e le relative licenze sono elencati in THIRD_PARTY_NOTICES, incluso in ogni versione.
about-source = Codice sorgente e segnalazioni su GitHub
tools-xml-validate = Convalida XML
tools-yaml-validate = Convalida YAML
tools-markup-valid = Il { $format } è valido
file-open-recent = Apri recenti…
file-save-all = Salva tutto
file-revert = Ripristina versione salvata
recent-hint = Digita per filtrare i file recenti
recent-none = Ancora nessun file recente.
revert-title = Scartare le modifiche a { $name }?
revert-body = Il file viene ricaricato dal disco. Le modifiche non salvate vanno perse e non possono essere annullate.
revert-confirm = Scarta e ricarica
revert-done = { $name } è stato ricaricato dal disco.
status-saved-all = { $count ->
    [0] Niente da salvare.
    [one] 1 file salvato.
   *[other] { $count } file salvati.
    }
edit-toggle-comment = Attiva/disattiva commento riga
edit-indent = Aumenta rientro righe
edit-outdent = Riduci rientro righe
comment-unsupported = Questo tipo di file non ha commenti di riga.
tab-close-others = Chiudi le altre schede
tab-close-all = Chiudi tutte le schede
file-copy-path = Copia percorso file
file-reveal-macos = Mostra nel Finder
file-reveal-windows = Mostra in Esplora file
file-reveal-other = Mostra nel file manager
status-path-copied = Percorso del file copiato.
tabs-kept-unsaved = { $count ->
    [one] 1 scheda con modifiche non salvate è stata lasciata aperta.
   *[other] { $count } schede con modifiche non salvate sono state lasciate aperte.
    }
status-indent-tabs = Tabulazioni
status-indent-spaces = Spazi: { $count }
find-searching = Ricerca in corso…
compare-ignore-whitespace = Ignora spazi
tools-compare-file = Confronta con un file…
compare-file-failed = Impossibile confrontare con { $name }: { $reason }
cli-help =
    Uso:
      sanain [FILE…]                   apri file
      sanain --diff SINISTRA DESTRA    confronta due file affiancati
      sanain --version                 mostra la versione
      sanain --help                    mostra questa guida
    Usa -- prima dei nomi di file che iniziano con un trattino.
cli-unknown-option = Opzione sconosciuta { $option }.
cli-diff-needs-two = --diff richiede due file: sanain --diff SINISTRA DESTRA
window-move-tab-left = Sposta scheda a sinistra
window-move-tab-right = Sposta scheda a destra
edit-skip-occurrence = Salta all’occorrenza successiva
tools-play-macro-times = Esegui macro più volte…
macro-times-hint = Numero di volte
macro-times-note = Da 1 a { $max }
macro-times-invalid = Inserisci un numero da 1 a { $max }.
view-outline = Struttura
outline-empty = Nessuna intestazione in questo documento.
outline-close = Chiudi struttura
folder-replace-placeholder = Sostituisci con
folder-replace-run = Sostituisci nei file…
replace-files-title = Sostituire { $count ->
    [one] 1 corrispondenza
   *[other] { $count } corrispondenze
    } in { $files ->
    [one] 1 file
   *[other] { $files } file
    }?
replace-files-body = I file aperti in una scheda vengono modificati lì e la modifica può essere annullata; salvali tu. Gli altri file vengono salvati subito e il loro contenuto precedente viene conservato nella cronologia locale, se è attiva. Un file modificato sul disco nel frattempo viene saltato.
replace-files-confirm = Sostituisci
replace-files-done = { $count ->
    [one] Sostituita 1 corrispondenza
   *[other] Sostituite { $count } corrispondenze
    } in { $files ->
    [one] 1 file
   *[other] { $files } file
    } (aperti in schede: { $open }, non salvati).
replace-files-failed = { $failed ->
    [one] 1 file non è stato modificato:
   *[other] { $failed } file non sono stati modificati; il primo:
    } { $first }
image-outside = Questa immagine si trova fuori dalla cartella del documento.
image-allow = Mostra le immagini di questa cartella
cli-help-integration =
    Integrazione con il file manager (solo per l’utente corrente):
      sanain --register           aggiunge Sanain ad «Apri con» (Linux, Windows)
      sanain --unregister         lo rimuove di nuovo
cli-registered = Sanain è stato aggiunto ad «Apri con». { $place }
cli-unregistered = Sanain è stato rimosso da «Apri con».
cli-register-failed = Impossibile modificare l’integrazione con il file manager: { $reason }
cli-register-macos = Su macOS non serve registrare nulla: quando Sanain.app si trova nella cartella Applicazioni, il Finder lo propone in «Apri con».
settings-tab-general = Generale
settings-tab-editor = Editor
settings-tab-appearance = Aspetto
settings-shortcut-search = Cerca comandi o tasti
settings-font = Carattere
settings-font-default = Monospaziato integrato
settings-font-loading = Ricerca dei caratteri installati…
settings-font-missing = Il carattere { $font } non è stato trovato; viene usato il carattere integrato.
settings-theme-more = Altri temi
settings-theme-user-note = Temi personali: file .theme nella cartella { $folder } (vedi docs/THEMES.md).
language-auto = Rileva automaticamente
language-choose = Scegli il linguaggio di questa scheda
tools-xml-format = Formatta XML
tools-save-macro = Salva macro…
tools-play-saved-macro = Esegui macro salvata…
tools-delete-saved-macro = Elimina macro salvata…
macro-save-hint = Nome della macro
macro-save-note = Una macro salvata con lo stesso nome viene sostituita.
macro-save-empty = Niente da salvare: registra prima una macro.
macro-saved = Macro salvata: { $name }
macro-deleted = Macro eliminata: { $name }
macro-name-invalid = Inserisci un nome da 1 a { $max } caratteri su una sola riga.
macro-save-too-large = Questa macro è troppo lunga per essere salvata.
macro-save-full = Si possono salvare al massimo { $max } macro. Eliminane prima una.
macro-save-failed = Impossibile salvare le macro: { $error }
macro-saved-hint = Digita per filtrare le macro salvate
macro-saved-none = Ancora nessuna macro salvata.
settings-shortcut-import-npp = Importa da Notepad++…
settings-shortcut-import-npp-hint = Scegli un file shortcuts.xml di Notepad++. Su Windows si trova in %AppData%\Notepad++\shortcuts.xml.
settings-shortcut-import-npp-done = Importate da Notepad++: { $imported }. Saltate: { $skipped }. Conflitti rimasti: { $conflicts }.
settings-shortcut-import-npp-failed = Impossibile importare { $name }: { $reason }
settings-shortcut-import-npp-too-large = Il file supera 1 MiB.
settings-shortcut-import-npp-invalid = Il file non è uno shortcuts.xml valido di Notepad++.
view-split-right = Dividi a destra
view-split-down = Dividi in basso
view-close-pane = Chiudi riquadro
view-focus-next-pane = Riquadro successivo
view-focus-prev-pane = Riquadro precedente
status-pane-limit = Limite di riquadri raggiunto (al massimo { $max }).
