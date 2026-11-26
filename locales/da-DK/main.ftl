# Machine translation by Claude (AI); review by a native speaker required before release.
app-name = Sanain
menu-file = Filer
file-open = Åbn…
file-save = Gem
status-tabs = { $count ->
    [one] Én fane
   *[other] { $count } faner
    }
error-open = Kunne ikke åbne { $name }.
menu-edit = Rediger
file-new = Ny fane
file-save-as = Gem som…
edit-undo = Fortryd
edit-redo = Annuller fortryd
edit-select-all = Marker alt
untitled = Uden navn
window-title = { $dirty ->
    [yes] ● { $name } — Sanain
   *[no] { $name } — Sanain
    }
status-position = Linje { $line }, kol. { $column }
status-selections = { $count ->
    [one] 1 markering
   *[other] { $count } markeringer
    }
status-saved = Gemte { $name }
encoding-utf8 = UTF-8
encoding-utf8-bom = UTF-8 med BOM
encoding-utf16le = UTF-16 LE
encoding-utf16be = UTF-16 BE
line-ending-lf = LF
line-ending-crlf = CRLF
line-ending-cr = CR
line-ending-mixed = Blandede linjeskift
editor-accessible-name = Teksteditor
button-save = Gem
button-dont-save = Gem ikke
button-cancel = Annuller
button-ok = OK
button-overwrite = Overskriv
button-save-as = Gem som…
dialog-unsaved-title = Vil du gemme ændringerne i { $name }?
dialog-unsaved-body = Dine ændringer går tabt, hvis du ikke gemmer dem.
dialog-conflict-title = { $name } er ændret på disken
dialog-conflict-body = Et andet program har ændret denne fil, efter du åbnede den. Hvis du overskriver, erstattes disse ændringer.
dialog-readonly-title = { $name } er skrivebeskyttet
dialog-readonly-body = Gem i stedet en kopi under et andet navn.
error-open-encoding = { $name } er ikke UTF-8- eller UTF-16-tekst. Filen blev ikke åbnet, så indholdet er uændret. Brug Filer > Genåbn med tegnkodning for at vælge tegnkodning.
error-save = Kunne ikke gemme { $name }: { $reason }
menu-window = Vindue
edit-cut = Klip
edit-copy = Kopier
edit-paste = Indsæt
app-quit = Afslut Sanain
app-hide = Skjul Sanain
app-hide-others = Skjul andre
app-show-all = Vis alle
window-minimize = Minimer
window-zoom = Zoom
status-recovered = Ikke-gemte ændringer fra sidste session er gendannet
status-recovery-failed = Kunne ikke gemme en gendannelseskopi: { $reason }
edit-duplicate-line = Dupliker linje
edit-delete-line = Slet linje
edit-move-line-up = Flyt linje op
edit-move-line-down = Flyt linje ned
file-close-tab = Luk fane
file-reopen-tab = Genåbn lukket fane
window-next-tab = Næste fane
window-prev-tab = Forrige fane
tab-close = Luk fane
edit-find = Søg…
edit-find-next = Søg efter næste
edit-find-prev = Søg efter forrige
edit-replace = Erstat…
find-placeholder = Søg
replace-placeholder = Erstat med
find-match-case = Forskel på store og små bogstaver
find-whole-word = Hele ord
find-regex = Regulært udtryk
find-no-results = Ingen resultater
find-count = { $total ->
    [one] 1 forekomst
   *[other] { $total } forekomster
    }
find-count-current = { $current } af { $total }
find-prev = Forrige forekomst (Shift+Enter)
find-next = Næste forekomst (Enter)
find-close = Luk (Esc)
replace-one = Erstat
replace-all = Erstat alle
status-replaced = { $count ->
    [one] Erstattede 1 forekomst
   *[other] Erstattede { $count } forekomster
    }
