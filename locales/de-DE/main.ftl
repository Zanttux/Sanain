# Machine translation by Claude (AI); review by a native speaker required before release.
app-name = Sanain
menu-file = Datei
file-open = Öffnen…
file-save = Sichern
status-tabs = { $count ->
    [one] Ein Tab
   *[other] { $count } Tabs
    }
error-open = { $name } konnte nicht geöffnet werden.
menu-edit = Bearbeiten
file-new = Neuer Tab
file-save-as = Sichern unter…
edit-undo = Widerrufen
edit-redo = Wiederholen
edit-select-all = Alles auswählen
untitled = Unbenannt
window-title = { $dirty ->
    [yes] ● { $name } — Sanain
   *[no] { $name } — Sanain
    }
status-position = Z. { $line }, Sp. { $column }
status-selections = { $count ->
    [one] 1 Auswahl
   *[other] { $count } Auswahlen
    }
status-saved = { $name } gesichert
encoding-utf8 = UTF-8
encoding-utf8-bom = UTF-8 mit BOM
encoding-utf16le = UTF-16 LE
encoding-utf16be = UTF-16 BE
line-ending-lf = LF
line-ending-crlf = CRLF
line-ending-cr = CR
line-ending-mixed = Gemischte Zeilenenden
editor-accessible-name = Texteditor
button-save = Sichern
button-dont-save = Nicht sichern
button-cancel = Abbrechen
button-ok = OK
button-overwrite = Überschreiben
button-save-as = Sichern unter…
dialog-unsaved-title = Änderungen an { $name } sichern?
dialog-unsaved-body = Ihre Änderungen gehen verloren, wenn Sie sie nicht sichern.
dialog-conflict-title = { $name } wurde auf dem Datenträger geändert
dialog-conflict-body = Ein anderes Programm hat diese Datei geändert, nachdem Sie sie geöffnet haben. Beim Überschreiben gehen diese Änderungen verloren.
dialog-readonly-title = { $name } ist schreibgeschützt
dialog-readonly-body = Sichern Sie stattdessen eine Kopie unter einem anderen Namen.
error-open-encoding = { $name } ist kein UTF-8- oder UTF-16-Text. Die Datei wurde nicht geöffnet, ihr Inhalt ist unverändert. Wählen Sie die Codierung unter Datei > Mit Codierung erneut öffnen.
error-save = { $name } konnte nicht gesichert werden: { $reason }
menu-window = Fenster
edit-cut = Ausschneiden
edit-copy = Kopieren
edit-paste = Einsetzen
app-quit = Sanain beenden
app-hide = Sanain ausblenden
app-hide-others = Andere ausblenden
app-show-all = Alle einblenden
window-minimize = Im Dock ablegen
window-zoom = Zoomen
status-recovered = Nicht gesicherte Änderungen der letzten Sitzung wiederhergestellt
status-recovery-failed = Wiederherstellungskopie konnte nicht gesichert werden: { $reason }
edit-duplicate-line = Zeile duplizieren
edit-delete-line = Zeile löschen
edit-move-line-up = Zeile nach oben verschieben
edit-move-line-down = Zeile nach unten verschieben
file-close-tab = Tab schließen
file-reopen-tab = Geschlossenen Tab erneut öffnen
window-next-tab = Nächster Tab
window-prev-tab = Vorheriger Tab
tab-close = Tab schließen
edit-find = Suchen…
edit-find-next = Weitersuchen
edit-find-prev = Rückwärts suchen
edit-replace = Ersetzen…
find-placeholder = Suchen
replace-placeholder = Ersetzen durch
find-match-case = Groß-/Kleinschreibung beachten
find-whole-word = Ganzes Wort
find-regex = Regulärer Ausdruck
find-no-results = Keine Ergebnisse
find-count = { $total ->
    [one] 1 Treffer
   *[other] { $total } Treffer
    }
find-count-current = { $current } von { $total }
find-prev = Vorheriger Treffer (Shift+Enter)
find-next = Nächster Treffer (Enter)
find-close = Schließen (Esc)
replace-one = Ersetzen
replace-all = Alle ersetzen
status-replaced = { $count ->
    [one] 1 Treffer ersetzt
   *[other] { $count } Treffer ersetzt
    }
