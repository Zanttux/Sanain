# Machine translation by Claude (AI); review by a native speaker required before release.
app-name = Sanain
menu-file = Fichier
file-open = Ouvrir…
file-save = Enregistrer
status-tabs = { $count ->
    [one] Un onglet
    [many] { $count } onglets
   *[other] { $count } onglets
    }
error-open = Impossible d’ouvrir { $name }.
menu-edit = Édition
file-new = Nouvel onglet
file-save-as = Enregistrer sous…
edit-undo = Annuler
edit-redo = Rétablir
edit-select-all = Tout sélectionner
untitled = Sans titre
window-title = { $dirty ->
    [yes] ● { $name } — Sanain
   *[no] { $name } — Sanain
    }
status-position = Ligne { $line }, col. { $column }
status-selections = { $count ->
    [one] { $count } sélection
    [many] { $count } sélections
   *[other] { $count } sélections
    }
status-saved = { $name } enregistré
encoding-utf8 = UTF-8
encoding-utf8-bom = UTF-8 avec BOM
encoding-utf16le = UTF-16 LE
encoding-utf16be = UTF-16 BE
line-ending-lf = LF
line-ending-crlf = CRLF
line-ending-cr = CR
line-ending-mixed = Fins de ligne mixtes
editor-accessible-name = Éditeur de texte
button-save = Enregistrer
button-dont-save = Ne pas enregistrer
button-cancel = Annuler
button-ok = OK
button-overwrite = Remplacer
button-save-as = Enregistrer sous…
dialog-unsaved-title = Enregistrer les modifications apportées à { $name } ?
dialog-unsaved-body = Vos modifications seront perdues si vous ne les enregistrez pas.
dialog-conflict-title = { $name } a été modifié sur le disque
dialog-conflict-body = Un autre programme a modifié ce fichier après son ouverture. Le remplacer supprimera ces modifications.
dialog-readonly-title = { $name } est en lecture seule
dialog-readonly-body = Enregistrez plutôt une copie sous un autre nom.
error-open-encoding = { $name } n’est pas un texte UTF-8 ou UTF-16. Il n’a pas été ouvert ; son contenu est donc inchangé. Utilisez Fichier > Rouvrir avec l’encodage pour choisir son encodage.
error-save = Impossible d’enregistrer { $name } : { $reason }
menu-window = Fenêtre
edit-cut = Couper
edit-copy = Copier
edit-paste = Coller
app-quit = Quitter Sanain
app-hide = Masquer Sanain
app-hide-others = Masquer les autres
app-show-all = Tout afficher
window-minimize = Placer dans le Dock
window-zoom = Réduire/agrandir
status-recovered = Modifications non enregistrées de la dernière session restaurées
status-recovery-failed = Impossible d’enregistrer une copie de récupération : { $reason }
edit-duplicate-line = Dupliquer la ligne
edit-delete-line = Supprimer la ligne
edit-move-line-up = Déplacer la ligne vers le haut
edit-move-line-down = Déplacer la ligne vers le bas
file-close-tab = Fermer l’onglet
file-reopen-tab = Rouvrir l’onglet fermé
window-next-tab = Onglet suivant
window-prev-tab = Onglet précédent
tab-close = Fermer l’onglet
edit-find = Rechercher…
edit-find-next = Rechercher le suivant
edit-find-prev = Rechercher le précédent
edit-replace = Remplacer…
find-placeholder = Rechercher
replace-placeholder = Remplacer par
find-match-case = Respecter la casse
find-whole-word = Mot entier
find-regex = Expression régulière
find-no-results = Aucun résultat
find-count = { $total ->
    [one] { $total } occurrence
    [many] { $total } occurrences
   *[other] { $total } occurrences
    }
find-count-current = { $current } sur { $total }
find-prev = Occurrence précédente (Shift+Enter)
find-next = Occurrence suivante (Enter)
find-close = Fermer (Esc)
replace-one = Remplacer
replace-all = Tout remplacer
status-replaced = { $count ->
    [one] { $count } occurrence remplacée
    [many] { $count } occurrences remplacées
   *[other] { $count } occurrences remplacées
    }
