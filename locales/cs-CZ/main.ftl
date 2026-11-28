# Machine translation by Claude (AI); review by a native speaker required before release.
app-name = Text³
menu-file = Soubor
file-open = Otevřít…
file-save = Uložit
status-tabs = { $count ->
    [one] Jedna karta
    [few] { $count } karty
    [many] { $count } karty
   *[other] { $count } karet
    }
error-open = Soubor { $name } nelze otevřít.
menu-edit = Úpravy
file-new = Nová karta
file-save-as = Uložit jako…
edit-undo = Zpět
edit-redo = Znovu
edit-select-all = Vybrat vše
untitled = Bez názvu
window-title = { $dirty ->
    [yes] ● { $name } — Text³
   *[no] { $name } — Text³
    }
status-position = Ř. { $line }, sl. { $column }
status-selections = { $count ->
    [one] 1 výběr
    [few] { $count } výběry
    [many] { $count } výběru
   *[other] { $count } výběrů
    }
status-saved = Soubor { $name } uložen
encoding-utf8 = UTF-8
encoding-utf8-bom = UTF-8 s BOM
encoding-utf16le = UTF-16 LE
encoding-utf16be = UTF-16 BE
line-ending-lf = LF
line-ending-crlf = CRLF
line-ending-cr = CR
line-ending-mixed = Smíšené konce řádků
editor-accessible-name = Textový editor
button-save = Uložit
button-dont-save = Neukládat
button-cancel = Zrušit
button-ok = OK
button-overwrite = Přepsat
button-save-as = Uložit jako…
dialog-unsaved-title = Uložit změny v souboru { $name }?
dialog-unsaved-body = Pokud změny neuložíte, budou ztraceny.
dialog-conflict-title = Soubor { $name } byl na disku změněn
dialog-conflict-body = Jiný program tento soubor upravil poté, co jste ho otevřeli. Přepsáním tyto změny nahradíte.
dialog-readonly-title = Soubor { $name } je jen pro čtení
dialog-readonly-body = Uložte místo toho kopii pod jiným názvem.
error-open-encoding = Soubor { $name } neobsahuje text v UTF-8 ani UTF-16. Nebyl otevřen, takže jeho obsah zůstal beze změny. Kódování zvolíte pomocí Soubor > Znovu otevřít s kódováním.
error-save = Soubor { $name } nelze uložit: { $reason }
menu-window = Okno
edit-cut = Vyjmout
edit-copy = Kopírovat
edit-paste = Vložit
app-quit = Ukončit Text³
app-hide = Skrýt Text³
app-hide-others = Skrýt ostatní
app-show-all = Zobrazit vše
window-minimize = Minimalizovat
window-zoom = Zvětšit
status-recovered = Neuložené změny z minulé relace byly obnoveny
status-recovery-failed = Nelze uložit záložní kopii: { $reason }
status-hot-exit-failed = Neuložené změny se nepodařilo uchovat pro další spuštění ({ $reason }); aplikace se místo toho zeptá.
status-hot-exit-too-large = { $name } je příliš velký na uchování pro další spuštění; aplikace se místo toho zeptá.
status-restored-changed = { $name } se na disku změnil poté, co byly uchovány vaše neuložené změny. Váš text je v kartě; souboru se nic nestalo.
status-restored-missing = { $name } už na disku není. Jeho neuložený text zůstává v kartě; uložením se soubor vytvoří znovu.
edit-duplicate-line = Duplikovat řádek
edit-delete-line = Odstranit řádek
edit-move-line-up = Přesunout řádek nahoru
edit-move-line-down = Přesunout řádek dolů
file-close-tab = Zavřít kartu
file-reopen-tab = Znovu otevřít zavřenou kartu
window-next-tab = Další karta
window-prev-tab = Předchozí karta
tab-close = Zavřít kartu
tab-missing-on-disk = Na disku nenalezeno
edit-find = Najít…
edit-find-next = Najít další
edit-find-prev = Najít předchozí
edit-replace = Nahradit…
find-placeholder = Najít
replace-placeholder = Nahradit čím
find-match-case = Rozlišovat velikost písmen
find-whole-word = Celá slova
find-regex = Regulární výraz
find-no-results = Žádné výsledky
find-count = { $total ->
    [one] 1 shoda
    [few] { $total } shody
    [many] { $total } shody
   *[other] { $total } shod
    }
