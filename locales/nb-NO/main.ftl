# Machine translation by Claude (AI); review by a native speaker required before release.
app-name = Sanain
menu-file = Fil
file-open = Åpne…
file-save = Lagre
status-tabs = { $count ->
    [one] Én fane
   *[other] { $count } faner
    }
error-open = Kunne ikke åpne { $name }.
menu-edit = Rediger
file-new = Ny fane
file-save-as = Lagre som…
edit-undo = Angre
edit-redo = Gjør om
edit-select-all = Merk alt
untitled = Uten navn
window-title = { $dirty ->
    [yes] ● { $name } — Sanain
   *[no] { $name } — Sanain
    }
status-position = Linje { $line }, kol. { $column }
status-selections = { $count ->
    [one] 1 markering
   *[other] { $count } markeringer
    }
status-saved = Lagret { $name }
encoding-utf8 = UTF-8
encoding-utf8-bom = UTF-8 med BOM
encoding-utf16le = UTF-16 LE
encoding-utf16be = UTF-16 BE
line-ending-lf = LF
line-ending-crlf = CRLF
line-ending-cr = CR
line-ending-mixed = Blandede linjeskift
editor-accessible-name = Tekstredigeringsprogram
button-save = Lagre
button-dont-save = Ikke lagre
button-cancel = Avbryt
button-ok = OK
button-overwrite = Overskriv
button-save-as = Lagre som…
dialog-unsaved-title = Vil du lagre endringene i { $name }?
dialog-unsaved-body = Endringene går tapt hvis du ikke lagrer dem.
dialog-conflict-title = { $name } er endret på disken
dialog-conflict-body = Et annet program endret denne filen etter at du åpnet den. Overskriving erstatter disse endringene.
dialog-readonly-title = { $name } er skrivebeskyttet
dialog-readonly-body = Lagre i stedet en kopi under et annet navn.
error-open-encoding = { $name } er ikke UTF-8- eller UTF-16-tekst. Filen ble ikke åpnet, så innholdet er uendret. Bruk Fil > Åpne på nytt med koding for å velge koding.
error-save = Kunne ikke lagre { $name }: { $reason }
menu-window = Vindu
edit-cut = Klipp ut
edit-copy = Kopier
edit-paste = Lim inn
app-quit = Avslutt Sanain
app-hide = Skjul Sanain
app-hide-others = Skjul andre
app-show-all = Vis alle
window-minimize = Minimer
window-zoom = Zoom
status-recovered = Gjenopprettet ulagrede endringer fra forrige økt
status-recovery-failed = Kunne ikke lagre en gjenopprettingskopi: { $reason }
edit-duplicate-line = Dupliser linje
edit-delete-line = Slett linje
edit-move-line-up = Flytt linje opp
edit-move-line-down = Flytt linje ned
file-close-tab = Lukk fane
file-reopen-tab = Åpne lukket fane på nytt
window-next-tab = Neste fane
window-prev-tab = Forrige fane
tab-close = Lukk fane
edit-find = Søk…
edit-find-next = Søk etter neste
edit-find-prev = Søk etter forrige
edit-replace = Erstatt…
find-placeholder = Søk
replace-placeholder = Erstatt med
find-match-case = Skill mellom store og små bokstaver
find-whole-word = Hele ord
find-regex = Regulært uttrykk
find-no-results = Ingen treff
find-count = { $total ->
    [one] 1 treff
   *[other] { $total } treff
    }
find-count-current = { $current } av { $total }
find-prev = Forrige treff (Shift+Enter)
find-next = Neste treff (Enter)
find-close = Lukk (Esc)
replace-one = Erstatt
replace-all = Erstatt alle
status-replaced = { $count ->
    [one] Erstattet 1 treff
   *[other] Erstattet { $count } treff
    }
