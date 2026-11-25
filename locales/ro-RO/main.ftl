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
preview-mermaid-unsupported = Diagramele Mermaid nu sunt încă randate; se afișează sursa.
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
view-structured = Vizualizare structurată (CSV, JSON)
csv-delimiter = Delimitator
csv-rows = { $count ->
    [one] 1 rând
    [few] { $count } rânduri
   *[other] { $count } de rânduri
    }
csv-unterminated = Ghilimele neînchise începând de la linia { $line }
csv-truncated = Se afișează primele 200.000 de rânduri
json-more = … încă { $count }
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