find-count-current = { $current } z { $total }
find-prev = Předchozí shoda (Shift+Enter)
find-next = Další shoda (Enter)
find-close = Zavřít (Esc)
replace-one = Nahradit
replace-all = Nahradit vše
status-replaced = { $count ->
    [one] Nahrazena 1 shoda
    [few] Nahrazeny { $count } shody
    [many] Nahrazeno { $count } shody
   *[other] Nahrazeno { $count } shod
    }
language-plain = Prostý text
menu-view = Zobrazení
app-settings = Nastavení…
view-zoom-in = Přiblížit
view-zoom-out = Oddálit
view-zoom-reset = Skutečná velikost
settings-title = Nastavení
settings-font-size = Velikost písma
settings-tab-width = Šířka tabulátoru
settings-insert-spaces = Při stisku Tab vkládat mezery
settings-theme = Vzhled
settings-theme-system = Podle systému
settings-theme-light = Světlý
settings-theme-dark = Tmavý
settings-restore-session = Znovu otevřít soubory z minulé relace
settings-hot-exit = Pamatovat si neuložené změny při ukončení
settings-hot-exit-detail = Při ukončení se aplikace nebude ptát na každou neuloženou kartu. Neuložené karty i karty bez názvu se při dalším spuštění vrátí i s textem, i když je znovuotevírání souborů z minulé relace vypnuté. Při zavření jednotlivé karty se aplikace stále zeptá.
settings-language = Jazyk
settings-language-system = Podle systému
settings-language-note = Změna jazyka se projeví po restartu aplikace Text³.
settings-save-failed = Nastavení nelze uložit: { $reason }
view-command-palette = Paleta příkazů…
palette-placeholder = Zadejte příkaz
view-markdown-source = Zdroj Markdown
view-markdown-split = Zdroj a náhled Markdown
view-markdown-preview = Náhled Markdown
preview-too-large = Tento dokument je pro náhled příliš velký.
preview-mermaid-unsupported-type = Typ diagramu není podporován; zobrazuje se zdroj.
preview-mermaid-unsupported-syntax = Diagram používá syntaxi, kterou nelze vykreslit; zobrazuje se zdroj.
preview-mermaid-too-large = Diagram je příliš velký na vykreslení; zobrazuje se zdroj.
menu-tools = Nástroje
tools-json-validate = Ověřit JSON
tools-json-format = Formátovat JSON
tools-json-minify = Minifikovat JSON
tools-sort-lines = Seřadit řádky
tools-sort-lines-desc = Seřadit řádky sestupně
tools-unique-lines = Odstranit duplicitní řádky
tools-trim-trailing = Odstranit koncové mezery
tools-upper-case = Velká písmena
tools-lower-case = Malá písmena
tools-to-lf = Převést konce řádků na LF
tools-to-crlf = Převést konce řádků na CRLF
tools-json-valid = JSON je platný
tools-done = Hotovo. Akci lze vrátit příkazem Zpět.
tools-no-change = Není co měnit
tools-json-error = Neplatný JSON na řádku { $line }, sloupci { $column }: { $reason }
json-reason-unterminated-string = neukončený řetězec
json-reason-invalid-u-escape = neplatná sekvence \u
json-reason-invalid-escape = neplatná escape sekvence
json-reason-control-character-in-string = řídicí znak v řetězci
json-reason-invalid-number = neplatné číslo
json-reason-unexpected-character = neočekávaný znak
json-reason-unexpected-content-after-the-value = neočekávaný obsah za hodnotou
json-reason-unexpected-comma = neočekávaná čárka
json-reason-expected-a-string-key = očekáván řetězcový klíč
json-reason-expected = očekáváno ':'
json-reason-expected-or-a-closing-bracket = očekávána ',' nebo uzavírací závorka
json-reason-expected-a-value = očekávána hodnota
json-reason-empty-document = prázdný dokument
json-reason-unexpected-end-of-input = neočekávaný konec vstupu
json-reason-too-deeply-nested-to-format = příliš hluboké vnoření pro formátování
tools-compare = Porovnat s…
compare-need-two = Otevřete další kartu k porovnání.
compare-pick-title = Porovnat { $name } s
compare-count = { $count ->
    [0] Žádné rozdíly
    [one] 1 změna
    [few] { $count } změny
    [many] { $count } změny
   *[other] { $count } změn
    }
