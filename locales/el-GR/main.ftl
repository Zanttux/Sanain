# Machine translation by Claude (AI); review by a native speaker required before release.
app-name = Sanain
menu-file = Αρχείο
file-open = Άνοιγμα…
file-save = Αποθήκευση
status-tabs = { $count ->
    [one] Μία καρτέλα
   *[other] { $count } καρτέλες
    }
error-open = Δεν ήταν δυνατό το άνοιγμα του { $name }.
menu-edit = Επεξεργασία
file-new = Νέα καρτέλα
file-save-as = Αποθήκευση ως…
edit-undo = Αναίρεση
edit-redo = Επανάληψη
edit-select-all = Επιλογή όλων
untitled = Χωρίς τίτλο
window-title = { $dirty ->
    [yes] ● { $name } — Sanain
   *[no] { $name } — Sanain
    }
status-position = Γρ. { $line }, Στ. { $column }
status-selections = { $count ->
    [one] 1 επιλογή
   *[other] { $count } επιλογές
    }
status-saved = Αποθηκεύτηκε: { $name }
encoding-utf8 = UTF-8
encoding-utf8-bom = UTF-8 με BOM
encoding-utf16le = UTF-16 LE
encoding-utf16be = UTF-16 BE
line-ending-lf = LF
line-ending-crlf = CRLF
line-ending-cr = CR
line-ending-mixed = Μικτές αλλαγές γραμμής
editor-accessible-name = Επεξεργαστής κειμένου
button-save = Αποθήκευση
button-dont-save = Να μην αποθηκευτεί
button-cancel = Ακύρωση
button-ok = OK
button-overwrite = Αντικατάσταση
button-save-as = Αποθήκευση ως…
dialog-unsaved-title = Αποθήκευση αλλαγών στο { $name };
dialog-unsaved-body = Οι αλλαγές σας θα χαθούν αν δεν τις αποθηκεύσετε.
dialog-conflict-title = Το { $name } άλλαξε στον δίσκο
dialog-conflict-body = Ένα άλλο πρόγραμμα τροποποίησε αυτό το αρχείο αφού το ανοίξατε. Η αντικατάσταση θα αφαιρέσει αυτές τις αλλαγές.
dialog-readonly-title = Το { $name } είναι μόνο για ανάγνωση
dialog-readonly-body = Αποθηκεύστε ένα αντίγραφο με διαφορετικό όνομα.
error-open-encoding = Το { $name } δεν είναι κείμενο UTF-8 ή UTF-16. Δεν άνοιξε, οπότε το περιεχόμενό του δεν άλλαξε. Χρησιμοποιήστε Αρχείο > Επαναφόρτωση με κωδικοποίηση για να επιλέξετε την κωδικοποίησή του.
error-save = Δεν ήταν δυνατή η αποθήκευση του { $name }: { $reason }
menu-window = Παράθυρο
edit-cut = Αποκοπή
edit-copy = Αντιγραφή
edit-paste = Επικόλληση
app-quit = Έξοδος από το Sanain
app-hide = Απόκρυψη του Sanain
app-hide-others = Απόκρυψη των υπολοίπων
app-show-all = Εμφάνιση όλων
window-minimize = Ελαχιστοποίηση
window-zoom = Ζουμ
status-recovered = Ανακτήθηκαν μη αποθηκευμένες αλλαγές από την προηγούμενη περίοδο λειτουργίας
status-recovery-failed = Δεν ήταν δυνατή η αποθήκευση αντιγράφου ανάκτησης: { $reason }
edit-duplicate-line = Διπλασιασμός γραμμής
edit-delete-line = Διαγραφή γραμμής
edit-move-line-up = Μετακίνηση γραμμής πάνω
edit-move-line-down = Μετακίνηση γραμμής κάτω
file-close-tab = Κλείσιμο καρτέλας
file-reopen-tab = Επαναφορά κλειστής καρτέλας
window-next-tab = Επόμενη καρτέλα
window-prev-tab = Προηγούμενη καρτέλα
tab-close = Κλείσιμο καρτέλας
edit-find = Εύρεση…
edit-find-next = Εύρεση επόμενου
edit-find-prev = Εύρεση προηγούμενου
edit-replace = Αντικατάσταση…
find-placeholder = Εύρεση
replace-placeholder = Αντικατάσταση με
find-match-case = Διάκριση πεζών-κεφαλαίων
find-whole-word = Ολόκληρη λέξη
find-regex = Κανονική έκφραση
find-no-results = Κανένα αποτέλεσμα
find-count = { $total ->
    [one] 1 αντιστοιχία
   *[other] { $total } αντιστοιχίες
    }
