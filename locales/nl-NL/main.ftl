# Machine translation by Claude (AI); review by a native speaker required before release.
app-name = Text³
menu-file = Bestand
file-open = Open…
file-save = Bewaar
status-tabs = { $count ->
    [one] Eén tabblad
   *[other] { $count } tabbladen
    }
error-open = Kan { $name } niet openen.
menu-edit = Wijzig
file-new = Nieuw tabblad
file-save-as = Bewaar als…
edit-undo = Herstel
edit-redo = Opnieuw
edit-select-all = Selecteer alles
untitled = Naamloos
window-title = { $dirty ->
    [yes] ● { $name } — Text³
   *[no] { $name } — Text³
    }
status-position = Rg { $line }, Kol { $column }
status-selections = { $count ->
    [one] 1 selectie
   *[other] { $count } selecties
    }
status-saved = { $name } bewaard
encoding-utf8 = UTF-8
encoding-utf8-bom = UTF-8 met BOM
encoding-utf16le = UTF-16 LE
encoding-utf16be = UTF-16 BE
line-ending-lf = LF
line-ending-crlf = CRLF
line-ending-cr = CR
line-ending-mixed = Gemengde regeleinden
editor-accessible-name = Teksteditor
button-save = Bewaar
button-dont-save = Niet bewaren
button-cancel = Annuleer
button-ok = OK
button-overwrite = Overschrijf
button-save-as = Bewaar als…
dialog-unsaved-title = Wijzigingen in { $name } bewaren?
dialog-unsaved-body = Je wijzigingen gaan verloren als je ze niet bewaart.
dialog-conflict-title = { $name } is op schijf gewijzigd
dialog-conflict-body = Een ander programma heeft dit bestand gewijzigd nadat je het opende. Overschrijven vervangt die wijzigingen.
dialog-readonly-title = { $name } is alleen-lezen
dialog-readonly-body = Bewaar in plaats daarvan een kopie onder een andere naam.
error-open-encoding = { $name } is geen UTF-8- of UTF-16-tekst. Het bestand is niet geopend, dus de inhoud is ongewijzigd. Kies de codering via Bestand > Opnieuw openen met codering.
error-save = Kan { $name } niet bewaren: { $reason }
menu-window = Venster
edit-cut = Knip
edit-copy = Kopieer
edit-paste = Plak
app-quit = Stop Text³
app-hide = Verberg Text³
app-hide-others = Verberg andere
app-show-all = Toon alles
window-minimize = Minimaliseer
window-zoom = Zoom
status-recovered = Niet-bewaarde wijzigingen van de vorige sessie zijn hersteld
status-recovery-failed = Kan geen herstelkopie bewaren: { $reason }
status-hot-exit-failed = Kan niet-opgeslagen wijzigingen niet bewaren voor de volgende start ({ $reason }); in plaats daarvan wordt gevraagd.
status-hot-exit-too-large = { $name } is te groot om te bewaren voor de volgende start; in plaats daarvan wordt gevraagd.
status-restored-changed = { $name } is op de schijf gewijzigd nadat je niet-opgeslagen wijzigingen waren bewaard. Je tekst staat in het tabblad; het bestand is niet aangeraakt.
status-restored-missing = { $name } staat niet meer op de schijf. De niet-opgeslagen tekst blijft in het tabblad; opslaan maakt het bestand opnieuw aan.
edit-duplicate-line = Dupliceer regel
edit-delete-line = Verwijder regel
edit-move-line-up = Verplaats regel omhoog
edit-move-line-down = Verplaats regel omlaag
file-close-tab = Sluit tabblad
file-reopen-tab = Open gesloten tabblad opnieuw
window-next-tab = Volgend tabblad
window-prev-tab = Vorig tabblad
tab-close = Sluit tabblad
tab-missing-on-disk = Niet gevonden op de schijf
edit-find = Zoek…
edit-find-next = Zoek volgende
edit-find-prev = Zoek vorige
edit-replace = Vervang…
find-placeholder = Zoek
replace-placeholder = Vervang door
find-match-case = Hoofdlettergevoelig
find-whole-word = Heel woord
find-regex = Reguliere expressie
find-no-results = Geen resultaten
find-count = { $total ->
    [one] 1 overeenkomst
   *[other] { $total } overeenkomsten
    }