compare-close = Zavřít porovnání
compare-to-left = Zkopírovat tuto změnu doleva
compare-to-right = Zkopírovat tuto změnu doprava
compare-stale = Během porovnání se dokument změnil. Porovnání bylo obnoveno; zkuste to znovu.
view-structured = Strukturované zobrazení (CSV, JSON, XML, YAML)
csv-delimiter = Oddělovač
csv-rows = { $count ->
    [one] 1 řádek
    [few] { $count } řádky
    [many] { $count } řádku
   *[other] { $count } řádků
    }
csv-unterminated = Neukončené uvozovky začínající na řádku { $line }
csv-truncated = Zobrazuje se prvních 200 000 řádků
csv-edit-cell = Upravit buňku
csv-insert-row-below = Vložit řádek pod
csv-delete-row = Odstranit řádek
csv-read-only = Soubor jen pro čtení: mřížku nelze upravovat
json-more = … a { $count } dalších
json-edit-value = Upravit hodnotu
json-rename-key = Přejmenovat klíč
json-add-after = Přidat za
json-add-inside = Přidat dovnitř
json-delete-node = Odstranit
json-read-only = Soubor jen pro čtení: strom nelze upravovat
json-invalid-value = Neplatná hodnota JSON
json-duplicate-key = Objekt už má klíč s tímto názvem
xml-edit-value = Upravit hodnotu
xml-edit-text = Upravit text
xml-rename = Přejmenovat
xml-add-attribute = Přidat atribut
xml-add-child = Přidat podřízený prvek
xml-add-after = Přidat za
xml-delete = Odstranit
xml-invalid-name = Neplatný název XML
xml-duplicate-attribute = Prvek už má atribut s tímto názvem
xml-not-well-formed = Tato změna by z XML udělala neplatný dokument
yaml-not-preserved = Tato změna by upravila i jiné části YAML
yaml-key-too-long = Klíč může mít nejvýše 1024 znaků
yaml-read-only-flow = Styl flow (zapsaný v závorkách) zatím nelze ve stromu upravovat
yaml-read-only-anchor = Hodnoty s kotvou (&) nelze ve stromu upravovat
yaml-read-only-tag = Hodnoty se značkou (!) nelze ve stromu upravovat
yaml-read-only-alias = Aliasy (*) nelze ve stromu upravovat
yaml-read-only-block-scalar = Bloky textu (| nebo >) zatím nelze ve stromu upravovat
yaml-read-only-complex-key = Složené klíče nelze ve stromu upravovat
yaml-read-only-document = Ve stromu lze upravovat jen první dokument
yaml-read-only-layout = Tato položka sdílí řádky s jiným obsahem, a proto ji zde nelze změnit
yaml-read-only-only-child = Jedinou položku nelze odstranit
dialog-open-link-title = Otevřít tento odkaz ve webovém prohlížeči?
button-open = Otevřít
link-blocked = Tento druh odkazu se z bezpečnostních důvodů neotevírá.
link-not-found = Odkazovaný soubor nebyl nalezen: { $target }
enc-windows-1252 = Západoevropské
enc-iso-8859-15 = Západoevropské
enc-windows-1250 = Středoevropské
enc-iso-8859-2 = Středoevropské
enc-windows-1251 = Cyrilice
enc-koi8-r = Cyrilice
enc-windows-1253 = Řecké
enc-windows-1254 = Turecké
enc-windows-1255 = Hebrejské
enc-windows-1256 = Arabské
enc-windows-1257 = Baltské
enc-windows-874 = Thajské
enc-shift_jis = Japonské
enc-euc-jp = Japonské
enc-gb18030 = Čínské zjednodušené
enc-big5 = Čínské tradiční
enc-euc-kr = Korejské
file-reopen-encoding = Znovu otevřít s kódováním…
file-save-encoding = Uložit s kódováním…
encoding-reopen-title = Znovu otevřít s kódováním
encoding-save-title = Uložit s kódováním
encoding-not-exact = Toto kódování nedokáže soubor otevřít beze změny: { $reason }
encoding-unencodable = Znak „{ $character }“ nelze uložit v kódování { $encoding }. Zvolte jiné kódování nebo upravte text; nic nebylo zapsáno.
encoding-save-first = Před opětovným otevřením s jiným kódováním změny uložte nebo vraťte zpět.
image-remote = Vzdálené obrázky se nenačítají.
image-blocked = Zobrazují se pouze obrázky ze složky tohoto dokumentu.
image-unsupported = Nepodporovaný formát obrázku.
image-too-large = Obrázek je příliš velký na zobrazení.
image-unreadable = Obrázek nelze načíst.
settings-theme-high-contrast = Vysoký kontrast
view-fold = Sbalit
view-unfold = Rozbalit
view-unfold-all = Rozbalit vše
settings-shortcuts = Klávesové zkratky
settings-keymap-preset = Předvolba
settings-keymap-default = Text³
settings-keymap-notepadpp = Notepad++
settings-shortcut-change = Změnit
settings-shortcut-reset = Obnovit výchozí
settings-shortcut-press = Stiskněte zkratku… (Esc zruší)
settings-shortcut-moved = Zkratka přesunuta z:
edit-find-in-folder = Najít ve složce…
folder-search-placeholder = Najít ve složce
folder-search-run = Hledat
folder-search-stop = Zastavit
folder-search-no-folder = Vybrat složku
folder-search-summary = { $count ->
    [one] 1 shoda
    [few] { $count } shody
    [many] { $count } shody
   *[other] { $count } shod
    } v { $files ->
    [one] 1 souboru
    [few] { $files } souborech
    [many] { $files } souboru
   *[other] { $files } souborech
    }
