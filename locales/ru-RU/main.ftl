# Machine translation by Claude (AI); review by a native speaker required before release.
app-name = Sanain
menu-file = Файл
file-open = Открыть…
file-save = Сохранить
status-tabs = { $count ->
    [one] { $count } вкладка
    [few] { $count } вкладки
    [many] { $count } вкладок
   *[other] { $count } вкладки
    }
error-open = Не удалось открыть { $name }.
menu-edit = Правка
file-new = Новая вкладка
file-save-as = Сохранить как…
edit-undo = Отменить
edit-redo = Повторить
edit-select-all = Выделить всё
untitled = Без имени
window-title = { $dirty ->
    [yes] ● { $name } — Sanain
   *[no] { $name } — Sanain
    }
status-position = Стр { $line }, стлб { $column }
status-selections = { $count ->
    [one] { $count } выделение
    [few] { $count } выделения
    [many] { $count } выделений
   *[other] { $count } выделения
    }
status-saved = Сохранено: { $name }
encoding-utf8 = UTF-8
encoding-utf8-bom = UTF-8 с BOM
encoding-utf16le = UTF-16 LE
encoding-utf16be = UTF-16 BE
line-ending-lf = LF
line-ending-crlf = CRLF
line-ending-cr = CR
line-ending-mixed = Смешанные концы строк
editor-accessible-name = Текстовый редактор
button-save = Сохранить
button-dont-save = Не сохранять
button-cancel = Отмена
button-ok = OK
button-overwrite = Перезаписать
button-save-as = Сохранить как…
dialog-unsaved-title = Сохранить изменения в { $name }?
dialog-unsaved-body = Если не сохранить изменения, они будут потеряны.
dialog-conflict-title = Файл { $name } изменён на диске
dialog-conflict-body = Другая программа изменила этот файл после того, как вы его открыли. Перезапись заменит эти изменения.
dialog-readonly-title = Файл { $name } доступен только для чтения
dialog-readonly-body = Сохраните копию под другим именем.
error-open-encoding = { $name } не является текстом в UTF-8 или UTF-16. Файл не открыт, поэтому его содержимое не изменено. Чтобы выбрать кодировку, используйте Файл > Открыть заново в кодировке.
error-save = Не удалось сохранить { $name }: { $reason }
menu-window = Окно
edit-cut = Вырезать
edit-copy = Копировать
edit-paste = Вставить
app-quit = Завершить Sanain
app-hide = Скрыть Sanain
app-hide-others = Скрыть остальные
app-show-all = Показать все
window-minimize = Свернуть
window-zoom = Изменить масштаб
status-recovered = Восстановлены несохранённые изменения из прошлого сеанса
status-recovery-failed = Не удалось сохранить резервную копию: { $reason }
edit-duplicate-line = Дублировать строку
edit-delete-line = Удалить строку
edit-move-line-up = Переместить строку вверх
edit-move-line-down = Переместить строку вниз
file-close-tab = Закрыть вкладку
file-reopen-tab = Открыть закрытую вкладку
window-next-tab = Следующая вкладка
window-prev-tab = Предыдущая вкладка
tab-close = Закрыть вкладку
edit-find = Найти…
edit-find-next = Найти далее
edit-find-prev = Найти ранее
edit-replace = Заменить…
find-placeholder = Найти
replace-placeholder = Заменить на
find-match-case = С учётом регистра
find-whole-word = Слово целиком
find-regex = Регулярное выражение
find-no-results = Нет результатов
find-count = { $total ->
    [one] { $total } совпадение
    [few] { $total } совпадения
    [many] { $total } совпадений
   *[other] { $total } совпадения
    }
find-count-current = { $current } из { $total }
find-prev = Предыдущее совпадение (Shift+Enter)
find-next = Следующее совпадение (Enter)
find-close = Закрыть (Esc)
replace-one = Заменить
replace-all = Заменить все
status-replaced = { $count ->
    [one] Заменено { $count } совпадение
    [few] Заменено { $count } совпадения
    [many] Заменено { $count } совпадений
   *[other] Заменено { $count } совпадения
    }