find-count-current = { $current } van { $total }
find-prev = Vorige overeenkomst (Shift+Enter)
find-next = Volgende overeenkomst (Enter)
find-close = Sluit (Esc)
replace-one = Vervang
replace-all = Vervang alles
status-replaced = { $count ->
    [one] 1 overeenkomst vervangen
   *[other] { $count } overeenkomsten vervangen
    }
language-plain = Platte tekst
menu-view = Weergave
app-settings = Instellingen…
view-zoom-in = Zoom in
view-zoom-out = Zoom uit
view-zoom-reset = Werkelijke grootte
settings-title = Instellingen
settings-font-size = Lettergrootte
settings-tab-width = Tabbreedte
settings-insert-spaces = Spaties invoegen bij drukken op Tab
settings-theme = Weergave
settings-theme-system = Volg systeem
settings-theme-light = Licht
settings-theme-dark = Donker
settings-restore-session = Bestanden van de vorige sessie opnieuw openen
settings-hot-exit = Niet-opgeslagen wijzigingen onthouden bij afsluiten
settings-hot-exit-detail = Afsluiten zonder vragen per niet-opgeslagen tabblad. Niet-opgeslagen en naamloze tabbladen komen bij de volgende start terug met hun tekst, ook als het opnieuw openen van bestanden van de vorige sessie uit staat. Bij het sluiten van één tabblad wordt nog steeds gevraagd.
settings-language = Taal
settings-language-system = Volg systeem
settings-language-note = Taalwijzigingen worden toegepast nadat Text³ opnieuw is gestart.
settings-save-failed = Kan instellingen niet bewaren: { $reason }
view-command-palette = Opdrachtpalet…
palette-placeholder = Typ een opdracht
view-markdown-source = Markdown-bron
view-markdown-split = Markdown-bron en voorvertoning
view-markdown-preview = Markdown-voorvertoning
preview-too-large = Dit document is te groot voor een voorvertoning.
preview-mermaid-unsupported-type = Diagramtype wordt niet ondersteund; de bron wordt getoond.
preview-mermaid-unsupported-syntax = Het diagram gebruikt syntaxis die niet getekend kan worden; de bron wordt getoond.
preview-mermaid-too-large = Het diagram is te groot om te tekenen; de bron wordt getoond.
menu-tools = Gereedschap
tools-json-validate = Valideer JSON
tools-json-format = Formatteer JSON
tools-json-minify = Minimaliseer JSON
tools-sort-lines = Sorteer regels
tools-sort-lines-desc = Sorteer regels aflopend
tools-unique-lines = Verwijder dubbele regels
tools-trim-trailing = Verwijder witruimte aan regeleinden
tools-upper-case = Hoofdletters
tools-lower-case = Kleine letters
tools-to-lf = Zet regeleinden om naar LF
tools-to-crlf = Zet regeleinden om naar CRLF
tools-json-valid = JSON is geldig
tools-done = Klaar. Herstel maakt het ongedaan.
tools-no-change = Niets te wijzigen
tools-json-error = Ongeldige JSON op regel { $line }, kolom { $column }: { $reason }
json-reason-unterminated-string = niet-afgesloten tekenreeks
json-reason-invalid-u-escape = ongeldige \u-escape
json-reason-invalid-escape = ongeldige escape
json-reason-control-character-in-string = besturingsteken in tekenreeks
json-reason-invalid-number = ongeldig getal
json-reason-unexpected-character = onverwacht teken
json-reason-unexpected-content-after-the-value = onverwachte inhoud na de waarde
json-reason-unexpected-comma = onverwachte komma
json-reason-expected-a-string-key = tekenreekssleutel verwacht
json-reason-expected = ':' verwacht
json-reason-expected-or-a-closing-bracket = ',' of een sluithaak verwacht
json-reason-expected-a-value = waarde verwacht
json-reason-empty-document = leeg document
json-reason-unexpected-end-of-input = onverwacht einde van invoer
json-reason-too-deeply-nested-to-format = te diep genest om te formatteren
tools-compare = Vergelijk met…
compare-need-two = Open een ander tabblad om mee te vergelijken.
compare-pick-title = Vergelijk { $name } met
compare-count = { $count ->
    [0] Geen verschillen
    [one] 1 wijziging
   *[other] { $count } wijzigingen
    }
