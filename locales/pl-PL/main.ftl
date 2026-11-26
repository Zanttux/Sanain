# Machine translation by Claude (AI); review by a native speaker required before release.
app-name = Sanain
menu-file = Plik
file-open = Otwórz…
file-save = Zapisz
status-tabs = { $count ->
    [one] Jedna karta
    [few] { $count } karty
    [many] { $count } kart
   *[other] { $count } karty
    }
error-open = Nie można otworzyć pliku { $name }.
menu-edit = Edycja
file-new = Nowa karta
file-save-as = Zapisz jako…
edit-undo = Cofnij
edit-redo = Ponów
edit-select-all = Zaznacz wszystko
untitled = Bez tytułu
window-title = { $dirty ->
    [yes] ● { $name } — Sanain
   *[no] { $name } — Sanain
    }
status-position = Wiersz { $line }, kol. { $column }
status-selections = { $count ->
    [one] 1 zaznaczenie
    [few] { $count } zaznaczenia
    [many] { $count } zaznaczeń
   *[other] { $count } zaznaczenia
    }
status-saved = Zapisano { $name }
encoding-utf8 = UTF-8
encoding-utf8-bom = UTF-8 z BOM
encoding-utf16le = UTF-16 LE
encoding-utf16be = UTF-16 BE
line-ending-lf = LF
line-ending-crlf = CRLF
line-ending-cr = CR
line-ending-mixed = Mieszane znaki końca wiersza
editor-accessible-name = Edytor tekstu
button-save = Zapisz
button-dont-save = Nie zapisuj
button-cancel = Anuluj
button-ok = OK
button-overwrite = Zastąp
button-save-as = Zapisz jako…
dialog-unsaved-title = Zapisać zmiany w pliku { $name }?
dialog-unsaved-body = Jeśli ich nie zapiszesz, zmiany zostaną utracone.
dialog-conflict-title = Plik { $name } zmienił się na dysku
dialog-conflict-body = Inny program zmodyfikował ten plik po jego otwarciu. Zastąpienie usunie te zmiany.
dialog-readonly-title = Plik { $name } jest tylko do odczytu
dialog-readonly-body = Zapisz zamiast tego kopię pod inną nazwą.
error-open-encoding = Plik { $name } nie zawiera tekstu UTF-8 ani UTF-16. Nie został otwarty, więc jego zawartość pozostała niezmieniona. Użyj polecenia Plik > Otwórz ponownie z kodowaniem, aby wybrać kodowanie.
error-save = Nie można zapisać pliku { $name }: { $reason }
menu-window = Okno
edit-cut = Wytnij
edit-copy = Kopiuj
edit-paste = Wklej
app-quit = Zakończ Sanain
app-hide = Ukryj Sanain
app-hide-others = Ukryj pozostałe
app-show-all = Pokaż wszystko
window-minimize = Minimalizuj
window-zoom = Powiększ
status-recovered = Przywrócono niezapisane zmiany z poprzedniej sesji
status-recovery-failed = Nie można zapisać kopii odzyskiwania: { $reason }
edit-duplicate-line = Duplikuj wiersz
edit-delete-line = Usuń wiersz
edit-move-line-up = Przenieś wiersz w górę
edit-move-line-down = Przenieś wiersz w dół
file-close-tab = Zamknij kartę
file-reopen-tab = Otwórz ponownie zamkniętą kartę
window-next-tab = Następna karta
window-prev-tab = Poprzednia karta
tab-close = Zamknij kartę
edit-find = Znajdź…
edit-find-next = Znajdź następny
edit-find-prev = Znajdź poprzedni
edit-replace = Zamień…
find-placeholder = Znajdź
replace-placeholder = Zamień na
find-match-case = Uwzględniaj wielkość liter
find-whole-word = Całe wyrazy
find-regex = Wyrażenie regularne
find-no-results = Brak wyników
find-count = { $total ->
    [one] 1 dopasowanie
    [few] { $total } dopasowania
    [many] { $total } dopasowań
   *[other] { $total } dopasowania
    }
