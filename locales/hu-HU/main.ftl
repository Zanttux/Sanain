# Machine translation by Claude (AI); review by a native speaker required before release.
app-name = Sanain
menu-file = Fájl
file-open = Megnyitás…
file-save = Mentés
status-tabs = { $count ->
    [one] Egy lap
   *[other] { $count } lap
    }
error-open = Nem sikerült megnyitni: { $name }.
menu-edit = Szerkesztés
file-new = Új lap
file-save-as = Mentés másként…
edit-undo = Visszavonás
edit-redo = Ismétlés
edit-select-all = Összes kijelölése
untitled = Névtelen
window-title = { $dirty ->
    [yes] ● { $name } — Sanain
   *[no] { $name } — Sanain
    }
status-position = Sor { $line }, oszlop { $column }
status-selections = { $count ->
    [one] 1 kijelölés
   *[other] { $count } kijelölés
    }
status-saved = Mentve: { $name }
encoding-utf8 = UTF-8
encoding-utf8-bom = UTF-8 BOM-mal
encoding-utf16le = UTF-16 LE
encoding-utf16be = UTF-16 BE
line-ending-lf = LF
line-ending-crlf = CRLF
line-ending-cr = CR
line-ending-mixed = Vegyes sorvégek
editor-accessible-name = Szövegszerkesztő
button-save = Mentés
button-dont-save = Nincs mentés
button-cancel = Mégse
button-ok = OK
button-overwrite = Felülírás
button-save-as = Mentés másként…
dialog-unsaved-title = Menti a(z) { $name } módosításait?
dialog-unsaved-body = Ha nem menti, a módosítások elvesznek.
dialog-conflict-title = A(z) { $name } megváltozott a lemezen
dialog-conflict-body = Egy másik program módosította ezt a fájlt, miután megnyitotta. A felülírás ezeket a módosításokat lecseréli.
dialog-readonly-title = A(z) { $name } csak olvasható
dialog-readonly-body = Mentsen inkább egy másolatot más néven.
error-open-encoding = A(z) { $name } nem UTF-8 vagy UTF-16 szöveg. Nem lett megnyitva, így a tartalma változatlan. A kódolás kiválasztásához használja a Fájl > Újranyitás kódolással menüpontot.
error-save = Nem sikerült menteni: { $name }: { $reason }
menu-window = Ablak
edit-cut = Kivágás
edit-copy = Másolás
edit-paste = Beillesztés
app-quit = Kilépés a Sanainból
app-hide = Sanain elrejtése
app-hide-others = Többi elrejtése
app-show-all = Összes megjelenítése
window-minimize = Kis méret
window-zoom = Nagyítás
status-recovered = Az előző munkamenet nem mentett módosításai visszaállítva
status-recovery-failed = Nem sikerült helyreállítási másolatot menteni: { $reason }
edit-duplicate-line = Sor megkettőzése
edit-delete-line = Sor törlése
edit-move-line-up = Sor mozgatása fel
edit-move-line-down = Sor mozgatása le
file-close-tab = Lap bezárása
file-reopen-tab = Bezárt lap újranyitása
window-next-tab = Következő lap
window-prev-tab = Előző lap
tab-close = Lap bezárása
edit-find = Keresés…
edit-find-next = Következő keresése
edit-find-prev = Előző keresése
edit-replace = Csere…
find-placeholder = Keresés
replace-placeholder = Csere erre
find-match-case = Kis- és nagybetűk
find-whole-word = Teljes szó
find-regex = Reguláris kifejezés
find-no-results = Nincs találat
find-count = { $total ->
    [one] 1 találat
   *[other] { $total } találat
    }
find-count-current = { $current } / { $total }
find-prev = Előző találat (Shift+Enter)
find-next = Következő találat (Enter)
find-close = Bezárás (Esc)
replace-one = Csere
replace-all = Összes cseréje
status-replaced = { $count ->
    [one] 1 találat lecserélve
   *[other] { $count } találat lecserélve
    }
