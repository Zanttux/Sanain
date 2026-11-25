# Machine translation by Claude (AI); review by a native speaker required before release.
app-name = Sanain
menu-file = Arkiv
file-open = Öppna…
file-save = Spara
status-tabs = { $count ->
    [one] En flik
   *[other] { $count } flikar
    }
error-open = Det gick inte att öppna { $name }.
menu-edit = Redigera
file-new = Ny flik
file-save-as = Spara som…
edit-undo = Ångra
edit-redo = Gör om
edit-select-all = Markera allt
untitled = Namnlös
window-title = { $dirty ->
    [yes] ● { $name } — Sanain
   *[no] { $name } — Sanain
    }
status-position = Rad { $line }, kol { $column }
status-selections = { $count ->
    [one] 1 markering
   *[other] { $count } markeringar
    }
status-saved = { $name } har sparats
encoding-utf8 = UTF-8
encoding-utf8-bom = UTF-8 med BOM
encoding-utf16le = UTF-16 LE
encoding-utf16be = UTF-16 BE
line-ending-lf = LF
line-ending-crlf = CRLF
line-ending-cr = CR
line-ending-mixed = Blandade radslut
editor-accessible-name = Textredigerare
button-save = Spara
button-dont-save = Spara inte
button-cancel = Avbryt
button-ok = OK
button-overwrite = Skriv över
button-save-as = Spara som…
dialog-unsaved-title = Vill du spara ändringarna i { $name }?
dialog-unsaved-body = Dina ändringar går förlorade om du inte sparar dem.
dialog-conflict-title = { $name } har ändrats på disken
dialog-conflict-body = Ett annat program ändrade filen efter att du öppnade den. Om du skriver över ersätts de ändringarna.
dialog-readonly-title = { $name } är skrivskyddad
dialog-readonly-body = Spara en kopia med ett annat namn i stället.
error-open-encoding = { $name } är inte UTF-8- eller UTF-16-text. Filen öppnades inte, så innehållet är oförändrat. Välj teckenkodning med Arkiv > Öppna igen med teckenkodning.
error-save = Det gick inte att spara { $name }: { $reason }
menu-window = Fönster
edit-cut = Klipp ut
edit-copy = Kopiera
edit-paste = Klistra in
app-quit = Avsluta Sanain
app-hide = Göm Sanain
app-hide-others = Göm övriga
app-show-all = Visa alla
window-minimize = Minimera
window-zoom = Zooma
status-recovered = Osparade ändringar från förra sessionen har återställts
status-recovery-failed = Det gick inte att spara en återställningskopia: { $reason }
edit-duplicate-line = Duplicera rad
edit-delete-line = Radera rad
edit-move-line-up = Flytta rad uppåt
edit-move-line-down = Flytta rad nedåt
file-close-tab = Stäng flik
file-reopen-tab = Öppna stängd flik igen
window-next-tab = Nästa flik
window-prev-tab = Föregående flik
tab-close = Stäng flik
edit-find = Sök…
edit-find-next = Sök nästa
edit-find-prev = Sök föregående
edit-replace = Ersätt…
find-placeholder = Sök
replace-placeholder = Ersätt med
find-match-case = Matcha skiftläge
find-whole-word = Hela ord
find-regex = Reguljärt uttryck
find-no-results = Inga resultat
find-count = { $total ->
    [one] 1 träff
   *[other] { $total } träffar
    }
find-count-current = { $current } av { $total }
find-prev = Föregående träff (Shift+Enter)
find-next = Nästa träff (Enter)
find-close = Stäng (Esc)
replace-one = Ersätt
replace-all = Ersätt alla
status-replaced = { $count ->
    [one] 1 träff ersattes
   *[other] { $count } träffar ersattes
    }
language-plain = Oformaterad text
menu-view = Visa
app-settings = Inställningar…
view-zoom-in = Zooma in
view-zoom-out = Zooma ut
view-zoom-reset = Verklig storlek
settings-title = Inställningar
settings-font-size = Teckenstorlek
settings-tab-width = Tabbbredd
settings-insert-spaces = Infoga blanksteg när Tab trycks
settings-theme = Utseende
settings-theme-system = Följ systemet
settings-theme-light = Ljust
settings-theme-dark = Mörkt
settings-restore-session = Öppna filer från förra sessionen igen
settings-language = Språk
settings-language-system = Följ systemet
settings-language-note = Språkändringar börjar gälla när Sanain startas om.
settings-save-failed = Det gick inte att spara inställningarna: { $reason }
view-command-palette = Kommandopalett…
palette-placeholder = Skriv ett kommando
view-markdown-source = Markdown-källa
view-markdown-split = Markdown-källa och förhandsvisning
view-markdown-preview = Markdown-förhandsvisning
preview-too-large = Dokumentet är för stort för att förhandsvisas.
preview-mermaid-unsupported = Mermaid-diagram återges inte ännu; källan visas.
menu-tools = Verktyg
tools-json-validate = Validera JSON
tools-json-format = Formatera JSON
tools-json-minify = Minimera JSON
tools-sort-lines = Sortera rader
tools-sort-lines-desc = Sortera rader fallande
tools-unique-lines = Ta bort dubblettrader
tools-trim-trailing = Ta bort blanksteg i radslut
tools-upper-case = Versaler
tools-lower-case = Gemener
tools-to-lf = Konvertera radslut till LF
tools-to-crlf = Konvertera radslut till CRLF
tools-json-valid = JSON är giltig
tools-done = Klart. Ångra återställer ändringen.
tools-no-change = Inget att ändra
tools-json-error = Ogiltig JSON på rad { $line }, kolumn { $column }: { $reason }
json-reason-unterminated-string = oavslutad sträng
json-reason-invalid-u-escape = ogiltig \u-escape
json-reason-invalid-escape = ogiltig escape
json-reason-control-character-in-string = kontrolltecken i sträng
json-reason-invalid-number = ogiltigt tal
json-reason-unexpected-character = oväntat tecken
json-reason-unexpected-content-after-the-value = oväntat innehåll efter värdet
json-reason-unexpected-comma = oväntat kommatecken
json-reason-expected-a-string-key = förväntade en strängnyckel
json-reason-expected = förväntade ':'
json-reason-expected-or-a-closing-bracket = förväntade ',' eller en avslutande hakparentes
json-reason-expected-a-value = förväntade ett värde
json-reason-empty-document = tomt dokument
json-reason-unexpected-end-of-input = oväntat slut på indata
json-reason-too-deeply-nested-to-format = för djupt nästlat för att formateras
tools-compare = Jämför med…
compare-need-two = Öppna en annan flik att jämföra med.
compare-pick-title = Jämför { $name } med
compare-count = { $count ->
    [0] Inga skillnader
    [one] 1 ändring
   *[other] { $count } ändringar
    }