find-count-current = { $current } από { $total }
find-prev = Προηγούμενη αντιστοιχία (Shift+Enter)
find-next = Επόμενη αντιστοιχία (Enter)
find-close = Κλείσιμο (Esc)
replace-one = Αντικατάσταση
replace-all = Αντικατάσταση όλων
status-replaced = { $count ->
    [one] Αντικαταστάθηκε 1 αντιστοιχία
   *[other] Αντικαταστάθηκαν { $count } αντιστοιχίες
    }
language-plain = Απλό κείμενο
menu-view = Προβολή
app-settings = Ρυθμίσεις…
view-zoom-in = Μεγέθυνση
view-zoom-out = Σμίκρυνση
view-zoom-reset = Πραγματικό μέγεθος
settings-title = Ρυθμίσεις
settings-font-size = Μέγεθος γραμματοσειράς
settings-tab-width = Πλάτος Tab
settings-insert-spaces = Εισαγωγή κενών με το πλήκτρο Tab
settings-theme = Εμφάνιση
settings-theme-system = Όπως το σύστημα
settings-theme-light = Φωτεινό
settings-theme-dark = Σκούρο
settings-restore-session = Επανάνοιγμα αρχείων από την προηγούμενη περίοδο λειτουργίας
settings-language = Γλώσσα
settings-language-system = Όπως το σύστημα
settings-language-note = Οι αλλαγές γλώσσας εφαρμόζονται μετά την επανεκκίνηση του Sanain.
settings-save-failed = Δεν ήταν δυνατή η αποθήκευση των ρυθμίσεων: { $reason }
view-command-palette = Παλέτα εντολών…
palette-placeholder = Πληκτρολογήστε μια εντολή
view-markdown-source = Πηγαίος κώδικας Markdown
view-markdown-split = Πηγαίος κώδικας και προεπισκόπηση Markdown
view-markdown-preview = Προεπισκόπηση Markdown
preview-too-large = Το έγγραφο είναι πολύ μεγάλο για προεπισκόπηση.
preview-mermaid-unsupported-type = Ο τύπος διαγράμματος δεν υποστηρίζεται· εμφανίζεται ο πηγαίος κώδικας.
preview-mermaid-unsupported-syntax = Το διάγραμμα χρησιμοποιεί σύνταξη που δεν μπορεί να σχεδιαστεί· εμφανίζεται ο πηγαίος κώδικας.
preview-mermaid-too-large = Το διάγραμμα είναι πολύ μεγάλο για σχεδίαση· εμφανίζεται ο πηγαίος κώδικας.
menu-tools = Εργαλεία
tools-json-validate = Επικύρωση JSON
tools-json-format = Μορφοποίηση JSON
tools-json-minify = Ελαχιστοποίηση JSON
tools-sort-lines = Ταξινόμηση γραμμών
tools-sort-lines-desc = Φθίνουσα ταξινόμηση γραμμών
tools-unique-lines = Κατάργηση διπλότυπων γραμμών
tools-trim-trailing = Περικοπή κενών στο τέλος γραμμής
tools-upper-case = Κεφαλαία
tools-lower-case = Πεζά
tools-to-lf = Μετατροπή αλλαγών γραμμής σε LF
tools-to-crlf = Μετατροπή αλλαγών γραμμής σε CRLF
tools-json-valid = Το JSON είναι έγκυρο
tools-done = Ολοκληρώθηκε. Μπορείτε να το αναιρέσετε με Αναίρεση.
tools-no-change = Δεν υπάρχει κάτι προς αλλαγή
tools-json-error = Μη έγκυρο JSON στη γραμμή { $line }, στήλη { $column }: { $reason }
json-reason-unterminated-string = μη τερματισμένη συμβολοσειρά
json-reason-invalid-u-escape = μη έγκυρη ακολουθία διαφυγής \u
json-reason-invalid-escape = μη έγκυρη ακολουθία διαφυγής
json-reason-control-character-in-string = χαρακτήρας ελέγχου σε συμβολοσειρά
json-reason-invalid-number = μη έγκυρος αριθμός
json-reason-unexpected-character = μη αναμενόμενος χαρακτήρας
json-reason-unexpected-content-after-the-value = μη αναμενόμενο περιεχόμενο μετά την τιμή
json-reason-unexpected-comma = μη αναμενόμενο κόμμα
json-reason-expected-a-string-key = αναμενόταν κλειδί συμβολοσειράς
json-reason-expected = αναμενόταν ':'
json-reason-expected-or-a-closing-bracket = αναμενόταν ',' ή αγκύλη κλεισίματος
json-reason-expected-a-value = αναμενόταν τιμή
json-reason-empty-document = κενό έγγραφο
json-reason-unexpected-end-of-input = μη αναμενόμενο τέλος εισόδου
json-reason-too-deeply-nested-to-format = υπερβολικά βαθιά ένθεση για μορφοποίηση
tools-compare = Σύγκριση με…
compare-need-two = Ανοίξτε άλλη μία καρτέλα για σύγκριση.
compare-pick-title = Σύγκριση του { $name } με
compare-count = { $count ->
    [0] Καμία διαφορά
    [one] 1 αλλαγή
   *[other] { $count } αλλαγές
    }