language-plain = Reiner Text
menu-view = Ansicht
app-settings = Einstellungen…
view-zoom-in = Vergrößern
view-zoom-out = Verkleinern
view-zoom-reset = Originalgröße
settings-title = Einstellungen
settings-font-size = Schriftgröße
settings-tab-width = Tabulatorbreite
settings-insert-spaces = Leerzeichen beim Drücken von Tab einfügen
settings-theme = Erscheinungsbild
settings-theme-system = Wie System
settings-theme-light = Hell
settings-theme-dark = Dunkel
settings-restore-session = Dateien der letzten Sitzung erneut öffnen
settings-language = Sprache
settings-language-system = Wie System
settings-language-note = Sprachänderungen werden nach einem Neustart von Sanain wirksam.
settings-save-failed = Einstellungen konnten nicht gesichert werden: { $reason }
view-command-palette = Befehlspalette…
palette-placeholder = Befehl eingeben
view-markdown-source = Markdown-Quelltext
view-markdown-split = Markdown-Quelltext und Vorschau
view-markdown-preview = Markdown-Vorschau
preview-too-large = Dieses Dokument ist für eine Vorschau zu groß.
preview-mermaid-unsupported-type = Diagrammtyp wird nicht unterstützt; der Quelltext wird angezeigt.
preview-mermaid-unsupported-syntax = Das Diagramm verwendet Syntax, die nicht gezeichnet werden kann; der Quelltext wird angezeigt.
preview-mermaid-too-large = Das Diagramm ist zu groß zum Zeichnen; der Quelltext wird angezeigt.
menu-tools = Werkzeuge
tools-json-validate = JSON prüfen
tools-json-format = JSON formatieren
tools-json-minify = JSON minimieren
tools-sort-lines = Zeilen sortieren
tools-sort-lines-desc = Zeilen absteigend sortieren
tools-unique-lines = Doppelte Zeilen entfernen
tools-trim-trailing = Leerraum am Zeilenende entfernen
tools-upper-case = Großbuchstaben
tools-lower-case = Kleinbuchstaben
tools-to-lf = Zeilenenden in LF umwandeln
tools-to-crlf = Zeilenenden in CRLF umwandeln
tools-json-valid = JSON ist gültig
tools-done = Fertig. Widerrufen macht es rückgängig.
tools-no-change = Nichts zu ändern
tools-json-error = Ungültiges JSON in Zeile { $line }, Spalte { $column }: { $reason }
json-reason-unterminated-string = nicht abgeschlossene Zeichenkette
json-reason-invalid-u-escape = ungültige \u-Escape-Sequenz
json-reason-invalid-escape = ungültige Escape-Sequenz
json-reason-control-character-in-string = Steuerzeichen in Zeichenkette
json-reason-invalid-number = ungültige Zahl
json-reason-unexpected-character = unerwartetes Zeichen
json-reason-unexpected-content-after-the-value = unerwarteter Inhalt nach dem Wert
json-reason-unexpected-comma = unerwartetes Komma
json-reason-expected-a-string-key = Zeichenkettenschlüssel erwartet
json-reason-expected = ':' erwartet
json-reason-expected-or-a-closing-bracket = ',' oder schließende Klammer erwartet
json-reason-expected-a-value = Wert erwartet
json-reason-empty-document = leeres Dokument
json-reason-unexpected-end-of-input = unerwartetes Ende der Eingabe
json-reason-too-deeply-nested-to-format = zu tief verschachtelt zum Formatieren
tools-compare = Vergleichen mit…
compare-need-two = Öffnen Sie einen weiteren Tab zum Vergleichen.
compare-pick-title = { $name } vergleichen mit
compare-count = { $count ->
    [0] Keine Unterschiede
    [one] 1 Änderung
   *[other] { $count } Änderungen
    }