compare-close = Stäng jämförelse
compare-to-left = Kopiera ändringen till vänster
compare-to-right = Kopiera ändringen till höger
compare-stale = Ett dokument ändrades under jämförelsen. Jämförelsen har uppdaterats; försök igen.
view-structured = Strukturerad vy (CSV, JSON)
csv-delimiter = Avgränsare
csv-rows = { $count ->
    [one] 1 rad
   *[other] { $count } rader
    }
csv-unterminated = Oavslutat citattecken från rad { $line }
csv-truncated = Visar de första 200 000 raderna
json-more = … { $count } till
dialog-open-link-title = Vill du öppna länken i webbläsaren?
button-open = Öppna
link-blocked = Den här typen av länk öppnas inte av säkerhetsskäl.
link-not-found = Den länkade filen hittades inte: { $target }
enc-windows-1252 = Västeuropeisk
enc-iso-8859-15 = Västeuropeisk
enc-windows-1250 = Centraleuropeisk
enc-iso-8859-2 = Centraleuropeisk
enc-windows-1251 = Kyrillisk
enc-koi8-r = Kyrillisk
enc-windows-1253 = Grekisk
enc-windows-1254 = Turkisk
enc-windows-1255 = Hebreisk
enc-windows-1256 = Arabisk
enc-windows-1257 = Baltisk
enc-windows-874 = Thailändsk
enc-shift_jis = Japansk
enc-euc-jp = Japansk
enc-gb18030 = Kinesisk, förenklad
enc-big5 = Kinesisk, traditionell
enc-euc-kr = Koreansk
file-reopen-encoding = Öppna igen med teckenkodning…
file-save-encoding = Spara med teckenkodning…
encoding-reopen-title = Öppna igen med teckenkodning
encoding-save-title = Spara med teckenkodning
encoding-not-exact = Den här teckenkodningen kan inte öppna filen utan att ändra den: { $reason }
encoding-unencodable = ”{ $character }” kan inte sparas som { $encoding }. Välj en annan teckenkodning eller ändra texten; inget skrevs.
encoding-save-first = Spara eller ångra dina ändringar innan du öppnar igen med en annan teckenkodning.
image-remote = Fjärrbilder läses inte in.
image-blocked = Endast bilder i dokumentets mapp visas.
image-unsupported = Bildformatet stöds inte.
image-too-large = Bilden är för stor för att visas.
image-unreadable = Det gick inte att läsa bilden.
settings-theme-high-contrast = Hög kontrast
view-fold = Fäll ihop
view-unfold = Fäll ut
view-unfold-all = Fäll ut alla
settings-shortcuts = Kortkommandon
settings-keymap-preset = Förinställning
settings-keymap-default = Sanain
settings-keymap-notepadpp = Notepad++
settings-shortcut-change = Ändra
settings-shortcut-reset = Återställ
settings-shortcut-press = Tryck på ett kortkommando… (Esc avbryter)
settings-shortcut-moved = Kortkommandot flyttades från:
edit-find-in-folder = Sök i mapp…
folder-search-placeholder = Sök i mapp
folder-search-run = Sök
folder-search-stop = Stoppa
folder-search-no-folder = Välj mapp
folder-search-summary = { $count ->
    [one] 1 träff
   *[other] { $count } träffar
    } i { $files ->
    [one] 1 fil
   *[other] { $files } filer
    }
app-check-updates = Sök efter uppdateringar…
update-checking = Söker efter uppdateringar…
update-up-to-date = Sanain { $version } är den senaste versionen.
update-no-releases = Inga versioner av Sanain har publicerats ännu.
update-failed = Kunde inte söka efter uppdateringar: { $reason }
update-invalid = oväntat svar
update-available-title = Sanain { $version } är tillgänglig
update-available-body = Öppna nedladdningssidan för att hämta den. Sanain laddar inte ned eller installerar något på egen hand.
update-open-page = Öppna nedladdningssidan
update-later = Senare
settings-check-updates = Sök efter uppdateringar en gång om dagen
settings-check-updates-detail = Frågar github.com efter den senaste versionen. Inget annat skickas.
