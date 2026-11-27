# Machine translation by Claude (AI); review by a native speaker required before release.
app-name = Text³
menu-file = Archivo
file-open = Abrir…
file-save = Guardar
status-tabs = { $count ->
    [one] Una pestaña
   *[other] { $count } pestañas
    }
error-open = No se ha podido abrir { $name }.
menu-edit = Edición
file-new = Nueva pestaña
file-save-as = Guardar como…
edit-undo = Deshacer
edit-redo = Rehacer
edit-select-all = Seleccionar todo
untitled = Sin título
window-title = { $dirty ->
    [yes] ● { $name } — Text³
   *[no] { $name } — Text³
    }
status-position = Lín. { $line }, col. { $column }
status-selections = { $count ->
    [one] 1 selección
   *[other] { $count } selecciones
    }
status-saved = { $name } guardado
encoding-utf8 = UTF-8
encoding-utf8-bom = UTF-8 con BOM
encoding-utf16le = UTF-16 LE
encoding-utf16be = UTF-16 BE
line-ending-lf = LF
line-ending-crlf = CRLF
line-ending-cr = CR
line-ending-mixed = Finales de línea mixtos
editor-accessible-name = Editor de texto
button-save = Guardar
button-dont-save = No guardar
button-cancel = Cancelar
button-ok = Aceptar
button-overwrite = Sobrescribir
button-save-as = Guardar como…
dialog-unsaved-title = ¿Guardar los cambios en { $name }?
dialog-unsaved-body = Si no guarda los cambios, se perderán.
dialog-conflict-title = { $name } ha cambiado en el disco
dialog-conflict-body = Otro programa ha modificado este archivo después de abrirlo. Si lo sobrescribe, se reemplazarán esos cambios.
dialog-readonly-title = { $name } es de solo lectura
dialog-readonly-body = Guarde una copia con otro nombre.
error-open-encoding = { $name } no es texto UTF-8 ni UTF-16. No se ha abierto, por lo que su contenido no ha cambiado. Use Archivo > Volver a abrir con codificación para elegir su codificación.
error-save = No se ha podido guardar { $name }: { $reason }
menu-window = Ventana
edit-cut = Cortar
edit-copy = Copiar
edit-paste = Pegar
app-quit = Salir de Text³
app-hide = Ocultar Text³
app-hide-others = Ocultar otros
app-show-all = Mostrar todo
window-minimize = Minimizar
window-zoom = Zoom
status-recovered = Se han restaurado los cambios no guardados de la última sesión
status-recovery-failed = No se ha podido guardar una copia de recuperación: { $reason }
status-hot-exit-failed = No se han podido conservar los cambios sin guardar para el siguiente inicio ({ $reason }); se preguntará por ellos.
status-hot-exit-too-large = { $name } es demasiado grande para conservarlo hasta el siguiente inicio; se preguntará por él.
status-restored-changed = { $name } ha cambiado en el disco después de conservar sus cambios sin guardar. Su texto está en la pestaña; el archivo no se ha tocado.
status-restored-missing = { $name } ya no está en el disco. Su texto sin guardar se conserva en la pestaña; al guardar se vuelve a crear el archivo.
edit-duplicate-line = Duplicar línea
edit-delete-line = Eliminar línea
edit-move-line-up = Subir línea
edit-move-line-down = Bajar línea
file-close-tab = Cerrar pestaña
file-reopen-tab = Volver a abrir pestaña cerrada
window-next-tab = Pestaña siguiente
window-prev-tab = Pestaña anterior
tab-close = Cerrar pestaña
tab-missing-on-disk = No se encuentra en el disco
edit-find = Buscar…
edit-find-next = Buscar siguiente
edit-find-prev = Buscar anterior
edit-replace = Reemplazar…
find-placeholder = Buscar
replace-placeholder = Reemplazar por
find-match-case = Coincidir mayúsculas y minúsculas
find-whole-word = Palabra completa
find-regex = Expresión regular
find-no-results = Sin resultados
find-count = { $total ->
    [one] 1 coincidencia
   *[other] { $total } coincidencias
    }
find-count-current = { $current } de { $total }
find-prev = Coincidencia anterior (Shift+Enter)
find-next = Coincidencia siguiente (Enter)
find-close = Cerrar (Esc)
replace-one = Reemplazar
replace-all = Reemplazar todo
status-replaced = { $count ->
    [one] Se ha reemplazado 1 coincidencia
   *[other] Se han reemplazado { $count } coincidencias
    }