app-check-updates = Zkontrolovat aktualizace…
update-checking = Kontrolují se aktualizace…
update-up-to-date = Text³ { $version } je nejnovější verze.
update-no-releases = Zatím nebyla zveřejněna žádná vydání aplikace Text³.
update-failed = Aktualizace nelze zkontrolovat: { $reason }
update-invalid = neočekávaná odpověď
update-available-title = Je k dispozici Text³ { $version }
update-available-body = Otevřete stránku ke stažení a získejte jej. Text³ sám nic nestahuje ani neinstaluje.
update-open-page = Otevřít stránku ke stažení
update-later = Později
settings-check-updates = Kontrolovat aktualizace jednou denně
settings-check-updates-detail = Zjišťuje na github.com nejnovější vydání. Nic dalšího se neodesílá.
file-local-history = Místní historie…
file-clear-history = Vymazat místní historii…
history-title = Místní historie souboru { $name }
history-open = Otevřít
history-restore = Obnovit
history-empty = Zatím nejsou uloženy žádné verze. Při každém uložení se uchová kopie.
history-unreadable = Tuto verzi nelze přečíst.
history-just-now = Právě teď
history-minutes-ago = { $count ->
    [one] před 1 minutou
    [few] před { $count } minutami
    [many] před { $count } minutami
   *[other] před { $count } minutami
    }
history-hours-ago = { $count ->
    [one] před 1 hodinou
    [few] před { $count } hodinami
    [many] před { $count } hodinami
   *[other] před { $count } hodinami
    }
history-days-ago = { $count ->
    [one] před 1 dnem
    [few] před { $count } dny
    [many] před { $count } dny
   *[other] před { $count } dny
    }
history-purge-title = Odstranit celou místní historii?
history-purge-body = Uložené kopie všech souborů budou z tohoto počítače odstraněny. Samotné soubory se nezmění.
history-purge-confirm = Odstranit historii
history-purged = Místní historie byla odstraněna.
settings-local-history = Uchovávat místní historii uložených souborů
settings-local-history-detail = Uchovává až 50 verzí každého souboru po dobu 30 dnů, pouze na tomto počítači.
file-open-log = Otevřít v prohlížeči protokolů…
log-lines = { $count ->
    [one] 1 řádek
    [few] { $count } řádky
    [many] { $count } řádku
   *[other] { $count } řádků
    }
log-indexing = Načítání… { $percent } %
log-matches = { $count ->
    [one] 1 odpovídající řádek
    [few] { $count } odpovídající řádky
    [many] { $count } odpovídajícího řádku
   *[other] { $count } odpovídajících řádků
    }
log-filter-hint = Filtrovat řádky
log-bad-pattern = Neplatný vzor
log-follow = Sledovat
log-close = Zavřít prohlížeč
log-opened-large = Soubor { $name } je velký, proto byl otevřen jen pro čtení v prohlížeči protokolů.
tab-pin = Připnout kartu
tab-unpin = Odepnout kartu
edit-add-cursor-above = Přidat kurzor nad
edit-add-cursor-below = Přidat kurzor pod
view-word-wrap = Zalamování řádků
status-wrap-on = Dlouhé řádky se nyní zalamují podle šířky okna.
status-wrap-off = Dlouhé řádky se již nezalamují.
settings-word-wrap = Zalamovat dlouhé řádky
edit-go-to-line = Přejít na řádek…
edit-go-to-heading = Přejít na nadpis…
goto-line-hint = Řádek nebo řádek:sloupec
goto-line-note = { $count ->
    [one] Dokument má 1 řádek.
    [few] Dokument má { $count } řádky.
    [many] Dokument má { $count } řádku.
   *[other] Dokument má { $count } řádků.
    }