language-plain = Ren tekst
menu-view = Vis
app-settings = Innstillinger…
view-zoom-in = Zoom inn
view-zoom-out = Zoom ut
view-zoom-reset = Faktisk størrelse
settings-title = Innstillinger
settings-font-size = Skriftstørrelse
settings-tab-width = Tabulatorbredde
settings-insert-spaces = Sett inn mellomrom ved trykk på Tab
settings-theme = Utseende
settings-theme-system = Følg systemet
settings-theme-light = Lyst
settings-theme-dark = Mørkt
settings-restore-session = Åpne filer fra forrige økt på nytt
settings-language = Språk
settings-language-system = Følg systemet
settings-language-note = Språkendringer trer i kraft når Sanain startes på nytt.
settings-save-failed = Kunne ikke lagre innstillingene: { $reason }
view-command-palette = Kommandopalett…
palette-placeholder = Skriv inn en kommando
view-markdown-source = Markdown-kilde
view-markdown-split = Markdown-kilde og forhåndsvisning
view-markdown-preview = Markdown-forhåndsvisning
preview-too-large = Dokumentet er for stort til å forhåndsvises.
preview-mermaid-unsupported = Mermaid-diagrammer gjengis ikke ennå; kilden vises.
menu-tools = Verktøy
tools-json-validate = Valider JSON
tools-json-format = Formater JSON
tools-json-minify = Minimer JSON
tools-sort-lines = Sorter linjer
tools-sort-lines-desc = Sorter linjer synkende
tools-unique-lines = Fjern dupliserte linjer
tools-trim-trailing = Fjern etterfølgende mellomrom
tools-upper-case = Store bokstaver
tools-lower-case = Små bokstaver
tools-to-lf = Konverter linjeskift til LF
tools-to-crlf = Konverter linjeskift til CRLF
tools-json-valid = JSON er gyldig
tools-done = Ferdig. Angre tilbakestiller endringen.
tools-no-change = Ingenting å endre
tools-json-error = Ugyldig JSON på linje { $line }, kolonne { $column }: { $reason }
json-reason-unterminated-string = uavsluttet streng
json-reason-invalid-u-escape = ugyldig \u-escape
json-reason-invalid-escape = ugyldig escape-sekvens
json-reason-control-character-in-string = kontrolltegn i streng
json-reason-invalid-number = ugyldig tall
json-reason-unexpected-character = uventet tegn
json-reason-unexpected-content-after-the-value = uventet innhold etter verdien
json-reason-unexpected-comma = uventet komma
json-reason-expected-a-string-key = forventet en strengnøkkel
json-reason-expected = forventet ':'
json-reason-expected-or-a-closing-bracket = forventet ',' eller en avsluttende parentes
json-reason-expected-a-value = forventet en verdi
json-reason-empty-document = tomt dokument
json-reason-unexpected-end-of-input = uventet slutt på inndata
json-reason-too-deeply-nested-to-format = for dypt nestet til å formateres
tools-compare = Sammenlign med…
compare-need-two = Åpne en annen fane å sammenligne med.
compare-pick-title = Sammenlign { $name } med
compare-count = { $count ->
    [0] Ingen forskjeller
    [one] 1 endring
   *[other] { $count } endringer
    }
compare-close = Lukk sammenligning
compare-to-left = Kopier denne endringen til venstre
compare-to-right = Kopier denne endringen til høyre
compare-stale = Et dokument ble endret under sammenligningen. Sammenligningen er oppdatert; prøv igjen.
view-structured = Strukturert visning (CSV, JSON, XML, YAML)
csv-delimiter = Skilletegn
csv-rows = { $count ->
    [one] 1 rad
   *[other] { $count } rader
    }