compare-close = Sluit vergelijking
compare-to-left = Kopieer deze wijziging naar links
compare-to-right = Kopieer deze wijziging naar rechts
compare-stale = Een document is tijdens de vergelijking gewijzigd. De vergelijking is vernieuwd; probeer het opnieuw.
view-structured = Gestructureerde weergave (CSV, JSON, XML, YAML)
csv-delimiter = Scheidingsteken
csv-rows = { $count ->
    [one] 1 rij
   *[other] { $count } rijen
    }
csv-unterminated = Niet-afgesloten aanhalingsteken vanaf regel { $line }
csv-truncated = De eerste 200.000 rijen worden getoond
csv-edit-cell = Cel bewerken
csv-insert-row-below = Rij eronder invoegen
csv-delete-row = Rij verwijderen
csv-read-only = Alleen-lezen bestand: het raster kan niet worden bewerkt
json-more = … nog { $count }
json-edit-value = Waarde bewerken
json-rename-key = Sleutel hernoemen
json-add-after = Erna toevoegen
json-add-inside = Erin toevoegen
json-delete-node = Verwijderen
json-read-only = Alleen-lezen bestand: de boom kan niet worden bewerkt
json-invalid-value = Geen geldige JSON-waarde
json-duplicate-key = Het object heeft al een sleutel met deze naam
xml-edit-value = Waarde bewerken
xml-edit-text = Tekst bewerken
xml-rename = Hernoemen
xml-add-attribute = Attribuut toevoegen
xml-add-child = Onderliggend element toevoegen
xml-add-after = Erna toevoegen
xml-delete = Verwijderen
xml-invalid-name = Geen geldige XML-naam
xml-duplicate-attribute = Het element heeft al een attribuut met deze naam
xml-not-well-formed = Deze wijziging zou de XML ongeldig maken
yaml-not-preserved = Deze wijziging zou andere delen van de YAML veranderen
yaml-key-too-long = Een sleutel mag hoogstens 1024 tekens lang zijn
yaml-read-only-flow = Flow-stijl (tussen haakjes geschreven) kan nog niet in de boom worden bewerkt
yaml-read-only-anchor = Waarden met een anker (&) kunnen niet in de boom worden bewerkt
yaml-read-only-tag = Waarden met een tag (!) kunnen niet in de boom worden bewerkt
yaml-read-only-alias = Aliassen (*) kunnen niet in de boom worden bewerkt
yaml-read-only-block-scalar = Tekstblokken (| of >) kunnen nog niet in de boom worden bewerkt
yaml-read-only-complex-key = Complexe sleutels kunnen niet in de boom worden bewerkt
yaml-read-only-document = Alleen het eerste document kan in de boom worden bewerkt
yaml-read-only-layout = Dit item deelt zijn regels met andere inhoud en kan hier niet worden gewijzigd
yaml-read-only-only-child = Het enige item kan niet worden verwijderd
dialog-open-link-title = Deze link openen in je webbrowser?
button-open = Open
link-blocked = Dit soort link wordt om veiligheidsredenen niet geopend.
link-not-found = Gekoppeld bestand niet gevonden: { $target }
enc-windows-1252 = West-Europees
enc-iso-8859-15 = West-Europees
enc-windows-1250 = Centraal-Europees
enc-iso-8859-2 = Centraal-Europees
enc-windows-1251 = Cyrillisch
enc-koi8-r = Cyrillisch
enc-windows-1253 = Grieks
enc-windows-1254 = Turks
enc-windows-1255 = Hebreeuws
enc-windows-1256 = Arabisch
enc-windows-1257 = Baltisch
enc-windows-874 = Thai
enc-shift_jis = Japans
enc-euc-jp = Japans
enc-gb18030 = Chinees, vereenvoudigd
enc-big5 = Chinees, traditioneel
enc-euc-kr = Koreaans
file-reopen-encoding = Opnieuw openen met codering…
file-save-encoding = Bewaar met codering…
encoding-reopen-title = Opnieuw openen met codering
encoding-save-title = Bewaar met codering
encoding-not-exact = Met deze codering kan het bestand niet worden geopend zonder het te wijzigen: { $reason }
encoding-unencodable = “{ $character }” kan niet worden bewaard als { $encoding }. Kies een andere codering of wijzig de tekst; er is niets geschreven.
encoding-save-first = Bewaar je wijzigingen of maak ze ongedaan voordat je opnieuw opent met een andere codering.
image-remote = Externe afbeeldingen worden niet geladen.
image-blocked = Alleen afbeeldingen in de map van dit document worden getoond.
image-unsupported = Niet-ondersteunde afbeeldingsindeling.
image-too-large = Afbeelding is te groot om te tonen.
image-unreadable = Afbeelding kan niet worden gelezen.
settings-theme-high-contrast = Hoog contrast
view-fold = Vouw in
view-unfold = Vouw uit
view-unfold-all = Vouw alles uit
settings-shortcuts = Toetscombinaties
settings-keymap-preset = Voorinstelling
settings-keymap-default = Text³
settings-keymap-notepadpp = Notepad++
settings-shortcut-change = Wijzig
settings-shortcut-reset = Herstel standaard
settings-shortcut-press = Druk op een toetscombinatie… (Esc annuleert)
settings-shortcut-moved = Toetscombinatie verplaatst van:
edit-find-in-folder = Zoek in map…
folder-search-placeholder = Zoek in map
folder-search-run = Zoek
folder-search-stop = Stop
folder-search-no-folder = Kies map
folder-search-summary = { $count ->
    [one] 1 overeenkomst
   *[other] { $count } overeenkomsten
    } in { $files ->
    [one] 1 bestand
   *[other] { $files } bestanden
    }