compare-close = Κλείσιμο σύγκρισης
compare-to-left = Αντιγραφή αυτής της αλλαγής αριστερά
compare-to-right = Αντιγραφή αυτής της αλλαγής δεξιά
compare-stale = Ένα έγγραφο άλλαξε κατά τη σύγκριση. Η σύγκριση ανανεώθηκε· δοκιμάστε ξανά.
view-structured = Δομημένη προβολή (CSV, JSON, XML, YAML)
csv-delimiter = Διαχωριστικό
csv-rows = { $count ->
    [one] 1 γραμμή
   *[other] { $count } γραμμές
    }
csv-unterminated = Μη τερματισμένα εισαγωγικά από τη γραμμή { $line }
csv-truncated = Εμφανίζονται οι πρώτες 200.000 γραμμές
csv-edit-cell = Επεξεργασία κελιού
csv-insert-row-below = Εισαγωγή γραμμής από κάτω
csv-delete-row = Διαγραφή γραμμής
csv-read-only = Αρχείο μόνο για ανάγνωση: το πλέγμα δεν μπορεί να επεξεργαστεί
json-more = … { $count } ακόμη
json-edit-value = Επεξεργασία τιμής
json-rename-key = Μετονομασία κλειδιού
json-add-after = Προσθήκη μετά
json-add-inside = Προσθήκη μέσα
json-delete-node = Διαγραφή
json-read-only = Αρχείο μόνο για ανάγνωση: το δέντρο δεν μπορεί να επεξεργαστεί
json-invalid-value = Μη έγκυρη τιμή JSON
json-duplicate-key = Το αντικείμενο έχει ήδη κλειδί με αυτό το όνομα
xml-edit-value = Επεξεργασία τιμής
xml-edit-text = Επεξεργασία κειμένου
xml-rename = Μετονομασία
xml-add-attribute = Προσθήκη χαρακτηριστικού
xml-add-child = Προσθήκη θυγατρικού στοιχείου
xml-add-after = Προσθήκη μετά
xml-delete = Διαγραφή
xml-invalid-name = Μη έγκυρο όνομα XML
xml-duplicate-attribute = Το στοιχείο έχει ήδη χαρακτηριστικό με αυτό το όνομα
xml-not-well-formed = Αυτή η αλλαγή θα έκανε το XML μη έγκυρο
yaml-not-preserved = Αυτή η αλλαγή θα άλλαζε και άλλα μέρη του YAML
yaml-key-too-long = Ένα κλειδί μπορεί να έχει έως 1024 χαρακτήρες
yaml-read-only-flow = Το στυλ flow (γραμμένο μέσα σε αγκύλες) δεν μπορεί ακόμη να επεξεργαστεί στο δέντρο
yaml-read-only-anchor = Οι τιμές με άγκυρα (&) δεν μπορούν να επεξεργαστούν στο δέντρο
yaml-read-only-tag = Οι τιμές με ετικέτα (!) δεν μπορούν να επεξεργαστούν στο δέντρο
yaml-read-only-alias = Τα ψευδώνυμα (*) δεν μπορούν να επεξεργαστούν στο δέντρο
yaml-read-only-block-scalar = Τα μπλοκ κειμένου (| ή >) δεν μπορούν ακόμη να επεξεργαστούν στο δέντρο
yaml-read-only-complex-key = Τα σύνθετα κλειδιά δεν μπορούν να επεξεργαστούν στο δέντρο
yaml-read-only-document = Στο δέντρο μπορεί να επεξεργαστεί μόνο το πρώτο έγγραφο
yaml-read-only-layout = Αυτή η καταχώριση μοιράζεται τις γραμμές της με άλλο περιεχόμενο και δεν μπορεί να αλλάξει εδώ
yaml-read-only-only-child = Η μοναδική καταχώριση δεν μπορεί να αφαιρεθεί
dialog-open-link-title = Άνοιγμα αυτού του συνδέσμου στο πρόγραμμα περιήγησης;
button-open = Άνοιγμα
link-blocked = Αυτό το είδος συνδέσμου δεν ανοίγει, για λόγους ασφαλείας.
link-not-found = Το συνδεδεμένο αρχείο δεν βρέθηκε: { $target }
enc-windows-1252 = Δυτικοευρωπαϊκή
enc-iso-8859-15 = Δυτικοευρωπαϊκή
enc-windows-1250 = Κεντροευρωπαϊκή
enc-iso-8859-2 = Κεντροευρωπαϊκή
enc-windows-1251 = Κυριλλική
enc-koi8-r = Κυριλλική
enc-windows-1253 = Ελληνική
enc-windows-1254 = Τουρκική
enc-windows-1255 = Εβραϊκή
enc-windows-1256 = Αραβική
enc-windows-1257 = Βαλτική
enc-windows-874 = Ταϊλανδική
enc-shift_jis = Ιαπωνική
enc-euc-jp = Ιαπωνική
enc-gb18030 = Κινεζική, Απλοποιημένη
enc-big5 = Κινεζική, Παραδοσιακή
enc-euc-kr = Κορεατική
file-reopen-encoding = Επαναφόρτωση με κωδικοποίηση…
file-save-encoding = Αποθήκευση με κωδικοποίηση…
encoding-reopen-title = Επαναφόρτωση με κωδικοποίηση
encoding-save-title = Αποθήκευση με κωδικοποίηση
encoding-not-exact = Αυτή η κωδικοποίηση δεν μπορεί να ανοίξει το αρχείο χωρίς να το αλλάξει: { $reason }
encoding-unencodable = Ο χαρακτήρας «{ $character }» δεν μπορεί να αποθηκευτεί ως { $encoding }. Επιλέξτε άλλη κωδικοποίηση ή αλλάξτε το κείμενο· δεν γράφτηκε τίποτα.
encoding-save-first = Αποθηκεύστε ή αναιρέστε τις αλλαγές σας πριν από την επαναφόρτωση με άλλη κωδικοποίηση.
image-remote = Οι απομακρυσμένες εικόνες δεν φορτώνονται.
image-blocked = Εμφανίζονται μόνο εικόνες από τον φάκελο αυτού του εγγράφου.
image-unsupported = Μη υποστηριζόμενη μορφή εικόνας.
image-too-large = Η εικόνα είναι πολύ μεγάλη για εμφάνιση.
image-unreadable = Δεν ήταν δυνατή η ανάγνωση της εικόνας.
settings-theme-high-contrast = Υψηλή αντίθεση
view-fold = Σύμπτυξη
view-unfold = Ανάπτυξη
view-unfold-all = Ανάπτυξη όλων
settings-shortcuts = Συντομεύσεις πληκτρολογίου
settings-keymap-preset = Προεπιλογή
settings-keymap-default = Sanain
settings-keymap-notepadpp = Notepad++
settings-shortcut-change = Αλλαγή
settings-shortcut-reset = Επαναφορά
settings-shortcut-press = Πατήστε μια συντόμευση… (Esc για ακύρωση)
settings-shortcut-moved = Η συντόμευση μετακινήθηκε από:
edit-find-in-folder = Εύρεση σε φάκελο…
folder-search-placeholder = Εύρεση σε φάκελο
folder-search-run = Αναζήτηση
folder-search-stop = Διακοπή
folder-search-no-folder = Επιλογή φακέλου
folder-search-summary = { $count ->
    [one] 1 αντιστοιχία
   *[other] { $count } αντιστοιχίες
    } σε { $files ->
    [one] 1 αρχείο
   *[other] { $files } αρχεία
    }
