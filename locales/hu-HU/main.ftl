# Machine translation by Claude (AI); review by a native speaker required before release.
app-name = Text³
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
    [yes] ● { $name } — Text³
   *[no] { $name } — Text³
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
app-quit = Kilépés a Text³ programból
app-hide = Text³ elrejtése
app-hide-others = Többi elrejtése
app-show-all = Összes megjelenítése
window-minimize = Kis méret
window-zoom = Nagyítás
status-recovered = Az előző munkamenet nem mentett módosításai visszaállítva
status-recovery-failed = Nem sikerült helyreállítási másolatot menteni: { $reason }
status-hot-exit-failed = A nem mentett módosításokat nem sikerült megőrizni a következő indításig ({ $reason }), ezért rákérdez.
status-hot-exit-too-large = { $name } túl nagy ahhoz, hogy a következő indításig megőrizze, ezért rákérdez.
status-restored-changed = { $name } megváltozott a lemezen, miután a nem mentett módosításait megőrizte. A szöveg a lapon van; a fájlhoz nem nyúlt.
status-restored-missing = { $name } már nincs a lemezen. A nem mentett szövege a lapon maradt; mentéskor a fájl újra létrejön.
edit-duplicate-line = Sor megkettőzése
edit-delete-line = Sor törlése
edit-move-line-up = Sor mozgatása fel
edit-move-line-down = Sor mozgatása le
file-close-tab = Lap bezárása
file-reopen-tab = Bezárt lap újranyitása
window-next-tab = Következő lap
window-prev-tab = Előző lap
tab-close = Lap bezárása
tab-missing-on-disk = Nem található a lemezen
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
settings-hot-exit = Nem mentett módosítások megőrzése kilépéskor
settings-hot-exit-detail = Kilépés anélkül, hogy minden nem mentett lapra rákérdezne. A nem mentett és a névtelen lapok a következő indításkor a szövegükkel együtt visszatérnek, akkor is, ha az előző munkamenet fájljainak újranyitása ki van kapcsolva. Egyetlen lap bezárásakor továbbra is rákérdez.
settings-language = Nyelv
settings-language-system = Rendszerbeállítás szerint
settings-language-note = A nyelv módosítása a Text³ újraindítása után lép életbe.
settings-save-failed = Nem sikerült menteni a beállításokat: { $reason }
view-command-palette = Parancspaletta…
palette-placeholder = Írjon be egy parancsot
view-markdown-source = Markdown forrás
view-markdown-split = Markdown forrás és előnézet
view-markdown-preview = Markdown előnézet
preview-too-large = Ez a dokumentum túl nagy az előnézethez.
preview-mermaid-unsupported-type = A diagramtípus nem támogatott; a forrás látható.
preview-mermaid-unsupported-syntax = A diagram olyan szintaxist használ, amely nem rajzolható meg; a forrás látható.
preview-mermaid-too-large = A diagram túl nagy a megrajzoláshoz; a forrás látható.
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
view-structured = Strukturált nézet (CSV, JSON, XML, YAML)
csv-delimiter = Elválasztó
csv-rows = { $count ->
    [one] 1 sor
   *[other] { $count } sor
    }