language-plain = Texto sin formato
menu-view = Visualización
app-settings = Ajustes…
view-zoom-in = Ampliar
view-zoom-out = Reducir
view-zoom-reset = Tamaño real
settings-title = Ajustes
settings-font-size = Tamaño de letra
settings-tab-width = Ancho de tabulación
settings-insert-spaces = Insertar espacios al pulsar Tab
settings-theme = Apariencia
settings-theme-system = Según el sistema
settings-theme-light = Claro
settings-theme-dark = Oscuro
settings-restore-session = Volver a abrir los archivos de la última sesión
settings-hot-exit = Recordar los cambios sin guardar al salir
settings-hot-exit-detail = Salir sin que se pregunte por cada pestaña sin guardar. Las pestañas sin guardar y sin título vuelven con su texto en el siguiente inicio, aunque esté desactivado volver a abrir los archivos de la última sesión. Al cerrar una sola pestaña se sigue preguntando.
settings-language = Idioma
settings-language-system = Según el sistema
settings-language-note = Los cambios de idioma se aplican al reiniciar Text³.
settings-save-failed = No se han podido guardar los ajustes: { $reason }
view-command-palette = Paleta de comandos…
palette-placeholder = Escriba un comando
view-markdown-source = Código Markdown
view-markdown-split = Código y vista previa de Markdown
view-markdown-preview = Vista previa de Markdown
preview-too-large = Este documento es demasiado grande para la vista previa.
preview-mermaid-unsupported-type = Tipo de diagrama no compatible; se muestra el código.
preview-mermaid-unsupported-syntax = El diagrama usa una sintaxis que no se puede dibujar; se muestra el código.
preview-mermaid-too-large = El diagrama es demasiado grande para dibujarlo; se muestra el código.
menu-tools = Herramientas
tools-json-validate = Validar JSON
tools-json-format = Formatear JSON
tools-json-minify = Minimizar JSON
tools-sort-lines = Ordenar líneas
tools-sort-lines-desc = Ordenar líneas de forma descendente
tools-unique-lines = Eliminar líneas duplicadas
tools-trim-trailing = Eliminar espacios al final de línea
tools-upper-case = Mayúsculas
tools-lower-case = Minúsculas
tools-to-lf = Convertir finales de línea a LF
tools-to-crlf = Convertir finales de línea a CRLF
tools-json-valid = El JSON es válido
tools-done = Hecho. Deshacer lo revierte.
tools-no-change = No hay nada que cambiar
tools-json-error = JSON no válido en la línea { $line }, columna { $column }: { $reason }
json-reason-unterminated-string = cadena sin terminar
json-reason-invalid-u-escape = secuencia de escape \u no válida
json-reason-invalid-escape = secuencia de escape no válida
json-reason-control-character-in-string = carácter de control en la cadena
json-reason-invalid-number = número no válido
json-reason-unexpected-character = carácter inesperado
json-reason-unexpected-content-after-the-value = contenido inesperado después del valor
json-reason-unexpected-comma = coma inesperada
json-reason-expected-a-string-key = se esperaba una clave de tipo cadena
json-reason-expected = se esperaba ':'
json-reason-expected-or-a-closing-bracket = se esperaba ',' o un corchete de cierre
json-reason-expected-a-value = se esperaba un valor
json-reason-empty-document = documento vacío
json-reason-unexpected-end-of-input = fin de entrada inesperado
json-reason-too-deeply-nested-to-format = anidamiento demasiado profundo para formatear
tools-compare = Comparar con…
compare-need-two = Abra otra pestaña para comparar.
compare-pick-title = Comparar { $name } con
compare-count = { $count ->
    [0] Sin diferencias
    [one] 1 cambio
   *[other] { $count } cambios
    }
compare-close = Cerrar comparación
compare-to-left = Copiar este cambio a la izquierda
compare-to-right = Copiar este cambio a la derecha
compare-stale = Un documento ha cambiado durante la comparación. Se ha actualizado la comparación; inténtelo de nuevo.
view-structured = Vista estructurada (CSV, JSON, XML, YAML)
csv-delimiter = Delimitador
csv-rows = { $count ->
    [one] 1 fila
   *[other] { $count } filas
    }