app-check-updates = Έλεγχος για ενημερώσεις…
update-checking = Έλεγχος για ενημερώσεις…
update-up-to-date = Το Sanain { $version } είναι η νεότερη έκδοση.
update-no-releases = Δεν έχουν δημοσιευτεί ακόμη εκδόσεις του Sanain.
update-failed = Δεν ήταν δυνατός ο έλεγχος για ενημερώσεις: { $reason }
update-invalid = μη αναμενόμενη απόκριση
update-available-title = Το Sanain { $version } είναι διαθέσιμο
update-available-body = Ανοίξτε τη σελίδα λήψης για να το αποκτήσετε. Το Sanain δεν κατεβάζει ούτε εγκαθιστά τίποτα από μόνο του.
update-open-page = Άνοιγμα σελίδας λήψης
update-later = Αργότερα
settings-check-updates = Έλεγχος για ενημερώσεις μία φορά την ημέρα
settings-check-updates-detail = Ζητά από το github.com την τελευταία έκδοση. Δεν αποστέλλεται τίποτε άλλο.
file-local-history = Τοπικό ιστορικό…
file-clear-history = Εκκαθάριση τοπικού ιστορικού…
history-title = Τοπικό ιστορικό του { $name }
history-open = Άνοιγμα
history-restore = Επαναφορά
history-empty = Δεν υπάρχουν ακόμη αποθηκευμένες εκδόσεις. Σε κάθε αποθήκευση διατηρείται ένα αντίγραφο.
history-unreadable = Δεν ήταν δυνατή η ανάγνωση αυτής της έκδοσης.
history-just-now = Μόλις τώρα
history-minutes-ago = { $count ->
    [one] πριν από 1 λεπτό
   *[other] πριν από { $count } λεπτά
    }