csv-unterminated = Lezáratlan idézőjel a(z) { $line }. sortól kezdve
csv-truncated = Az első 200 000 sor látható
csv-edit-cell = Cella szerkesztése
csv-insert-row-below = Sor beszúrása alá
csv-delete-row = Sor törlése
csv-read-only = Írásvédett fájl: a rács nem szerkeszthető
json-more = … még { $count }
json-edit-value = Érték szerkesztése
json-rename-key = Kulcs átnevezése
json-add-after = Hozzáadás utána
json-add-inside = Hozzáadás bele
json-delete-node = Törlés
json-read-only = Írásvédett fájl: a fa nem szerkeszthető
json-invalid-value = Nem érvényes JSON-érték
json-duplicate-key = Az objektumban már van ilyen nevű kulcs
xml-edit-value = Érték szerkesztése
xml-edit-text = Szöveg szerkesztése
xml-rename = Átnevezés
xml-add-attribute = Attribútum hozzáadása
xml-add-child = Gyermekelem hozzáadása
xml-add-after = Hozzáadás utána
xml-delete = Törlés
xml-invalid-name = Nem érvényes XML-név
xml-duplicate-attribute = Az elemnek már van ilyen nevű attribútuma
xml-not-well-formed = Ez a módosítás érvénytelenné tenné az XML-t
yaml-not-preserved = Ez a módosítás a YAML más részeit is megváltoztatná
yaml-key-too-long = Egy kulcs legfeljebb 1024 karakter hosszú lehet
yaml-read-only-flow = A flow stílus (zárójelek között írva) még nem szerkeszthető a fában
yaml-read-only-anchor = A horgonnyal (&) ellátott értékek nem szerkeszthetők a fában
yaml-read-only-tag = A címkével (!) ellátott értékek nem szerkeszthetők a fában
yaml-read-only-alias = Az aliasok (*) nem szerkeszthetők a fában
yaml-read-only-block-scalar = A szövegblokkok (| vagy >) még nem szerkeszthetők a fában
yaml-read-only-complex-key = Az összetett kulcsok nem szerkeszthetők a fában
yaml-read-only-document = A fában csak az első dokumentum szerkeszthető
yaml-read-only-layout = Ez a bejegyzés más tartalommal osztozik a sorain, ezért itt nem módosítható
yaml-read-only-only-child = Az egyetlen bejegyzés nem távolítható el
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
settings-keymap-default = Text³
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
app-check-updates = Frissítések keresése…
update-checking = Frissítések keresése…
update-up-to-date = A Text³ { $version } a legújabb verzió.
update-no-releases = A Text³ programból még nem jelent meg kiadás.
update-failed = Nem sikerült frissítéseket keresni: { $reason }
update-invalid = váratlan válasz
update-available-title = Elérhető a Text³ { $version }
update-available-body = A letöltéshez nyissa meg a letöltési oldalt. A Text³ magától semmit sem tölt le és nem telepít.
update-open-page = Letöltési oldal megnyitása
update-later = Később
settings-check-updates = Frissítések keresése naponta egyszer
settings-check-updates-detail = A legújabb kiadást kérdezi le a github.com-tól. Semmi mást nem küld.
file-local-history = Helyi előzmények…
file-clear-history = Helyi előzmények törlése…
history-title = { $name } helyi előzményei
history-open = Megnyitás
history-restore = Visszaállítás
history-empty = Még nincsenek mentett verziók. Minden mentéskor megmarad egy másolat.
history-unreadable = Ez a verzió nem olvasható.
history-just-now = Épp most
history-minutes-ago = { $count ->
    [one] 1 perce
   *[other] { $count } perce
    }
history-hours-ago = { $count ->
    [one] 1 órája
   *[other] { $count } órája
    }
history-days-ago = { $count ->
    [one] 1 napja
   *[other] { $count } napja
    }
history-purge-title = Törli a teljes helyi előzményt?
history-purge-body = Az összes fájl mentett másolatai törlődnek erről a számítógépről. Maguk a fájlok nem változnak.
history-purge-confirm = Előzmények törlése
history-purged = A helyi előzmények törölve.
settings-local-history = Mentett fájlok helyi előzményeinek megőrzése
settings-local-history-detail = Fájlonként legfeljebb 50 verziót őriz meg 30 napig, csak ezen a számítógépen.
file-open-log = Megnyitás naplómegjelenítőben…
log-lines = { $count ->
    [one] 1 sor
   *[other] { $count } sor
    }
log-indexing = Olvasás… { $percent }%
log-matches = { $count ->
    [one] 1 egyező sor
   *[other] { $count } egyező sor
    }
log-filter-hint = Sorok szűrése
log-bad-pattern = Érvénytelen minta
log-follow = Követés
log-close = Megjelenítő bezárása
log-opened-large = A(z) { $name } nagy, ezért csak olvasható módon nyílt meg a naplómegjelenítőben.
tab-pin = Lap rögzítése
tab-unpin = Lap rögzítésének feloldása
edit-add-cursor-above = Kurzor hozzáadása fölé
edit-add-cursor-below = Kurzor hozzáadása alá
view-word-wrap = Automatikus sortörés
status-wrap-on = A hosszú sorok mostantól az ablak szélességénél törnek.
status-wrap-off = A hosszú sorok már nem törnek.
settings-word-wrap = Hosszú sorok tördelése
edit-go-to-line = Ugrás sorra…
edit-go-to-heading = Ugrás címsorra…
goto-line-hint = Sor vagy sor:oszlop
goto-line-note = { $count ->
    [one] A dokumentum 1 sorból áll.
   *[other] A dokumentum { $count } sorból áll.
    }
