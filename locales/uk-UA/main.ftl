# Machine translation by Claude (AI); review by a native speaker required before release.
app-name = Sanain
menu-file = Файл
file-open = Відкрити…
file-save = Зберегти
status-tabs = { $count ->
    [one] { $count } вкладка
    [few] { $count } вкладки
    [many] { $count } вкладок
   *[other] { $count } вкладки
    }
error-open = Не вдалося відкрити { $name }.
menu-edit = Редагування
file-new = Нова вкладка
file-save-as = Зберегти як…
edit-undo = Скасувати
edit-redo = Повторити
edit-select-all = Виділити все
untitled = Без назви
window-title = { $dirty ->
    [yes] ● { $name } — Sanain
   *[no] { $name } — Sanain
    }
status-position = Рядок { $line }, стовпець { $column }
status-selections = { $count ->
    [one] { $count } виділення
    [few] { $count } виділення
    [many] { $count } виділень
   *[other] { $count } виділення
    }
status-saved = Збережено: { $name }
encoding-utf8 = UTF-8
encoding-utf8-bom = UTF-8 з BOM
encoding-utf16le = UTF-16 LE
encoding-utf16be = UTF-16 BE
line-ending-lf = LF
line-ending-crlf = CRLF
line-ending-cr = CR
line-ending-mixed = Змішані закінчення рядків
editor-accessible-name = Текстовий редактор
button-save = Зберегти
button-dont-save = Не зберігати
button-cancel = Скасувати
button-ok = OK
button-overwrite = Перезаписати
button-save-as = Зберегти як…
dialog-unsaved-title = Зберегти зміни в { $name }?
dialog-unsaved-body = Якщо не зберегти зміни, їх буде втрачено.
dialog-conflict-title = Файл { $name } змінено на диску
dialog-conflict-body = Інша програма змінила цей файл після того, як ви його відкрили. Перезапис замінить ці зміни.
dialog-readonly-title = Файл { $name } доступний лише для читання
dialog-readonly-body = Збережіть копію під іншою назвою.
error-open-encoding = { $name } не є текстом у UTF-8 чи UTF-16. Файл не відкрито, тому його вміст не змінено. Щоб вибрати кодування, скористайтеся Файл > Відкрити знову з кодуванням.
error-save = Не вдалося зберегти { $name }: { $reason }
menu-window = Вікно
edit-cut = Вирізати
edit-copy = Копіювати
edit-paste = Вставити
app-quit = Вийти з Sanain
app-hide = Сховати Sanain
app-hide-others = Сховати інші
app-show-all = Показати все
window-minimize = Згорнути
window-zoom = Масштабувати
status-recovered = Відновлено незбережені зміни з попереднього сеансу
status-recovery-failed = Не вдалося зберегти резервну копію: { $reason }
edit-duplicate-line = Дублювати рядок
edit-delete-line = Видалити рядок
edit-move-line-up = Перемістити рядок вгору
edit-move-line-down = Перемістити рядок вниз
file-close-tab = Закрити вкладку
file-reopen-tab = Відкрити закриту вкладку
window-next-tab = Наступна вкладка
window-prev-tab = Попередня вкладка
tab-close = Закрити вкладку
edit-find = Знайти…
edit-find-next = Знайти наступне
edit-find-prev = Знайти попереднє
edit-replace = Замінити…
find-placeholder = Знайти
replace-placeholder = Замінити на
find-match-case = З урахуванням регістру
find-whole-word = Ціле слово
find-regex = Регулярний вираз
find-no-results = Немає результатів
find-count = { $total ->
    [one] { $total } збіг
    [few] { $total } збіги
    [many] { $total } збігів
   *[other] { $total } збігу
    }
find-count-current = { $current } з { $total }
find-prev = Попередній збіг (Shift+Enter)
find-next = Наступний збіг (Enter)
find-close = Закрити (Esc)
replace-one = Замінити
replace-all = Замінити все
status-replaced = { $count ->
    [one] Замінено { $count } збіг
    [few] Замінено { $count } збіги
    [many] Замінено { $count } збігів
   *[other] Замінено { $count } збігу
    }
