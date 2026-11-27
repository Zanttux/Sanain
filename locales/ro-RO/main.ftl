# Machine translation by Claude (AI); review by a native speaker required before release.
app-name = Sanain
menu-file = Fișier
file-open = Deschide…
file-save = Salvează
status-tabs = { $count ->
    [one] O filă
    [few] { $count } file
   *[other] { $count } de file
    }
error-open = Nu s-a putut deschide { $name }.
menu-edit = Editare
file-new = Filă nouă
file-save-as = Salvează ca…
edit-undo = Anulează
edit-redo = Refă
edit-select-all = Selectează tot
untitled = Fără titlu
window-title = { $dirty ->
    [yes] ● { $name } — Sanain
   *[no] { $name } — Sanain
    }
status-position = Rd { $line }, Col { $column }
status-selections = { $count ->
    [one] 1 selecție
    [few] { $count } selecții
   *[other] { $count } de selecții
    }
status-saved = S-a salvat { $name }
encoding-utf8 = UTF-8
encoding-utf8-bom = UTF-8 cu BOM
encoding-utf16le = UTF-16 LE
encoding-utf16be = UTF-16 BE
line-ending-lf = LF
line-ending-crlf = CRLF
line-ending-cr = CR
line-ending-mixed = Terminații de linie mixte
editor-accessible-name = Editor de text
button-save = Salvează
button-dont-save = Nu salva
button-cancel = Anulează
button-ok = OK
button-overwrite = Suprascrie
button-save-as = Salvează ca…
dialog-unsaved-title = Salvați modificările din { $name }?
dialog-unsaved-body = Modificările se vor pierde dacă nu le salvați.
dialog-conflict-title = { $name } s-a modificat pe disc
dialog-conflict-body = Alt program a modificat acest fișier după ce l-ați deschis. Suprascrierea înlocuiește acele modificări.
dialog-readonly-title = { $name } este doar în citire
dialog-readonly-body = Salvați în schimb o copie sub alt nume.
error-open-encoding = { $name } nu este text UTF-8 sau UTF-16. Nu a fost deschis, deci conținutul său este neschimbat. Folosiți Fișier > Redeschide cu codificarea pentru a-i alege codificarea.
error-save = Nu s-a putut salva { $name }: { $reason }
menu-window = Fereastră
edit-cut = Decupează
edit-copy = Copiază
edit-paste = Lipește
app-quit = Ieși din Sanain
app-hide = Ascunde Sanain
app-hide-others = Ascunde celelalte
app-show-all = Afișează tot
window-minimize = Minimizează
window-zoom = Zoom
status-recovered = Au fost restaurate modificările nesalvate din sesiunea anterioară
status-recovery-failed = Nu s-a putut salva o copie de recuperare: { $reason }
edit-duplicate-line = Duplică linia
edit-delete-line = Șterge linia
edit-move-line-up = Mută linia în sus
edit-move-line-down = Mută linia în jos
file-close-tab = Închide fila
file-reopen-tab = Redeschide fila închisă
window-next-tab = Fila următoare
window-prev-tab = Fila anterioară
tab-close = Închide fila
edit-find = Caută…
edit-find-next = Caută următorul
edit-find-prev = Caută anteriorul
edit-replace = Înlocuiește…
find-placeholder = Caută
replace-placeholder = Înlocuiește cu
find-match-case = Potrivire majuscule/minuscule
find-whole-word = Cuvânt întreg
find-regex = Expresie regulată
find-no-results = Niciun rezultat
find-count = { $total ->
    [one] 1 potrivire
    [few] { $total } potriviri
   *[other] { $total } de potriviri
    }
find-count-current = { $current } din { $total }
find-prev = Potrivirea anterioară (Shift+Enter)
find-next = Potrivirea următoare (Enter)
find-close = Închide (Esc)
replace-one = Înlocuiește
replace-all = Înlocuiește tot
status-replaced = { $count ->
    [one] S-a înlocuit 1 potrivire
    [few] S-au înlocuit { $count } potriviri
   *[other] S-au înlocuit { $count } de potriviri
    }