language-plain = Обычный текст
menu-view = Вид
app-settings = Настройки…
view-zoom-in = Увеличить
view-zoom-out = Уменьшить
view-zoom-reset = Фактический размер
settings-title = Настройки
settings-font-size = Размер шрифта
settings-tab-width = Ширина табуляции
settings-insert-spaces = Вставлять пробелы при нажатии Tab
settings-theme = Оформление
settings-theme-system = Как в системе
settings-theme-light = Светлое
settings-theme-dark = Тёмное
settings-restore-session = Открывать файлы из прошлого сеанса
settings-language = Язык
settings-language-system = Как в системе
settings-language-note = Смена языка вступит в силу после перезапуска Sanain.
settings-save-failed = Не удалось сохранить настройки: { $reason }
view-command-palette = Палитра команд…
palette-placeholder = Введите команду
view-markdown-source = Исходный текст Markdown
view-markdown-split = Исходный текст и просмотр Markdown
view-markdown-preview = Просмотр Markdown
preview-too-large = Документ слишком большой для предварительного просмотра.
preview-mermaid-unsupported = Диаграммы Mermaid пока не отображаются; показан исходный текст.
menu-tools = Инструменты
tools-json-validate = Проверить JSON
tools-json-format = Форматировать JSON
tools-json-minify = Сжать JSON
tools-sort-lines = Сортировать строки
tools-sort-lines-desc = Сортировать строки по убыванию
tools-unique-lines = Удалить повторяющиеся строки
tools-trim-trailing = Удалить пробелы в конце строк
tools-upper-case = Верхний регистр
tools-lower-case = Нижний регистр
tools-to-lf = Преобразовать концы строк в LF
tools-to-crlf = Преобразовать концы строк в CRLF
tools-json-valid = JSON корректен
tools-done = Готово. Команда «Отменить» вернёт изменения.
tools-no-change = Нечего изменять
tools-json-error = Некорректный JSON в строке { $line }, столбце { $column }: { $reason }
json-reason-unterminated-string = незакрытая строка
json-reason-invalid-u-escape = некорректная escape-последовательность \u
json-reason-invalid-escape = некорректная escape-последовательность
json-reason-control-character-in-string = управляющий символ в строке
json-reason-invalid-number = некорректное число
json-reason-unexpected-character = неожиданный символ
json-reason-unexpected-content-after-the-value = неожиданное содержимое после значения
json-reason-unexpected-comma = неожиданная запятая
json-reason-expected-a-string-key = ожидался строковый ключ
json-reason-expected = ожидалось ':'
json-reason-expected-or-a-closing-bracket = ожидалась ',' или закрывающая скобка
json-reason-expected-a-value = ожидалось значение
json-reason-empty-document = пустой документ
json-reason-unexpected-end-of-input = неожиданный конец данных
json-reason-too-deeply-nested-to-format = слишком глубокая вложенность для форматирования
tools-compare = Сравнить с…
compare-need-two = Откройте ещё одну вкладку для сравнения.
compare-pick-title = Сравнить { $name } с
compare-count = { $count ->
    [0] Различий нет
    [one] { $count } изменение
    [few] { $count } изменения
    [many] { $count } изменений
   *[other] { $count } изменения
    }
compare-close = Закрыть сравнение
compare-to-left = Скопировать это изменение влево
compare-to-right = Скопировать это изменение вправо
compare-stale = Документ изменился во время сравнения. Сравнение обновлено; повторите попытку.
view-structured = Структурированный вид (CSV, JSON, XML, YAML)
csv-delimiter = Разделитель
csv-rows = { $count ->
    [one] { $count } строка
    [few] { $count } строки
    [many] { $count } строк
   *[other] { $count } строки
    }