goto-line-invalid = Zadejte číslo řádku, případně následované :sloupec.
goto-heading-hint = Psaním filtrujte nadpisy
goto-heading-none = Tento dokument nemá žádné nadpisy.
edit-goto-anything = Přejít kamkoli…
goto-anything-hint = Název souboru, nebo :řádek, @symbol, #text
goto-anything-line = Přejít na řádek { $line }
goto-anything-open = otevřeno
goto-anything-scanning = Načítají se soubory ve složce { $folder }…
goto-anything-scanned = Soubory ve složce { $folder }: { $count }
goto-anything-limit = Zobrazuje se jen prvních { $max } souborů ve složce { $folder }.
goto-anything-no-folder = Uložte soubor nebo použijte Najít ve složce, aby se zobrazily soubory složky.
goto-anything-symbols-none = V tomto dokumentu nejsou žádné symboly.
goto-anything-matches-none = V tomto dokumentu nebyla nalezena žádná shoda.
format-xml = XML
format-yaml = YAML
tools-tree-error = Neplatný { $format } na řádku { $line }, sloupci { $column }: { $reason }
edit-select-next-occurrence = Vybrat další výskyt
tools-record-macro = Spustit nebo zastavit záznam makra
tools-play-macro = Přehrát makro
macro-recording = Nahrává se makro
macro-recorded = { $count ->
    [one] Makro zaznamenáno: 1 krok.
    [few] Makro zaznamenáno: { $count } kroky.
    [many] Makro zaznamenáno: { $count } kroku.
   *[other] Makro zaznamenáno: { $count } kroků.
    }
tools-compare-saved = Porovnat s uloženou verzí
tools-compare-clipboard = Porovnat se schránkou
compare-saved-label = { $name } (na disku)
compare-clipboard-label = Schránka
compare-saved-failed = Uložený soubor nelze přečíst: { $reason }
compare-clipboard-empty = Schránka neobsahuje žádný text k porovnání.
error-save-disk-full = Soubor { $name } nebyl uložen: disk je plný. Soubor na disku se nezměnil a váš text je stále zde. Uvolněte místo nebo jej pomocí Uložit jako uložte jinam.
error-save-permission = Soubor { $name } nebyl uložen: nemáte oprávnění do tohoto umístění zapisovat. Soubor na disku se nezměnil. Pomocí Uložit jako jej uložte jinam.
error-save-read-only-volume = Soubor { $name } nebyl uložen: disk je pouze pro čtení. Soubor na disku se nezměnil. Pomocí Uložit jako jej uložte jinam.
replace-preview-title = { $count ->
    [one] Nahradit 1 shodu?
    [few] Nahradit { $count } shody?
    [many] Nahradit { $count } shody?
   *[other] Nahradit { $count } shod?
    }
replace-preview-confirm = { $count ->
    [one] Nahradit 1
    [few] Nahradit { $count }
    [many] Nahradit { $count }
   *[other] Nahradit { $count }
    }