app-check-updates = Zoeken naar updates…
update-checking = Zoeken naar updates…
update-up-to-date = Text³ { $version } is de nieuwste versie.
update-no-releases = Er zijn nog geen versies van Text³ gepubliceerd.
update-failed = Kan niet zoeken naar updates: { $reason }
update-invalid = onverwacht antwoord
update-available-title = Text³ { $version } is beschikbaar
update-available-body = Open de downloadpagina om het te downloaden. Text³ downloadt of installeert zelf niets.
update-open-page = Downloadpagina openen
update-later = Later
settings-check-updates = Eén keer per dag zoeken naar updates
settings-check-updates-detail = Vraagt github.com naar de nieuwste release. Er wordt niets anders verzonden.
file-local-history = Lokale geschiedenis…
file-clear-history = Wis lokale geschiedenis…
history-title = Lokale geschiedenis van { $name }
history-open = Open
history-restore = Herstel
history-empty = Nog geen bewaarde versies. Bij elke keer bewaren wordt een kopie behouden.
history-unreadable = Deze versie kan niet worden gelezen.
history-just-now = Zojuist
history-minutes-ago = { $count ->
    [one] 1 minuut geleden
   *[other] { $count } minuten geleden
    }
history-hours-ago = { $count ->
    [one] 1 uur geleden
   *[other] { $count } uur geleden
    }
history-days-ago = { $count ->
    [one] 1 dag geleden
   *[other] { $count } dagen geleden
    }
history-purge-title = Alle lokale geschiedenis verwijderen?
history-purge-body = Bewaarde kopieën van alle bestanden worden van deze computer verwijderd. Je bestanden zelf worden niet gewijzigd.
history-purge-confirm = Verwijder geschiedenis
history-purged = Lokale geschiedenis verwijderd.
settings-local-history = Lokale geschiedenis van bewaarde bestanden bijhouden
settings-local-history-detail = Houdt maximaal 50 versies per bestand 30 dagen bij, alleen op deze computer.
file-open-log = Openen in logviewer…
log-lines = { $count ->
    [one] 1 regel
   *[other] { $count } regels
    }
log-indexing = Lezen… { $percent }%
log-matches = { $count ->
    [one] 1 overeenkomende regel
   *[other] { $count } overeenkomende regels
    }
log-filter-hint = Regels filteren
log-bad-pattern = Ongeldig patroon
log-follow = Volgen
log-close = Viewer sluiten
log-opened-large = { $name } is groot en is daarom alleen-lezen geopend in de logviewer.
tab-pin = Tabblad vastmaken
tab-unpin = Tabblad losmaken
edit-add-cursor-above = Cursor erboven toevoegen
edit-add-cursor-below = Cursor eronder toevoegen
view-word-wrap = Automatische terugloop
status-wrap-on = Lange regels lopen nu terug op de vensterbreedte.
status-wrap-off = Lange regels lopen niet meer terug.
settings-word-wrap = Lange regels laten teruglopen
edit-go-to-line = Ga naar regel…
edit-go-to-heading = Ga naar kop…
goto-line-hint = Regel of regel:kolom
goto-line-note = { $count ->
    [one] Het document heeft 1 regel.
   *[other] Het document heeft { $count } regels.
    }