language-plain = Text simplu
menu-view = Vizualizare
app-settings = Setări…
view-zoom-in = Mărește
view-zoom-out = Micșorează
view-zoom-reset = Dimensiune reală
settings-title = Setări
settings-font-size = Dimensiunea fontului
settings-tab-width = Lățimea tabulatorului
settings-insert-spaces = Inserează spații la apăsarea tastei Tab
settings-theme = Aspect
settings-theme-system = Ca sistemul
settings-theme-light = Luminos
settings-theme-dark = Întunecat
settings-restore-session = Redeschide fișierele din sesiunea anterioară
settings-language = Limbă
settings-language-system = Ca sistemul
settings-language-note = Schimbarea limbii se aplică după repornirea Sanain.
settings-save-failed = Nu s-au putut salva setările: { $reason }
view-command-palette = Paleta de comenzi…
palette-placeholder = Tastați o comandă
view-markdown-source = Sursă Markdown
view-markdown-split = Sursă și previzualizare Markdown
view-markdown-preview = Previzualizare Markdown
preview-too-large = Acest document este prea mare pentru previzualizare.
preview-mermaid-unsupported-type = Tipul de diagramă nu este acceptat; se afișează sursa.
preview-mermaid-unsupported-syntax = Diagrama folosește o sintaxă care nu poate fi desenată; se afișează sursa.
preview-mermaid-too-large = Diagrama este prea mare pentru a fi desenată; se afișează sursa.
menu-tools = Instrumente
tools-json-validate = Validează JSON
tools-json-format = Formatează JSON
tools-json-minify = Minimizează JSON
tools-sort-lines = Sortează liniile
tools-sort-lines-desc = Sortează liniile descrescător
tools-unique-lines = Elimină liniile duplicate
tools-trim-trailing = Elimină spațiile de la final
tools-upper-case = Majuscule
tools-lower-case = Minuscule
tools-to-lf = Convertește terminațiile de linie în LF
tools-to-crlf = Convertește terminațiile de linie în CRLF
tools-json-valid = JSON-ul este valid
tools-done = Gata. Anulează revine asupra modificării.
tools-no-change = Nimic de modificat
tools-json-error = JSON nevalid la linia { $line }, coloana { $column }: { $reason }
json-reason-unterminated-string = șir neterminat
json-reason-invalid-u-escape = secvență \u nevalidă
json-reason-invalid-escape = secvență escape nevalidă
json-reason-control-character-in-string = caracter de control în șir
json-reason-invalid-number = număr nevalid
json-reason-unexpected-character = caracter neașteptat
json-reason-unexpected-content-after-the-value = conținut neașteptat după valoare
json-reason-unexpected-comma = virgulă neașteptată
json-reason-expected-a-string-key = se aștepta o cheie de tip șir
json-reason-expected = se aștepta ':'
json-reason-expected-or-a-closing-bracket = se aștepta ',' sau o paranteză de închidere
json-reason-expected-a-value = se aștepta o valoare
json-reason-empty-document = document gol
json-reason-unexpected-end-of-input = sfârșit neașteptat al intrării
json-reason-too-deeply-nested-to-format = imbricare prea adâncă pentru formatare
tools-compare = Compară cu…
compare-need-two = Deschideți altă filă pentru comparare.
compare-pick-title = Compară { $name } cu
compare-count = { $count ->
    [0] Nicio diferență
    [one] 1 modificare
    [few] { $count } modificări
   *[other] { $count } de modificări
    }
compare-close = Închide compararea
compare-to-left = Copiază această modificare în stânga
compare-to-right = Copiază această modificare în dreapta
compare-stale = Un document s-a modificat în timpul comparării. Compararea a fost reîmprospătată; încercați din nou.
view-structured = Vizualizare structurată (CSV, JSON, XML, YAML)
csv-delimiter = Delimitator
csv-rows = { $count ->
    [one] 1 rând
    [few] { $count } rânduri
   *[other] { $count } de rânduri
    }