compare-close = Vergleich schließen
compare-to-left = Diese Änderung nach links kopieren
compare-to-right = Diese Änderung nach rechts kopieren
compare-stale = Ein Dokument wurde während des Vergleichs geändert. Der Vergleich wurde aktualisiert; versuchen Sie es erneut.
view-structured = Strukturierte Ansicht (CSV, JSON, XML, YAML)
csv-delimiter = Trennzeichen
csv-rows = { $count ->
    [one] 1 Zeile
   *[other] { $count } Zeilen
    }
csv-unterminated = Nicht abgeschlossenes Anführungszeichen ab Zeile { $line }
csv-truncated = Die ersten 200.000 Zeilen werden angezeigt
csv-edit-cell = Zelle bearbeiten
csv-insert-row-below = Zeile darunter einfügen
csv-delete-row = Zeile löschen
csv-read-only = Schreibgeschützte Datei: Das Raster kann nicht bearbeitet werden
json-more = … { $count } weitere
json-edit-value = Wert bearbeiten
json-rename-key = Schlüssel umbenennen
json-add-after = Danach einfügen
json-add-inside = Darin einfügen
json-delete-node = Löschen
json-read-only = Schreibgeschützte Datei: Der Baum kann nicht bearbeitet werden
json-invalid-value = Kein gültiger JSON-Wert
json-duplicate-key = Das Objekt hat bereits einen Schlüssel mit diesem Namen
xml-edit-value = Wert bearbeiten
xml-edit-text = Text bearbeiten
xml-rename = Umbenennen
xml-add-attribute = Attribut hinzufügen
xml-add-child = Kindelement hinzufügen
xml-add-after = Danach einfügen
xml-delete = Löschen
xml-invalid-name = Kein gültiger XML-Name
xml-duplicate-attribute = Das Element hat bereits ein Attribut mit diesem Namen
xml-not-well-formed = Diese Änderung würde das XML ungültig machen
yaml-not-preserved = Diese Änderung würde andere Teile des YAML verändern
yaml-key-too-long = Ein Schlüssel darf höchstens 1024 Zeichen lang sein
yaml-read-only-flow = Flow-Stil (in Klammern geschrieben) kann im Baum noch nicht bearbeitet werden
yaml-read-only-anchor = Werte mit Anker (&) können im Baum nicht bearbeitet werden
yaml-read-only-tag = Werte mit Tag (!) können im Baum nicht bearbeitet werden
yaml-read-only-alias = Aliasse (*) können im Baum nicht bearbeitet werden
yaml-read-only-block-scalar = Textblöcke (| oder >) können im Baum noch nicht bearbeitet werden
yaml-read-only-complex-key = Komplexe Schlüssel können im Baum nicht bearbeitet werden
yaml-read-only-document = Im Baum kann nur das erste Dokument bearbeitet werden
yaml-read-only-layout = Dieser Eintrag teilt seine Zeilen mit anderem Inhalt und kann hier nicht geändert werden
yaml-read-only-only-child = Der einzige Eintrag kann nicht entfernt werden
dialog-open-link-title = Diesen Link im Webbrowser öffnen?
button-open = Öffnen
link-blocked = Links dieser Art werden aus Sicherheitsgründen nicht geöffnet.
link-not-found = Verlinkte Datei nicht gefunden: { $target }
enc-windows-1252 = Westeuropäisch
enc-iso-8859-15 = Westeuropäisch
enc-windows-1250 = Mitteleuropäisch
enc-iso-8859-2 = Mitteleuropäisch
enc-windows-1251 = Kyrillisch
enc-koi8-r = Kyrillisch
enc-windows-1253 = Griechisch
enc-windows-1254 = Türkisch
enc-windows-1255 = Hebräisch
enc-windows-1256 = Arabisch
enc-windows-1257 = Baltisch
enc-windows-874 = Thailändisch
enc-shift_jis = Japanisch
enc-euc-jp = Japanisch
enc-gb18030 = Chinesisch, vereinfacht
enc-big5 = Chinesisch, traditionell
enc-euc-kr = Koreanisch
file-reopen-encoding = Mit Codierung erneut öffnen…
file-save-encoding = Mit Codierung sichern…
encoding-reopen-title = Mit Codierung erneut öffnen
encoding-save-title = Mit Codierung sichern
encoding-not-exact = Mit dieser Codierung kann die Datei nicht unverändert geöffnet werden: { $reason }
encoding-unencodable = „{ $character }“ kann nicht als { $encoding } gesichert werden. Wählen Sie eine andere Codierung oder ändern Sie den Text; es wurde nichts geschrieben.
encoding-save-first = Sichern oder widerrufen Sie Ihre Änderungen, bevor Sie mit einer anderen Codierung erneut öffnen.
image-remote = Externe Bilder werden nicht geladen.
image-blocked = Es werden nur Bilder aus dem Ordner dieses Dokuments angezeigt.
image-unsupported = Nicht unterstütztes Bildformat.
image-too-large = Das Bild ist zu groß für die Anzeige.
image-unreadable = Das Bild konnte nicht gelesen werden.
settings-theme-high-contrast = Hoher Kontrast
view-fold = Einklappen
view-unfold = Ausklappen
view-unfold-all = Alle ausklappen
settings-shortcuts = Tastenkürzel
settings-keymap-preset = Vorlage
settings-keymap-default = Sanain
settings-keymap-notepadpp = Notepad++
settings-shortcut-change = Ändern
settings-shortcut-reset = Zurücksetzen
settings-shortcut-press = Tastenkürzel drücken… (Esc bricht ab)
settings-shortcut-moved = Tastenkürzel verschoben von:
edit-find-in-folder = In Ordner suchen…
folder-search-placeholder = In Ordner suchen
folder-search-run = Suchen
folder-search-stop = Stoppen
folder-search-no-folder = Ordner auswählen
folder-search-summary = { $count ->
    [one] 1 Treffer
   *[other] { $count } Treffer
    } in { $files ->
    [one] 1 Datei
   *[other] { $files } Dateien
    }