csv-unterminated = Comillas sin cerrar a partir de la línea { $line }
csv-truncated = Se muestran las primeras 200.000 filas
csv-edit-cell = Editar celda
csv-insert-row-below = Insertar fila debajo
csv-delete-row = Eliminar fila
csv-read-only = Archivo de solo lectura: la cuadrícula no se puede editar
json-more = … { $count } más
json-edit-value = Editar valor
json-rename-key = Cambiar nombre de clave
json-add-after = Añadir después
json-add-inside = Añadir dentro
json-delete-node = Eliminar
json-read-only = Archivo de solo lectura: el árbol no se puede editar
json-invalid-value = No es un valor JSON válido
json-duplicate-key = El objeto ya tiene una clave con este nombre
xml-edit-value = Editar valor
xml-edit-text = Editar texto
xml-rename = Cambiar nombre
xml-add-attribute = Añadir atributo
xml-add-child = Añadir elemento hijo
xml-add-after = Añadir después
xml-delete = Eliminar
xml-invalid-name = No es un nombre XML válido
xml-duplicate-attribute = El elemento ya tiene un atributo con este nombre
xml-not-well-formed = Este cambio haría que el XML no fuera válido
yaml-not-preserved = Este cambio alteraría otras partes del YAML
yaml-key-too-long = Una clave puede tener como máximo 1024 caracteres
yaml-read-only-flow = El estilo de flujo (escrito entre corchetes) aún no se puede editar en el árbol
yaml-read-only-anchor = Los valores con un ancla (&) no se pueden editar en el árbol
yaml-read-only-tag = Los valores con una etiqueta (!) no se pueden editar en el árbol
yaml-read-only-alias = Los alias (*) no se pueden editar en el árbol
yaml-read-only-block-scalar = Los bloques de texto (| o >) aún no se pueden editar en el árbol
yaml-read-only-complex-key = Las claves complejas no se pueden editar en el árbol
yaml-read-only-document = Solo se puede editar el primer documento en el árbol
yaml-read-only-layout = Esta entrada comparte sus líneas con otro contenido y no se puede cambiar aquí
yaml-read-only-only-child = La única entrada no se puede eliminar
dialog-open-link-title = ¿Abrir este enlace en el navegador web?
button-open = Abrir
link-blocked = Por seguridad, este tipo de enlace no se abre.
link-not-found = No se ha encontrado el archivo enlazado: { $target }
enc-windows-1252 = Europeo occidental
enc-iso-8859-15 = Europeo occidental
enc-windows-1250 = Europeo central
enc-iso-8859-2 = Europeo central
enc-windows-1251 = Cirílico
enc-koi8-r = Cirílico
enc-windows-1253 = Griego
enc-windows-1254 = Turco
enc-windows-1255 = Hebreo
enc-windows-1256 = Árabe
enc-windows-1257 = Báltico
enc-windows-874 = Tailandés
enc-shift_jis = Japonés
enc-euc-jp = Japonés
enc-gb18030 = Chino simplificado
enc-big5 = Chino tradicional
enc-euc-kr = Coreano
file-reopen-encoding = Volver a abrir con codificación…
file-save-encoding = Guardar con codificación…
encoding-reopen-title = Volver a abrir con codificación
encoding-save-title = Guardar con codificación
encoding-not-exact = Esta codificación no puede abrir el archivo sin modificarlo: { $reason }
encoding-unencodable = «{ $character }» no se puede guardar como { $encoding }. Elija otra codificación o modifique el texto; no se ha escrito nada.
encoding-save-first = Guarde o deshaga los cambios antes de volver a abrir con otra codificación.
image-remote = Las imágenes remotas no se cargan.
image-blocked = Solo se muestran las imágenes de la carpeta de este documento.
image-unsupported = Formato de imagen no compatible.
image-too-large = La imagen es demasiado grande para mostrarla.
image-unreadable = No se ha podido leer la imagen.
settings-theme-high-contrast = Alto contraste
view-fold = Contraer
view-unfold = Expandir
view-unfold-all = Expandir todo
settings-shortcuts = Atajos de teclado
settings-keymap-preset = Predefinido
settings-keymap-default = Text³
settings-keymap-notepadpp = Notepad++
settings-shortcut-change = Cambiar
settings-shortcut-reset = Restablecer
settings-shortcut-press = Pulse un atajo… (Esc para cancelar)
settings-shortcut-moved = Atajo movido desde:
edit-find-in-folder = Buscar en carpeta…
folder-search-placeholder = Buscar en carpeta
folder-search-run = Buscar
folder-search-stop = Detener
folder-search-no-folder = Elegir carpeta
folder-search-summary = { $count ->
    [one] 1 coincidencia
   *[other] { $count } coincidencias
    } en { $files ->
    [one] 1 archivo
   *[other] { $files } archivos
    }