goto-line-invalid = Voer een regelnummer in, eventueel gevolgd door :kolom.
goto-heading-hint = Typ om koppen te filteren
goto-heading-none = Dit document heeft geen koppen.
format-xml = XML
format-yaml = YAML
tools-tree-error = Ongeldige { $format } op regel { $line }, kolom { $column }: { $reason }
edit-select-next-occurrence = Selecteer volgende overeenkomst
tools-record-macro = Macro-opname starten of stoppen
tools-play-macro = Macro afspelen
macro-recording = Macro wordt opgenomen
macro-recorded = { $count ->
    [one] Macro opgenomen: 1 stap.
   *[other] Macro opgenomen: { $count } stappen.
    }
tools-compare-saved = Vergelijk met opgeslagen versie
tools-compare-clipboard = Vergelijk met klembord
compare-saved-label = { $name } (op schijf)
compare-clipboard-label = Klembord
compare-saved-failed = Kan het opgeslagen bestand niet lezen: { $reason }
compare-clipboard-empty = Het klembord bevat geen tekst om te vergelijken.
error-save-disk-full = { $name } is niet bewaard: de schijf is vol. Het bestand op schijf is ongewijzigd en je tekst staat er nog. Maak ruimte vrij of gebruik Bewaar als om het ergens anders op te slaan.
error-save-permission = { $name } is niet bewaard: je hebt geen toestemming om daar te schrijven. Het bestand op schijf is ongewijzigd. Gebruik Bewaar als om het ergens anders op te slaan.
error-save-read-only-volume = { $name } is niet bewaard: de schijf is alleen-lezen. Het bestand op schijf is ongewijzigd. Gebruik Bewaar als om het ergens anders op te slaan.
replace-preview-title = { $count ->
    [one] 1 overeenkomst vervangen?
   *[other] { $count } overeenkomsten vervangen?
    }
replace-preview-confirm = { $count ->
    [one] 1 vervangen
   *[other] { $count } vervangen
    }
replace-preview-more = Alleen de eerste 200 gewijzigde regels worden getoond.
replace-preview-stale = Het document is gewijzigd; er is niets vervangen. Voer Vervang alles opnieuw uit.
encoding-detected = Geopend als { $encoding } (gedetecteerd uit de inhoud van het bestand).
encoding-suggested = Voorgesteld op basis van de inhoud van het bestand: { $encoding }. Controleer het voorbeeld voordat u opent.
app-about = Over Text³
menu-help = Help
about-version = Versie { $version }
about-license = Vrij en open source onder de Mozilla Public License 2.0.
about-ai = Ontwikkeld met hulp van AI (Claude van Anthropic) onder leiding van de beheerder.
about-notices = Componenten van derden en hun licenties staan vermeld in THIRD_PARTY_NOTICES, dat bij elke release wordt meegeleverd.
about-source = Broncode en meldingen op GitHub
tools-xml-validate = Valideer XML
tools-yaml-validate = Valideer YAML
tools-markup-valid = { $format } is geldig
file-open-recent = Open recent…
file-save-all = Bewaar alles
file-revert = Zet terug naar bewaarde versie
recent-hint = Typ om recente bestanden te filteren
recent-none = Nog geen recente bestanden.
revert-title = Wijzigingen in { $name } weggooien?
revert-body = Het bestand wordt opnieuw van schijf geladen. Niet-bewaarde wijzigingen gaan verloren en kunnen niet ongedaan worden gemaakt.
revert-confirm = Gooi weg en laad opnieuw
revert-done = { $name } is opnieuw van schijf geladen.
status-saved-all = { $count ->
    [0] Niets om te bewaren.
    [one] 1 bestand bewaard.
   *[other] { $count } bestanden bewaard.
    }
edit-toggle-comment = Schakel regelcommentaar in/uit
edit-indent = Laat regels inspringen
edit-outdent = Verklein inspringing van regels
comment-unsupported = Dit bestandstype heeft geen regelcommentaar.
tab-close-others = Sluit andere tabbladen
tab-close-all = Sluit alle tabbladen
file-copy-path = Kopieer bestandspad
file-reveal-macos = Toon in Finder
file-reveal-windows = Weergeven in Verkenner
file-reveal-other = Toon in bestandsbeheer
status-path-copied = Bestandspad gekopieerd.
tabs-kept-unsaved = { $count ->
    [one] 1 tabblad met niet-opgeslagen wijzigingen is open gelaten.
   *[other] { $count } tabbladen met niet-opgeslagen wijzigingen zijn open gelaten.
    }