history-hours-ago = { $count ->
    [one] πριν από 1 ώρα
   *[other] πριν από { $count } ώρες
    }
history-days-ago = { $count ->
    [one] πριν από 1 ημέρα
   *[other] πριν από { $count } ημέρες
    }
history-purge-title = Διαγραφή όλου του τοπικού ιστορικού;
history-purge-body = Τα αποθηκευμένα αντίγραφα όλων των αρχείων αφαιρούνται από αυτόν τον υπολογιστή. Τα ίδια τα αρχεία σας δεν αλλάζουν.
history-purge-confirm = Διαγραφή ιστορικού
history-purged = Το τοπικό ιστορικό διαγράφηκε.
settings-local-history = Διατήρηση τοπικού ιστορικού αποθηκευμένων αρχείων
settings-local-history-detail = Διατηρεί έως 50 εκδόσεις ανά αρχείο για 30 ημέρες, μόνο σε αυτόν τον υπολογιστή.
file-open-log = Άνοιγμα στην προβολή αρχείων καταγραφής…
log-lines = { $count ->
    [one] 1 γραμμή
   *[other] { $count } γραμμές
    }
log-indexing = Ανάγνωση… { $percent }%
log-matches = { $count ->
    [one] 1 γραμμή που ταιριάζει
   *[other] { $count } γραμμές που ταιριάζουν
    }
log-filter-hint = Φιλτράρισμα γραμμών
log-bad-pattern = Μη έγκυρο μοτίβο
log-follow = Παρακολούθηση
log-close = Κλείσιμο προβολής
log-opened-large = Το { $name } είναι μεγάλο, οπότε άνοιξε μόνο για ανάγνωση στην προβολή αρχείων καταγραφής.
tab-pin = Καρφίτσωμα καρτέλας
tab-unpin = Ξεκαρφίτσωμα καρτέλας
edit-add-cursor-above = Προσθήκη δρομέα επάνω
edit-add-cursor-below = Προσθήκη δρομέα κάτω
view-word-wrap = Αναδίπλωση λέξεων
status-wrap-on = Οι μεγάλες γραμμές αναδιπλώνονται πλέον στο πλάτος του παραθύρου.
status-wrap-off = Οι μεγάλες γραμμές δεν αναδιπλώνονται πλέον.
settings-word-wrap = Αναδίπλωση μεγάλων γραμμών
edit-go-to-line = Μετάβαση σε γραμμή…
edit-go-to-heading = Μετάβαση σε επικεφαλίδα…
goto-line-hint = Γραμμή ή γραμμή:στήλη
goto-line-note = { $count ->
    [one] Το έγγραφο έχει 1 γραμμή.
   *[other] Το έγγραφο έχει { $count } γραμμές.
    }
goto-line-invalid = Εισαγάγετε αριθμό γραμμής, προαιρετικά ακολουθούμενο από :στήλη.
goto-heading-hint = Πληκτρολογήστε για φιλτράρισμα επικεφαλίδων
goto-heading-none = Αυτό το έγγραφο δεν έχει επικεφαλίδες.
format-xml = XML
format-yaml = YAML
tools-tree-error = Μη έγκυρο { $format } στη γραμμή { $line }, στήλη { $column }: { $reason }
edit-select-next-occurrence = Επιλογή επόμενης εμφάνισης
tools-record-macro = Έναρξη ή διακοπή εγγραφής μακροεντολής
tools-play-macro = Αναπαραγωγή μακροεντολής
macro-recording = Εγγραφή μακροεντολής
macro-recorded = { $count ->
    [one] Η μακροεντολή καταγράφηκε: 1 βήμα.
   *[other] Η μακροεντολή καταγράφηκε: { $count } βήματα.
    }