find-count-current = { $current } z { $total }
find-prev = Poprzednie dopasowanie (Shift+Enter)
find-next = Następne dopasowanie (Enter)
find-close = Zamknij (Esc)
replace-one = Zamień
replace-all = Zamień wszystko
status-replaced = { $count ->
    [one] Zamieniono 1 dopasowanie
    [few] Zamieniono { $count } dopasowania
    [many] Zamieniono { $count } dopasowań
   *[other] Zamieniono { $count } dopasowania
    }
language-plain = Zwykły tekst
menu-view = Widok
app-settings = Ustawienia…
view-zoom-in = Powiększ
view-zoom-out = Pomniejsz
view-zoom-reset = Rzeczywisty rozmiar
settings-title = Ustawienia
settings-font-size = Rozmiar czcionki
settings-tab-width = Szerokość tabulacji
settings-insert-spaces = Wstawiaj spacje po naciśnięciu Tab
settings-theme = Wygląd
settings-theme-system = Zgodnie z systemem
settings-theme-light = Jasny
settings-theme-dark = Ciemny
settings-restore-session = Otwieraj ponownie pliki z poprzedniej sesji
settings-language = Język
settings-language-system = Zgodnie z systemem
settings-language-note = Zmiana języka zostanie zastosowana po ponownym uruchomieniu Sanain.
settings-save-failed = Nie można zapisać ustawień: { $reason }
view-command-palette = Paleta poleceń…
palette-placeholder = Wpisz polecenie
view-markdown-source = Źródło Markdown
view-markdown-split = Źródło i podgląd Markdown
view-markdown-preview = Podgląd Markdown
preview-too-large = Ten dokument jest zbyt duży, aby wyświetlić podgląd.
preview-mermaid-unsupported = Diagramy Mermaid nie są jeszcze renderowane; wyświetlane jest źródło.
menu-tools = Narzędzia
tools-json-validate = Sprawdź poprawność JSON
tools-json-format = Formatuj JSON
tools-json-minify = Minimalizuj JSON
tools-sort-lines = Sortuj wiersze
tools-sort-lines-desc = Sortuj wiersze malejąco
tools-unique-lines = Usuń zduplikowane wiersze
tools-trim-trailing = Usuń końcowe białe znaki
tools-upper-case = Wielkie litery
tools-lower-case = Małe litery
tools-to-lf = Konwertuj znaki końca wiersza na LF
tools-to-crlf = Konwertuj znaki końca wiersza na CRLF
tools-json-valid = JSON jest poprawny
tools-done = Gotowe. Cofnij odwraca zmianę.
tools-no-change = Brak zmian do wprowadzenia
tools-json-error = Niepoprawny JSON w wierszu { $line }, kolumnie { $column }: { $reason }
json-reason-unterminated-string = niezakończony ciąg znaków
json-reason-invalid-u-escape = niepoprawna sekwencja \u
json-reason-invalid-escape = niepoprawna sekwencja ucieczki
json-reason-control-character-in-string = znak sterujący w ciągu znaków
json-reason-invalid-number = niepoprawna liczba
json-reason-unexpected-character = nieoczekiwany znak
json-reason-unexpected-content-after-the-value = nieoczekiwana zawartość po wartości
json-reason-unexpected-comma = nieoczekiwany przecinek
json-reason-expected-a-string-key = oczekiwano klucza w postaci ciągu znaków
json-reason-expected = oczekiwano ':'
json-reason-expected-or-a-closing-bracket = oczekiwano ',' lub nawiasu zamykającego
json-reason-expected-a-value = oczekiwano wartości
json-reason-empty-document = pusty dokument
json-reason-unexpected-end-of-input = nieoczekiwany koniec danych
json-reason-too-deeply-nested-to-format = zbyt głębokie zagnieżdżenie, aby sformatować
tools-compare = Porównaj z…
compare-need-two = Otwórz inną kartę, aby porównać.
compare-pick-title = Porównaj { $name } z
compare-count = { $count ->
    [0] Brak różnic
    [one] 1 zmiana
    [few] { $count } zmiany
    [many] { $count } zmian
   *[other] { $count } zmiany
    }
compare-close = Zamknij porównanie
compare-to-left = Skopiuj tę zmianę na lewą stronę
compare-to-right = Skopiuj tę zmianę na prawą stronę
compare-stale = Dokument zmienił się podczas porównywania. Porównanie zostało odświeżone; spróbuj ponownie.
view-structured = Widok strukturalny (CSV, JSON, XML, YAML)
csv-delimiter = Separator
csv-rows = { $count ->
    [one] 1 wiersz
    [few] { $count } wiersze
    [many] { $count } wierszy
   *[other] { $count } wiersza
    }