app-check-updates = Nach Updates suchen…
update-checking = Suche nach Updates…
update-up-to-date = Sanain { $version } ist die neueste Version.
update-no-releases = Es wurden noch keine Sanain-Versionen veröffentlicht.
update-failed = Suche nach Updates fehlgeschlagen: { $reason }
update-invalid = unerwartete Antwort
update-available-title = Sanain { $version } ist verfügbar
update-available-body = Öffnen Sie die Download-Seite, um es zu erhalten. Sanain lädt und installiert nichts selbstständig.
update-open-page = Download-Seite öffnen
update-later = Später
settings-check-updates = Einmal täglich nach Updates suchen
settings-check-updates-detail = Fragt github.com nach der neuesten Version. Sonst wird nichts gesendet.
file-local-history = Lokaler Verlauf…
file-clear-history = Lokalen Verlauf löschen…
history-title = Lokaler Verlauf von { $name }
history-open = Öffnen
history-restore = Wiederherstellen
history-empty = Noch keine gesicherten Versionen. Bei jedem Sichern wird eine Kopie aufbewahrt.
history-unreadable = Diese Version konnte nicht gelesen werden.
history-just-now = Gerade eben
history-minutes-ago = { $count ->
    [one] vor 1 Minute
   *[other] vor { $count } Minuten
    }
history-hours-ago = { $count ->
    [one] vor 1 Stunde
   *[other] vor { $count } Stunden
    }
history-days-ago = { $count ->
    [one] vor 1 Tag
   *[other] vor { $count } Tagen
    }
history-purge-title = Gesamten lokalen Verlauf löschen?
history-purge-body = Gesicherte Kopien aller Dateien werden von diesem Computer entfernt. Die Dateien selbst werden nicht verändert.
history-purge-confirm = Verlauf löschen
history-purged = Lokaler Verlauf gelöscht.
settings-local-history = Lokalen Verlauf gesicherter Dateien aufbewahren
settings-local-history-detail = Bewahrt bis zu 50 Versionen pro Datei für 30 Tage auf, nur auf diesem Computer.
file-open-log = In Protokollanzeige öffnen…
log-lines = { $count ->
    [one] 1 Zeile
   *[other] { $count } Zeilen
    }
log-indexing = Wird gelesen… { $percent } %
log-matches = { $count ->
    [one] 1 passende Zeile
   *[other] { $count } passende Zeilen
    }