app-check-updates = Buscar actualizaciones…
update-checking = Buscando actualizaciones…
update-up-to-date = Text³ { $version } es la versión más reciente.
update-no-releases = Todavía no se ha publicado ninguna versión de Text³.
update-failed = No se han podido buscar actualizaciones: { $reason }
update-invalid = respuesta inesperada
update-available-title = Text³ { $version } está disponible
update-available-body = Abre la página de descarga para obtenerla. Text³ no descarga ni instala nada por sí mismo.
update-open-page = Abrir página de descarga
update-later = Más tarde
settings-check-updates = Buscar actualizaciones una vez al día
settings-check-updates-detail = Consulta a github.com la última versión publicada. No se envía nada más.
file-local-history = Historial local…
file-clear-history = Borrar historial local…
history-title = Historial local de { $name }
history-open = Abrir
history-restore = Restaurar
history-empty = Aún no hay versiones guardadas. Se conserva una copia cada vez que guardas.
history-unreadable = No se ha podido leer esta versión.
history-just-now = Ahora mismo
history-minutes-ago = { $count ->
    [one] hace 1 minuto
   *[other] hace { $count } minutos
    }
history-hours-ago = { $count ->
    [one] hace 1 hora
   *[other] hace { $count } horas
    }
history-days-ago = { $count ->
    [one] hace 1 día
   *[other] hace { $count } días
    }
history-purge-title = ¿Eliminar todo el historial local?
history-purge-body = Las copias guardadas de todos los archivos se eliminan de este ordenador. Tus archivos no se modifican.
history-purge-confirm = Eliminar historial
history-purged = Historial local eliminado.
settings-local-history = Conservar el historial local de los archivos guardados
settings-local-history-detail = Conserva hasta 50 versiones por archivo durante 30 días, solo en este ordenador.
file-open-log = Abrir en el visor de registros…
log-lines = { $count ->
    [one] 1 línea
   *[other] { $count } líneas
    }
log-indexing = Leyendo… { $percent } %
log-matches = { $count ->
    [one] 1 línea coincidente
   *[other] { $count } líneas coincidentes
    }
log-filter-hint = Filtrar líneas
log-bad-pattern = Patrón no válido
log-follow = Seguir
log-close = Cerrar visor
log-opened-large = { $name } es grande, así que se abrió en solo lectura en el visor de registros.
tab-pin = Fijar pestaña
tab-unpin = Desfijar pestaña
edit-add-cursor-above = Agregar cursor arriba
edit-add-cursor-below = Agregar cursor abajo
view-word-wrap = Ajuste de línea
status-wrap-on = Las líneas largas se ajustan ahora al ancho de la ventana.
status-wrap-off = Las líneas largas ya no se ajustan.
settings-word-wrap = Ajustar las líneas largas
edit-go-to-line = Ir a la línea…
edit-go-to-heading = Ir al encabezado…
goto-line-hint = Línea o línea:columna
goto-line-note = { $count ->
    [one] El documento tiene 1 línea.
   *[other] El documento tiene { $count } líneas.
    }
goto-line-invalid = Introduce un número de línea, seguido opcionalmente de :columna.
goto-heading-hint = Escribe para filtrar los encabezados
goto-heading-none = Este documento no tiene encabezados.
edit-goto-anything = Ir a cualquier cosa…
goto-anything-hint = Nombre de archivo, o :línea, @símbolo, #texto
goto-anything-line = Ir a la línea { $line }
goto-anything-open = abierto
goto-anything-scanning = Listando archivos de { $folder }…
goto-anything-scanned = Archivos en { $folder }: { $count }
goto-anything-limit = Solo se muestran los primeros { $max } archivos de { $folder }.
goto-anything-no-folder = Guarda el archivo o usa Buscar en carpeta para ver los archivos de una carpeta.
goto-anything-symbols-none = No hay símbolos en este documento.
goto-anything-matches-none = No hay coincidencias en este documento.
format-xml = XML
format-yaml = YAML
tools-tree-error = { $format } no válido en la línea { $line }, columna { $column }: { $reason }
edit-select-next-occurrence = Seleccionar siguiente coincidencia
tools-record-macro = Iniciar o detener la grabación de macro
tools-play-macro = Reproducir macro
macro-recording = Grabando macro
macro-recorded = { $count ->
    [one] Macro grabada: 1 paso.
   *[other] Macro grabada: { $count } pasos.
    }