csv-unterminated = Ghilimele neînchise începând de la linia { $line }
csv-truncated = Se afișează primele 200.000 de rânduri
csv-edit-cell = Editează celula
csv-insert-row-below = Inserează rând dedesubt
csv-delete-row = Șterge rândul
csv-read-only = Fișier doar în citire: grila nu poate fi editată
json-more = … încă { $count }
json-edit-value = Editează valoarea
json-rename-key = Redenumește cheia
json-add-after = Adaugă după
json-add-inside = Adaugă în interior
json-delete-node = Șterge
json-read-only = Fișier doar în citire: arborele nu poate fi editat
json-invalid-value = Nu este o valoare JSON validă
json-duplicate-key = Obiectul are deja o cheie cu acest nume
xml-edit-value = Editează valoarea
xml-edit-text = Editează textul
xml-rename = Redenumește
xml-add-attribute = Adaugă atribut
xml-add-child = Adaugă element copil
xml-add-after = Adaugă după
xml-delete = Șterge
xml-invalid-name = Nu este un nume XML valid
xml-duplicate-attribute = Elementul are deja un atribut cu acest nume
xml-not-well-formed = Această modificare ar face XML-ul invalid
yaml-not-preserved = Această modificare ar schimba alte părți ale YAML-ului
yaml-key-too-long = O cheie poate avea cel mult 1024 de caractere
yaml-read-only-flow = Stilul flow (scris între paranteze) nu poate fi încă editat în arbore
yaml-read-only-anchor = Valorile cu ancoră (&) nu pot fi editate în arbore
yaml-read-only-tag = Valorile cu etichetă (!) nu pot fi editate în arbore
yaml-read-only-alias = Aliasurile (*) nu pot fi editate în arbore
yaml-read-only-block-scalar = Blocurile de text (| sau >) nu pot fi încă editate în arbore
yaml-read-only-complex-key = Cheile complexe nu pot fi editate în arbore
yaml-read-only-document = Doar primul document poate fi editat în arbore
yaml-read-only-layout = Această intrare își împarte liniile cu alt conținut și nu poate fi modificată aici
yaml-read-only-only-child = Singura intrare nu poate fi eliminată
dialog-open-link-title = Deschideți acest link în browserul web?
button-open = Deschide
link-blocked = Acest tip de link nu este deschis, din motive de siguranță.
link-not-found = Fișierul legat nu a fost găsit: { $target }
enc-windows-1252 = Europa de Vest
enc-iso-8859-15 = Europa de Vest
enc-windows-1250 = Europa Centrală
enc-iso-8859-2 = Europa Centrală
enc-windows-1251 = Chirilic
enc-koi8-r = Chirilic
enc-windows-1253 = Greacă
enc-windows-1254 = Turcă
enc-windows-1255 = Ebraică
enc-windows-1256 = Arabă
enc-windows-1257 = Baltică
enc-windows-874 = Thailandeză
enc-shift_jis = Japoneză
enc-euc-jp = Japoneză
enc-gb18030 = Chineză simplificată
enc-big5 = Chineză tradițională
enc-euc-kr = Coreeană
file-reopen-encoding = Redeschide cu codificarea…
file-save-encoding = Salvează cu codificarea…
encoding-reopen-title = Redeschide cu codificarea
encoding-save-title = Salvează cu codificarea
encoding-not-exact = Această codificare nu poate deschide fișierul fără a-l modifica: { $reason }
encoding-unencodable = „{ $character }” nu poate fi salvat ca { $encoding }. Alegeți altă codificare sau modificați textul; nu s-a scris nimic.
encoding-save-first = Salvați sau anulați modificările înainte de a redeschide cu altă codificare.
image-remote = Imaginile de la distanță nu sunt încărcate.
image-blocked = Sunt afișate doar imaginile din dosarul acestui document.
image-unsupported = Format de imagine neacceptat.
image-too-large = Imaginea este prea mare pentru a fi afișată.
image-unreadable = Imaginea nu a putut fi citită.
settings-theme-high-contrast = Contrast ridicat
view-fold = Pliază
view-unfold = Depliază
view-unfold-all = Depliază tot
settings-shortcuts = Comenzi rapide de la tastatură
settings-keymap-preset = Presetare
settings-keymap-default = Sanain
settings-keymap-notepadpp = Notepad++
settings-shortcut-change = Modifică
settings-shortcut-reset = Resetează
settings-shortcut-press = Apăsați o comandă rapidă… (Esc anulează)
settings-shortcut-moved = Comandă rapidă mutată de la:
edit-find-in-folder = Caută în dosar…
folder-search-placeholder = Caută în dosar
folder-search-run = Caută
folder-search-stop = Oprește
folder-search-no-folder = Alegeți dosarul
folder-search-summary = { $count ->
    [one] 1 potrivire
    [few] { $count } potriviri
   *[other] { $count } de potriviri
    } în { $files ->
    [one] 1 fișier
    [few] { $files } fișiere
   *[other] { $files } de fișiere
    }