language-plain = Texte brut
menu-view = Présentation
app-settings = Réglages…
view-zoom-in = Zoom avant
view-zoom-out = Zoom arrière
view-zoom-reset = Taille réelle
settings-title = Réglages
settings-font-size = Taille de police
settings-tab-width = Largeur de tabulation
settings-insert-spaces = Insérer des espaces avec la touche Tab
settings-theme = Apparence
settings-theme-system = Selon le système
settings-theme-light = Clair
settings-theme-dark = Sombre
settings-restore-session = Rouvrir les fichiers de la dernière session
settings-language = Langue
settings-language-system = Selon le système
settings-language-note = Le changement de langue prend effet au redémarrage de Sanain.
settings-save-failed = Impossible d’enregistrer les réglages : { $reason }
view-command-palette = Palette de commandes…
palette-placeholder = Saisissez une commande
view-markdown-source = Source Markdown
view-markdown-split = Source et aperçu Markdown
view-markdown-preview = Aperçu Markdown
preview-too-large = Ce document est trop volumineux pour l’aperçu.
preview-mermaid-unsupported = Les diagrammes Mermaid ne sont pas encore rendus ; la source est affichée.
menu-tools = Outils
tools-json-validate = Valider le JSON
tools-json-format = Formater le JSON
tools-json-minify = Minifier le JSON
tools-sort-lines = Trier les lignes
tools-sort-lines-desc = Trier les lignes par ordre décroissant
tools-unique-lines = Supprimer les lignes en double
tools-trim-trailing = Supprimer les espaces en fin de ligne
tools-upper-case = Majuscules
tools-lower-case = Minuscules
tools-to-lf = Convertir les fins de ligne en LF
tools-to-crlf = Convertir les fins de ligne en CRLF
tools-json-valid = Le JSON est valide
tools-done = Terminé. Annuler rétablit l’état précédent.
tools-no-change = Rien à modifier
tools-json-error = JSON non valide à la ligne { $line }, colonne { $column } : { $reason }
json-reason-unterminated-string = chaîne non terminée
json-reason-invalid-u-escape = séquence d’échappement \u non valide
json-reason-invalid-escape = séquence d’échappement non valide
json-reason-control-character-in-string = caractère de contrôle dans une chaîne
json-reason-invalid-number = nombre non valide
json-reason-unexpected-character = caractère inattendu
json-reason-unexpected-content-after-the-value = contenu inattendu après la valeur
json-reason-unexpected-comma = virgule inattendue
json-reason-expected-a-string-key = clé de type chaîne attendue
json-reason-expected = ':' attendu
json-reason-expected-or-a-closing-bracket = ',' ou crochet fermant attendu
json-reason-expected-a-value = valeur attendue
json-reason-empty-document = document vide
json-reason-unexpected-end-of-input = fin de saisie inattendue
json-reason-too-deeply-nested-to-format = imbrication trop profonde pour le formatage
tools-compare = Comparer avec…
compare-need-two = Ouvrez un autre onglet pour effectuer la comparaison.
compare-pick-title = Comparer { $name } avec
compare-count = { $count ->
    [0] Aucune différence
    [one] { $count } modification
    [many] { $count } modifications
   *[other] { $count } modifications
    }
compare-close = Fermer la comparaison
compare-to-left = Copier cette modification à gauche
compare-to-right = Copier cette modification à droite
compare-stale = Un document a été modifié pendant la comparaison. La comparaison a été actualisée ; réessayez.
view-structured = Vue structurée (CSV, JSON, XML, YAML)
csv-delimiter = Séparateur
csv-rows = { $count ->
    [one] { $count } ligne
    [many] { $count } lignes
   *[other] { $count } lignes
    }
