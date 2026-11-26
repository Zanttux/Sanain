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
view-structured = Strukturált nézet (CSV, JSON, XML, YAML)
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
app-check-updates = Frissítések keresése…
update-checking = Frissítések keresése…
update-up-to-date = A Sanain { $version } a legújabb verzió.
update-no-releases = A Sanainból még nem jelent meg kiadás.
update-failed = Nem sikerült frissítéseket keresni: { $reason }
update-invalid = váratlan válasz
update-available-title = Elérhető a Sanain { $version }
update-available-body = A letöltéshez nyissa meg a letöltési oldalt. A Sanain magától semmit sem tölt le és nem telepít.
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
view-split-editor = Szerkesztő felosztása
encoding-detected = Megnyitva { $encoding } kódolással (a fájl tartalmából felismerve).
encoding-suggested = A fájl tartalma alapján javasolt: { $encoding }. Megnyitás előtt ellenőrizze az előnézetet.
app-about = A Sanain névjegye
menu-help = Súgó
about-tagline = A szavak ereje.
about-version = Verzió: { $version }
about-license = Szabad és nyílt forráskódú szoftver a Mozilla Public License 2.0 licenc alatt.
about-ai = MI építette. Ember vezeti. A közösség formálja.
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
      sanain [FÁJL…]            fájlok megnyitása
      sanain --diff BAL JOBB    két fájl összehasonlítása egymás mellett
      sanain --version          a verzió megjelenítése
      sanain --help             ennek a súgónak a megjelenítése
    Kötőjellel kezdődő fájlnevek előtt használja a -- jelet.
cli-unknown-option = Ismeretlen kapcsoló: { $option }.
cli-diff-needs-two = A --diff két fájlt igényel: sanain --diff BAL JOBB
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