goto-line-invalid = Adjon meg egy sorszámot, opcionálisan :oszlop utótaggal.
goto-heading-hint = Gépeljen a címsorok szűréséhez
goto-heading-none = Ebben a dokumentumban nincsenek címsorok.
edit-goto-anything = Ugrás bárhová…
goto-anything-hint = Fájlnév, vagy :sor, @szimbólum, #szöveg
goto-anything-line = Ugrás a(z) { $line }. sorra
goto-anything-open = megnyitva
goto-anything-scanning = Fájlok listázása: { $folder }…
goto-anything-scanned = Fájlok itt: { $folder }: { $count }
goto-anything-limit = Csak az első { $max } fájl látható innen: { $folder }.
goto-anything-no-folder = Mentse a fájlt, vagy használja a Keresés mappában funkciót egy mappa fájljainak listázásához.
goto-anything-symbols-none = Ebben a dokumentumban nincsenek szimbólumok.
goto-anything-matches-none = Ebben a dokumentumban nincs találat.
format-xml = XML
format-yaml = YAML
tools-tree-error = Érvénytelen { $format } a(z) { $line }. sor { $column }. oszlopában: { $reason }
edit-select-next-occurrence = Következő előfordulás kijelölése
tools-record-macro = Makrórögzítés indítása vagy leállítása
tools-play-macro = Makró lejátszása
macro-recording = Makró rögzítése
macro-recorded = { $count ->
    [one] Makró rögzítve: 1 lépés.
   *[other] Makró rögzítve: { $count } lépés.
    }
tools-compare-saved = Összehasonlítás a mentett verzióval
tools-compare-clipboard = Összehasonlítás a vágólappal
compare-saved-label = { $name } (lemezen)
compare-clipboard-label = Vágólap
compare-saved-failed = A mentett fájl nem olvasható: { $reason }
compare-clipboard-empty = A vágólapon nincs összehasonlítható szöveg.
error-save-disk-full = A(z) { $name } nem lett mentve: a lemez megtelt. A lemezen lévő fájl nem változott, és a szöveg továbbra is itt van. Szabadítson fel helyet, vagy a Mentés másként paranccsal mentse máshová.
error-save-permission = A(z) { $name } nem lett mentve: nincs engedélye arra, hogy ide írjon. A lemezen lévő fájl nem változott. A Mentés másként paranccsal mentse máshová.
error-save-read-only-volume = A(z) { $name } nem lett mentve: a lemez csak olvasható. A lemezen lévő fájl nem változott. A Mentés másként paranccsal mentse máshová.
replace-preview-title = { $count ->
    [one] Lecserél 1 találatot?
   *[other] Lecserél { $count } találatot?
    }
replace-preview-confirm = { $count ->
    [one] 1 cseréje
   *[other] { $count } cseréje
    }
replace-preview-more = Csak az első 200 módosuló sor látható.
replace-preview-stale = A dokumentum megváltozott, semmi sem lett lecserélve. Futtassa újra az Összes cseréje parancsot.
encoding-detected = Megnyitva { $encoding } kódolással (a fájl tartalmából felismerve).
encoding-suggested = A fájl tartalma alapján javasolt: { $encoding }. Megnyitás előtt ellenőrizze az előnézetet.
app-about = A Text³ névjegye
menu-help = Súgó
about-version = Verzió: { $version }
about-license = Szabad és nyílt forráskódú szoftver a Mozilla Public License 2.0 licenc alatt.
about-ai = MI segítségével (az Anthropic Claude modelljével) fejlesztve, a karbantartó irányításával.
about-notices = A külső komponensek és licenceik a THIRD_PARTY_NOTICES fájlban találhatók, amely minden kiadás része.
about-source = Forráskód és hibajegyek a GitHubon
tools-xml-validate = XML ellenőrzése
tools-yaml-validate = YAML ellenőrzése
tools-markup-valid = A { $format } érvényes
file-open-recent = Legutóbbi megnyitása…
file-save-all = Összes mentése
file-revert = Visszaállítás a mentett változatra
recent-hint = Gépeljen a legutóbbi fájlok szűréséhez
recent-none = Még nincsenek legutóbbi fájlok.
revert-title = Elveti a(z) { $name } módosításait?
revert-body = A fájl újra betöltődik a lemezről. A nem mentett módosítások elvesznek, és nem vonhatók vissza.
revert-confirm = Elvetés és újratöltés
revert-done = A(z) { $name } újra betöltve a lemezről.
status-saved-all = { $count ->
    [0] Nincs mit menteni.
    [one] 1 fájl mentve.
   *[other] { $count } fájl mentve.
    }