tools-compare-saved = Comparar con la versión guardada
tools-compare-clipboard = Comparar con el portapapeles
compare-saved-label = { $name } (en disco)
compare-clipboard-label = Portapapeles
compare-saved-failed = No se ha podido leer el archivo guardado: { $reason }
compare-clipboard-empty = El portapapeles no contiene texto para comparar.
error-save-disk-full = No se ha guardado { $name }: el disco está lleno. El archivo del disco no ha cambiado y su texto sigue aquí. Libere espacio o use Guardar como para guardarlo en otro lugar.
error-save-permission = No se ha guardado { $name }: no tiene permiso para escribir en esa ubicación. El archivo del disco no ha cambiado. Use Guardar como para guardarlo en otro lugar.
error-save-read-only-volume = No se ha guardado { $name }: el disco es de solo lectura. El archivo del disco no ha cambiado. Use Guardar como para guardarlo en otro lugar.
replace-preview-title = { $count ->
    [one] ¿Reemplazar 1 coincidencia?
   *[other] ¿Reemplazar { $count } coincidencias?
    }
replace-preview-confirm = { $count ->
    [one] Reemplazar 1
   *[other] Reemplazar { $count }
    }
replace-preview-more = Solo se muestran las primeras 200 líneas modificadas.
replace-preview-stale = El documento ha cambiado; no se ha reemplazado nada. Vuelva a ejecutar Reemplazar todo.
encoding-detected = Abierto como { $encoding } (detectado a partir del contenido del archivo).
encoding-suggested = Sugerida a partir del contenido del archivo: { $encoding }. Revise la vista previa antes de abrir.
app-about = Acerca de Text³
menu-help = Ayuda
about-version = Versión { $version }
about-license = Libre y de código abierto bajo la Mozilla Public License 2.0.
about-ai = Desarrollado con ayuda de IA (Claude, de Anthropic) bajo la dirección de su responsable.
about-notices = Los componentes de terceros y sus licencias se enumeran en THIRD_PARTY_NOTICES, incluido en cada versión.
about-source = Código fuente e incidencias en GitHub
tools-xml-validate = Validar XML
tools-yaml-validate = Validar YAML
tools-markup-valid = El { $format } es válido
file-open-recent = Abrir reciente…
file-save-all = Guardar todo
file-revert = Volver a la versión guardada
recent-hint = Escribe para filtrar los archivos recientes
recent-none = Aún no hay archivos recientes.
revert-title = ¿Descartar los cambios en { $name }?
revert-body = El archivo se vuelve a cargar desde el disco. Los cambios no guardados se pierden y no se pueden deshacer.
revert-confirm = Descartar y volver a cargar
revert-done = { $name } se ha vuelto a cargar desde el disco.
status-saved-all = { $count ->
    [0] No hay nada que guardar.
    [one] Se ha guardado 1 archivo.
   *[other] Se han guardado { $count } archivos.
    }
edit-toggle-comment = Alternar comentario de línea
edit-indent = Aumentar sangría de líneas
edit-outdent = Reducir sangría de líneas
comment-unsupported = Este tipo de archivo no tiene comentarios de línea.
tab-close-others = Cerrar las demás pestañas
tab-close-all = Cerrar todas las pestañas
file-copy-path = Copiar ruta del archivo
file-reveal-macos = Mostrar en Finder
file-reveal-windows = Mostrar en el Explorador de archivos
file-reveal-other = Mostrar en el gestor de archivos
status-path-copied = Ruta del archivo copiada.
tabs-kept-unsaved = { $count ->
    [one] Se mantuvo abierta 1 pestaña con cambios sin guardar.
   *[other] Se mantuvieron abiertas { $count } pestañas con cambios sin guardar.
    }