language-plain = Звичайний текст
menu-view = Вигляд
app-settings = Налаштування…
view-zoom-in = Збільшити
view-zoom-out = Зменшити
view-zoom-reset = Фактичний розмір
settings-title = Налаштування
settings-font-size = Розмір шрифту
settings-tab-width = Ширина табуляції
settings-insert-spaces = Вставляти пробіли при натисканні Tab
settings-theme = Оформлення
settings-theme-system = Як у системі
settings-theme-light = Світле
settings-theme-dark = Темне
settings-restore-session = Відкривати файли з попереднього сеансу
settings-language = Мова
settings-language-system = Як у системі
settings-language-note = Зміна мови набуде чинності після перезапуску Sanain.
settings-save-failed = Не вдалося зберегти налаштування: { $reason }
view-command-palette = Палітра команд…
palette-placeholder = Введіть команду
view-markdown-source = Вихідний текст Markdown
view-markdown-split = Вихідний текст і перегляд Markdown
view-markdown-preview = Перегляд Markdown
preview-too-large = Документ завеликий для попереднього перегляду.
preview-mermaid-unsupported = Діаграми Mermaid поки не відображаються; показано вихідний текст.
menu-tools = Інструменти
tools-json-validate = Перевірити JSON
tools-json-format = Форматувати JSON
tools-json-minify = Стиснути JSON
tools-sort-lines = Сортувати рядки
tools-sort-lines-desc = Сортувати рядки за спаданням
tools-unique-lines = Видалити повторювані рядки
tools-trim-trailing = Видалити пробіли в кінці рядків
tools-upper-case = Верхній регістр
tools-lower-case = Нижній регістр
tools-to-lf = Перетворити закінчення рядків на LF
tools-to-crlf = Перетворити закінчення рядків на CRLF
tools-json-valid = JSON коректний
tools-done = Готово. Команда «Скасувати» поверне зміни.
tools-no-change = Нічого змінювати
tools-json-error = Некоректний JSON у рядку { $line }, стовпці { $column }: { $reason }
json-reason-unterminated-string = незакритий рядок
json-reason-invalid-u-escape = некоректна escape-послідовність \u
json-reason-invalid-escape = некоректна escape-послідовність
json-reason-control-character-in-string = керівний символ у рядку
json-reason-invalid-number = некоректне число
json-reason-unexpected-character = неочікуваний символ
json-reason-unexpected-content-after-the-value = неочікуваний вміст після значення
json-reason-unexpected-comma = неочікувана кома
json-reason-expected-a-string-key = очікувався рядковий ключ
json-reason-expected = очікувалося ':'
json-reason-expected-or-a-closing-bracket = очікувалася ',' або закривна дужка
json-reason-expected-a-value = очікувалося значення
json-reason-empty-document = порожній документ
json-reason-unexpected-end-of-input = неочікуваний кінець даних
json-reason-too-deeply-nested-to-format = надто глибока вкладеність для форматування
tools-compare = Порівняти з…
compare-need-two = Відкрийте ще одну вкладку для порівняння.
compare-pick-title = Порівняти { $name } з
compare-count = { $count ->
    [0] Відмінностей немає
    [one] { $count } зміна
    [few] { $count } зміни
    [many] { $count } змін
   *[other] { $count } зміни
    }
compare-close = Закрити порівняння
compare-to-left = Скопіювати цю зміну ліворуч
compare-to-right = Скопіювати цю зміну праворуч
compare-stale = Документ змінився під час порівняння. Порівняння оновлено; спробуйте ще раз.
view-structured = Структурований вигляд (CSV, JSON, XML, YAML)
csv-delimiter = Роздільник
csv-rows = { $count ->
    [one] { $count } рядок
    [few] { $count } рядки
    [many] { $count } рядків
   *[other] { $count } рядка
    }
