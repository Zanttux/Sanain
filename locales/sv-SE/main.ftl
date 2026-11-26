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
view-structured = Strukturerad vy (CSV, JSON, XML, YAML)
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
file-local-history = Lokal historik…
file-clear-history = Rensa lokal historik…
history-title = Lokal historik för { $name }
history-open = Öppna
history-restore = Återställ
history-empty = Inga sparade versioner ännu. En kopia behålls varje gång du sparar.
history-unreadable = Den här versionen kunde inte läsas.
history-just-now = Nyss
history-minutes-ago = { $count ->
    [one] för 1 minut sedan
   *[other] för { $count } minuter sedan
    }
history-hours-ago = { $count ->
    [one] för 1 timme sedan
   *[other] för { $count } timmar sedan
    }
history-days-ago = { $count ->
    [one] för 1 dag sedan
   *[other] för { $count } dagar sedan
    }
history-purge-title = Radera all lokal historik?
history-purge-body = Sparade kopior av alla filer tas bort från den här datorn. Själva filerna ändras inte.
history-purge-confirm = Radera historik
history-purged = Lokal historik har raderats.
settings-local-history = Behåll lokal historik för sparade filer
settings-local-history-detail = Behåller upp till 50 versioner per fil i 30 dagar, endast på den här datorn.
file-open-log = Öppna i loggvisaren…
log-lines = { $count ->
    [one] 1 rad
   *[other] { $count } rader
    }
log-indexing = Läser… { $percent } %
log-matches = { $count ->
    [one] 1 matchande rad
   *[other] { $count } matchande rader
    }
log-filter-hint = Filtrera rader
log-bad-pattern = Ogiltigt mönster
log-follow = Följ
log-close = Stäng visaren
log-opened-large = { $name } är stor och öppnades därför skrivskyddad i loggvisaren.
tab-pin = Fäst flik
tab-unpin = Lossa flik
edit-add-cursor-above = Lägg till markör ovanför
edit-add-cursor-below = Lägg till markör nedanför
view-word-wrap = Radbrytning
status-wrap-on = Långa rader bryts nu vid fönsterbredden.
status-wrap-off = Långa rader bryts inte längre.
settings-word-wrap = Bryt långa rader
edit-go-to-line = Gå till rad…
edit-go-to-heading = Gå till rubrik…
goto-line-hint = Rad eller rad:kolumn
goto-line-note = { $count ->
    [one] Dokumentet har 1 rad.
   *[other] Dokumentet har { $count } rader.
    }
goto-line-invalid = Ange ett radnummer, eventuellt följt av :kolumn.
goto-heading-hint = Skriv för att filtrera rubriker
goto-heading-none = Det här dokumentet har inga rubriker.
format-xml = XML
format-yaml = YAML
tools-tree-error = Ogiltig { $format } på rad { $line }, kolumn { $column }: { $reason }
edit-select-next-occurrence = Markera nästa förekomst
tools-record-macro = Starta eller stoppa makroinspelning
tools-play-macro = Spela upp makro
macro-recording = Spelar in makro
macro-recorded = { $count ->
    [one] Makro inspelat: 1 steg.
   *[other] Makro inspelat: { $count } steg.
    }
tools-compare-saved = Jämför med sparad version
tools-compare-clipboard = Jämför med urklipp
compare-saved-label = { $name } (på disk)
compare-clipboard-label = Urklipp
compare-saved-failed = Det gick inte att läsa den sparade filen: { $reason }
compare-clipboard-empty = Urklipp innehåller ingen text att jämföra.
error-save-disk-full = { $name } sparades inte: disken är full. Filen på disken är oförändrad och din text finns kvar här. Frigör utrymme eller använd Spara som för att spara den någon annanstans.
error-save-permission = { $name } sparades inte: du har inte behörighet att skriva där. Filen på disken är oförändrad. Använd Spara som för att spara den någon annanstans.
error-save-read-only-volume = { $name } sparades inte: disken är skrivskyddad. Filen på disken är oförändrad. Använd Spara som för att spara den någon annanstans.
replace-preview-title = { $count ->
    [one] Ersätta 1 träff?
   *[other] Ersätta { $count } träffar?
    }
replace-preview-confirm = { $count ->
    [one] Ersätt 1
   *[other] Ersätt { $count }
    }
replace-preview-more = Endast de första 200 ändrade raderna visas.
replace-preview-stale = Dokumentet har ändrats; inget ersattes. Kör Ersätt alla igen.
view-split-editor = Dela redigerare
encoding-detected = Öppnad som { $encoding } (identifierad utifrån filens innehåll).
encoding-suggested = Föreslagen utifrån filens innehåll: { $encoding }. Kontrollera förhandsgranskningen innan du öppnar.
app-about = Om Sanain
menu-help = Hjälp
about-tagline = Ordens kraft.
about-version = Version { $version }
about-license = Fri och öppen källkod under Mozilla Public License 2.0.
about-ai = Byggd av AI. Ledd av människor. Driven av gemenskapen.
about-notices = Komponenter från tredje part och deras licenser listas i THIRD_PARTY_NOTICES, som ingår i varje version.
about-source = Källkod och ärenden på GitHub
tools-xml-validate = Validera XML
tools-yaml-validate = Validera YAML
tools-markup-valid = { $format } är giltig
file-open-recent = Öppna senaste…
file-save-all = Spara alla
file-revert = Återgå till sparad version
recent-hint = Skriv för att filtrera senaste filer
recent-none = Inga senaste filer ännu.
revert-title = Vill du ignorera ändringarna i { $name }?
revert-body = Filen läses in på nytt från disken. Osparade ändringar går förlorade och kan inte ångras.
revert-confirm = Ignorera och läs in igen
revert-done = { $name } har lästs in på nytt från disken.
status-saved-all = { $count ->
    [0] Inget att spara.
    [one] 1 fil har sparats.
   *[other] { $count } filer har sparats.
    }