csv-unterminated = Niezamknięty cudzysłów zaczynający się w wierszu { $line }
csv-truncated = Wyświetlane jest pierwsze 200 000 wierszy
json-more = … i { $count } więcej
dialog-open-link-title = Otworzyć ten link w przeglądarce internetowej?
button-open = Otwórz
link-blocked = Ze względów bezpieczeństwa ten rodzaj linku nie jest otwierany.
link-not-found = Nie znaleziono połączonego pliku: { $target }
enc-windows-1252 = Zachodnioeuropejskie
enc-iso-8859-15 = Zachodnioeuropejskie
enc-windows-1250 = Środkowoeuropejskie
enc-iso-8859-2 = Środkowoeuropejskie
enc-windows-1251 = Cyrylica
enc-koi8-r = Cyrylica
enc-windows-1253 = Greckie
enc-windows-1254 = Tureckie
enc-windows-1255 = Hebrajskie
enc-windows-1256 = Arabskie
enc-windows-1257 = Bałtyckie
enc-windows-874 = Tajskie
enc-shift_jis = Japońskie
enc-euc-jp = Japońskie
enc-gb18030 = Chińskie uproszczone
enc-big5 = Chińskie tradycyjne
enc-euc-kr = Koreańskie
file-reopen-encoding = Otwórz ponownie z kodowaniem…
file-save-encoding = Zapisz z kodowaniem…
encoding-reopen-title = Otwórz ponownie z kodowaniem
encoding-save-title = Zapisz z kodowaniem
encoding-not-exact = To kodowanie nie może otworzyć pliku bez jego zmiany: { $reason }
encoding-unencodable = Nie można zapisać znaku „{ $character }” w kodowaniu { $encoding }. Wybierz inne kodowanie lub zmień tekst; nic nie zostało zapisane.
encoding-save-first = Zapisz lub cofnij zmiany przed ponownym otwarciem z innym kodowaniem.
image-remote = Obrazy zdalne nie są wczytywane.
image-blocked = Wyświetlane są tylko obrazy z folderu tego dokumentu.
image-unsupported = Nieobsługiwany format obrazu.
image-too-large = Obraz jest zbyt duży, aby go wyświetlić.
image-unreadable = Nie można odczytać obrazu.
settings-theme-high-contrast = Wysoki kontrast
view-fold = Zwiń
view-unfold = Rozwiń
view-unfold-all = Rozwiń wszystko
settings-shortcuts = Skróty klawiaturowe
settings-keymap-preset = Zestaw
settings-keymap-default = Sanain
settings-keymap-notepadpp = Notepad++
settings-shortcut-change = Zmień
settings-shortcut-reset = Resetuj
settings-shortcut-press = Naciśnij skrót… (Esc anuluje)
settings-shortcut-moved = Skrót przeniesiono z:
edit-find-in-folder = Znajdź w folderze…
folder-search-placeholder = Znajdź w folderze
folder-search-run = Szukaj
folder-search-stop = Zatrzymaj
folder-search-no-folder = Wybierz folder
folder-search-summary = { $count ->
    [one] 1 dopasowanie
    [few] { $count } dopasowania
    [many] { $count } dopasowań
   *[other] { $count } dopasowania
    } w { $files ->
    [one] 1 pliku
    [few] { $files } plikach
    [many] { $files } plikach
   *[other] { $files } pliku
    }