language-plain = Almindelig tekst
menu-view = Vis
app-settings = Indstillinger…
view-zoom-in = Zoom ind
view-zoom-out = Zoom ud
view-zoom-reset = Faktisk størrelse
settings-title = Indstillinger
settings-font-size = Skriftstørrelse
settings-tab-width = Tabulatorbredde
settings-insert-spaces = Indsæt mellemrum ved tryk på Tab
settings-theme = Udseende
settings-theme-system = Følg systemet
settings-theme-light = Lyst
settings-theme-dark = Mørkt
settings-restore-session = Genåbn filer fra sidste session
settings-language = Sprog
settings-language-system = Følg systemet
settings-language-note = Sprogændringer træder i kraft, når Sanain genstartes.
settings-save-failed = Kunne ikke gemme indstillingerne: { $reason }
view-command-palette = Kommandopalet…
palette-placeholder = Skriv en kommando
view-markdown-source = Markdown-kilde
view-markdown-split = Markdown-kilde og eksempel
view-markdown-preview = Markdown-eksempel
preview-too-large = Dokumentet er for stort til at blive vist som eksempel.
preview-mermaid-unsupported = Mermaid-diagrammer gengives endnu ikke; kilden vises.
menu-tools = Værktøjer
tools-json-validate = Valider JSON
tools-json-format = Formater JSON
tools-json-minify = Minimer JSON
tools-sort-lines = Sorter linjer
tools-sort-lines-desc = Sorter linjer faldende
tools-unique-lines = Fjern dublerede linjer
tools-trim-trailing = Fjern efterstillede mellemrum
tools-upper-case = Store bogstaver
tools-lower-case = Små bogstaver
tools-to-lf = Konverter linjeskift til LF
tools-to-crlf = Konverter linjeskift til CRLF
tools-json-valid = JSON er gyldig
tools-done = Færdig. Fortryd tilbagefører ændringen.
tools-no-change = Intet at ændre
tools-json-error = Ugyldig JSON på linje { $line }, kolonne { $column }: { $reason }
json-reason-unterminated-string = uafsluttet streng
json-reason-invalid-u-escape = ugyldig \u-escape
json-reason-invalid-escape = ugyldig escape-sekvens
json-reason-control-character-in-string = kontroltegn i streng
json-reason-invalid-number = ugyldigt tal
json-reason-unexpected-character = uventet tegn
json-reason-unexpected-content-after-the-value = uventet indhold efter værdien
json-reason-unexpected-comma = uventet komma
json-reason-expected-a-string-key = forventede en strengnøgle
json-reason-expected = forventede ':'
json-reason-expected-or-a-closing-bracket = forventede ',' eller en afsluttende parentes
json-reason-expected-a-value = forventede en værdi
json-reason-empty-document = tomt dokument
json-reason-unexpected-end-of-input = uventet slutning på input
json-reason-too-deeply-nested-to-format = for dybt indlejret til at formatere
tools-compare = Sammenlign med…
compare-need-two = Åbn en anden fane at sammenligne med.
compare-pick-title = Sammenlign { $name } med
compare-count = { $count ->
    [0] Ingen forskelle
    [one] 1 ændring
   *[other] { $count } ændringer
    }
compare-close = Luk sammenligning
compare-to-left = Kopier denne ændring til venstre
compare-to-right = Kopier denne ændring til højre
compare-stale = Et dokument blev ændret under sammenligningen. Sammenligningen er opdateret; prøv igen.
view-structured = Struktureret visning (CSV, JSON, XML, YAML)
csv-delimiter = Skilletegn
csv-rows = { $count ->
    [one] 1 række
   *[other] { $count } rækker
    }