csv-unterminated = Uavsluttet anførselstegn som starter på linje { $line }
csv-truncated = Viser de første 200 000 radene
json-more = … { $count } til
dialog-open-link-title = Vil du åpne denne lenken i nettleseren?
button-open = Åpne
link-blocked = Denne typen lenke åpnes ikke av sikkerhetshensyn.
link-not-found = Fant ikke den lenkede filen: { $target }
enc-windows-1252 = Vesteuropeisk
enc-iso-8859-15 = Vesteuropeisk
enc-windows-1250 = Sentraleuropeisk
enc-iso-8859-2 = Sentraleuropeisk
enc-windows-1251 = Kyrillisk
enc-koi8-r = Kyrillisk
enc-windows-1253 = Gresk
enc-windows-1254 = Tyrkisk
enc-windows-1255 = Hebraisk
enc-windows-1256 = Arabisk
enc-windows-1257 = Baltisk
enc-windows-874 = Thai
enc-shift_jis = Japansk
enc-euc-jp = Japansk
enc-gb18030 = Kinesisk, forenklet
enc-big5 = Kinesisk, tradisjonell
enc-euc-kr = Koreansk
file-reopen-encoding = Åpne på nytt med koding…
file-save-encoding = Lagre med koding…
encoding-reopen-title = Åpne på nytt med koding
encoding-save-title = Lagre med koding
encoding-not-exact = Denne kodingen kan ikke åpne filen uten å endre den: { $reason }
encoding-unencodable = «{ $character }» kan ikke lagres som { $encoding }. Velg en annen koding eller endre teksten; ingenting ble skrevet.
encoding-save-first = Lagre eller angre endringene før du åpner på nytt med en annen koding.
image-remote = Eksterne bilder lastes ikke inn.
image-blocked = Bare bilder i dokumentets mappe vises.
image-unsupported = Bildeformatet støttes ikke.
image-too-large = Bildet er for stort til å vises.
image-unreadable = Bildet kunne ikke leses.
settings-theme-high-contrast = Høy kontrast
view-fold = Brett sammen
view-unfold = Brett ut
view-unfold-all = Brett ut alle
settings-shortcuts = Hurtigtaster
settings-keymap-preset = Forhåndsinnstilling
settings-keymap-default = Sanain
settings-keymap-notepadpp = Notepad++
settings-shortcut-change = Endre
settings-shortcut-reset = Tilbakestill
settings-shortcut-press = Trykk en hurtigtast… (Esc avbryter)
settings-shortcut-moved = Hurtigtasten ble flyttet fra:
edit-find-in-folder = Søk i mappe…
folder-search-placeholder = Søk i mappe
folder-search-run = Søk
folder-search-stop = Stopp
folder-search-no-folder = Velg mappe
folder-search-summary = { $count ->
    [one] 1 treff
   *[other] { $count } treff
    } i { $files ->
    [one] 1 fil
   *[other] { $files } filer
    }
app-check-updates = Se etter oppdateringer…
update-checking = Ser etter oppdateringer…
update-up-to-date = Sanain { $version } er den nyeste versjonen.
update-no-releases = Ingen versjoner av Sanain er publisert ennå.
update-failed = Kunne ikke se etter oppdateringer: { $reason }
update-invalid = uventet svar
update-available-title = Sanain { $version } er tilgjengelig
update-available-body = Åpne nedlastingssiden for å hente den. Sanain laster ikke ned eller installerer noe av seg selv.
update-open-page = Åpne nedlastingsside
update-later = Senere
settings-check-updates = Se etter oppdateringer én gang om dagen
settings-check-updates-detail = Spør github.com om den nyeste utgivelsen. Ingenting annet sendes.
file-local-history = Lokal historikk…
file-clear-history = Tøm lokal historikk…
history-title = Lokal historikk for { $name }
history-open = Åpne
history-restore = Gjenopprett
history-empty = Ingen lagrede versjoner ennå. En kopi beholdes hver gang du lagrer.
history-unreadable = Denne versjonen kunne ikke leses.
history-just-now = Akkurat nå
history-minutes-ago = { $count ->
    [one] for 1 minutt siden
   *[other] for { $count } minutter siden
    }
history-hours-ago = { $count ->
    [one] for 1 time siden
   *[other] for { $count } timer siden
    }
history-days-ago = { $count ->
    [one] for 1 dag siden
   *[other] for { $count } dager siden
    }
history-purge-title = Slette all lokal historikk?
history-purge-body = Lagrede kopier av alle filer fjernes fra denne datamaskinen. Selve filene endres ikke.
history-purge-confirm = Slett historikk
history-purged = Lokal historikk er slettet.
settings-local-history = Behold lokal historikk for lagrede filer
settings-local-history-detail = Beholder opptil 50 versjoner per fil i 30 dager, kun på denne datamaskinen.
file-open-log = Åpne i loggviser…
log-lines = { $count ->
    [one] 1 linje
   *[other] { $count } linjer
    }