app-check-updates = Sprawdź aktualizacje…
update-checking = Sprawdzanie aktualizacji…
update-up-to-date = Sanain { $version } to najnowsza wersja.
update-no-releases = Nie opublikowano jeszcze żadnych wydań Sanain.
update-failed = Nie można sprawdzić aktualizacji: { $reason }
update-invalid = nieoczekiwana odpowiedź
update-available-title = Dostępna jest wersja Sanain { $version }
update-available-body = Otwórz stronę pobierania, aby ją pobrać. Sanain sam niczego nie pobiera ani nie instaluje.
update-open-page = Otwórz stronę pobierania
update-later = Później
settings-check-updates = Sprawdzaj aktualizacje raz dziennie
settings-check-updates-detail = Pyta github.com o najnowsze wydanie. Nic więcej nie jest wysyłane.
file-local-history = Historia lokalna…
file-clear-history = Wyczyść historię lokalną…
history-title = Historia lokalna pliku { $name }
history-open = Otwórz
history-restore = Przywróć
history-empty = Brak zapisanych wersji. Przy każdym zapisie zachowywana jest kopia.
history-unreadable = Nie można odczytać tej wersji.
history-just-now = Przed chwilą
history-minutes-ago = { $count ->
    [one] 1 minutę temu
    [few] { $count } minuty temu
    [many] { $count } minut temu
   *[other] { $count } minuty temu
    }
history-hours-ago = { $count ->
    [one] 1 godzinę temu
    [few] { $count } godziny temu
    [many] { $count } godzin temu
   *[other] { $count } godziny temu
    }
history-days-ago = { $count ->
    [one] 1 dzień temu
    [few] { $count } dni temu
    [many] { $count } dni temu
   *[other] { $count } dnia temu
    }
history-purge-title = Usunąć całą historię lokalną?
history-purge-body = Zapisane kopie wszystkich plików zostaną usunięte z tego komputera. Same pliki nie zostaną zmienione.
history-purge-confirm = Usuń historię
history-purged = Usunięto historię lokalną.
settings-local-history = Zachowuj lokalną historię zapisanych plików
settings-local-history-detail = Zachowuje do 50 wersji każdego pliku przez 30 dni, tylko na tym komputerze.
file-open-log = Otwórz w przeglądarce logów…
log-lines = { $count ->
    [one] 1 wiersz
    [few] { $count } wiersze
    [many] { $count } wierszy
   *[other] { $count } wiersza
    }
log-indexing = Wczytywanie… { $percent }%
log-matches = { $count ->
    [one] 1 pasujący wiersz
    [few] { $count } pasujące wiersze
    [many] { $count } pasujących wierszy
   *[other] { $count } pasującego wiersza
    }
log-filter-hint = Filtruj wiersze
log-bad-pattern = Nieprawidłowy wzorzec
log-follow = Śledź
log-close = Zamknij przeglądarkę
log-opened-large = Plik { $name } jest duży, więc otwarto go tylko do odczytu w przeglądarce logów.
tab-pin = Przypnij kartę
tab-unpin = Odepnij kartę
edit-add-cursor-above = Dodaj kursor powyżej
edit-add-cursor-below = Dodaj kursor poniżej
view-word-wrap = Zawijanie wierszy
status-wrap-on = Długie wiersze są teraz zawijane do szerokości okna.
status-wrap-off = Długie wiersze nie są już zawijane.
settings-word-wrap = Zawijaj długie wiersze
edit-go-to-line = Przejdź do wiersza…
edit-go-to-heading = Przejdź do nagłówka…
goto-line-hint = Wiersz lub wiersz:kolumna
goto-line-note = { $count ->
    [one] Dokument ma 1 wiersz.
    [few] Dokument ma { $count } wiersze.
    [many] Dokument ma { $count } wierszy.
   *[other] Dokument ma { $count } wiersza.
    }
goto-line-invalid = Wpisz numer wiersza, opcjonalnie z dopiskiem :kolumna.
goto-heading-hint = Pisz, aby filtrować nagłówki
goto-heading-none = Ten dokument nie ma nagłówków.
format-xml = XML
format-yaml = YAML
tools-tree-error = Niepoprawny { $format } w wierszu { $line }, kolumnie { $column }: { $reason }
edit-select-next-occurrence = Zaznacz następne wystąpienie
tools-record-macro = Rozpocznij lub zatrzymaj nagrywanie makra
tools-play-macro = Odtwórz makro
macro-recording = Nagrywanie makra
macro-recorded = { $count ->
    [one] Nagrano makro: 1 krok.
    [few] Nagrano makro: { $count } kroki.
    [many] Nagrano makro: { $count } kroków.
   *[other] Nagrano makro: { $count } kroku.
    }