tools-compare-saved = Σύγκριση με την αποθηκευμένη έκδοση
tools-compare-clipboard = Σύγκριση με το πρόχειρο
compare-saved-label = { $name } (στον δίσκο)
compare-clipboard-label = Πρόχειρο
compare-saved-failed = Δεν ήταν δυνατή η ανάγνωση του αποθηκευμένου αρχείου: { $reason }
compare-clipboard-empty = Το πρόχειρο δεν περιέχει κείμενο για σύγκριση.
error-save-disk-full = Το { $name } δεν αποθηκεύτηκε: ο δίσκος είναι πλήρης. Το αρχείο στον δίσκο δεν άλλαξε και το κείμενό σας είναι ακόμη εδώ. Ελευθερώστε χώρο ή χρησιμοποιήστε την εντολή Αποθήκευση ως για να το αποθηκεύσετε αλλού.
error-save-permission = Το { $name } δεν αποθηκεύτηκε: δεν έχετε δικαίωμα εγγραφής σε αυτή τη θέση. Το αρχείο στον δίσκο δεν άλλαξε. Χρησιμοποιήστε την εντολή Αποθήκευση ως για να το αποθηκεύσετε αλλού.
error-save-read-only-volume = Το { $name } δεν αποθηκεύτηκε: ο δίσκος είναι μόνο για ανάγνωση. Το αρχείο στον δίσκο δεν άλλαξε. Χρησιμοποιήστε την εντολή Αποθήκευση ως για να το αποθηκεύσετε αλλού.
replace-preview-title = { $count ->
    [one] Αντικατάσταση 1 αντιστοιχίας;
   *[other] Αντικατάσταση { $count } αντιστοιχιών;
    }
replace-preview-confirm = { $count ->
    [one] Αντικατάσταση 1
   *[other] Αντικατάσταση { $count }
    }
replace-preview-more = Εμφανίζονται μόνο οι πρώτες 200 γραμμές που αλλάζουν.
replace-preview-stale = Το έγγραφο άλλαξε· δεν αντικαταστάθηκε τίποτα. Εκτελέστε ξανά την Αντικατάσταση όλων.
encoding-detected = Άνοιξε ως { $encoding } (εντοπίστηκε από τα περιεχόμενα του αρχείου).
encoding-suggested = Προτείνεται από τα περιεχόμενα του αρχείου: { $encoding }. Ελέγξτε την προεπισκόπηση πριν από το άνοιγμα.
app-about = Σχετικά με το Sanain
menu-help = Βοήθεια
about-tagline = Η δύναμη των λέξεων.
about-version = Έκδοση { $version }
about-license = Ελεύθερο λογισμικό ανοιχτού κώδικα με την άδεια Mozilla Public License 2.0.
about-ai = Φτιαγμένο από AI. Με ανθρώπινη καθοδήγηση. Με τη δύναμη της κοινότητας.
about-notices = Τα στοιχεία τρίτων και οι άδειές τους αναφέρονται στο αρχείο THIRD_PARTY_NOTICES, που περιλαμβάνεται σε κάθε έκδοση.
about-source = Πηγαίος κώδικας και αναφορές προβλημάτων στο GitHub
tools-xml-validate = Επικύρωση XML
tools-yaml-validate = Επικύρωση YAML
tools-markup-valid = Το { $format } είναι έγκυρο
file-open-recent = Άνοιγμα πρόσφατου…
file-save-all = Αποθήκευση όλων
file-revert = Επαναφορά στην αποθηκευμένη έκδοση
recent-hint = Πληκτρολογήστε για φιλτράρισμα των πρόσφατων αρχείων
recent-none = Δεν υπάρχουν ακόμη πρόσφατα αρχεία.
revert-title = Απόρριψη αλλαγών στο { $name };
revert-body = Το αρχείο φορτώνεται ξανά από τον δίσκο. Οι μη αποθηκευμένες αλλαγές χάνονται και δεν μπορούν να αναιρεθούν.
revert-confirm = Απόρριψη και επαναφόρτωση
revert-done = Το { $name } φορτώθηκε ξανά από τον δίσκο.
status-saved-all = { $count ->
    [0] Δεν υπάρχει τίποτα για αποθήκευση.
    [one] Αποθηκεύτηκε 1 αρχείο.
   *[other] Αποθηκεύτηκαν { $count } αρχεία.
    }