app-check-updates = Caută actualizări…
update-checking = Se caută actualizări…
update-up-to-date = Sanain { $version } este cea mai nouă versiune.
update-no-releases = Nu a fost publicată încă nicio versiune Sanain.
update-failed = Nu s-a putut căuta actualizări: { $reason }
update-invalid = răspuns neașteptat
update-available-title = Sanain { $version } este disponibil
update-available-body = Deschide pagina de descărcare pentru a-l obține. Sanain nu descarcă și nu instalează nimic de la sine.
update-open-page = Deschide pagina de descărcare
update-later = Mai târziu
settings-check-updates = Caută actualizări o dată pe zi
settings-check-updates-detail = Interoghează github.com pentru cea mai recentă versiune. Nu se trimite nimic altceva.
file-local-history = Istoric local…
file-clear-history = Golește istoricul local…
history-title = Istoricul local al { $name }
history-open = Deschide
history-restore = Restaurează
history-empty = Încă nu există versiuni salvate. La fiecare salvare se păstrează o copie.
history-unreadable = Această versiune nu a putut fi citită.
history-just-now = Chiar acum
history-minutes-ago = { $count ->
    [one] acum 1 minut
    [few] acum { $count } minute
   *[other] acum { $count } de minute
    }
history-hours-ago = { $count ->
    [one] acum 1 oră
    [few] acum { $count } ore
   *[other] acum { $count } de ore
    }
history-days-ago = { $count ->
    [one] acum 1 zi
    [few] acum { $count } zile
   *[other] acum { $count } de zile
    }
history-purge-title = Ștergeți tot istoricul local?
history-purge-body = Copiile salvate ale tuturor fișierelor sunt eliminate de pe acest computer. Fișierele în sine nu sunt modificate.
history-purge-confirm = Șterge istoricul
history-purged = Istoricul local a fost șters.
settings-local-history = Păstrează istoricul local al fișierelor salvate
settings-local-history-detail = Păstrează până la 50 de versiuni per fișier timp de 30 de zile, doar pe acest computer.
file-open-log = Deschide în vizualizatorul de jurnale…
log-lines = { $count ->
    [one] 1 linie
    [few] { $count } linii
   *[other] { $count } de linii
    }
log-indexing = Se citește… { $percent } %
log-matches = { $count ->
    [one] 1 linie potrivită
    [few] { $count } linii potrivite
   *[other] { $count } de linii potrivite
    }
log-filter-hint = Filtrează liniile
log-bad-pattern = Model nevalid
log-follow = Urmărește
log-close = Închide vizualizatorul
log-opened-large = { $name } este mare, așa că a fost deschis doar în citire în vizualizatorul de jurnale.
tab-pin = Fixează fila
tab-unpin = Anulează fixarea filei
edit-add-cursor-above = Adaugă cursor deasupra
edit-add-cursor-below = Adaugă cursor dedesubt
view-word-wrap = Încadrare cuvinte
status-wrap-on = Rândurile lungi se încadrează acum în lățimea ferestrei.
status-wrap-off = Rândurile lungi nu se mai încadrează.
settings-word-wrap = Încadrează rândurile lungi
edit-go-to-line = Salt la linia…
edit-go-to-heading = Salt la titlu…
goto-line-hint = Linie sau linie:coloană
goto-line-note = { $count ->
    [one] Documentul are 1 linie.
    [few] Documentul are { $count } linii.
   *[other] Documentul are { $count } de linii.
    }