tools-compare-saved = Porównaj z zapisaną wersją
tools-compare-clipboard = Porównaj ze schowkiem
compare-saved-label = { $name } (na dysku)
compare-clipboard-label = Schowek
compare-saved-failed = Nie można odczytać zapisanego pliku: { $reason }
compare-clipboard-empty = Schowek nie zawiera tekstu do porównania.
error-save-disk-full = Nie zapisano pliku { $name }: dysk jest pełny. Plik na dysku nie został zmieniony, a tekst nadal jest dostępny. Zwolnij miejsce lub użyj polecenia Zapisz jako, aby zapisać plik w innym miejscu.
error-save-permission = Nie zapisano pliku { $name }: brak uprawnień do zapisu w tej lokalizacji. Plik na dysku nie został zmieniony. Użyj polecenia Zapisz jako, aby zapisać plik w innym miejscu.
error-save-read-only-volume = Nie zapisano pliku { $name }: dysk jest tylko do odczytu. Plik na dysku nie został zmieniony. Użyj polecenia Zapisz jako, aby zapisać plik w innym miejscu.
replace-preview-title = { $count ->
    [one] Zamienić 1 dopasowanie?
    [few] Zamienić { $count } dopasowania?
    [many] Zamienić { $count } dopasowań?
   *[other] Zamienić { $count } dopasowania?
    }
replace-preview-confirm = { $count ->
    [one] Zamień 1
    [few] Zamień { $count }
    [many] Zamień { $count }
   *[other] Zamień { $count }
    }
replace-preview-more = Wyświetlane jest tylko pierwsze 200 zmienionych wierszy.
replace-preview-stale = Dokument się zmienił; nic nie zostało zamienione. Uruchom ponownie Zamień wszystko.
view-split-editor = Podziel edytor
encoding-detected = Otwarto w kodowaniu { $encoding } (wykryto na podstawie zawartości pliku).
encoding-suggested = Zasugerowano na podstawie zawartości pliku: { $encoding }. Przed otwarciem sprawdź podgląd.
app-about = O programie Sanain
menu-help = Pomoc
about-tagline = Siła słów.
about-version = Wersja { $version }
about-license = Wolne i otwarte oprogramowanie na licencji Mozilla Public License 2.0.
about-ai = Zbudowany przez AI. Prowadzony przez ludzi. Rozwijany przez społeczność.
about-notices = Komponenty innych firm i ich licencje wymieniono w pliku THIRD_PARTY_NOTICES dołączonym do każdego wydania.
about-source = Kod źródłowy i zgłoszenia w serwisie GitHub
tools-xml-validate = Sprawdź poprawność XML
tools-yaml-validate = Sprawdź poprawność YAML
tools-markup-valid = { $format } jest poprawny
file-open-recent = Otwórz ostatnie…
file-save-all = Zapisz wszystko
file-revert = Przywróć zapisaną wersję
recent-hint = Pisz, aby filtrować ostatnie pliki
recent-none = Brak ostatnich plików.
revert-title = Odrzucić zmiany w pliku { $name }?
revert-body = Plik zostanie ponownie wczytany z dysku. Niezapisane zmiany zostaną utracone i nie można ich cofnąć.
revert-confirm = Odrzuć i wczytaj ponownie
revert-done = Ponownie wczytano plik { $name } z dysku.
status-saved-all = { $count ->
    [0] Nie ma nic do zapisania.
    [one] Zapisano 1 plik.
    [few] Zapisano { $count } pliki.
    [many] Zapisano { $count } plików.
   *[other] Zapisano { $count } pliku.
    }
edit-toggle-comment = Przełącz komentarz wiersza
edit-indent = Zwiększ wcięcie wierszy
edit-outdent = Zmniejsz wcięcie wierszy
comment-unsupported = Ten typ pliku nie obsługuje komentarzy wierszowych.
tab-close-others = Zamknij pozostałe karty
tab-close-all = Zamknij wszystkie karty
file-copy-path = Kopiuj ścieżkę pliku
file-reveal-macos = Pokaż w Finderze
file-reveal-windows = Pokaż w Eksploratorze plików
file-reveal-other = Pokaż w menedżerze plików
status-path-copied = Skopiowano ścieżkę pliku.
tabs-kept-unsaved = { $count ->
    [one] 1 karta z niezapisanymi zmianami pozostała otwarta.
    [few] { $count } karty z niezapisanymi zmianami pozostały otwarte.
    [many] { $count } kart z niezapisanymi zmianami pozostało otwartych.
   *[other] { $count } karty z niezapisanymi zmianami pozostało otwartych.
    }