log-indexing = Leser… { $percent } %
log-matches = { $count ->
    [one] 1 samsvarende linje
   *[other] { $count } samsvarende linjer
    }
log-filter-hint = Filtrer linjer
log-bad-pattern = Ugyldig mønster
log-follow = Følg
log-close = Lukk viser
log-opened-large = { $name } er stor, så den ble åpnet skrivebeskyttet i loggviseren.
tab-pin = Fest fane
tab-unpin = Løsne fane
edit-add-cursor-above = Legg til markør over
edit-add-cursor-below = Legg til markør under
view-word-wrap = Tekstbryting
status-wrap-on = Lange linjer brytes nå ved vindusbredden.
status-wrap-off = Lange linjer brytes ikke lenger.
settings-word-wrap = Bryt lange linjer
edit-go-to-line = Gå til linje…
edit-go-to-heading = Gå til overskrift…
goto-line-hint = Linje eller linje:kolonne
goto-line-note = { $count ->
    [one] Dokumentet har 1 linje.
   *[other] Dokumentet har { $count } linjer.
    }
goto-line-invalid = Skriv inn et linjenummer, eventuelt etterfulgt av :kolonne.
goto-heading-hint = Skriv for å filtrere overskrifter
goto-heading-none = Dette dokumentet har ingen overskrifter.
format-xml = XML
format-yaml = YAML
tools-tree-error = Ugyldig { $format } på linje { $line }, kolonne { $column }: { $reason }
edit-select-next-occurrence = Merk neste forekomst
tools-record-macro = Start eller stopp makroopptak
tools-play-macro = Spill av makro
macro-recording = Tar opp makro
macro-recorded = { $count ->
    [one] Makro tatt opp: 1 trinn.
   *[other] Makro tatt opp: { $count } trinn.
    }
tools-compare-saved = Sammenlign med lagret versjon
tools-compare-clipboard = Sammenlign med utklippstavlen
compare-saved-label = { $name } (på disk)
compare-clipboard-label = Utklippstavle
compare-saved-failed = Kunne ikke lese den lagrede filen: { $reason }
compare-clipboard-empty = Utklippstavlen inneholder ingen tekst å sammenligne.
error-save-disk-full = { $name } ble ikke lagret: disken er full. Filen på disken er uendret, og teksten din er fortsatt her. Frigjør plass, eller bruk Lagre som for å lagre den et annet sted.
error-save-permission = { $name } ble ikke lagret: du har ikke tillatelse til å skrive der. Filen på disken er uendret. Bruk Lagre som for å lagre den et annet sted.
error-save-read-only-volume = { $name } ble ikke lagret: disken er skrivebeskyttet. Filen på disken er uendret. Bruk Lagre som for å lagre den et annet sted.
replace-preview-title = { $count ->
    [one] Erstatte 1 treff?
   *[other] Erstatte { $count } treff?
    }
replace-preview-confirm = { $count ->
    [one] Erstatt 1
   *[other] Erstatt { $count }
    }
replace-preview-more = Bare de første 200 endrede linjene vises.
replace-preview-stale = Dokumentet er endret; ingenting ble erstattet. Kjør Erstatt alle på nytt.
view-split-editor = Del redigeringsprogram
encoding-detected = Åpnet som { $encoding } (oppdaget fra innholdet i filen).
encoding-suggested = Foreslått ut fra innholdet i filen: { $encoding }. Kontroller forhåndsvisningen før du åpner.
app-about = Om Sanain
menu-help = Hjelp
about-tagline = Ordenes kraft.
about-version = Versjon { $version }
about-license = Fri og åpen kildekode under Mozilla Public License 2.0.
about-ai = Bygget av KI. Ledet av mennesker. Drevet av fellesskapet.
about-notices = Tredjepartskomponenter og lisensene deres er oppført i THIRD_PARTY_NOTICES, som følger med hver utgivelse.
about-source = Kildekode og feilrapporter på GitHub