csv-unterminated = Незакрытая кавычка, начиная со строки { $line }
csv-truncated = Показаны первые 200 000 строк
json-more = … ещё { $count }
dialog-open-link-title = Открыть эту ссылку в веб-браузере?
button-open = Открыть
link-blocked = Ссылки такого типа не открываются из соображений безопасности.
link-not-found = Связанный файл не найден: { $target }
enc-windows-1252 = Западноевропейская
enc-iso-8859-15 = Западноевропейская
enc-windows-1250 = Центральноевропейская
enc-iso-8859-2 = Центральноевропейская
enc-windows-1251 = Кириллица
enc-koi8-r = Кириллица
enc-windows-1253 = Греческая
enc-windows-1254 = Турецкая
enc-windows-1255 = Иврит
enc-windows-1256 = Арабская
enc-windows-1257 = Балтийская
enc-windows-874 = Тайская
enc-shift_jis = Японская
enc-euc-jp = Японская
enc-gb18030 = Китайская упрощённая
enc-big5 = Китайская традиционная
enc-euc-kr = Корейская
file-reopen-encoding = Открыть заново в кодировке…
file-save-encoding = Сохранить в кодировке…
encoding-reopen-title = Открыть заново в кодировке
encoding-save-title = Сохранить в кодировке
encoding-not-exact = Эта кодировка не может открыть файл без его изменения: { $reason }
encoding-unencodable = «{ $character }» нельзя сохранить в кодировке { $encoding }. Выберите другую кодировку или измените текст; ничего не записано.
encoding-save-first = Сохраните или отмените изменения, прежде чем открывать файл в другой кодировке.
image-remote = Удалённые изображения не загружаются.
image-blocked = Показываются только изображения из папки этого документа.
image-unsupported = Неподдерживаемый формат изображения.
image-too-large = Изображение слишком большое для показа.
image-unreadable = Не удалось прочитать изображение.
settings-theme-high-contrast = Высокая контрастность
view-fold = Свернуть
view-unfold = Развернуть
view-unfold-all = Развернуть все
settings-shortcuts = Сочетания клавиш
settings-keymap-preset = Набор
settings-keymap-default = Sanain
settings-keymap-notepadpp = Notepad++
settings-shortcut-change = Изменить
settings-shortcut-reset = Сбросить
settings-shortcut-press = Нажмите сочетание клавиш… (Esc — отмена)
settings-shortcut-moved = Сочетание перенесено с:
edit-find-in-folder = Найти в папке…
folder-search-placeholder = Найти в папке
folder-search-run = Искать
folder-search-stop = Остановить
folder-search-no-folder = Выбрать папку
folder-search-summary = { $count ->
    [one] { $count } совпадение
    [few] { $count } совпадения
    [many] { $count } совпадений
   *[other] { $count } совпадения
    } в { $files ->
    [one] { $files } файле
    [few] { $files } файлах
    [many] { $files } файлах
   *[other] { $files } файла
    }
app-check-updates = Проверить обновления…
update-checking = Проверка обновлений…
update-up-to-date = Sanain { $version } — новейшая версия.
update-no-releases = Выпуски Sanain ещё не опубликованы.
update-failed = Не удалось проверить обновления: { $reason }
update-invalid = неожиданный ответ
update-available-title = Доступна версия Sanain { $version }
update-available-body = Откройте страницу загрузки, чтобы получить её. Sanain ничего не загружает и не устанавливает самостоятельно.
update-open-page = Открыть страницу загрузки
update-later = Позже
settings-check-updates = Проверять обновления раз в день
settings-check-updates-detail = Запрашивает у github.com последний выпуск. Больше ничего не отправляется.
file-local-history = Локальная история…
file-clear-history = Очистить локальную историю…
history-title = Локальная история файла { $name }
history-open = Открыть
history-restore = Восстановить
history-empty = Сохранённых версий пока нет. При каждом сохранении сохраняется копия.
history-unreadable = Не удалось прочитать эту версию.
history-just-now = Только что
history-minutes-ago = { $count ->
    [one] { $count } минуту назад
    [few] { $count } минуты назад
    [many] { $count } минут назад
   *[other] { $count } минуты назад
    }
history-hours-ago = { $count ->
    [one] { $count } час назад
    [few] { $count } часа назад
    [many] { $count } часов назад
   *[other] { $count } часа назад
    }
history-days-ago = { $count ->
    [one] { $count } день назад
    [few] { $count } дня назад
    [many] { $count } дней назад
   *[other] { $count } дня назад
    }
history-purge-title = Удалить всю локальную историю?
history-purge-body = Сохранённые копии всех файлов будут удалены с этого компьютера. Сами файлы не изменятся.
history-purge-confirm = Удалить историю
history-purged = Локальная история удалена.
settings-local-history = Хранить локальную историю сохранённых файлов
settings-local-history-detail = Хранит до 50 версий каждого файла в течение 30 дней, только на этом компьютере.
file-open-log = Открыть в просмотрщике журналов…
log-lines = { $count ->
    [one] { $count } строка
    [few] { $count } строки
    [many] { $count } строк
   *[other] { $count } строки
    }
log-indexing = Чтение… { $percent } %
log-matches = { $count ->
    [one] { $count } совпадающая строка
    [few] { $count } совпадающие строки
    [many] { $count } совпадающих строк
   *[other] { $count } совпадающей строки
    }