goto-line-invalid = Introdu un număr de linie, urmat opțional de :coloană.
goto-heading-hint = Tastează pentru a filtra titlurile
goto-heading-none = Acest document nu are titluri.
format-xml = XML
format-yaml = YAML
tools-tree-error = { $format } nevalid la linia { $line }, coloana { $column }: { $reason }
edit-select-next-occurrence = Selectează următoarea apariție
tools-record-macro = Pornește sau oprește înregistrarea macro
tools-play-macro = Redă macro
macro-recording = Se înregistrează macro
macro-recorded = { $count ->
    [one] Macro înregistrat: 1 pas.
    [few] Macro înregistrat: { $count } pași.
   *[other] Macro înregistrat: { $count } de pași.
    }
tools-compare-saved = Compară cu versiunea salvată
tools-compare-clipboard = Compară cu clipboardul
compare-saved-label = { $name } (pe disc)
compare-clipboard-label = Clipboard
compare-saved-failed = Fișierul salvat nu a putut fi citit: { $reason }
compare-clipboard-empty = Clipboardul nu conține text de comparat.
error-save-disk-full = { $name } nu a fost salvat: discul este plin. Fișierul de pe disc nu s-a modificat, iar textul dvs. este încă aici. Eliberați spațiu sau folosiți Salvează ca pentru a-l salva în altă parte.
error-save-permission = { $name } nu a fost salvat: nu aveți permisiunea de a scrie în acea locație. Fișierul de pe disc nu s-a modificat. Folosiți Salvează ca pentru a-l salva în altă parte.
error-save-read-only-volume = { $name } nu a fost salvat: discul este doar în citire. Fișierul de pe disc nu s-a modificat. Folosiți Salvează ca pentru a-l salva în altă parte.
replace-preview-title = { $count ->
    [one] Se înlocuiește 1 potrivire?
    [few] Se înlocuiesc { $count } potriviri?
   *[other] Se înlocuiesc { $count } de potriviri?
    }
replace-preview-confirm = { $count ->
    [one] Înlocuiește 1
    [few] Înlocuiește { $count }
   *[other] Înlocuiește { $count }
    }
replace-preview-more = Sunt afișate doar primele 200 de rânduri modificate.
replace-preview-stale = Documentul s-a modificat; nu s-a înlocuit nimic. Rulați din nou Înlocuiește tot.
encoding-detected = Deschis ca { $encoding } (detectat din conținutul fișierului).
encoding-suggested = Sugerată pe baza conținutului fișierului: { $encoding }. Verificați previzualizarea înainte de deschidere.
app-about = Despre Sanain
menu-help = Ajutor
about-tagline = Puterea cuvintelor.
about-version = Versiunea { $version }
about-license = Liber și open source sub licența Mozilla Public License 2.0.
about-ai = Construit de AI. Condus de oameni. Susținut de comunitate.
about-notices = Componentele terțe și licențele lor sunt enumerate în THIRD_PARTY_NOTICES, inclus în fiecare versiune.
about-source = Cod sursă și probleme pe GitHub
tools-xml-validate = Validează XML
tools-yaml-validate = Validează YAML
tools-markup-valid = { $format } este valid
file-open-recent = Deschide recente…
file-save-all = Salvează tot
file-revert = Revino la versiunea salvată
recent-hint = Tastează pentru a filtra fișierele recente
recent-none = Încă nu există fișiere recente.
revert-title = Renunți la modificările din { $name }?
revert-body = Fișierul este reîncărcat de pe disc. Modificările nesalvate se pierd și nu pot fi anulate.
revert-confirm = Renunță și reîncarcă
revert-done = { $name } a fost reîncărcat de pe disc.
status-saved-all = { $count ->
    [0] Nimic de salvat.
    [one] A fost salvat 1 fișier.
    [few] Au fost salvate { $count } fișiere.
   *[other] Au fost salvate { $count } de fișiere.
    }
edit-toggle-comment = Comută comentariul de linie
edit-indent = Indentează liniile
edit-outdent = Anulează indentarea liniilor
comment-unsupported = Acest tip de fișier nu are comentarii de linie.
tab-close-others = Închide celelalte file
tab-close-all = Închide toate filele
file-copy-path = Copiază calea fișierului
file-reveal-macos = Afișează în Finder
file-reveal-windows = Afișează în Explorer
file-reveal-other = Afișează în managerul de fișiere
status-path-copied = Calea fișierului a fost copiată.
tabs-kept-unsaved = { $count ->
    [one] 1 filă cu modificări nesalvate a rămas deschisă.
    [few] { $count } file cu modificări nesalvate au rămas deschise.
   *[other] { $count } de file cu modificări nesalvate au rămas deschise.
    }