log-filter-hint = Zeilen filtern
log-bad-pattern = Ungültiges Muster
log-follow = Folgen
log-close = Anzeige schließen
log-opened-large = { $name } ist groß und wurde daher schreibgeschützt in der Protokollanzeige geöffnet.
tab-pin = Tab anheften
tab-unpin = Tab lösen
edit-add-cursor-above = Cursor oberhalb hinzufügen
edit-add-cursor-below = Cursor unterhalb hinzufügen
view-word-wrap = Zeilenumbruch
status-wrap-on = Lange Zeilen werden jetzt an der Fensterbreite umbrochen.
status-wrap-off = Lange Zeilen werden nicht mehr umbrochen.
settings-word-wrap = Lange Zeilen umbrechen
edit-go-to-line = Gehe zu Zeile…
edit-go-to-heading = Gehe zu Überschrift…
goto-line-hint = Zeile oder Zeile:Spalte
goto-line-note = { $count ->
    [one] Das Dokument hat 1 Zeile.
   *[other] Das Dokument hat { $count } Zeilen.
    }
goto-line-invalid = Geben Sie eine Zeilennummer ein, optional gefolgt von :Spalte.
goto-heading-hint = Tippen, um Überschriften zu filtern
goto-heading-none = Dieses Dokument hat keine Überschriften.
format-xml = XML
format-yaml = YAML
tools-tree-error = Ungültiges { $format } in Zeile { $line }, Spalte { $column }: { $reason }
edit-select-next-occurrence = Nächstes Vorkommen auswählen
tools-record-macro = Makroaufzeichnung starten oder beenden
tools-play-macro = Makro abspielen
macro-recording = Makro wird aufgezeichnet
macro-recorded = { $count ->
    [one] Makro aufgezeichnet: 1 Schritt.
   *[other] Makro aufgezeichnet: { $count } Schritte.
    }
tools-compare-saved = Mit gespeicherter Version vergleichen
tools-compare-clipboard = Mit Zwischenablage vergleichen
compare-saved-label = { $name } (auf dem Datenträger)
compare-clipboard-label = Zwischenablage
compare-saved-failed = Die gespeicherte Datei konnte nicht gelesen werden: { $reason }
compare-clipboard-empty = Die Zwischenablage enthält keinen Text zum Vergleichen.
error-save-disk-full = { $name } wurde nicht gesichert: Der Datenträger ist voll. Die Datei auf dem Datenträger ist unverändert, und Ihr Text ist noch hier. Geben Sie Speicherplatz frei, oder verwenden Sie „Sichern unter“, um sie an einem anderen Ort zu sichern.
error-save-permission = { $name } wurde nicht gesichert: Sie haben keine Berechtigung, an diesem Ort zu schreiben. Die Datei auf dem Datenträger ist unverändert. Verwenden Sie „Sichern unter“, um sie an einem anderen Ort zu sichern.
error-save-read-only-volume = { $name } wurde nicht gesichert: Der Datenträger ist schreibgeschützt. Die Datei auf dem Datenträger ist unverändert. Verwenden Sie „Sichern unter“, um sie an einem anderen Ort zu sichern.
replace-preview-title = { $count ->
    [one] 1 Treffer ersetzen?
   *[other] { $count } Treffer ersetzen?
    }
replace-preview-confirm = { $count ->
    [one] 1 ersetzen
   *[other] { $count } ersetzen
    }