language-plain = Egyszerű szöveg
menu-view = Nézet
app-settings = Beállítások…
view-zoom-in = Nagyítás
view-zoom-out = Kicsinyítés
view-zoom-reset = Eredeti méret
settings-title = Beállítások
settings-font-size = Betűméret
settings-tab-width = Tabulátor szélessége
settings-insert-spaces = Szóközök beszúrása a Tab lenyomásakor
settings-theme = Megjelenés
settings-theme-system = Rendszerbeállítás szerint
settings-theme-light = Világos
settings-theme-dark = Sötét
settings-restore-session = Az előző munkamenet fájljainak újranyitása
settings-language = Nyelv
settings-language-system = Rendszerbeállítás szerint
settings-language-note = A nyelv módosítása a Sanain újraindítása után lép életbe.
settings-save-failed = Nem sikerült menteni a beállításokat: { $reason }
view-command-palette = Parancspaletta…
palette-placeholder = Írjon be egy parancsot
view-markdown-source = Markdown forrás
view-markdown-split = Markdown forrás és előnézet
view-markdown-preview = Markdown előnézet
preview-too-large = Ez a dokumentum túl nagy az előnézethez.
preview-mermaid-unsupported = A Mermaid-diagramok még nem jelennek meg; a forrás látható.
menu-tools = Eszközök
tools-json-validate = JSON ellenőrzése
tools-json-format = JSON formázása
tools-json-minify = JSON tömörítése
tools-sort-lines = Sorok rendezése
tools-sort-lines-desc = Sorok rendezése csökkenő sorrendben
tools-unique-lines = Ismétlődő sorok eltávolítása
tools-trim-trailing = Sorvégi szóközök eltávolítása
tools-upper-case = Nagybetűs
tools-lower-case = Kisbetűs
tools-to-lf = Sorvégek átalakítása LF-re
tools-to-crlf = Sorvégek átalakítása CRLF-re
tools-json-valid = A JSON érvényes
tools-done = Kész. A Visszavonás visszaállítja.
tools-no-change = Nincs mit módosítani
tools-json-error = Érvénytelen JSON a(z) { $line }. sor { $column }. oszlopában: { $reason }
json-reason-unterminated-string = lezáratlan karakterlánc
json-reason-invalid-u-escape = érvénytelen \u escape-szekvencia
json-reason-invalid-escape = érvénytelen escape-szekvencia
json-reason-control-character-in-string = vezérlőkarakter a karakterláncban
json-reason-invalid-number = érvénytelen szám
json-reason-unexpected-character = váratlan karakter
json-reason-unexpected-content-after-the-value = váratlan tartalom az érték után
json-reason-unexpected-comma = váratlan vessző
json-reason-expected-a-string-key = karakterlánc kulcs várható
json-reason-expected = ':' várható
json-reason-expected-or-a-closing-bracket = ',' vagy záró zárójel várható
json-reason-expected-a-value = érték várható
json-reason-empty-document = üres dokumentum
json-reason-unexpected-end-of-input = a bemenet váratlanul véget ért
json-reason-too-deeply-nested-to-format = túl mélyen egymásba ágyazott a formázáshoz
tools-compare = Összehasonlítás ezzel…
compare-need-two = Nyisson meg egy másik lapot az összehasonlításhoz.
compare-pick-title = { $name } összehasonlítása ezzel:
compare-count = { $count ->
    [0] Nincs eltérés
    [one] 1 módosítás
   *[other] { $count } módosítás
    }
compare-close = Összehasonlítás bezárása
compare-to-left = Módosítás másolása balra
compare-to-right = Módosítás másolása jobbra
compare-stale = Egy dokumentum megváltozott az összehasonlítás közben. Az összehasonlítás frissült; próbálja újra.
view-structured = Strukturált nézet (CSV, JSON)
csv-delimiter = Elválasztó
csv-rows = { $count ->
    [one] 1 sor
   *[other] { $count } sor
    }
csv-unterminated = Lezáratlan idézőjel a(z) { $line }. sortól kezdve
csv-truncated = Az első 200 000 sor látható
json-more = … még { $count }
dialog-open-link-title = Megnyitja ezt a hivatkozást a webböngészőben?
button-open = Megnyitás
link-blocked = Biztonsági okokból az ilyen típusú hivatkozások nem nyílnak meg.
link-not-found = A hivatkozott fájl nem található: { $target }
enc-windows-1252 = Nyugat-európai
enc-iso-8859-15 = Nyugat-európai
enc-windows-1250 = Közép-európai
enc-iso-8859-2 = Közép-európai
enc-windows-1251 = Cirill
enc-koi8-r = Cirill
enc-windows-1253 = Görög
enc-windows-1254 = Török
enc-windows-1255 = Héber
enc-windows-1256 = Arab
enc-windows-1257 = Balti
enc-windows-874 = Thai
enc-shift_jis = Japán
enc-euc-jp = Japán
enc-gb18030 = Kínai, egyszerűsített
enc-big5 = Kínai, hagyományos
enc-euc-kr = Koreai
file-reopen-encoding = Újranyitás kódolással…
file-save-encoding = Mentés kódolással…
encoding-reopen-title = Újranyitás kódolással
encoding-save-title = Mentés kódolással
encoding-not-exact = Ez a kódolás nem tudja módosítás nélkül megnyitni a fájlt: { $reason }
encoding-unencodable = A(z) „{ $character }” nem menthető { $encoding } kódolással. Válasszon másik kódolást, vagy módosítsa a szöveget; semmi sem lett kiírva.
encoding-save-first = Mentse vagy vonja vissza a módosításokat, mielőtt más kódolással nyitja meg újra.
image-remote = A távoli képek nem töltődnek be.
image-blocked = Csak a dokumentum mappájában lévő képek jelennek meg.
image-unsupported = Nem támogatott képformátum.
image-too-large = A kép túl nagy a megjelenítéshez.
image-unreadable = A kép nem olvasható.
settings-theme-high-contrast = Nagy kontraszt
view-fold = Összecsukás
view-unfold = Kibontás
view-unfold-all = Összes kibontása
settings-shortcuts = Billentyűparancsok
settings-keymap-preset = Előbeállítás
settings-keymap-default = Sanain
settings-keymap-notepadpp = Notepad++
settings-shortcut-change = Módosítás
settings-shortcut-reset = Visszaállítás
settings-shortcut-press = Nyomja le a billentyűparancsot… (Esc: mégse)
settings-shortcut-moved = Billentyűparancs áthelyezve innen:
edit-find-in-folder = Keresés mappában…
folder-search-placeholder = Keresés a mappában
folder-search-run = Keresés
folder-search-stop = Leállítás
folder-search-no-folder = Mappa kiválasztása
folder-search-summary = { $count ->
    [one] 1 találat
   *[other] { $count } találat
    } { $files ->
    [one] 1 fájlban
   *[other] { $files } fájlban
    }