status-indent-tabs = Tabs
status-indent-spaces = Spaties: { $count }
find-searching = Zoeken…
compare-ignore-whitespace = Negeer witruimte
tools-compare-file = Vergelijk met bestand…
compare-file-failed = Kan niet vergelijken met { $name }: { $reason }
cli-help =
    Gebruik:
      text3 [BESTAND…]              bestanden openen
      text3 --diff LINKS RECHTS     twee bestanden naast elkaar vergelijken
      text3 --version               de versie tonen
      text3 --help                  deze hulp tonen
    Gebruik -- vóór bestandsnamen die met een streepje beginnen.
cli-unknown-option = Onbekende optie { $option }.
cli-diff-needs-two = --diff heeft twee bestanden nodig: text3 --diff LINKS RECHTS
window-move-tab-left = Tabblad naar links verplaatsen
window-move-tab-right = Tabblad naar rechts verplaatsen
edit-skip-occurrence = Spring naar volgende overeenkomst
tools-play-macro-times = Macro meerdere keren afspelen…
macro-times-hint = Aantal keren
macro-times-note = Van 1 tot en met { $max }
macro-times-invalid = Voer een getal in van 1 tot en met { $max }.
view-outline = Overzicht
outline-empty = Geen koppen in dit document.
outline-close = Sluit overzicht
folder-replace-placeholder = Vervang door
folder-replace-run = Vervang in bestanden…
replace-files-title = { $count ->
    [one] 1 overeenkomst
   *[other] { $count } overeenkomsten
    } in { $files ->
    [one] 1 bestand
   *[other] { $files } bestanden
    } vervangen?
replace-files-body = Bestanden die in een tabblad open zijn, worden daar gewijzigd en dat kan ongedaan worden gemaakt; sla ze zelf op. Andere bestanden worden nu opgeslagen en hun vorige inhoud blijft bewaard in de lokale geschiedenis als die is ingeschakeld. Een bestand dat intussen op schijf is gewijzigd, wordt overgeslagen.
replace-files-confirm = Vervang
replace-files-done = { $count ->
    [one] 1 overeenkomst
   *[other] { $count } overeenkomsten
    } vervangen in { $files ->
    [one] 1 bestand
   *[other] { $files } bestanden
    } (open in tabbladen: { $open }, niet opgeslagen).
replace-files-failed = { $failed ->
    [one] 1 bestand is niet gewijzigd:
   *[other] { $failed } bestanden zijn niet gewijzigd; eerste:
    } { $first }
image-outside = Deze afbeelding staat buiten de map van het document.
image-allow = Afbeeldingen uit deze map tonen
cli-help-integration =
    Integratie met het bestandsbeheer (alleen huidige gebruiker):
      text3 --register            Text³ toevoegen aan ‘Openen met’ (Linux, Windows)
      text3 --unregister          weer verwijderen