status-indent-tabs = Tabulatoare
status-indent-spaces = Spații: { $count }
find-searching = Se caută…
compare-ignore-whitespace = Ignoră spațiile albe
tools-compare-file = Compară cu un fișier…
compare-file-failed = Nu s-a putut compara cu { $name }: { $reason }
cli-help =
    Utilizare:
      sanain [FIȘIER…]                deschide fișiere
      sanain --diff STÂNGA DREAPTA    compară două fișiere unul lângă altul
      sanain --version                afișează versiunea
      sanain --help                   afișează acest ajutor
    Folosiți -- înaintea numelor de fișiere care încep cu o cratimă.
cli-unknown-option = Opțiune necunoscută { $option }.
cli-diff-needs-two = --diff necesită două fișiere: sanain --diff STÂNGA DREAPTA
window-move-tab-left = Mută fila la stânga
window-move-tab-right = Mută fila la dreapta
edit-skip-occurrence = Sari la următoarea apariție
tools-play-macro-times = Redă macro de mai multe ori…
macro-times-hint = Număr de repetări
macro-times-note = De la 1 la { $max }
macro-times-invalid = Introdu un număr de la 1 la { $max }.
view-outline = Schiță
outline-empty = Nu există titluri în acest document.
outline-close = Închide schița
folder-replace-placeholder = Înlocuiește cu
folder-replace-run = Înlocuiește în fișiere…
replace-files-title = Înlocuiți { $count ->
    [one] 1 potrivire
    [few] { $count } potriviri
   *[other] { $count } de potriviri
    } în { $files ->
    [one] 1 fișier
    [few] { $files } fișiere
   *[other] { $files } de fișiere
    }?
replace-files-body = Fișierele deschise într-o filă sunt modificate acolo, iar modificarea poate fi anulată; salvați-le dvs. Celelalte fișiere sunt salvate acum, iar conținutul lor anterior este păstrat în istoricul local, dacă este activat. Un fișier care s-a modificat între timp pe disc este omis.
replace-files-confirm = Înlocuiește
replace-files-done = { $count ->
    [one] A fost înlocuită 1 potrivire
    [few] Au fost înlocuite { $count } potriviri
   *[other] Au fost înlocuite { $count } de potriviri
    } în { $files ->
    [one] 1 fișier
    [few] { $files } fișiere
   *[other] { $files } de fișiere
    } (deschise în file: { $open }, nesalvate).
replace-files-failed = { $failed ->
    [one] 1 fișier nu a fost modificat:
    [few] { $failed } fișiere nu au fost modificate; primul:
   *[other] { $failed } de fișiere nu au fost modificate; primul:
    } { $first }
image-outside = Această imagine se află în afara dosarului documentului.
image-allow = Afișează imaginile din acest dosar
cli-help-integration =
    Integrare cu managerul de fișiere (doar pentru utilizatorul curent):
      sanain --register           adaugă Sanain la „Deschide cu” (Linux, Windows)
      sanain --unregister         îl elimină din nou