status-indent-tabs = Tabulaciones
status-indent-spaces = Espacios: { $count }
find-searching = Buscando…
compare-ignore-whitespace = Ignorar espacios en blanco
tools-compare-file = Comparar con un archivo…
compare-file-failed = No se ha podido comparar con { $name }: { $reason }
cli-help =
    Uso:
      text3 [ARCHIVO…]                   abrir archivos
      text3 --diff IZQUIERDA DERECHA     comparar dos archivos en paralelo
      text3 --version                    mostrar la versión
      text3 --help                       mostrar esta ayuda
    Use -- antes de los nombres de archivo que empiecen por un guion.
cli-unknown-option = Opción desconocida { $option }.
cli-diff-needs-two = --diff necesita dos archivos: text3 --diff IZQUIERDA DERECHA
window-move-tab-left = Mover pestaña a la izquierda
window-move-tab-right = Mover pestaña a la derecha
edit-skip-occurrence = Saltar a la siguiente coincidencia
tools-play-macro-times = Reproducir macro varias veces…
macro-times-hint = Número de veces
macro-times-note = De 1 a { $max }
macro-times-invalid = Introduce un número de 1 a { $max }.
view-outline = Esquema
outline-empty = No hay encabezados en este documento.
outline-close = Cerrar esquema
folder-replace-placeholder = Reemplazar por
folder-replace-run = Reemplazar en archivos…
replace-files-title = ¿Reemplazar { $count ->
    [one] 1 coincidencia
   *[other] { $count } coincidencias
    } en { $files ->
    [one] 1 archivo
   *[other] { $files } archivos
    }?
replace-files-body = Los archivos abiertos en una pestaña se cambian allí y el cambio se puede deshacer; guárdelos usted. Los demás archivos se guardan ahora y su contenido anterior se conserva en el historial local si está activado. Si un archivo ha cambiado en el disco mientras tanto, se omite.
replace-files-confirm = Reemplazar
replace-files-done = { $count ->
    [one] Se ha reemplazado 1 coincidencia
   *[other] Se han reemplazado { $count } coincidencias
    } en { $files ->
    [one] 1 archivo
   *[other] { $files } archivos
    } (abiertos en pestañas: { $open }, sin guardar).
replace-files-failed = { $failed ->
    [one] 1 archivo no se ha cambiado:
   *[other] { $failed } archivos no se han cambiado; el primero:
    } { $first }
image-outside = Esta imagen está fuera de la carpeta del documento.
image-allow = Mostrar imágenes de esta carpeta
cli-help-integration =
    Integración con el gestor de archivos (solo para el usuario actual):
      text3 --register            añadir Text³ a «Abrir con» (Linux, Windows)
      text3 --unregister          quitarlo de nuevo