edit-toggle-comment = Växla radkommentar
edit-indent = Öka indrag för rader
edit-outdent = Minska indrag för rader
comment-unsupported = Den här filtypen har inga radkommentarer.
tab-close-others = Stäng andra flikar
tab-close-all = Stäng alla flikar
file-copy-path = Kopiera filsökväg
file-reveal-macos = Visa i Finder
file-reveal-windows = Visa i Utforskaren
file-reveal-other = Visa i filhanteraren
status-path-copied = Filsökvägen kopierades.
tabs-kept-unsaved = { $count ->
    [one] 1 flik med osparade ändringar hölls öppen.
   *[other] { $count } flikar med osparade ändringar hölls öppna.
    }
status-indent-tabs = Tabbar
status-indent-spaces = Blanksteg: { $count }
find-searching = Söker…
compare-ignore-whitespace = Ignorera blanksteg
tools-compare-file = Jämför med fil…
compare-file-failed = Det gick inte att jämföra med { $name }: { $reason }
cli-help =
    Användning:
      sanain [FIL…]                  öppna filer
      sanain --diff VÄNSTER HÖGER    jämför två filer sida vid sida
      sanain --version               visa versionen
      sanain --help                  visa den här hjälpen
    Använd -- före filnamn som börjar med bindestreck.
cli-unknown-option = Okänt alternativ { $option }.
cli-diff-needs-two = --diff behöver två filer: sanain --diff VÄNSTER HÖGER
window-move-tab-left = Flytta flik åt vänster
window-move-tab-right = Flytta flik åt höger
edit-skip-occurrence = Hoppa till nästa förekomst
tools-play-macro-times = Spela upp makro flera gånger…
macro-times-hint = Antal gånger
macro-times-note = Från 1 till { $max }
macro-times-invalid = Ange ett tal från 1 till { $max }.
view-outline = Disposition
outline-empty = Inga rubriker i det här dokumentet.
outline-close = Stäng disposition
folder-replace-placeholder = Ersätt med
folder-replace-run = Ersätt i filer…
replace-files-title = Ersätta { $count ->
    [one] 1 träff
   *[other] { $count } träffar
    } i { $files ->
    [one] 1 fil
   *[other] { $files } filer
    }?
replace-files-body = Filer som är öppna i en flik ändras där och kan ångras; spara dem själv. Övriga filer sparas nu, och deras tidigare innehåll behålls i den lokala historiken om den är aktiverad. En fil som har ändrats på disken under tiden hoppas över.
replace-files-confirm = Ersätt
replace-files-done = Ersatte { $count ->
    [one] 1 träff
   *[other] { $count } träffar
    } i { $files ->
    [one] 1 fil
   *[other] { $files } filer
    } (öppna i flikar: { $open }, inte sparade).
replace-files-failed = { $failed ->
    [one] 1 fil ändrades inte:
   *[other] { $failed } filer ändrades inte; första:
    } { $first }
image-outside = Den här bilden ligger utanför dokumentets mapp.
image-allow = Visa bilder från den här mappen
cli-help-integration =
    Integrering med filhanteraren (endast aktuell användare):
      sanain --register           lägg till Sanain i ”Öppna med” (Linux, Windows)
      sanain --unregister         ta bort det igen
cli-registered = Sanain lades till i ”Öppna med”. { $place }
cli-unregistered = Sanain togs bort från ”Öppna med”.
cli-register-failed = Integreringen med filhanteraren kunde inte ändras: { $reason }
cli-register-macos = I macOS behöver ingenting registreras: när Sanain.app ligger i mappen Program erbjuder Finder den under ”Öppna med”.
settings-tab-general = Allmänt
settings-tab-editor = Redigerare
settings-tab-appearance = Utseende
settings-shortcut-search = Sök kommandon eller tangenter
settings-font = Teckensnitt
settings-font-default = Inbyggt med fast bredd
settings-font-loading = Söker efter installerade teckensnitt…
settings-font-missing = Teckensnittet { $font } hittades inte; det inbyggda teckensnittet används.
settings-theme-more = Fler teman
settings-theme-user-note = Egna teman: .theme-filer i mappen { $folder } (se docs/THEMES.md).
language-auto = Identifiera automatiskt
language-choose = Välj språk för den här fliken
tools-xml-format = Formatera XML