cli-registered = Sanain a fost adăugat la „Deschide cu”. { $place }
cli-unregistered = Sanain a fost eliminat din „Deschide cu”.
cli-register-failed = Integrarea cu managerul de fișiere nu a putut fi modificată: { $reason }
cli-register-macos = Pe macOS nu trebuie înregistrat nimic: odată ce Sanain.app se află în dosarul Aplicații, Finder îl oferă în „Deschide cu”.
settings-tab-general = General
settings-tab-editor = Editor
settings-tab-appearance = Aspect
settings-shortcut-search = Caută comenzi sau taste
settings-font = Font
settings-font-default = Monospațiat integrat
settings-font-loading = Se caută fonturile instalate…
settings-font-missing = Fontul { $font } nu a fost găsit; se folosește fontul integrat.
settings-theme-more = Mai multe teme
settings-theme-user-note = Teme proprii: fișiere .theme în dosarul { $folder } (vezi docs/THEMES.md).
language-auto = Detectează automat
language-choose = Alegeți limbajul acestei file
tools-xml-format = Formatează XML
tools-save-macro = Salvează macro…
tools-play-saved-macro = Redă macro salvat…
tools-delete-saved-macro = Șterge macro salvat…
macro-save-hint = Numele macro-ului
macro-save-note = Un macro salvat cu același nume este înlocuit.
macro-save-empty = Nimic de salvat: înregistrează mai întâi un macro.
macro-saved = Macro salvat: { $name }
macro-deleted = Macro șters: { $name }
macro-name-invalid = Introdu un nume de 1 până la { $max } caractere pe un singur rând.
macro-save-too-large = Acest macro este prea lung pentru a fi salvat.
macro-save-full = Se pot salva cel mult { $max } macro-uri. Șterge mai întâi unul.
macro-save-failed = Macro-urile nu au putut fi salvate: { $error }
macro-saved-hint = Tastează pentru a filtra macro-urile salvate
macro-saved-none = Încă nu există macro-uri salvate.
settings-shortcut-import-npp = Importă din Notepad++…
settings-shortcut-import-npp-hint = Alegeți un fișier shortcuts.xml din Notepad++. Pe Windows se află în %AppData%\Notepad++\shortcuts.xml.
settings-shortcut-import-npp-done = Importate din Notepad++: { $imported }. Omise: { $skipped }. Conflicte rămase: { $conflicts }.
settings-shortcut-import-npp-failed = Nu s-a putut importa { $name }: { $reason }
settings-shortcut-import-npp-too-large = Fișierul depășește 1 MiB.
settings-shortcut-import-npp-invalid = Fișierul nu este un shortcuts.xml valid din Notepad++.
view-split-right = Împarte la dreapta
view-split-down = Împarte în jos
view-close-pane = Închide panoul
view-focus-next-pane = Panoul următor
view-focus-prev-pane = Panoul anterior
view-focus-next-region = Zona următoare
view-focus-prev-region = Zona anterioară
status-pane-limit = S-a atins limita de panouri (cel mult { $max }).
a11y-csv-grid = Tabel CSV
a11y-grid-cell = Rândul { $row }, coloana { $column }: { $name }
a11y-grid-cell-plain = Rândul { $row }, coloana { $column }
a11y-grid-column-header = Antetul coloanei { $column }
a11y-grid-row-header = Rândul { $row }
a11y-edit-field = Editare: { $item }
a11y-json-tree = Arbore JSON
a11y-xml-tree = Arbore XML
a11y-yaml-tree = Arbore YAML
a11y-tree-item = Nivelul { $level }
a11y-tree-item-expanded = Nivelul { $level }, extins
a11y-tree-item-collapsed = Nivelul { $level }, restrâns
a11y-tree-object = Obiect, intrări: { $count }
a11y-tree-list = Listă, elemente: { $count }
a11y-tree-element = Elementul { $name }
a11y-tree-attribute = Atributul { $name }
a11y-tree-text = Text
a11y-pane = Panoul { $pane } din { $count }: { $name }
a11y-pane-active = Panoul { $pane } din { $count }: { $name }, activ
a11y-pane-editor = Editor de text, panoul { $pane } din { $count }: { $name }
a11y-mermaid-flowchart = Diagramă de flux. Noduri: { $nodes }, legături: { $links }.
a11y-mermaid-sequence = Diagramă de secvență. Participanți: { $participants }, mesaje: { $messages }.
a11y-mermaid-state = Diagramă de stări. Stări: { $states }, tranziții: { $transitions }.
a11y-mermaid-class = Diagramă de clase. Clase: { $classes }, relații: { $relations }.
a11y-shortcut-change = Schimbă scurtătura pentru { $command }
a11y-shortcut-change-current = Schimbă scurtătura pentru { $command }, acum { $keys }
a11y-shortcut-reset = Resetează scurtătura pentru { $command }
status-line-ending-choose = Schimbă sfârșiturile de linie ale fișierului
status-encoding-choose = Redeschide sau salvează cu altă codificare
status-indent-choose = Setări de indentare
a11y-settings-sections = Secțiuni de setări
a11y-tab-strip = Fișiere deschise
a11y-tab-unsaved = { $name }, modificări nesalvate