log-filter-hint = Фильтр строк
log-bad-pattern = Недопустимый шаблон
log-follow = Следить
log-close = Закрыть просмотр
log-opened-large = Файл { $name } слишком большой, поэтому он открыт только для чтения в просмотрщике журналов.
tab-pin = Закрепить вкладку
tab-unpin = Открепить вкладку
edit-add-cursor-above = Добавить курсор выше
edit-add-cursor-below = Добавить курсор ниже
view-word-wrap = Перенос по словам
status-wrap-on = Длинные строки теперь переносятся по ширине окна.
status-wrap-off = Длинные строки больше не переносятся.
settings-word-wrap = Переносить длинные строки
edit-go-to-line = Перейти к строке…
edit-go-to-heading = Перейти к заголовку…
goto-line-hint = Строка или строка:столбец
goto-line-note = { $count ->
    [one] В документе { $count } строка.
    [few] В документе { $count } строки.
    [many] В документе { $count } строк.
   *[other] В документе { $count } строки.
    }
goto-line-invalid = Введите номер строки, при необходимости с :столбец.
goto-heading-hint = Введите текст для фильтрации заголовков
goto-heading-none = В этом документе нет заголовков.
format-xml = XML
format-yaml = YAML
tools-tree-error = Некорректный { $format } в строке { $line }, столбце { $column }: { $reason }
edit-select-next-occurrence = Выделить следующее вхождение
tools-record-macro = Начать или остановить запись макроса
tools-play-macro = Воспроизвести макрос
macro-recording = Идёт запись макроса
macro-recorded = { $count ->
    [one] Макрос записан: { $count } шаг.
    [few] Макрос записан: { $count } шага.
    [many] Макрос записан: { $count } шагов.
   *[other] Макрос записан: { $count } шага.
    }
tools-compare-saved = Сравнить с сохранённой версией
tools-compare-clipboard = Сравнить с буфером обмена
compare-saved-label = { $name } (на диске)
compare-clipboard-label = Буфер обмена
compare-saved-failed = Не удалось прочитать сохранённый файл: { $reason }
compare-clipboard-empty = В буфере обмена нет текста для сравнения.
error-save-disk-full = Файл { $name } не сохранён: диск заполнен. Файл на диске не изменён, а ваш текст по-прежнему здесь. Освободите место или используйте команду «Сохранить как», чтобы сохранить файл в другом месте.
error-save-permission = Файл { $name } не сохранён: у вас нет прав на запись в это место. Файл на диске не изменён. Используйте команду «Сохранить как», чтобы сохранить файл в другом месте.
error-save-read-only-volume = Файл { $name } не сохранён: диск доступен только для чтения. Файл на диске не изменён. Используйте команду «Сохранить как», чтобы сохранить файл в другом месте.
replace-preview-title = { $count ->
    [one] Заменить { $count } совпадение?
    [few] Заменить { $count } совпадения?
    [many] Заменить { $count } совпадений?
   *[other] Заменить { $count } совпадения?
    }
replace-preview-confirm = { $count ->
    [one] Заменить { $count }
    [few] Заменить { $count }
    [many] Заменить { $count }
   *[other] Заменить { $count }
    }
replace-preview-more = Показаны только первые 200 изменяемых строк.
replace-preview-stale = Документ изменился; ничего не заменено. Выполните «Заменить все» ещё раз.
view-split-editor = Разделить редактор
encoding-detected = Открыто в кодировке { $encoding } (определено по содержимому файла).
encoding-suggested = Предложено по содержимому файла: { $encoding }. Перед открытием проверьте предпросмотр.
app-about = О программе Sanain
menu-help = Справка
about-tagline = Сила слов.
about-version = Версия { $version }
about-license = Свободное ПО с открытым исходным кодом под лицензией Mozilla Public License 2.0.
about-ai = Создан ИИ. Направляется людьми. Развивается сообществом.
about-notices = Сторонние компоненты и их лицензии перечислены в файле THIRD_PARTY_NOTICES, входящем в каждый выпуск.
about-source = Исходный код и задачи на GitHub
tools-xml-validate = Проверить XML
tools-yaml-validate = Проверить YAML
tools-markup-valid = { $format } корректен
file-open-recent = Открыть недавние…
file-save-all = Сохранить все
file-revert = Вернуть сохранённую версию
recent-hint = Введите текст для фильтрации недавних файлов
recent-none = Недавних файлов пока нет.
revert-title = Отменить изменения в файле { $name }?
revert-body = Файл будет заново загружен с диска. Несохранённые изменения будут потеряны без возможности отмены.
revert-confirm = Отменить и перезагрузить
revert-done = Файл { $name } заново загружен с диска.
status-saved-all = { $count ->
    [0] Нечего сохранять.
    [one] Сохранён { $count } файл.
    [few] Сохранено { $count } файла.
    [many] Сохранено { $count } файлов.
   *[other] Сохранено { $count } файла.
    }