csv-unterminated = Незакрита лапка, починаючи з рядка { $line }
csv-truncated = Показано перші 200 000 рядків
json-more = … ще { $count }
dialog-open-link-title = Відкрити це посилання у веббраузері?
button-open = Відкрити
link-blocked = Посилання такого типу не відкриваються з міркувань безпеки.
link-not-found = Пов’язаний файл не знайдено: { $target }
enc-windows-1252 = Західноєвропейське
enc-iso-8859-15 = Західноєвропейське
enc-windows-1250 = Центральноєвропейське
enc-iso-8859-2 = Центральноєвропейське
enc-windows-1251 = Кирилиця
enc-koi8-r = Кирилиця
enc-windows-1253 = Грецьке
enc-windows-1254 = Турецьке
enc-windows-1255 = Іврит
enc-windows-1256 = Арабське
enc-windows-1257 = Балтійське
enc-windows-874 = Тайське
enc-shift_jis = Японське
enc-euc-jp = Японське
enc-gb18030 = Китайське спрощене
enc-big5 = Китайське традиційне
enc-euc-kr = Корейське
file-reopen-encoding = Відкрити знову з кодуванням…
file-save-encoding = Зберегти з кодуванням…
encoding-reopen-title = Відкрити знову з кодуванням
encoding-save-title = Зберегти з кодуванням
encoding-not-exact = Це кодування не може відкрити файл без його зміни: { $reason }
encoding-unencodable = «{ $character }» неможливо зберегти в кодуванні { $encoding }. Виберіть інше кодування або змініть текст; нічого не записано.
encoding-save-first = Збережіть або скасуйте зміни, перш ніж відкривати файл з іншим кодуванням.
image-remote = Віддалені зображення не завантажуються.
image-blocked = Показуються лише зображення з теки цього документа.
image-unsupported = Непідтримуваний формат зображення.
image-too-large = Зображення завелике для показу.
image-unreadable = Не вдалося прочитати зображення.
settings-theme-high-contrast = Висока контрастність
view-fold = Згорнути
view-unfold = Розгорнути
view-unfold-all = Розгорнути все
settings-shortcuts = Сполучення клавіш
settings-keymap-preset = Набір
settings-keymap-default = Sanain
settings-keymap-notepadpp = Notepad++
settings-shortcut-change = Змінити
settings-shortcut-reset = Скинути
settings-shortcut-press = Натисніть сполучення клавіш… (Esc — скасувати)
settings-shortcut-moved = Сполучення перенесено з:
edit-find-in-folder = Знайти в теці…
folder-search-placeholder = Знайти в теці
folder-search-run = Шукати
folder-search-stop = Зупинити
folder-search-no-folder = Вибрати теку
folder-search-summary = { $count ->
    [one] { $count } збіг
    [few] { $count } збіги
    [many] { $count } збігів
   *[other] { $count } збігу
    } у { $files ->
    [one] { $files } файлі
    [few] { $files } файлах
    [many] { $files } файлах
   *[other] { $files } файла
    }
app-check-updates = Перевірити оновлення…
update-checking = Перевірка оновлень…
update-up-to-date = Sanain { $version } — найновіша версія.
update-no-releases = Випуски Sanain ще не опубліковано.
update-failed = Не вдалося перевірити оновлення: { $reason }
update-invalid = неочікувана відповідь
update-available-title = Доступна версія Sanain { $version }
update-available-body = Відкрийте сторінку завантаження, щоб отримати її. Sanain нічого не завантажує й не встановлює самостійно.
update-open-page = Відкрити сторінку завантаження
update-later = Пізніше
settings-check-updates = Перевіряти оновлення раз на день
settings-check-updates-detail = Запитує в github.com останній випуск. Більше нічого не надсилається.
file-local-history = Локальна історія…
file-clear-history = Очистити локальну історію…
history-title = Локальна історія файлу { $name }
history-open = Відкрити
history-restore = Відновити
history-empty = Збережених версій ще немає. Під час кожного збереження зберігається копія.
history-unreadable = Не вдалося прочитати цю версію.
history-just-now = Щойно
history-minutes-ago = { $count ->
    [one] { $count } хвилину тому
    [few] { $count } хвилини тому
    [many] { $count } хвилин тому
   *[other] { $count } хвилини тому
    }
history-hours-ago = { $count ->
    [one] { $count } годину тому
    [few] { $count } години тому
    [many] { $count } годин тому
   *[other] { $count } години тому
    }
history-days-ago = { $count ->
    [one] { $count } день тому
    [few] { $count } дні тому
    [many] { $count } днів тому
   *[other] { $count } дня тому
    }
history-purge-title = Видалити всю локальну історію?
history-purge-body = Збережені копії всіх файлів буде видалено з цього комп’ютера. Самі файли не змінюються.
history-purge-confirm = Видалити історію
history-purged = Локальну історію видалено.
settings-local-history = Зберігати локальну історію збережених файлів
settings-local-history-detail = Зберігає до 50 версій кожного файлу протягом 30 днів, лише на цьому комп’ютері.
file-open-log = Відкрити в переглядачі журналів…
log-lines = { $count ->
    [one] { $count } рядок
    [few] { $count } рядки
    [many] { $count } рядків
   *[other] { $count } рядка
    }
log-indexing = Читання… { $percent } %
log-matches = { $count ->
    [one] { $count } відповідний рядок
    [few] { $count } відповідні рядки
    [many] { $count } відповідних рядків
   *[other] { $count } відповідного рядка
    }