csv-unterminated = Guillemet non fermé à partir de la ligne { $line }
csv-truncated = Affichage des 200 000 premières lignes
json-more = … { $count } de plus
dialog-open-link-title = Ouvrir ce lien dans votre navigateur web ?
button-open = Ouvrir
link-blocked = Ce type de lien n’est pas ouvert, par sécurité.
link-not-found = Fichier lié introuvable : { $target }
enc-windows-1252 = Europe occidentale
enc-iso-8859-15 = Europe occidentale
enc-windows-1250 = Europe centrale
enc-iso-8859-2 = Europe centrale
enc-windows-1251 = Cyrillique
enc-koi8-r = Cyrillique
enc-windows-1253 = Grec
enc-windows-1254 = Turc
enc-windows-1255 = Hébreu
enc-windows-1256 = Arabe
enc-windows-1257 = Balte
enc-windows-874 = Thaï
enc-shift_jis = Japonais
enc-euc-jp = Japonais
enc-gb18030 = Chinois simplifié
enc-big5 = Chinois traditionnel
enc-euc-kr = Coréen
file-reopen-encoding = Rouvrir avec l’encodage…
file-save-encoding = Enregistrer avec l’encodage…
encoding-reopen-title = Rouvrir avec l’encodage
encoding-save-title = Enregistrer avec l’encodage
encoding-not-exact = Cet encodage ne permet pas d’ouvrir le fichier sans le modifier : { $reason }
encoding-unencodable = « { $character } » ne peut pas être enregistré en { $encoding }. Choisissez un autre encodage ou modifiez le texte ; rien n’a été écrit.
encoding-save-first = Enregistrez ou annulez vos modifications avant de rouvrir avec un autre encodage.
image-remote = Les images distantes ne sont pas chargées.
image-blocked = Seules les images du dossier de ce document sont affichées.
image-unsupported = Format d’image non pris en charge.
image-too-large = L’image est trop grande pour être affichée.
image-unreadable = Impossible de lire l’image.
settings-theme-high-contrast = Contraste élevé
view-fold = Replier
view-unfold = Déplier
view-unfold-all = Tout déplier
settings-shortcuts = Raccourcis clavier
settings-keymap-preset = Préréglage
settings-keymap-default = Sanain
settings-keymap-notepadpp = Notepad++
settings-shortcut-change = Modifier
settings-shortcut-reset = Réinitialiser
settings-shortcut-press = Appuyez sur un raccourci… (Esc pour annuler)
settings-shortcut-moved = Raccourci déplacé depuis :
edit-find-in-folder = Rechercher dans le dossier…
folder-search-placeholder = Rechercher dans le dossier
folder-search-run = Rechercher
folder-search-stop = Arrêter
folder-search-no-folder = Choisir un dossier
folder-search-summary = { $count ->
    [one] { $count } occurrence
    [many] { $count } occurrences
   *[other] { $count } occurrences
    } dans { $files ->
    [one] { $files } fichier
    [many] { $files } fichiers
   *[other] { $files } fichiers
    }
app-check-updates = Rechercher des mises à jour…
update-checking = Recherche de mises à jour…
update-up-to-date = Sanain { $version } est la version la plus récente.
update-no-releases = Aucune version de Sanain n’a encore été publiée.
update-failed = Impossible de rechercher des mises à jour : { $reason }
update-invalid = réponse inattendue
update-available-title = Sanain { $version } est disponible
update-available-body = Ouvrez la page de téléchargement pour l’obtenir. Sanain ne télécharge ni n’installe rien de lui-même.
update-open-page = Ouvrir la page de téléchargement
update-later = Plus tard
settings-check-updates = Rechercher des mises à jour une fois par jour
settings-check-updates-detail = Demande à github.com la dernière version publiée. Rien d’autre n’est envoyé.
file-local-history = Historique local…
file-clear-history = Effacer l’historique local…
history-title = Historique local de { $name }
history-open = Ouvrir
history-restore = Restaurer
history-empty = Aucune version enregistrée pour l’instant. Une copie est conservée à chaque enregistrement.
history-unreadable = Impossible de lire cette version.
history-just-now = À l’instant
history-minutes-ago = { $count ->
    [one] il y a 1 minute
    [many] il y a { $count } minutes
   *[other] il y a { $count } minutes
    }
history-hours-ago = { $count ->
    [one] il y a 1 heure
    [many] il y a { $count } heures
   *[other] il y a { $count } heures
    }
history-days-ago = { $count ->
    [one] il y a 1 jour
    [many] il y a { $count } jours
   *[other] il y a { $count } jours
    }
history-purge-title = Supprimer tout l’historique local ?
history-purge-body = Les copies enregistrées de tous les fichiers sont supprimées de cet ordinateur. Vos fichiers eux-mêmes ne sont pas modifiés.
history-purge-confirm = Supprimer l’historique
history-purged = Historique local supprimé.
settings-local-history = Conserver un historique local des fichiers enregistrés
settings-local-history-detail = Conserve jusqu’à 50 versions par fichier pendant 30 jours, uniquement sur cet ordinateur.
file-open-log = Ouvrir dans la visionneuse de journaux…
log-lines = { $count ->
    [one] { $count } ligne
    [many] { $count } lignes
   *[other] { $count } lignes
    }