replace-preview-more = Zobrazeno je pouze prvních 200 změněných řádků.
replace-preview-stale = Dokument se změnil, nic nebylo nahrazeno. Spusťte znovu Nahradit vše.
encoding-detected = Otevřeno v kódování { $encoding } (zjištěno z obsahu souboru).
encoding-suggested = Navrženo podle obsahu souboru: { $encoding }. Před otevřením zkontrolujte náhled.
app-about = O aplikaci Text³
menu-help = Nápověda
about-version = Verze { $version }
about-license = Svobodný a otevřený software pod licencí Mozilla Public License 2.0.
about-ai = Vyvíjeno s pomocí AI (Claude od Anthropic) pod vedením správce projektu.
about-notices = Komponenty třetích stran a jejich licence jsou uvedeny v souboru THIRD_PARTY_NOTICES, který je součástí každého vydání.
about-source = Zdrojový kód a hlášení chyb na GitHubu
tools-xml-validate = Ověřit XML
tools-yaml-validate = Ověřit YAML
tools-markup-valid = { $format } je platný
file-open-recent = Otevřít nedávné…
file-save-all = Uložit vše
file-revert = Vrátit k uloženému
recent-hint = Pište pro filtrování nedávných souborů
recent-none = Zatím žádné nedávné soubory.
revert-title = Zahodit změny v souboru { $name }?
revert-body = Soubor bude znovu načten z disku. Neuložené změny budou ztraceny a nelze je vrátit zpět.
revert-confirm = Zahodit a znovu načíst
revert-done = Soubor { $name } byl znovu načten z disku.
status-saved-all = { $count ->
    [0] Není co uložit.
    [one] Uložen 1 soubor.
    [few] Uloženy { $count } soubory.
    [many] Uloženo { $count } souboru.
   *[other] Uloženo { $count } souborů.
    }
edit-toggle-comment = Přepnout řádkový komentář
edit-indent = Odsadit řádky
edit-outdent = Zrušit odsazení řádků
comment-unsupported = Tento typ souboru nemá řádkové komentáře.
tab-close-others = Zavřít ostatní karty
tab-close-all = Zavřít všechny karty
file-copy-path = Kopírovat cestu k souboru
file-reveal-macos = Zobrazit ve Finderu
file-reveal-windows = Zobrazit v Průzkumníkovi
file-reveal-other = Zobrazit ve správci souborů
status-path-copied = Cesta k souboru byla zkopírována.
tabs-kept-unsaved = { $count ->
    [one] 1 karta s neuloženými změnami zůstala otevřená.
    [few] { $count } karty s neuloženými změnami zůstaly otevřené.
    [many] { $count } karty s neuloženými změnami zůstalo otevřených.
   *[other] { $count } karet s neuloženými změnami zůstalo otevřených.
    }
status-indent-tabs = Tabulátory
status-indent-spaces = Mezery: { $count }
find-searching = Hledání…
compare-ignore-whitespace = Ignorovat mezery
tools-compare-file = Porovnat se souborem…
compare-file-failed = Nelze porovnat se souborem { $name }: { $reason }
cli-help =
    Použití:
      text3 [SOUBOR…]             otevřít soubory
      text3 --diff LEVÝ PRAVÝ     porovnat dva soubory vedle sebe
      text3 --version             zobrazit verzi
      text3 --help                zobrazit tuto nápovědu
    Před názvy souborů začínajícími pomlčkou použijte --.
cli-unknown-option = Neznámá volba { $option }.
cli-diff-needs-two = --diff vyžaduje dva soubory: text3 --diff LEVÝ PRAVÝ
window-move-tab-left = Přesunout kartu doleva
window-move-tab-right = Přesunout kartu doprava
edit-skip-occurrence = Přeskočit na další výskyt
tools-play-macro-times = Přehrát makro vícekrát…
macro-times-hint = Počet opakování
macro-times-note = Od 1 do { $max }
macro-times-invalid = Zadejte číslo od 1 do { $max }.
view-outline = Osnova
outline-empty = V tomto dokumentu nejsou žádné nadpisy.
outline-close = Zavřít osnovu
folder-replace-placeholder = Nahradit čím
folder-replace-run = Nahradit v souborech…
replace-files-title = Nahradit { $count ->
    [one] 1 shodu
    [few] { $count } shody
    [many] { $count } shody
   *[other] { $count } shod
    } v { $files ->
    [one] 1 souboru
    [few] { $files } souborech
    [many] { $files } souboru
   *[other] { $files } souborech
    }?
replace-files-body = Soubory otevřené na kartě se změní přímo tam a změnu lze vrátit zpět; uložte je sami. Ostatní soubory se uloží hned a jejich předchozí obsah se uchová v místní historii, pokud je zapnutá. Soubor, který se mezitím změnil na disku, se přeskočí.
replace-files-confirm = Nahradit
replace-files-done = { $count ->
    [one] Nahrazena 1 shoda
    [few] Nahrazeny { $count } shody
    [many] Nahrazeno { $count } shody
   *[other] Nahrazeno { $count } shod
    } v { $files ->
    [one] 1 souboru
    [few] { $files } souborech
    [many] { $files } souboru
   *[other] { $files } souborech
    } (otevřeno na kartách: { $open }, neuloženo).