edit-toggle-comment = Закомментировать/раскомментировать строки
edit-indent = Увеличить отступ строк
edit-outdent = Уменьшить отступ строк
comment-unsupported = Этот тип файлов не поддерживает строчные комментарии.
tab-close-others = Закрыть другие вкладки
tab-close-all = Закрыть все вкладки
file-copy-path = Копировать путь к файлу
file-reveal-macos = Показать в Finder
file-reveal-windows = Показать в проводнике
file-reveal-other = Показать в файловом менеджере
status-path-copied = Путь к файлу скопирован.
tabs-kept-unsaved = { $count ->
    [one] { $count } вкладка с несохранёнными изменениями осталась открытой.
    [few] { $count } вкладки с несохранёнными изменениями остались открытыми.
    [many] { $count } вкладок с несохранёнными изменениями остались открытыми.
   *[other] { $count } вкладки с несохранёнными изменениями остались открытыми.
    }
status-indent-tabs = Табуляция
status-indent-spaces = Пробелы: { $count }
find-searching = Поиск…
compare-ignore-whitespace = Игнорировать пробелы
tools-compare-file = Сравнить с файлом…
compare-file-failed = Не удалось сравнить с { $name }: { $reason }
cli-help =
    Использование:
      sanain [ФАЙЛ…]                открыть файлы
      sanain --diff ЛЕВЫЙ ПРАВЫЙ    сравнить два файла бок о бок
      sanain --version              показать версию
      sanain --help                 показать эту справку
    Используйте -- перед именами файлов, начинающимися с дефиса.
cli-unknown-option = Неизвестный параметр { $option }.
cli-diff-needs-two = Для --diff нужны два файла: sanain --diff ЛЕВЫЙ ПРАВЫЙ
window-move-tab-left = Переместить вкладку влево
window-move-tab-right = Переместить вкладку вправо
edit-skip-occurrence = Перейти к следующему вхождению
tools-play-macro-times = Воспроизвести макрос несколько раз…
macro-times-hint = Количество повторов
macro-times-note = От 1 до { $max }
macro-times-invalid = Введите число от 1 до { $max }.
view-outline = Структура
outline-empty = В этом документе нет заголовков.
outline-close = Закрыть структуру
folder-replace-placeholder = Заменить на
folder-replace-run = Заменить в файлах…
replace-files-title = Заменить { $count ->
    [one] { $count } совпадение
    [few] { $count } совпадения
    [many] { $count } совпадений
   *[other] { $count } совпадения
    } в { $files ->
    [one] { $files } файле
    [few] { $files } файлах
    [many] { $files } файлах
   *[other] { $files } файла
    }?
replace-files-body = Файлы, открытые во вкладке, изменяются там, и изменение можно отменить; сохраните их сами. Остальные файлы сохраняются сразу, а их прежнее содержимое остаётся в локальной истории, если она включена. Файл, который за это время изменился на диске, пропускается.
replace-files-confirm = Заменить
replace-files-done = { $count ->
    [one] Заменено { $count } совпадение
    [few] Заменено { $count } совпадения
    [many] Заменено { $count } совпадений
   *[other] Заменено { $count } совпадения
    } в { $files ->
    [one] { $files } файле
    [few] { $files } файлах
    [many] { $files } файлах
   *[other] { $files } файла
    } (открыто во вкладках: { $open }, не сохранено).
replace-files-failed = { $failed ->
    [one] { $failed } файл не изменён:
    [few] { $failed } файла не изменены; первый:
    [many] { $failed } файлов не изменены; первый:
   *[other] { $failed } файла не изменены; первый:
    } { $first }
image-outside = Это изображение находится вне папки документа.
image-allow = Показать изображения из этой папки
cli-help-integration =
    Интеграция с файловым менеджером (только для текущего пользователя):
      sanain --register           добавить Sanain в «Открыть с помощью» (Linux, Windows)
      sanain --unregister         снова удалить его
cli-registered = Sanain добавлен в «Открыть с помощью». { $place }
cli-unregistered = Sanain удалён из «Открыть с помощью».
cli-register-failed = Не удалось изменить интеграцию с файловым менеджером: { $reason }
cli-register-macos = Интеграция с Finder в macOS пока недоступна. Открывайте файлы из Sanain (Файл › Открыть, перетаскивание) или из командной строки.