csv-unterminated = Uafsluttet anførselstegn, der starter på linje { $line }
csv-truncated = Viser de første 200.000 rækker
json-more = … { $count } mere
dialog-open-link-title = Vil du åbne dette link i din webbrowser?
button-open = Åbn
link-blocked = Denne type link åbnes ikke af sikkerhedshensyn.
link-not-found = Den linkede fil blev ikke fundet: { $target }
enc-windows-1252 = Vesteuropæisk
enc-iso-8859-15 = Vesteuropæisk
enc-windows-1250 = Centraleuropæisk
enc-iso-8859-2 = Centraleuropæisk
enc-windows-1251 = Kyrillisk
enc-koi8-r = Kyrillisk
enc-windows-1253 = Græsk
enc-windows-1254 = Tyrkisk
enc-windows-1255 = Hebraisk
enc-windows-1256 = Arabisk
enc-windows-1257 = Baltisk
enc-windows-874 = Thai
enc-shift_jis = Japansk
enc-euc-jp = Japansk
enc-gb18030 = Kinesisk, forenklet
enc-big5 = Kinesisk, traditionelt
enc-euc-kr = Koreansk
file-reopen-encoding = Genåbn med tegnkodning…
file-save-encoding = Gem med tegnkodning…
encoding-reopen-title = Genåbn med tegnkodning
encoding-save-title = Gem med tegnkodning
encoding-not-exact = Denne tegnkodning kan ikke åbne filen uden at ændre den: { $reason }
encoding-unencodable = “{ $character }” kan ikke gemmes som { $encoding }. Vælg en anden tegnkodning, eller ret teksten; intet blev skrevet.
encoding-save-first = Gem eller fortryd dine ændringer, før du genåbner med en anden tegnkodning.
image-remote = Eksterne billeder indlæses ikke.
image-blocked = Kun billeder i dokumentets mappe vises.
image-unsupported = Billedformatet understøttes ikke.
image-too-large = Billedet er for stort til at blive vist.
image-unreadable = Billedet kunne ikke læses.
settings-theme-high-contrast = Høj kontrast
view-fold = Fold sammen
view-unfold = Fold ud
view-unfold-all = Fold alle ud
settings-shortcuts = Tastaturgenveje
settings-keymap-preset = Forudindstilling
settings-keymap-default = Sanain
settings-keymap-notepadpp = Notepad++
settings-shortcut-change = Skift
settings-shortcut-reset = Nulstil
settings-shortcut-press = Tryk på en genvej… (Esc annullerer)
settings-shortcut-moved = Genvejen er flyttet fra:
edit-find-in-folder = Søg i mappe…
folder-search-placeholder = Søg i mappe
folder-search-run = Søg
folder-search-stop = Stop
folder-search-no-folder = Vælg mappe
folder-search-summary = { $count ->
    [one] 1 forekomst
   *[other] { $count } forekomster
    } i { $files ->
    [one] 1 fil
   *[other] { $files } filer
    }
app-check-updates = Søg efter opdateringer…
update-checking = Søger efter opdateringer…
update-up-to-date = Sanain { $version } er den nyeste version.
update-no-releases = Der er endnu ikke udgivet nogen versioner af Sanain.
update-failed = Kunne ikke søge efter opdateringer: { $reason }
update-invalid = uventet svar
update-available-title = Sanain { $version } er tilgængelig
update-available-body = Åbn downloadsiden for at hente den. Sanain downloader eller installerer ikke noget af sig selv.
update-open-page = Åbn downloadside
update-later = Senere
settings-check-updates = Søg efter opdateringer én gang om dagen
settings-check-updates-detail = Spørger github.com om den seneste udgivelse. Der sendes intet andet.
file-local-history = Lokal historik…
file-clear-history = Ryd lokal historik…
history-title = Lokal historik for { $name }
history-open = Åbn
history-restore = Gendan
history-empty = Ingen gemte versioner endnu. Der gemmes en kopi, hver gang du gemmer.
history-unreadable = Denne version kunne ikke læses.
history-just-now = Lige nu
history-minutes-ago = { $count ->
    [one] for 1 minut siden
   *[other] for { $count } minutter siden
    }
history-hours-ago = { $count ->
    [one] for 1 time siden
   *[other] for { $count } timer siden
    }
history-days-ago = { $count ->
    [one] for 1 dag siden
   *[other] for { $count } dage siden
    }
history-purge-title = Slet al lokal historik?
history-purge-body = Gemte kopier af alle filer fjernes fra denne computer. Selve filerne ændres ikke.
history-purge-confirm = Slet historik
history-purged = Lokal historik er slettet.
settings-local-history = Behold lokal historik for gemte filer
settings-local-history-detail = Beholder op til 50 versioner pr. fil i 30 dage, kun på denne computer.
file-open-log = Åbn i logfremviser…
log-lines = { $count ->
    [one] 1 linje
   *[other] { $count } linjer
    }
log-indexing = Læser… { $percent } %
log-matches = { $count ->
    [one] 1 matchende linje
   *[other] { $count } matchende linjer
    }
log-filter-hint = Filtrer linjer
log-bad-pattern = Ugyldigt mønster
log-follow = Følg
log-close = Luk fremviser
log-opened-large = { $name } er stor, så den blev åbnet skrivebeskyttet i logfremviseren.
tab-pin = Fastgør fane
tab-unpin = Frigør fane
edit-add-cursor-above = Tilføj markør ovenfor
edit-add-cursor-below = Tilføj markør nedenfor
view-word-wrap = Automatisk linjeskift
status-wrap-on = Lange linjer ombrydes nu efter vinduets bredde.
status-wrap-off = Lange linjer ombrydes ikke længere.
settings-word-wrap = Ombryd lange linjer
edit-go-to-line = Gå til linje…
edit-go-to-heading = Gå til overskrift…
goto-line-hint = Linje eller linje:kolonne
goto-line-note = { $count ->
    [one] Dokumentet har 1 linje.
   *[other] Dokumentet har { $count } linjer.
    }