log-indexing = Lecture… { $percent } %
log-matches = { $count ->
    [one] { $count } ligne correspondante
    [many] { $count } lignes correspondantes
   *[other] { $count } lignes correspondantes
    }
log-filter-hint = Filtrer les lignes
log-bad-pattern = Motif non valide
log-follow = Suivre
log-close = Fermer la visionneuse
log-opened-large = { $name } est volumineux et a donc été ouvert en lecture seule dans la visionneuse de journaux.
tab-pin = Épingler l’onglet
tab-unpin = Détacher l’onglet
edit-add-cursor-above = Ajouter un curseur au-dessus
edit-add-cursor-below = Ajouter un curseur en dessous
view-word-wrap = Retour automatique à la ligne
status-wrap-on = Les lignes longues sont désormais renvoyées à la ligne selon la largeur de la fenêtre.
status-wrap-off = Les lignes longues ne sont plus renvoyées à la ligne.
settings-word-wrap = Renvoyer automatiquement les lignes longues à la ligne
edit-go-to-line = Aller à la ligne…
edit-go-to-heading = Aller au titre…
goto-line-hint = Ligne ou ligne:colonne
goto-line-note = { $count ->
    [one] Le document compte 1 ligne.
    [many] Le document compte { $count } de lignes.
   *[other] Le document compte { $count } lignes.
    }
goto-line-invalid = Saisissez un numéro de ligne, éventuellement suivi de :colonne.
goto-heading-hint = Tapez pour filtrer les titres
goto-heading-none = Ce document ne contient aucun titre.
format-xml = XML
format-yaml = YAML
tools-tree-error = { $format } non valide à la ligne { $line }, colonne { $column } : { $reason }
edit-select-next-occurrence = Sélectionner l’occurrence suivante
tools-record-macro = Démarrer ou arrêter l’enregistrement de la macro
tools-play-macro = Exécuter la macro
macro-recording = Enregistrement de la macro
macro-recorded = { $count ->
    [one] Macro enregistrée : 1 étape.
    [many] Macro enregistrée : { $count } d’étapes.
   *[other] Macro enregistrée : { $count } étapes.
    }
tools-compare-saved = Comparer avec la version enregistrée
tools-compare-clipboard = Comparer avec le presse-papiers
compare-saved-label = { $name } (sur le disque)
compare-clipboard-label = Presse-papiers
compare-saved-failed = Impossible de lire le fichier enregistré : { $reason }
compare-clipboard-empty = Le presse-papiers ne contient aucun texte à comparer.
error-save-disk-full = { $name } n’a pas été enregistré : le disque est plein. Le fichier sur le disque n’a pas changé et votre texte est toujours là. Libérez de l’espace ou utilisez Enregistrer sous pour l’enregistrer ailleurs.
error-save-permission = { $name } n’a pas été enregistré : vous n’avez pas l’autorisation d’écrire à cet emplacement. Le fichier sur le disque n’a pas changé. Utilisez Enregistrer sous pour l’enregistrer ailleurs.
error-save-read-only-volume = { $name } n’a pas été enregistré : le disque est en lecture seule. Le fichier sur le disque n’a pas changé. Utilisez Enregistrer sous pour l’enregistrer ailleurs.
replace-preview-title = { $count ->
    [one] Remplacer { $count } occurrence ?
    [many] Remplacer { $count } occurrences ?
   *[other] Remplacer { $count } occurrences ?
    }
replace-preview-confirm = { $count ->
    [one] Remplacer { $count }
    [many] Remplacer { $count }
   *[other] Remplacer { $count }
    }
replace-preview-more = Seules les 200 premières lignes modifiées sont affichées.
replace-preview-stale = Le document a changé ; rien n’a été remplacé. Relancez Tout remplacer.
view-split-editor = Fractionner l’éditeur
encoding-detected = Ouvert en { $encoding } (détecté à partir du contenu du fichier).
encoding-suggested = Suggéré d’après le contenu du fichier : { $encoding }. Vérifiez l’aperçu avant d’ouvrir.
app-about = À propos de Sanain
menu-help = Aide
about-tagline = Le pouvoir des mots.
about-version = Version { $version }
about-license = Libre et open source sous la Mozilla Public License 2.0.
about-ai = Conçu par l’IA. Dirigé par des humains. Porté par la communauté.
about-notices = Les composants tiers et leurs licences sont répertoriés dans THIRD_PARTY_NOTICES, inclus dans chaque version.
about-source = Code source et signalements sur GitHub