edit-toggle-comment = Εναλλαγή σχολίου γραμμής
edit-indent = Εσοχή γραμμών
edit-outdent = Αφαίρεση εσοχής γραμμών
comment-unsupported = Αυτός ο τύπος αρχείου δεν έχει σχόλια γραμμής.
tab-close-others = Κλείσιμο άλλων καρτελών
tab-close-all = Κλείσιμο όλων των καρτελών
file-copy-path = Αντιγραφή διαδρομής αρχείου
file-reveal-macos = Εμφάνιση στο Finder
file-reveal-windows = Εμφάνιση στην Εξερεύνηση αρχείων
file-reveal-other = Εμφάνιση στη διαχείριση αρχείων
status-path-copied = Η διαδρομή αρχείου αντιγράφηκε.
tabs-kept-unsaved = { $count ->
    [one] 1 καρτέλα με μη αποθηκευμένες αλλαγές παρέμεινε ανοιχτή.
   *[other] { $count } καρτέλες με μη αποθηκευμένες αλλαγές παρέμειναν ανοιχτές.
    }
status-indent-tabs = Στηλοθέτες
status-indent-spaces = Κενά: { $count }
find-searching = Αναζήτηση…
compare-ignore-whitespace = Παράβλεψη κενών
tools-compare-file = Σύγκριση με αρχείο…
compare-file-failed = Δεν ήταν δυνατή η σύγκριση με το { $name }: { $reason }
cli-help =
    Χρήση:
      sanain [ΑΡΧΕΙΟ…]                άνοιγμα αρχείων
      sanain --diff ΑΡΙΣΤΕΡΑ ΔΕΞΙΑ    σύγκριση δύο αρχείων δίπλα-δίπλα
      sanain --version                εμφάνιση της έκδοσης
      sanain --help                   εμφάνιση αυτής της βοήθειας
    Χρησιμοποιήστε -- πριν από ονόματα αρχείων που αρχίζουν με παύλα.
cli-unknown-option = Άγνωστη επιλογή { $option }.
cli-diff-needs-two = Το --diff χρειάζεται δύο αρχεία: sanain --diff ΑΡΙΣΤΕΡΑ ΔΕΞΙΑ
window-move-tab-left = Μετακίνηση καρτέλας αριστερά
window-move-tab-right = Μετακίνηση καρτέλας δεξιά
edit-skip-occurrence = Μετάβαση στην επόμενη εμφάνιση
tools-play-macro-times = Αναπαραγωγή μακροεντολής πολλές φορές…
macro-times-hint = Αριθμός επαναλήψεων
macro-times-note = Από 1 έως { $max }
macro-times-invalid = Εισαγάγετε έναν αριθμό από 1 έως { $max }.
view-outline = Διάρθρωση
outline-empty = Δεν υπάρχουν επικεφαλίδες σε αυτό το έγγραφο.
outline-close = Κλείσιμο διάρθρωσης
folder-replace-placeholder = Αντικατάσταση με
folder-replace-run = Αντικατάσταση σε αρχεία…
replace-files-title = Αντικατάσταση { $count ->
    [one] 1 αντιστοιχίας
   *[other] { $count } αντιστοιχιών
    } σε { $files ->
    [one] 1 αρχείο
   *[other] { $files } αρχεία
    };
replace-files-body = Τα αρχεία που είναι ανοιχτά σε καρτέλα αλλάζουν εκεί και η αλλαγή μπορεί να αναιρεθεί· αποθηκεύστε τα μόνοι σας. Τα υπόλοιπα αρχεία αποθηκεύονται τώρα και το προηγούμενο περιεχόμενό τους διατηρείται στο τοπικό ιστορικό, αν είναι ενεργοποιημένο. Ένα αρχείο που άλλαξε στον δίσκο στο μεταξύ παραλείπεται.
replace-files-confirm = Αντικατάσταση
replace-files-done = { $count ->
    [one] Αντικαταστάθηκε 1 αντιστοιχία
   *[other] Αντικαταστάθηκαν { $count } αντιστοιχίες
    } σε { $files ->
    [one] 1 αρχείο
   *[other] { $files } αρχεία
    } (ανοιχτά σε καρτέλες: { $open }, δεν αποθηκεύτηκαν).
replace-files-failed = { $failed ->
    [one] 1 αρχείο δεν άλλαξε:
   *[other] { $failed } αρχεία δεν άλλαξαν· πρώτο:
    } { $first }