edit-toggle-comment = Sorkomment be/ki
edit-indent = Sorok behúzása
edit-outdent = Sorok behúzásának csökkentése
comment-unsupported = Ez a fájltípus nem támogatja a sorkommenteket.
tab-close-others = Többi lap bezárása
tab-close-all = Összes lap bezárása
file-copy-path = Fájl elérési útjának másolása
file-reveal-macos = Megjelenítés a Finderben
file-reveal-windows = Megjelenítés a Fájlkezelőben
file-reveal-other = Megjelenítés a fájlkezelőben
status-path-copied = A fájl elérési útja másolva.
tabs-kept-unsaved = { $count ->
    [one] 1 nem mentett módosításokat tartalmazó lap nyitva maradt.
   *[other] { $count } nem mentett módosításokat tartalmazó lap nyitva maradt.
    }
status-indent-tabs = Tabulátorok
status-indent-spaces = Szóközök: { $count }
find-searching = Keresés…
compare-ignore-whitespace = Szóközök figyelmen kívül hagyása
tools-compare-file = Összehasonlítás fájllal…
compare-file-failed = Nem sikerült összehasonlítani a(z) { $name } fájllal: { $reason }
cli-help =
    Használat:
      text3 [FÁJL…]             fájlok megnyitása
      text3 --diff BAL JOBB     két fájl összehasonlítása egymás mellett
      text3 --version           a verzió megjelenítése
      text3 --help              ennek a súgónak a megjelenítése
    Kötőjellel kezdődő fájlnevek előtt használja a -- jelet.
cli-unknown-option = Ismeretlen kapcsoló: { $option }.
cli-diff-needs-two = A --diff két fájlt igényel: text3 --diff BAL JOBB
window-move-tab-left = Lap mozgatása balra
window-move-tab-right = Lap mozgatása jobbra
edit-skip-occurrence = Ugrás a következő előfordulásra
tools-play-macro-times = Makró lejátszása többször…
macro-times-hint = Ismétlések száma
macro-times-note = 1 és { $max } között
macro-times-invalid = Adjon meg egy számot 1 és { $max } között.
view-outline = Vázlat
outline-empty = Ebben a dokumentumban nincsenek címsorok.
outline-close = Vázlat bezárása
folder-replace-placeholder = Csere erre
folder-replace-run = Csere a fájlokban…
replace-files-title = { $count ->
    [one] 1 találat
   *[other] { $count } találat
    } cseréje { $files ->
    [one] 1 fájlban
   *[other] { $files } fájlban
    }?
replace-files-body = A lapon megnyitott fájlok ott módosulnak, és a módosítás visszavonható; ezeket mentse saját maga. A többi fájl most mentésre kerül, korábbi tartalmuk pedig megmarad a helyi előzményekben, ha azok be vannak kapcsolva. Az a fájl, amely közben megváltozott a lemezen, kimarad.
replace-files-confirm = Csere
replace-files-done = { $count ->
    [one] 1 találat
   *[other] { $count } találat
    } lecserélve { $files ->
    [one] 1 fájlban
   *[other] { $files } fájlban
    } (lapon megnyitva: { $open }, nincs mentve).
replace-files-failed = { $failed ->
    [one] 1 fájl nem módosult:
   *[other] { $failed } fájl nem módosult; az első:
    } { $first }
image-outside = Ez a kép a dokumentum mappáján kívül található.
image-allow = Képek megjelenítése ebből a mappából
cli-help-integration =
    Integráció a fájlkezelővel (csak az aktuális felhasználónak):
      text3 --register            a Text³ hozzáadása a „Megnyitás ezzel” menühöz (Linux, Windows)
      text3 --unregister          eltávolítás onnan
      text3 --default-apps[=plain,markdown,code,data,web]
                                  a Text³ legyen ezeknek a fájloknak az alapértelmezett alkalmazása
