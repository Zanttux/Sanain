# Machine translation by Claude (AI); review by a native speaker required before release.
app-name = Sanain
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
    [yes] ● { $name } — Sanain
   *[no] { $name } — Sanain
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
app-quit = Ukončit Sanain
app-hide = Skrýt Sanain
app-hide-others = Skrýt ostatní
app-show-all = Zobrazit vše
window-minimize = Minimalizovat
window-zoom = Zvětšit
status-recovered = Neuložené změny z minulé relace byly obnoveny
status-recovery-failed = Nelze uložit záložní kopii: { $reason }
edit-duplicate-line = Duplikovat řádek
edit-delete-line = Odstranit řádek
edit-move-line-up = Přesunout řádek nahoru
edit-move-line-down = Přesunout řádek dolů
file-close-tab = Zavřít kartu
file-reopen-tab = Znovu otevřít zavřenou kartu
window-next-tab = Další karta
window-prev-tab = Předchozí karta
tab-close = Zavřít kartu
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
settings-language = Jazyk
settings-language-system = Podle systému
settings-language-note = Změna jazyka se projeví po restartu aplikace Sanain.
settings-save-failed = Nastavení nelze uložit: { $reason }
view-command-palette = Paleta příkazů…
palette-placeholder = Zadejte příkaz
view-markdown-source = Zdroj Markdown
view-markdown-split = Zdroj a náhled Markdown
view-markdown-preview = Náhled Markdown
preview-too-large = Tento dokument je pro náhled příliš velký.
preview-mermaid-unsupported = Diagramy Mermaid se zatím nevykreslují; zobrazuje se zdroj.
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
json-more = … a { $count } dalších
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
settings-keymap-default = Sanain
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
update-up-to-date = Sanain { $version } je nejnovější verze.
update-no-releases = Zatím nebyla zveřejněna žádná vydání aplikace Sanain.
update-failed = Aktualizace nelze zkontrolovat: { $reason }
update-invalid = neočekávaná odpověď
update-available-title = Je k dispozici Sanain { $version }
update-available-body = Otevřete stránku ke stažení a získejte jej. Sanain sám nic nestahuje ani neinstaluje.
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