goto-line-invalid = Angiv et linjenummer, eventuelt efterfulgt af :kolonne.
goto-heading-hint = Skriv for at filtrere overskrifter
goto-heading-none = Dette dokument har ingen overskrifter.
format-xml = XML
format-yaml = YAML
tools-tree-error = Ugyldig { $format } på linje { $line }, kolonne { $column }: { $reason }
edit-select-next-occurrence = Marker næste forekomst
tools-record-macro = Start eller stop makrooptagelse
tools-play-macro = Afspil makro
macro-recording = Optager makro
macro-recorded = { $count ->
    [one] Makro optaget: 1 trin.
   *[other] Makro optaget: { $count } trin.
    }
tools-compare-saved = Sammenlign med gemt version
tools-compare-clipboard = Sammenlign med udklipsholder
compare-saved-label = { $name } (på disk)
compare-clipboard-label = Udklipsholder
compare-saved-failed = Den gemte fil kunne ikke læses: { $reason }
compare-clipboard-empty = Udklipsholderen indeholder ingen tekst at sammenligne.
error-save-disk-full = { $name } blev ikke gemt: disken er fuld. Filen på disken er uændret, og din tekst er her stadig. Frigør noget plads, eller brug Gem som for at gemme den et andet sted.
error-save-permission = { $name } blev ikke gemt: du har ikke tilladelse til at skrive den der. Filen på disken er uændret. Brug Gem som for at gemme den et andet sted.
error-save-read-only-volume = { $name } blev ikke gemt: disken er skrivebeskyttet. Filen på disken er uændret. Brug Gem som for at gemme den et andet sted.
replace-preview-title = { $count ->
    [one] Erstat 1 forekomst?
   *[other] Erstat { $count } forekomster?
    }
replace-preview-confirm = { $count ->
    [one] Erstat 1
   *[other] Erstat { $count }
    }
replace-preview-more = Kun de første 200 ændrede linjer vises.
replace-preview-stale = Dokumentet er ændret; intet blev erstattet. Kør Erstat alle igen.
view-split-editor = Opdel editor
encoding-detected = Åbnet som { $encoding } (registreret ud fra filens indhold).
encoding-suggested = Foreslået ud fra filens indhold: { $encoding }. Kontrollér forhåndsvisningen, før du åbner.
app-about = Om Sanain
menu-help = Hjælp
about-tagline = Ordenes kraft.
about-version = Version { $version }
about-license = Fri og open source under Mozilla Public License 2.0.
about-ai = Bygget af AI. Ledet af mennesker. Drevet af fællesskabet.
about-notices = Tredjepartskomponenter og deres licenser er angivet i THIRD_PARTY_NOTICES, som følger med hver udgivelse.
about-source = Kildekode og fejlrapporter på GitHub
tools-xml-validate = Valider XML
tools-yaml-validate = Valider YAML
tools-markup-valid = { $format } er gyldig
file-open-recent = Åbn seneste…
file-save-all = Gem alle
file-revert = Gendan gemt version
recent-hint = Skriv for at filtrere seneste filer
recent-none = Ingen seneste filer endnu.
revert-title = Kassér ændringer i { $name }?
revert-body = Filen genindlæses fra disken. Ikke-gemte ændringer går tabt og kan ikke fortrydes.
revert-confirm = Kassér og genindlæs
revert-done = { $name } er genindlæst fra disken.
status-saved-all = { $count ->
    [0] Intet at gemme.
    [one] 1 fil er gemt.
   *[other] { $count } filer er gemt.
    }
edit-toggle-comment = Slå linjekommentar til/fra
edit-indent = Indryk linjer
edit-outdent = Fjern indrykning af linjer
comment-unsupported = Denne filtype har ingen linjekommentarer.
tab-close-others = Luk andre faner
tab-close-all = Luk alle faner
file-copy-path = Kopiér filsti
file-reveal-macos = Vis i Finder
file-reveal-windows = Vis i Stifinder
file-reveal-other = Vis i filhåndtering
status-path-copied = Filstien blev kopieret.
tabs-kept-unsaved = { $count ->
    [one] 1 fane med ikke-gemte ændringer blev holdt åben.
   *[other] { $count } faner med ikke-gemte ændringer blev holdt åbne.
    }