cli-registered = A Text³ bekerült a „Megnyitás ezzel” menübe. { $place }
cli-unregistered = A Text³ kikerült a „Megnyitás ezzel” menüből.
cli-register-failed = A fájlkezelő-integrációt nem sikerült módosítani: { $reason }
cli-register-macos = macOS-en semmit sem kell regisztrálni: amint a Text³.app az Alkalmazások mappában van, a Finder felkínálja a „Megnyitás ezzel” menüben.
cli-default-apps-set = A Text³ mostantól ezeket a fájltípusokat nyitja meg: { $types }
cli-default-apps-unknown = Ismeretlen fájltípus-csoport: { $group }. Ismert csoportok: plain, markdown, code, data, web.
default-apps-title = Fájlok megnyitása a Text³ programmal
default-apps-body = A Text³ lehet az alkalmazás, amely dupla kattintásra megnyitja ezeket a fájlokat. Ezt később a Beállítások › Általános részben módosíthatod.
default-apps-body-windows = Windowson az alapértelmezett alkalmazásokat te választod ki: a Text³ megnyitja a Beállítások › Alapértelmezett alkalmazások oldalt, ahol minden fájltípushoz kiválaszthatod a Text³ programot.
default-apps-unavailable = A Text³ ezen példánya nem módosíthatja az alapértelmezett alkalmazásokat: hordozható példány, vagy macOS-en nem a Text³.app-ból indult.
default-apps-plain = Szöveg és naplók
default-apps-markdown = Markdown
default-apps-code = Forráskód és szkriptek
default-apps-data = Adatok és konfiguráció
default-apps-web = Weboldalak és stílusok
default-apps-is-default = már a Text³
default-apps-apply = A Text³ legyen az alapértelmezett
default-apps-open-settings = Windows Beállítások megnyitása
default-apps-not-now = Most nem
default-apps-close = Bezárás
default-apps-done = A Text³ mostantól a kiválasztott fájltípusokat nyitja meg.
default-apps-settings-opened = A Windows Beállítások meg van nyitva: válaszd ki a Text³ programot azokhoz a fájltípusokhoz, amelyeket meg kell nyitnia.
default-apps-failed = Az alapértelmezett alkalmazások nem módosíthatók: { $reason }
settings-default-apps = Alapértelmezett alkalmazások
settings-default-apps-button = Fájltípusok kiválasztása…
settings-tab-general = Általános
settings-tab-editor = Szerkesztő
settings-tab-appearance = Megjelenés
settings-shortcut-search = Parancsok vagy billentyűk keresése
settings-font = Betűtípus
settings-font-default = Beépített fix szélességű
settings-font-loading = Telepített betűtípusok keresése…
settings-font-missing = A(z) { $font } betűtípus nem található; a beépített betűtípus van használatban.
settings-theme-more = További témák
settings-theme-user-note = Saját témák: .theme fájlok ebben a mappában: { $folder } (lásd: docs/THEMES.md).
language-auto = Automatikus felismerés
language-choose = Válassza ki ennek a lapnak a nyelvét
tools-xml-format = XML formázása
tools-save-macro = Makró mentése…
tools-play-saved-macro = Mentett makró lejátszása…
tools-delete-saved-macro = Mentett makró törlése…
macro-save-hint = Makró neve
macro-save-note = Az azonos nevű mentett makró felülíródik.
macro-save-empty = Nincs mit menteni: előbb rögzítsen egy makrót.
macro-saved = Makró mentve: { $name }
macro-deleted = Makró törölve: { $name }
macro-name-invalid = Adjon meg egy 1–{ $max } karakteres nevet egy sorban.
macro-save-too-large = Ez a makró túl hosszú a mentéshez.
macro-save-full = Legfeljebb { $max } makró menthető. Előbb töröljön egyet.
macro-save-failed = A makrók mentése nem sikerült: { $error }
macro-saved-hint = Gépeljen a mentett makrók szűréséhez
macro-saved-none = Még nincsenek mentett makrók.
settings-shortcut-import-npp = Importálás a Notepad++-ból…
settings-shortcut-import-npp-hint = Válasszon egy Notepad++ shortcuts.xml fájlt. Windowson itt található: %AppData%\Notepad++\shortcuts.xml.
settings-shortcut-import-npp-done = Importálva a Notepad++-ból: { $imported }. Kihagyva: { $skipped }. Megmaradt ütközések: { $conflicts }.
settings-shortcut-import-npp-failed = A(z) { $name } importálása nem sikerült: { $reason }
settings-shortcut-import-npp-too-large = A fájl nagyobb 1 MiB-nál.
settings-shortcut-import-npp-invalid = A fájl nem érvényes Notepad++ shortcuts.xml.
view-split-right = Felosztás jobbra
view-split-down = Felosztás lefelé
view-close-pane = Panel bezárása
view-focus-next-pane = Következő panel
view-focus-prev-pane = Előző panel
view-focus-next-region = Következő terület
view-focus-prev-region = Előző terület
status-pane-limit = Elérte a panelek maximális számát (legfeljebb { $max }).
a11y-csv-grid = CSV-táblázat
a11y-grid-cell = { $row }. sor, { $column }. oszlop: { $name }
a11y-grid-cell-plain = { $row }. sor, { $column }. oszlop
a11y-grid-column-header = { $column }. oszlop fejléce
a11y-grid-row-header = { $row }. sor
a11y-edit-field = Szerkesztés: { $item }
a11y-json-tree = JSON-fa
a11y-xml-tree = XML-fa
a11y-yaml-tree = YAML-fa
a11y-tree-item = { $level }. szint
a11y-tree-item-expanded = { $level }. szint, kibontva
a11y-tree-item-collapsed = { $level }. szint, összecsukva
a11y-tree-object = Objektum, bejegyzések: { $count }
a11y-tree-list = Lista, elemek: { $count }
a11y-tree-element = Elem: { $name }
a11y-tree-attribute = Attribútum: { $name }
a11y-tree-text = Szöveg
a11y-pane = { $pane }. panel ({ $count } közül): { $name }
a11y-pane-active = { $pane }. panel ({ $count } közül): { $name }, aktív
a11y-pane-editor = Szövegszerkesztő, { $pane }. panel ({ $count } közül): { $name }
a11y-mermaid-flowchart = Folyamatábra. Csomópontok: { $nodes }, kapcsolatok: { $links }.
a11y-mermaid-sequence = Szekvenciadiagram. Résztvevők: { $participants }, üzenetek: { $messages }.
a11y-mermaid-state = Állapotdiagram. Állapotok: { $states }, átmenetek: { $transitions }.
a11y-mermaid-class = Osztálydiagram. Osztályok: { $classes }, kapcsolatok: { $relations }.
a11y-shortcut-change = Billentyűparancs módosítása: { $command }
a11y-shortcut-change-current = Billentyűparancs módosítása: { $command }, jelenleg { $keys }
a11y-shortcut-reset = Billentyűparancs visszaállítása: { $command }
status-line-ending-choose = A fájl sorvégjeleinek módosítása
status-encoding-choose = Újranyitás vagy mentés más kódolással
status-indent-choose = Behúzási beállítások
a11y-settings-sections = Beállítások szakaszai
a11y-tab-strip = Megnyitott fájlok
a11y-tab-unsaved = { $name }, nem mentett módosítások
migration-copied = A Sanain beállításai és adatai átmásolódtak a Text³ programba. A Sanain mappái változatlanok maradtak.

# Character and word count (status bar)
# How one thousand is written with digit grouping: the characters between
# 1 and 000 separate groups of three digits.
number-thousand = 1{ "\u00A0" }000
count-status = Karakterek: { $chars } · Szavak: { $words }
count-status-selection = Karakterek: { $chars } / { $total-chars } · Szavak: { $words } / { $total-words }
count-choose = Karakterek, szavak, sorok és bájtok száma
count-a11y-pending = Karakterek és szavak: még nincsenek megszámolva. Nyissa meg a számoláshoz.
count-heading-selection = Kijelölés
count-heading-document = Dokumentum
count-characters = Karakterek
count-characters-no-spaces = Karakterek (szóközök nélkül)
count-words = Szavak
count-lines = Sorok
count-bytes = Bájtok ({ $encoding })
count-unencodable = Néhány karakter nem menthető { $encoding } kódolással.
count-counting = Számolás…
settings-show-counts = Karakter- és szószám megjelenítése az állapotsorban