replace-files-failed = { $failed ->
    [one] 1 soubor nebyl změněn:
    [few] { $failed } soubory nebyly změněny; první:
    [many] { $failed } souboru nebylo změněno; první:
   *[other] { $failed } souborů nebylo změněno; první:
    } { $first }
image-outside = Tento obrázek je mimo složku dokumentu.
image-allow = Zobrazit obrázky z této složky
cli-help-integration =
    Integrace se správcem souborů (jen pro aktuálního uživatele):
      text3 --register            přidat Text³ do nabídky „Otevřít v aplikaci“ (Linux, Windows)
      text3 --unregister          znovu jej odebrat
      text3 --default-apps[=plain,markdown,code,data,web]
                                  nastavit Text³ jako výchozí aplikaci pro tyto soubory
cli-registered = Text³ byl přidán do nabídky „Otevřít v aplikaci“. { $place }
cli-unregistered = Text³ byl odebrán z nabídky „Otevřít v aplikaci“.
cli-register-failed = Integraci se správcem souborů nebylo možné změnit: { $reason }
cli-register-macos = V macOS není potřeba nic registrovat: jakmile je Text³.app ve složce Aplikace, Finder jej nabídne v nabídce „Otevřít v aplikaci“.
cli-default-apps-set = Text³ nyní otevírá tyto typy souborů: { $types }
cli-default-apps-unknown = Neznámá skupina typů souborů: { $group }. Známé skupiny: plain, markdown, code, data, web.
default-apps-title = Otevírat soubory v Text³
default-apps-body = Text³ může být aplikace, která tyto soubory otevře po dvojitém kliknutí. Později to můžete změnit v Nastavení › Obecné.
default-apps-body-windows = Ve Windows si výchozí aplikace volíte sami: Text³ otevře Nastavení › Výchozí aplikace, kde vyberete Text³ pro každý typ souboru.
default-apps-unavailable = Tato kopie Text³ nemůže měnit výchozí aplikace: je to přenosná kopie, nebo v macOS nebyla spuštěna z Text³.app.
default-apps-plain = Text a protokoly
default-apps-markdown = Markdown
default-apps-code = Zdrojový kód a skripty
default-apps-data = Data a konfigurace
default-apps-web = Webové stránky a styly
default-apps-is-default = již Text³
default-apps-apply = Nastavit Text³ jako výchozí
default-apps-open-settings = Otevřít Nastavení Windows
default-apps-not-now = Teď ne
default-apps-close = Zavřít
default-apps-done = Text³ nyní otevírá vybrané typy souborů.
default-apps-settings-opened = Nastavení Windows je otevřené: vyberte Text³ pro typy souborů, které má otevírat.
default-apps-failed = Výchozí aplikace se nepodařilo změnit: { $reason }
settings-default-apps = Výchozí aplikace
settings-default-apps-button = Vybrat typy souborů…
settings-tab-general = Obecné
settings-tab-editor = Editor
settings-tab-appearance = Vzhled
settings-shortcut-search = Hledat příkazy nebo klávesy
settings-font = Písmo
settings-font-default = Vestavěné neproporcionální
settings-font-loading = Hledání nainstalovaných písem…
settings-font-missing = Písmo { $font } nebylo nalezeno; používá se vestavěné písmo.
settings-theme-more = Další motivy
settings-theme-user-note = Vlastní motivy: soubory .theme ve složce { $folder } (viz docs/THEMES.md).
language-auto = Rozpoznat automaticky
language-choose = Vyberte jazyk této karty
tools-xml-format = Formátovat XML
tools-save-macro = Uložit makro…
tools-play-saved-macro = Přehrát uložené makro…
tools-delete-saved-macro = Smazat uložené makro…
macro-save-hint = Název makra
macro-save-note = Uložené makro se stejným názvem bude nahrazeno.
macro-save-empty = Není co uložit: nejprve nahrajte makro.
macro-saved = Makro uloženo: { $name }
macro-deleted = Makro smazáno: { $name }
macro-name-invalid = Zadejte název o délce 1 až { $max } znaků na jednom řádku.
macro-save-too-large = Toto makro je příliš dlouhé na uložení.
macro-save-full = Lze uložit nejvýše { $max } maker. Nejprve některé smažte.
macro-save-failed = Makra se nepodařilo uložit: { $error }
macro-saved-hint = Pište pro filtrování uložených maker
macro-saved-none = Zatím žádná uložená makra.
settings-shortcut-import-npp = Importovat z Notepad++…
settings-shortcut-import-npp-hint = Vyberte soubor shortcuts.xml z Notepad++. Ve Windows je v %AppData%\Notepad++\shortcuts.xml.
settings-shortcut-import-npp-done = Importováno z Notepad++: { $imported }. Přeskočeno: { $skipped }. Zbývající konflikty: { $conflicts }.
settings-shortcut-import-npp-failed = Soubor { $name } nelze importovat: { $reason }
settings-shortcut-import-npp-too-large = Soubor je větší než 1 MiB.
settings-shortcut-import-npp-invalid = Soubor není platný shortcuts.xml z Notepad++.
view-split-right = Rozdělit vpravo
view-split-down = Rozdělit dolů
view-close-pane = Zavřít podokno
view-focus-next-pane = Přejít na další podokno
view-focus-prev-pane = Přejít na předchozí podokno
view-focus-next-region = Přejít na další oblast
view-focus-prev-region = Přejít na předchozí oblast
status-pane-limit = Bylo dosaženo limitu podoken (nejvýše { $max }).
a11y-csv-grid = Tabulka CSV
a11y-grid-cell = Řádek { $row }, sloupec { $column }: { $name }
a11y-grid-cell-plain = Řádek { $row }, sloupec { $column }
a11y-grid-column-header = Záhlaví sloupce { $column }
a11y-grid-row-header = Řádek { $row }
a11y-edit-field = Úprava: { $item }
a11y-json-tree = Strom JSON
a11y-xml-tree = Strom XML
a11y-yaml-tree = Strom YAML
a11y-tree-item = Úroveň { $level }
a11y-tree-item-expanded = Úroveň { $level }, rozbaleno
a11y-tree-item-collapsed = Úroveň { $level }, sbaleno
a11y-tree-object = Objekt, položek: { $count }
a11y-tree-list = Seznam, prvků: { $count }
a11y-tree-element = Prvek { $name }
a11y-tree-attribute = Atribut { $name }
a11y-tree-text = Text
a11y-pane = Podokno { $pane } z { $count }: { $name }
a11y-pane-active = Podokno { $pane } z { $count }: { $name }, aktivní
a11y-pane-editor = Textový editor, podokno { $pane } z { $count }: { $name }
a11y-mermaid-flowchart = Vývojový diagram. Uzlů: { $nodes }, spojení: { $links }.
a11y-mermaid-sequence = Sekvenční diagram. Účastníků: { $participants }, zpráv: { $messages }.
a11y-mermaid-state = Stavový diagram. Stavů: { $states }, přechodů: { $transitions }.
a11y-mermaid-class = Diagram tříd. Tříd: { $classes }, vztahů: { $relations }.
a11y-shortcut-change = Změnit klávesovou zkratku pro { $command }
a11y-shortcut-change-current = Změnit klávesovou zkratku pro { $command }, nyní { $keys }
a11y-shortcut-reset = Obnovit klávesovou zkratku pro { $command }
status-line-ending-choose = Změnit konce řádků tohoto souboru
status-encoding-choose = Znovu otevřít nebo uložit s jiným kódováním
status-indent-choose = Nastavení odsazení
a11y-settings-sections = Oddíly nastavení
a11y-tab-strip = Otevřené soubory
a11y-tab-unsaved = { $name }, neuložené změny
migration-copied = Nastavení a data ze Sanain byla zkopírována do Text³. Složky Sanain zůstaly beze změny.

# Character and word count (status bar)
# How one thousand is written with digit grouping: the characters between
# 1 and 000 separate groups of three digits.
number-thousand = 1{ "\u00A0" }000
count-status = Znaky: { $chars } · Slova: { $words }
count-status-selection = Znaky: { $chars } z { $total-chars } · Slova: { $words } z { $total-words }
count-choose = Počet znaků, slov, řádků a bajtů
count-a11y-pending = Znaky a slova zatím nespočítány. Otevřete pro výpočet.
count-heading-selection = Výběr
count-heading-document = Dokument
count-characters = Znaky
count-characters-no-spaces = Znaky (bez mezer)
count-words = Slova
count-lines = Řádky
count-bytes = Bajty ({ $encoding })
count-unencodable = Některé znaky nelze uložit v kódování { $encoding }.
count-counting = Počítání…
settings-show-counts = Zobrazovat počet znaků a slov ve stavovém řádku