status-indent-tabs = Tabulatorer
status-indent-spaces = Mellemrum: { $count }
find-searching = Søger…
compare-ignore-whitespace = Ignorer mellemrum
tools-compare-file = Sammenlign med fil…
compare-file-failed = Kunne ikke sammenligne med { $name }: { $reason }
cli-help =
    Brug:
      sanain [FIL…]                  åbn filer
      sanain --diff VENSTRE HØJRE    sammenlign to filer side om side
      sanain --version               vis versionen
      sanain --help                  vis denne hjælp
    Brug -- før filnavne, der begynder med en bindestreg.
cli-unknown-option = Ukendt tilvalg { $option }.
cli-diff-needs-two = --diff kræver to filer: sanain --diff VENSTRE HØJRE
window-move-tab-left = Flyt fane til venstre
window-move-tab-right = Flyt fane til højre
edit-skip-occurrence = Spring til næste forekomst
tools-play-macro-times = Afspil makro flere gange…
macro-times-hint = Antal gange
macro-times-note = Fra 1 til { $max }
macro-times-invalid = Angiv et tal fra 1 til { $max }.
view-outline = Disposition
outline-empty = Ingen overskrifter i dette dokument.
outline-close = Luk disposition
folder-replace-placeholder = Erstat med
folder-replace-run = Erstat i filer…
replace-files-title = Erstat { $count ->
    [one] 1 forekomst
   *[other] { $count } forekomster
    } i { $files ->
    [one] 1 fil
   *[other] { $files } filer
    }?
replace-files-body = Filer, der er åbne i en fane, ændres dér og kan fortrydes; du skal selv gemme dem. Andre filer gemmes nu, og deres tidligere indhold bevares i den lokale historik, hvis den er slået til. En fil, der er ændret på disken i mellemtiden, springes over.
replace-files-confirm = Erstat
replace-files-done = Erstattede { $count ->
    [one] 1 forekomst
   *[other] { $count } forekomster
    } i { $files ->
    [one] 1 fil
   *[other] { $files } filer
    } (åbne i faner: { $open }, ikke gemt).
replace-files-failed = { $failed ->
    [one] 1 fil blev ikke ændret:
   *[other] { $failed } filer blev ikke ændret; første:
    } { $first }
image-outside = Dette billede ligger uden for dokumentets mappe.
image-allow = Vis billeder fra denne mappe
cli-help-integration =
    Integration med filhåndteringen (kun den aktuelle bruger):
      sanain --register           føj Sanain til “Åbn med” (Linux, Windows)
      sanain --unregister         fjern den igen
cli-registered = Sanain blev føjet til “Åbn med”. { $place }
cli-unregistered = Sanain blev fjernet fra “Åbn med”.
cli-register-failed = Integrationen med filhåndteringen kunne ikke ændres: { $reason }
cli-register-macos = På macOS skal intet registreres: Når Sanain.app ligger i mappen Programmer, tilbyder Finder den under “Åbn med”.
settings-tab-general = Generelt
settings-tab-editor = Editor
settings-tab-appearance = Udseende
settings-shortcut-search = Søg efter kommandoer eller taster
settings-font = Skrifttype
settings-font-default = Indbygget monospace
settings-font-loading = Søger efter installerede skrifttyper…
settings-font-missing = Skrifttypen { $font } blev ikke fundet; den indbyggede skrifttype bruges.
settings-theme-more = Flere temaer
settings-theme-user-note = Dine egne temaer: .theme-filer i { $folder } (se docs/THEMES.md).
language-auto = Registrer automatisk
language-choose = Vælg sproget for denne fane
tools-xml-format = Formater XML
tools-save-macro = Gem makro…
tools-play-saved-macro = Afspil gemt makro…
tools-delete-saved-macro = Slet gemt makro…
macro-save-hint = Makronavn
macro-save-note = En gemt makro med samme navn erstattes.
macro-save-empty = Intet at gemme: optag først en makro.
macro-saved = Makro gemt: { $name }
macro-deleted = Makro slettet: { $name }
macro-name-invalid = Angiv et navn på 1 til { $max } tegn på én linje.
macro-save-too-large = Makroen er for lang til at blive gemt.
macro-save-full = Der kan højst gemmes { $max } makroer. Slet først en.
macro-save-failed = Makroerne kunne ikke gemmes: { $error }
macro-saved-hint = Skriv for at filtrere gemte makroer
macro-saved-none = Ingen gemte makroer endnu.