replace-preview-more = Nur die ersten 200 geänderten Zeilen werden angezeigt.
replace-preview-stale = Das Dokument wurde geändert; nichts wurde ersetzt. Führen Sie „Alle ersetzen“ erneut aus.
encoding-detected = Als { $encoding } geöffnet (aus dem Dateiinhalt erkannt).
encoding-suggested = Aus dem Dateiinhalt vorgeschlagen: { $encoding }. Prüfen Sie vor dem Öffnen die Vorschau.
app-about = Über Sanain
menu-help = Hilfe
about-tagline = Die Kraft der Worte.
about-version = Version { $version }
about-license = Freie und quelloffene Software unter der Mozilla Public License 2.0.
about-ai = Von KI gebaut. Von Menschen geführt. Von der Community getragen.
about-notices = Komponenten von Drittanbietern und ihre Lizenzen sind in THIRD_PARTY_NOTICES aufgeführt, das jeder Version beiliegt.
about-source = Quellcode und Fehlermeldungen auf GitHub
tools-xml-validate = XML prüfen
tools-yaml-validate = YAML prüfen
tools-markup-valid = { $format } ist gültig
file-open-recent = Zuletzt verwendet öffnen…
file-save-all = Alle sichern
file-revert = Auf gesicherte Version zurücksetzen
recent-hint = Tippen, um zuletzt verwendete Dateien zu filtern
recent-none = Noch keine zuletzt verwendeten Dateien.
revert-title = Änderungen an { $name } verwerfen?
revert-body = Die Datei wird erneut vom Datenträger geladen. Nicht gesicherte Änderungen gehen verloren und können nicht widerrufen werden.
revert-confirm = Verwerfen und neu laden
revert-done = { $name } wurde erneut vom Datenträger geladen.
status-saved-all = { $count ->
    [0] Nichts zu sichern.
    [one] 1 Datei gesichert.
   *[other] { $count } Dateien gesichert.
    }
edit-toggle-comment = Zeilenkommentar umschalten
edit-indent = Zeilen einrücken
edit-outdent = Zeileneinzug verringern
comment-unsupported = Dieser Dateityp hat keine Zeilenkommentare.
tab-close-others = Andere Tabs schließen
tab-close-all = Alle Tabs schließen
file-copy-path = Dateipfad kopieren
file-reveal-macos = Im Finder anzeigen
file-reveal-windows = Im Explorer anzeigen
file-reveal-other = Im Dateimanager anzeigen
status-path-copied = Dateipfad kopiert.
tabs-kept-unsaved = { $count ->
    [one] 1 Tab mit ungespeicherten Änderungen wurde offen gelassen.
   *[other] { $count } Tabs mit ungespeicherten Änderungen wurden offen gelassen.
    }
status-indent-tabs = Tabs
status-indent-spaces = Leerzeichen: { $count }
find-searching = Suche läuft…
compare-ignore-whitespace = Leerraum ignorieren
tools-compare-file = Mit Datei vergleichen…
compare-file-failed = Vergleich mit { $name } nicht möglich: { $reason }
cli-help =
    Verwendung:
      sanain [DATEI…]               Dateien öffnen
      sanain --diff LINKS RECHTS    zwei Dateien nebeneinander vergleichen
      sanain --version              Version anzeigen
      sanain --help                 diese Hilfe anzeigen
    Verwenden Sie -- vor Dateinamen, die mit einem Bindestrich beginnen.
cli-unknown-option = Unbekannte Option { $option }.
cli-diff-needs-two = --diff benötigt zwei Dateien: sanain --diff LINKS RECHTS
window-move-tab-left = Tab nach links verschieben
window-move-tab-right = Tab nach rechts verschieben
edit-skip-occurrence = Zum nächsten Vorkommen springen
tools-play-macro-times = Makro mehrfach abspielen…
macro-times-hint = Anzahl der Wiederholungen
macro-times-note = Von 1 bis { $max }
macro-times-invalid = Geben Sie eine Zahl von 1 bis { $max } ein.
view-outline = Gliederung
outline-empty = Keine Überschriften in diesem Dokument.
outline-close = Gliederung schließen
folder-replace-placeholder = Ersetzen durch
folder-replace-run = In Dateien ersetzen…
replace-files-title = { $count ->
    [one] 1 Treffer
   *[other] { $count } Treffer
    } in { $files ->
    [one] 1 Datei
   *[other] { $files } Dateien
    } ersetzen?
replace-files-body = Dateien, die in einem Tab geöffnet sind, werden dort geändert und können rückgängig gemacht werden; speichern Sie sie selbst. Andere Dateien werden sofort gespeichert, und ihr vorheriger Inhalt bleibt im lokalen Verlauf erhalten, wenn dieser aktiviert ist. Eine Datei, die sich inzwischen auf dem Datenträger geändert hat, wird übersprungen.
replace-files-confirm = Ersetzen
replace-files-done = { $count ->
    [one] 1 Treffer
   *[other] { $count } Treffer
    } in { $files ->
    [one] 1 Datei
   *[other] { $files } Dateien
    } ersetzt (in Tabs geöffnet: { $open }, nicht gespeichert).