log-filter-hint = Фільтрувати рядки
log-bad-pattern = Неприпустимий шаблон
log-follow = Стежити
log-close = Закрити переглядач
log-opened-large = Файл { $name } завеликий, тому його відкрито лише для читання в переглядачі журналів.
tab-pin = Закріпити вкладку
tab-unpin = Відкріпити вкладку
edit-add-cursor-above = Додати курсор вище
edit-add-cursor-below = Додати курсор нижче
view-word-wrap = Перенесення слів
status-wrap-on = Довгі рядки тепер переносяться за шириною вікна.
status-wrap-off = Довгі рядки більше не переносяться.
settings-word-wrap = Переносити довгі рядки
edit-go-to-line = Перейти до рядка…
edit-go-to-heading = Перейти до заголовка…
goto-line-hint = Рядок або рядок:стовпець
goto-line-note = { $count ->
    [one] У документі { $count } рядок.
    [few] У документі { $count } рядки.
    [many] У документі { $count } рядків.
   *[other] У документі { $count } рядка.
    }
goto-line-invalid = Введіть номер рядка, за потреби з :стовпець.
goto-heading-hint = Введіть текст для фільтрування заголовків
goto-heading-none = У цьому документі немає заголовків.
format-xml = XML
format-yaml = YAML
tools-tree-error = Некоректний { $format } у рядку { $line }, стовпці { $column }: { $reason }
edit-select-next-occurrence = Виділити наступне входження
tools-record-macro = Почати або зупинити запис макросу
tools-play-macro = Відтворити макрос
macro-recording = Записується макрос
macro-recorded = { $count ->
    [one] Макрос записано: { $count } крок.
    [few] Макрос записано: { $count } кроки.
    [many] Макрос записано: { $count } кроків.
   *[other] Макрос записано: { $count } кроку.
    }
tools-compare-saved = Порівняти зі збереженою версією
tools-compare-clipboard = Порівняти з буфером обміну
compare-saved-label = { $name } (на диску)
compare-clipboard-label = Буфер обміну
compare-saved-failed = Не вдалося прочитати збережений файл: { $reason }
compare-clipboard-empty = У буфері обміну немає тексту для порівняння.
error-save-disk-full = Файл { $name } не збережено: диск заповнений. Файл на диску не змінено, а ваш текст досі тут. Звільніть місце або скористайтеся командою «Зберегти як», щоб зберегти файл в іншому місці.
error-save-permission = Файл { $name } не збережено: у вас немає дозволу на запис у це місце. Файл на диску не змінено. Скористайтеся командою «Зберегти як», щоб зберегти файл в іншому місці.
error-save-read-only-volume = Файл { $name } не збережено: диск доступний лише для читання. Файл на диску не змінено. Скористайтеся командою «Зберегти як», щоб зберегти файл в іншому місці.
replace-preview-title = { $count ->
    [one] Замінити { $count } збіг?
    [few] Замінити { $count } збіги?
    [many] Замінити { $count } збігів?
   *[other] Замінити { $count } збігу?
    }
replace-preview-confirm = { $count ->
    [one] Замінити { $count }
    [few] Замінити { $count }
    [many] Замінити { $count }
   *[other] Замінити { $count }
    }
replace-preview-more = Показано лише перші 200 змінених рядків.
replace-preview-stale = Документ змінився; нічого не замінено. Виконайте «Замінити все» ще раз.
view-split-editor = Розділити редактор
encoding-detected = Відкрито в кодуванні { $encoding } (визначено за вмістом файлу).
encoding-suggested = Запропоновано за вмістом файлу: { $encoding }. Перед відкриттям перевірте попередній перегляд.
app-about = Про Sanain
menu-help = Довідка
about-tagline = Сила слів.
about-version = Версія { $version }
about-license = Вільне ПЗ з відкритим кодом за ліцензією Mozilla Public License 2.0.
about-ai = Створено ШІ. Керовано людьми. Розвивається спільнотою.
about-notices = Сторонні компоненти та їхні ліцензії перелічено у файлі THIRD_PARTY_NOTICES, що входить до кожного випуску.
about-source = Вихідний код і задачі на GitHub
tools-xml-validate = Перевірити XML
tools-yaml-validate = Перевірити YAML
tools-markup-valid = { $format } коректний
file-open-recent = Відкрити недавні…
file-save-all = Зберегти все
file-revert = Повернути збережену версію
recent-hint = Введіть текст, щоб відфільтрувати недавні файли
recent-none = Недавніх файлів поки немає.
revert-title = Скасувати зміни у файлі { $name }?
revert-body = Файл буде повторно завантажено з диска. Незбережені зміни буде втрачено без можливості скасування.
revert-confirm = Скасувати й перезавантажити
revert-done = Файл { $name } повторно завантажено з диска.
status-saved-all = { $count ->
    [0] Нічого зберігати.
    [one] Збережено { $count } файл.
    [few] Збережено { $count } файли.
    [many] Збережено { $count } файлів.
   *[other] Збережено { $count } файлу.
    }