cli-registered = Text³ se ha añadido a «Abrir con». { $place }
cli-unregistered = Text³ se ha quitado de «Abrir con».
cli-register-failed = No se ha podido cambiar la integración con el gestor de archivos: { $reason }
cli-register-macos = En macOS no hay que registrar nada: en cuanto Text³.app esté en la carpeta Aplicaciones, Finder lo ofrecerá en «Abrir con».
settings-tab-general = General
settings-tab-editor = Editor
settings-tab-appearance = Apariencia
settings-shortcut-search = Buscar comandos o teclas
settings-font = Tipo de letra
settings-font-default = Monoespaciada integrada
settings-font-loading = Buscando tipos de letra instalados…
settings-font-missing = No se encontró el tipo de letra { $font }; se usa el tipo de letra integrado.
settings-theme-more = Más temas
settings-theme-user-note = Temas propios: archivos .theme en la carpeta { $folder } (ver docs/THEMES.md).
language-auto = Detectar automáticamente
language-choose = Elegir el lenguaje de esta pestaña
tools-xml-format = Formatear XML
tools-save-macro = Guardar macro…
tools-play-saved-macro = Reproducir macro guardada…
tools-delete-saved-macro = Eliminar macro guardada…
macro-save-hint = Nombre de la macro
macro-save-note = Se sustituye la macro guardada con el mismo nombre.
macro-save-empty = Nada que guardar: graba primero una macro.
macro-saved = Macro guardada: { $name }
macro-deleted = Macro eliminada: { $name }
macro-name-invalid = Escribe un nombre de 1 a { $max } caracteres en una sola línea.
macro-save-too-large = Esta macro es demasiado larga para guardarla.
macro-save-full = Se pueden guardar como máximo { $max } macros. Elimina una primero.
macro-save-failed = No se pudieron guardar las macros: { $error }
macro-saved-hint = Escribe para filtrar las macros guardadas
macro-saved-none = Aún no hay macros guardadas.
settings-shortcut-import-npp = Importar de Notepad++…
settings-shortcut-import-npp-hint = Elige un shortcuts.xml de Notepad++. En Windows está en %AppData%\Notepad++\shortcuts.xml.
settings-shortcut-import-npp-done = Importados de Notepad++: { $imported }. Omitidos: { $skipped }. Conflictos pendientes: { $conflicts }.
settings-shortcut-import-npp-failed = No se ha podido importar { $name }: { $reason }
settings-shortcut-import-npp-too-large = El archivo ocupa más de 1 MiB.
settings-shortcut-import-npp-invalid = El archivo no es un shortcuts.xml válido de Notepad++.
view-split-right = Dividir a la derecha
view-split-down = Dividir hacia abajo
view-close-pane = Cerrar panel
view-focus-next-pane = Ir al panel siguiente
view-focus-prev-pane = Ir al panel anterior
view-focus-next-region = Ir a la región siguiente
view-focus-prev-region = Ir a la región anterior
status-pane-limit = Se ha alcanzado el límite de paneles (máximo { $max }).
a11y-csv-grid = Tabla CSV
a11y-grid-cell = Fila { $row }, columna { $column }: { $name }
a11y-grid-cell-plain = Fila { $row }, columna { $column }
a11y-grid-column-header = Encabezado de la columna { $column }
a11y-grid-row-header = Fila { $row }
a11y-edit-field = Editando: { $item }
a11y-json-tree = Árbol JSON
a11y-xml-tree = Árbol XML
a11y-yaml-tree = Árbol YAML
a11y-tree-item = Nivel { $level }
a11y-tree-item-expanded = Nivel { $level }, expandido
a11y-tree-item-collapsed = Nivel { $level }, contraído
a11y-tree-object = Objeto, entradas: { $count }
a11y-tree-list = Lista, elementos: { $count }
a11y-tree-element = Elemento { $name }
a11y-tree-attribute = Atributo { $name }
a11y-tree-text = Texto
a11y-pane = Panel { $pane } de { $count }: { $name }
a11y-pane-active = Panel { $pane } de { $count }: { $name }, activo
a11y-pane-editor = Editor de texto, panel { $pane } de { $count }: { $name }
a11y-mermaid-flowchart = Diagrama de flujo. Nodos: { $nodes }, enlaces: { $links }.
a11y-mermaid-sequence = Diagrama de secuencia. Participantes: { $participants }, mensajes: { $messages }.
a11y-mermaid-state = Diagrama de estados. Estados: { $states }, transiciones: { $transitions }.
a11y-mermaid-class = Diagrama de clases. Clases: { $classes }, relaciones: { $relations }.
a11y-shortcut-change = Cambiar el atajo de { $command }
a11y-shortcut-change-current = Cambiar el atajo de { $command }, ahora { $keys }
a11y-shortcut-reset = Restablecer el atajo de { $command }
status-line-ending-choose = Cambiar los finales de línea de este archivo
status-encoding-choose = Reabrir o guardar con otra codificación
status-indent-choose = Ajustes de sangría
a11y-settings-sections = Secciones de ajustes
a11y-tab-strip = Archivos abiertos
a11y-tab-unsaved = { $name }, cambios sin guardar
migration-copied = Los ajustes y datos de Sanain se copiaron a Text³. Las carpetas de Sanain no se han modificado.

# Character and word count (status bar)
# How one thousand is written with digit grouping: the characters between
# 1 and 000 separate groups of three digits.
number-thousand = 1{ "." }000
count-status = Caracteres: { $chars } · Palabras: { $words }
count-status-selection = Caracteres: { $chars } de { $total-chars } · Palabras: { $words } de { $total-words }
count-choose = Recuento de caracteres, palabras, líneas y bytes
count-a11y-pending = Caracteres y palabras: aún sin contar. Ábralo para contarlos.
count-heading-selection = Selección
count-heading-document = Documento
count-characters = Caracteres
count-characters-no-spaces = Caracteres (sin espacios)
count-words = Palabras
count-lines = Líneas
count-bytes = Bytes ({ $encoding })
count-unencodable = Algunos caracteres no se pueden guardar como { $encoding }.
count-counting = Contando…
settings-show-counts = Mostrar el recuento de caracteres y palabras en la barra de estado