replace-files-failed = { $failed ->
    [one] 1 Datei wurde nicht geändert:
   *[other] { $failed } Dateien wurden nicht geändert; erste:
    } { $first }
image-outside = Dieses Bild liegt außerhalb des Ordners des Dokuments.
image-allow = Bilder aus diesem Ordner anzeigen
cli-help-integration =
    Integration in den Dateimanager (nur für den aktuellen Benutzer):
      sanain --register           Sanain zu „Öffnen mit“ hinzufügen (Linux, Windows)
      sanain --unregister         wieder entfernen
cli-registered = Sanain wurde zu „Öffnen mit“ hinzugefügt. { $place }
cli-unregistered = Sanain wurde aus „Öffnen mit“ entfernt.
cli-register-failed = Die Integration in den Dateimanager konnte nicht geändert werden: { $reason }
cli-register-macos = Unter macOS muss nichts registriert werden: Sobald sich Sanain.app im Ordner „Programme“ befindet, bietet der Finder es unter „Öffnen mit“ an.
settings-tab-general = Allgemein
settings-tab-editor = Editor
settings-tab-appearance = Erscheinungsbild
settings-shortcut-search = Befehle oder Tasten suchen
settings-font = Schriftart
settings-font-default = Integrierte Festbreitenschrift
settings-font-loading = Installierte Schriftarten werden gesucht…
settings-font-missing = Die Schriftart { $font } wurde nicht gefunden; die integrierte Schriftart wird verwendet.
settings-theme-more = Weitere Designs
settings-theme-user-note = Eigene Designs: .theme-Dateien im Ordner { $folder } (siehe docs/THEMES.md).
language-auto = Automatisch erkennen
language-choose = Sprache dieses Tabs auswählen
tools-xml-format = XML formatieren
tools-save-macro = Makro speichern…
tools-play-saved-macro = Gespeichertes Makro abspielen…
tools-delete-saved-macro = Gespeichertes Makro löschen…
macro-save-hint = Makroname
macro-save-note = Ein gespeichertes Makro mit demselben Namen wird ersetzt.
macro-save-empty = Nichts zu speichern: Zeichnen Sie zuerst ein Makro auf.
macro-saved = Makro gespeichert: { $name }
macro-deleted = Makro gelöscht: { $name }
macro-name-invalid = Geben Sie einen Namen mit 1 bis { $max } Zeichen in einer Zeile ein.
macro-save-too-large = Dieses Makro ist zu lang zum Speichern.
macro-save-full = Es können höchstens { $max } Makros gespeichert werden. Löschen Sie zuerst eines.
macro-save-failed = Die Makros konnten nicht gespeichert werden: { $error }
macro-saved-hint = Tippen, um gespeicherte Makros zu filtern
macro-saved-none = Noch keine gespeicherten Makros.
settings-shortcut-import-npp = Aus Notepad++ importieren…
settings-shortcut-import-npp-hint = Wählen Sie eine shortcuts.xml von Notepad++. Unter Windows liegt sie in %AppData%\Notepad++\shortcuts.xml.
settings-shortcut-import-npp-done = Aus Notepad++ importiert: { $imported }. Übersprungen: { $skipped }. Verbleibende Konflikte: { $conflicts }.
settings-shortcut-import-npp-failed = { $name } konnte nicht importiert werden: { $reason }
settings-shortcut-import-npp-too-large = Die Datei ist größer als 1 MiB.
settings-shortcut-import-npp-invalid = Die Datei ist keine gültige shortcuts.xml von Notepad++.
view-split-right = Rechts teilen
view-split-down = Unten teilen
view-close-pane = Bereich schließen
view-focus-next-pane = Nächster Bereich
view-focus-prev-pane = Vorheriger Bereich
status-pane-limit = Maximale Anzahl an Bereichen erreicht (höchstens { $max }).