cli-registered = Text³ is toegevoegd aan ‘Openen met’. { $place }
cli-unregistered = Text³ is verwijderd uit ‘Openen met’.
cli-register-failed = De integratie met het bestandsbeheer kon niet worden gewijzigd: { $reason }
cli-register-macos = Op macOS hoeft niets te worden geregistreerd: zodra Text³.app in de map Apps staat, biedt de Finder het aan onder ‘Openen met’.
settings-tab-general = Algemeen
settings-tab-editor = Editor
settings-tab-appearance = Weergave
settings-shortcut-search = Zoek opdrachten of toetsen
settings-font = Lettertype
settings-font-default = Ingebouwd monospace
settings-font-loading = Geïnstalleerde lettertypen zoeken…
settings-font-missing = Het lettertype { $font } is niet gevonden; het ingebouwde lettertype wordt gebruikt.
settings-theme-more = Meer thema's
settings-theme-user-note = Eigen thema's: .theme-bestanden in de map { $folder } (zie docs/THEMES.md).
language-auto = Automatisch detecteren
language-choose = Kies de taal van dit tabblad
tools-xml-format = Formatteer XML
tools-save-macro = Macro opslaan…
tools-play-saved-macro = Opgeslagen macro afspelen…
tools-delete-saved-macro = Opgeslagen macro verwijderen…
macro-save-hint = Naam van de macro
macro-save-note = Een opgeslagen macro met dezelfde naam wordt vervangen.
macro-save-empty = Niets om op te slaan: neem eerst een macro op.
macro-saved = Macro opgeslagen: { $name }
macro-deleted = Macro verwijderd: { $name }
macro-name-invalid = Voer een naam van 1 tot { $max } tekens op één regel in.
macro-save-too-large = Deze macro is te lang om op te slaan.
macro-save-full = Er kunnen maximaal { $max } macro’s worden opgeslagen. Verwijder er eerst een.
macro-save-failed = Kan de macro’s niet opslaan: { $error }
macro-saved-hint = Typ om opgeslagen macro’s te filteren
macro-saved-none = Nog geen opgeslagen macro’s.
settings-shortcut-import-npp = Importeren uit Notepad++…
settings-shortcut-import-npp-hint = Kies een shortcuts.xml van Notepad++. Op Windows staat die in %AppData%\Notepad++\shortcuts.xml.
settings-shortcut-import-npp-done = Geïmporteerd uit Notepad++: { $imported }. Overgeslagen: { $skipped }. Resterende conflicten: { $conflicts }.
settings-shortcut-import-npp-failed = Kan { $name } niet importeren: { $reason }
settings-shortcut-import-npp-too-large = Het bestand is groter dan 1 MiB.
settings-shortcut-import-npp-invalid = Het bestand is geen geldige shortcuts.xml van Notepad++.
view-split-right = Rechts splitsen
view-split-down = Omlaag splitsen
view-close-pane = Deelvenster sluiten
view-focus-next-pane = Volgend deelvenster
view-focus-prev-pane = Vorig deelvenster
view-focus-next-region = Volgend venstergebied
view-focus-prev-region = Vorig venstergebied
status-pane-limit = Maximumaantal deelvensters bereikt (hoogstens { $max }).
a11y-csv-grid = CSV-tabel
a11y-grid-cell = Rij { $row }, kolom { $column }: { $name }
a11y-grid-cell-plain = Rij { $row }, kolom { $column }
a11y-grid-column-header = Kop van kolom { $column }
a11y-grid-row-header = Rij { $row }
a11y-edit-field = Bewerken: { $item }
a11y-json-tree = JSON-boom
a11y-xml-tree = XML-boom
a11y-yaml-tree = YAML-boom
a11y-tree-item = Niveau { $level }
a11y-tree-item-expanded = Niveau { $level }, uitgevouwen
a11y-tree-item-collapsed = Niveau { $level }, samengevouwen
a11y-tree-object = Object, items: { $count }
a11y-tree-list = Lijst, elementen: { $count }
a11y-tree-element = Element { $name }
a11y-tree-attribute = Attribuut { $name }
a11y-tree-text = Tekst
a11y-pane = Deelvenster { $pane } van { $count }: { $name }
a11y-pane-active = Deelvenster { $pane } van { $count }: { $name }, actief
a11y-pane-editor = Teksteditor, deelvenster { $pane } van { $count }: { $name }
a11y-mermaid-flowchart = Stroomdiagram. Knooppunten: { $nodes }, verbindingen: { $links }.
a11y-mermaid-sequence = Sequentiediagram. Deelnemers: { $participants }, berichten: { $messages }.
a11y-mermaid-state = Toestandsdiagram. Toestanden: { $states }, overgangen: { $transitions }.
a11y-mermaid-class = Klassendiagram. Klassen: { $classes }, relaties: { $relations }.
a11y-shortcut-change = Sneltoets voor { $command } wijzigen
a11y-shortcut-change-current = Sneltoets voor { $command } wijzigen, nu { $keys }
a11y-shortcut-reset = Sneltoets voor { $command } herstellen
status-line-ending-choose = Regeleinden van dit bestand wijzigen
status-encoding-choose = Opnieuw openen of opslaan met een andere codering
status-indent-choose = Inspringingsinstellingen
a11y-settings-sections = Instellingsonderdelen
a11y-tab-strip = Geopende bestanden
a11y-tab-unsaved = { $name }, niet-opgeslagen wijzigingen
migration-copied = Instellingen en gegevens van Sanain zijn naar Text³ gekopieerd. De Sanain-mappen zijn ongewijzigd gebleven.