edit-toggle-comment = Закоментувати/розкоментувати рядки
edit-indent = Збільшити відступ рядків
edit-outdent = Зменшити відступ рядків
comment-unsupported = Цей тип файлів не підтримує рядкові коментарі.
tab-close-others = Закрити інші вкладки
tab-close-all = Закрити всі вкладки
file-copy-path = Копіювати шлях до файлу
file-reveal-macos = Показати у Finder
file-reveal-windows = Показати у Провіднику
file-reveal-other = Показати у файловому менеджері
status-path-copied = Шлях до файлу скопійовано.
tabs-kept-unsaved = { $count ->
    [one] { $count } вкладку з незбереженими змінами залишено відкритою.
    [few] { $count } вкладки з незбереженими змінами залишено відкритими.
    [many] { $count } вкладок з незбереженими змінами залишено відкритими.
   *[other] { $count } вкладки з незбереженими змінами залишено відкритими.
    }
status-indent-tabs = Табуляція
status-indent-spaces = Пробіли: { $count }
find-searching = Пошук…
compare-ignore-whitespace = Ігнорувати пробіли
tools-compare-file = Порівняти з файлом…
compare-file-failed = Не вдалося порівняти з { $name }: { $reason }
cli-help =
    Використання:
      sanain [ФАЙЛ…]                відкрити файли
      sanain --diff ЛІВИЙ ПРАВИЙ    порівняти два файли поруч
      sanain --version              показати версію
      sanain --help                 показати цю довідку
    Використовуйте -- перед назвами файлів, що починаються з дефіса.
cli-unknown-option = Невідомий параметр { $option }.
cli-diff-needs-two = Для --diff потрібні два файли: sanain --diff ЛІВИЙ ПРАВИЙ
window-move-tab-left = Перемістити вкладку ліворуч
window-move-tab-right = Перемістити вкладку праворуч
edit-skip-occurrence = Перейти до наступного входження
tools-play-macro-times = Відтворити макрос кілька разів…
macro-times-hint = Кількість повторів
macro-times-note = Від 1 до { $max }
macro-times-invalid = Введіть число від 1 до { $max }.
view-outline = Структура
outline-empty = У цьому документі немає заголовків.
outline-close = Закрити структуру
folder-replace-placeholder = Замінити на
folder-replace-run = Замінити у файлах…
replace-files-title = Замінити { $count ->
    [one] { $count } збіг
    [few] { $count } збіги
    [many] { $count } збігів
   *[other] { $count } збігу
    } у { $files ->
    [one] { $files } файлі
    [few] { $files } файлах
    [many] { $files } файлах
   *[other] { $files } файла
    }?
replace-files-body = Файли, відкриті у вкладці, змінюються там, і зміну можна скасувати; збережіть їх самостійно. Інші файли зберігаються одразу, а їхній попередній вміст зберігається в локальній історії, якщо її ввімкнено. Файл, який тим часом змінився на диску, пропускається.
replace-files-confirm = Замінити
replace-files-done = { $count ->
    [one] Замінено { $count } збіг
    [few] Замінено { $count } збіги
    [many] Замінено { $count } збігів
   *[other] Замінено { $count } збігу
    } у { $files ->
    [one] { $files } файлі
    [few] { $files } файлах
    [many] { $files } файлах
   *[other] { $files } файла
    } (відкрито у вкладках: { $open }, не збережено).
replace-files-failed = { $failed ->
    [one] { $failed } файл не змінено:
    [few] { $failed } файли не змінено; перший:
    [many] { $failed } файлів не змінено; перший:
   *[other] { $failed } файла не змінено; перший:
    } { $first }
image-outside = Це зображення розташоване поза текою документа.
image-allow = Показати зображення з цієї теки
cli-help-integration =
    Інтеграція з файловим менеджером (лише для поточного користувача):
      sanain --register           додати Sanain до «Відкрити за допомогою» (Linux, Windows)
      sanain --unregister         знову вилучити його
cli-registered = Sanain додано до «Відкрити за допомогою». { $place }
cli-unregistered = Sanain вилучено з «Відкрити за допомогою».
cli-register-failed = Не вдалося змінити інтеграцію з файловим менеджером: { $reason }
cli-register-macos = Інтеграція з Finder у macOS поки що недоступна. Відкривайте файли з Sanain (Файл › Відкрити, перетягування) або з командного рядка.
settings-tab-general = Загальні
settings-tab-editor = Редактор
settings-tab-appearance = Оформлення
settings-shortcut-search = Пошук команд або клавіш