image-outside = Αυτή η εικόνα βρίσκεται εκτός του φακέλου του εγγράφου.
image-allow = Εμφάνιση εικόνων από αυτόν τον φάκελο
cli-help-integration =
    Ενσωμάτωση στη διαχείριση αρχείων (μόνο για τον τρέχοντα χρήστη):
      sanain --register           προσθήκη του Sanain στο «Άνοιγμα με» (Linux, Windows)
      sanain --unregister         αφαίρεσή του ξανά
cli-registered = Το Sanain προστέθηκε στο «Άνοιγμα με». { $place }
cli-unregistered = Το Sanain αφαιρέθηκε από το «Άνοιγμα με».
cli-register-failed = Δεν ήταν δυνατή η αλλαγή της ενσωμάτωσης στη διαχείριση αρχείων: { $reason }
cli-register-macos = Στο macOS δεν χρειάζεται καμία καταχώριση: μόλις το Sanain.app βρίσκεται στον φάκελο «Εφαρμογές», το Finder το προσφέρει στο «Άνοιγμα με».
settings-tab-general = Γενικά
settings-tab-editor = Επεξεργαστής
settings-tab-appearance = Εμφάνιση
settings-shortcut-search = Αναζήτηση εντολών ή πλήκτρων
settings-font = Γραμματοσειρά
settings-font-default = Ενσωματωμένη σταθερού πλάτους
settings-font-loading = Αναζήτηση εγκατεστημένων γραμματοσειρών…
settings-font-missing = Η γραμματοσειρά { $font } δεν βρέθηκε· χρησιμοποιείται η ενσωματωμένη γραμματοσειρά.
settings-theme-more = Περισσότερα θέματα
settings-theme-user-note = Δικά σας θέματα: αρχεία .theme στον φάκελο { $folder } (δείτε docs/THEMES.md).
language-auto = Αυτόματη ανίχνευση
language-choose = Επιλέξτε τη γλώσσα αυτής της καρτέλας
tools-xml-format = Μορφοποίηση XML
tools-save-macro = Αποθήκευση μακροεντολής…
tools-play-saved-macro = Αναπαραγωγή αποθηκευμένης μακροεντολής…
tools-delete-saved-macro = Διαγραφή αποθηκευμένης μακροεντολής…
macro-save-hint = Όνομα μακροεντολής
macro-save-note = Μια αποθηκευμένη μακροεντολή με το ίδιο όνομα αντικαθίσταται.
macro-save-empty = Δεν υπάρχει κάτι για αποθήκευση: καταγράψτε πρώτα μια μακροεντολή.
macro-saved = Η μακροεντολή αποθηκεύτηκε: { $name }
macro-deleted = Η μακροεντολή διαγράφηκε: { $name }
macro-name-invalid = Εισαγάγετε όνομα από 1 έως { $max } χαρακτήρες σε μία γραμμή.
macro-save-too-large = Η μακροεντολή είναι πολύ μεγάλη για αποθήκευση.
macro-save-full = Μπορούν να αποθηκευτούν έως { $max } μακροεντολές. Διαγράψτε πρώτα μία.
macro-save-failed = Δεν ήταν δυνατή η αποθήκευση των μακροεντολών: { $error }
macro-saved-hint = Πληκτρολογήστε για φιλτράρισμα των αποθηκευμένων μακροεντολών
macro-saved-none = Δεν υπάρχουν ακόμη αποθηκευμένες μακροεντολές.
settings-shortcut-import-npp = Εισαγωγή από το Notepad++…
settings-shortcut-import-npp-hint = Επιλέξτε ένα αρχείο shortcuts.xml του Notepad++. Στα Windows βρίσκεται στο %AppData%\Notepad++\shortcuts.xml.
settings-shortcut-import-npp-done = Εισήχθησαν από το Notepad++: { $imported }. Παραλείφθηκαν: { $skipped }. Συγκρούσεις που απομένουν: { $conflicts }.
settings-shortcut-import-npp-failed = Δεν ήταν δυνατή η εισαγωγή του { $name }: { $reason }
settings-shortcut-import-npp-too-large = Το αρχείο είναι μεγαλύτερο από 1 MiB.
settings-shortcut-import-npp-invalid = Το αρχείο δεν είναι έγκυρο shortcuts.xml του Notepad++.
view-split-right = Διαίρεση δεξιά
view-split-down = Διαίρεση κάτω
view-close-pane = Κλείσιμο τμήματος
view-focus-next-pane = Επόμενο τμήμα
view-focus-prev-pane = Προηγούμενο τμήμα
status-pane-limit = Έχει επιτευχθεί το όριο τμημάτων (έως { $max }).