status-indent-tabs = Tabulatory
status-indent-spaces = Spacje: { $count }
find-searching = Wyszukiwanie…
compare-ignore-whitespace = Ignoruj białe znaki
tools-compare-file = Porównaj z plikiem…
compare-file-failed = Nie można porównać z { $name }: { $reason }
cli-help =
    Użycie:
      sanain [PLIK…]              otwórz pliki
      sanain --diff LEWY PRAWY    porównaj dwa pliki obok siebie
      sanain --version            pokaż wersję
      sanain --help               pokaż tę pomoc
    Użyj -- przed nazwami plików zaczynającymi się od myślnika.
cli-unknown-option = Nieznana opcja { $option }.
cli-diff-needs-two = --diff wymaga dwóch plików: sanain --diff LEWY PRAWY
window-move-tab-left = Przenieś kartę w lewo
window-move-tab-right = Przenieś kartę w prawo
edit-skip-occurrence = Przeskocz do następnego wystąpienia
tools-play-macro-times = Odtwórz makro wielokrotnie…
macro-times-hint = Liczba powtórzeń
macro-times-note = Od 1 do { $max }
macro-times-invalid = Wpisz liczbę od 1 do { $max }.
view-outline = Konspekt
outline-empty = Brak nagłówków w tym dokumencie.
outline-close = Zamknij konspekt
folder-replace-placeholder = Zamień na
folder-replace-run = Zamień w plikach…
replace-files-title = Zamienić { $count ->
    [one] 1 dopasowanie
    [few] { $count } dopasowania
    [many] { $count } dopasowań
   *[other] { $count } dopasowania
    } w { $files ->
    [one] 1 pliku
    [few] { $files } plikach
    [many] { $files } plikach
   *[other] { $files } pliku
    }?
replace-files-body = Pliki otwarte na karcie zostaną zmienione w niej i można to cofnąć; zapisz je samodzielnie. Pozostałe pliki zostaną zapisane od razu, a ich poprzednia zawartość zostanie zachowana w historii lokalnej, jeśli jest włączona. Plik, który w międzyczasie zmienił się na dysku, zostanie pominięty.
replace-files-confirm = Zamień
replace-files-done = { $count ->
    [one] Zamieniono 1 dopasowanie
    [few] Zamieniono { $count } dopasowania
    [many] Zamieniono { $count } dopasowań
   *[other] Zamieniono { $count } dopasowania
    } w { $files ->
    [one] 1 pliku
    [few] { $files } plikach
    [many] { $files } plikach
   *[other] { $files } pliku
    } (otwarte na kartach: { $open }, niezapisane).
replace-files-failed = { $failed ->
    [one] 1 plik nie został zmieniony:
    [few] { $failed } pliki nie zostały zmienione; pierwszy:
    [many] { $failed } plików nie zostało zmienionych; pierwszy:
   *[other] { $failed } pliku nie zostało zmienione; pierwszy:
    } { $first }
image-outside = Ten obraz znajduje się poza folderem dokumentu.
image-allow = Pokaż obrazy z tego folderu
cli-help-integration =
    Integracja z menedżerem plików (tylko bieżący użytkownik):
      sanain --register           dodaj Sanain do menu „Otwórz za pomocą” (Linux, Windows)
      sanain --unregister         usuń go ponownie
cli-registered = Dodano Sanain do menu „Otwórz za pomocą”. { $place }
cli-unregistered = Usunięto Sanain z menu „Otwórz za pomocą”.
cli-register-failed = Nie można zmienić integracji z menedżerem plików: { $reason }
cli-register-macos = Integracja z Finderem w systemie macOS nie jest jeszcze dostępna. Otwieraj pliki z poziomu Sanain (Plik › Otwórz, przeciągnij i upuść) lub z wiersza poleceń.
settings-tab-general = Ogólne
settings-tab-editor = Edytor
settings-tab-appearance = Wygląd
settings-shortcut-search = Szukaj poleceń lub klawiszy
