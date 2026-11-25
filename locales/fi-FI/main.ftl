# Draft translation; native review required before release.
app-name = Sanain
menu-file = Tiedosto
file-open = Avaa…
file-save = Tallenna
status-tabs = { $count ->
    [one] Yksi välilehti
   *[other] { $count } välilehteä
    }
error-open = Tiedostoa { $name } ei voitu avata.
menu-edit = Muokkaa
file-new = Uusi välilehti
file-save-as = Tallenna nimellä…
edit-undo = Kumoa
edit-redo = Tee uudelleen
edit-select-all = Valitse kaikki
untitled = Nimetön
window-title = { $dirty ->
    [yes] ● { $name } — Sanain
   *[no] { $name } — Sanain
    }
status-position = Rivi { $line }, sarake { $column }
status-selections = { $count ->
    [one] 1 valinta
   *[other] { $count } valintaa
    }
status-saved = { $name } tallennettu
encoding-utf8 = UTF-8
encoding-utf8-bom = UTF-8 ja BOM
encoding-utf16le = UTF-16 LE
encoding-utf16be = UTF-16 BE
line-ending-lf = LF
line-ending-crlf = CRLF
line-ending-cr = CR
line-ending-mixed = Sekalaiset rivinvaihdot
editor-accessible-name = Tekstieditori
button-save = Tallenna
button-dont-save = Älä tallenna
button-cancel = Peruuta
button-ok = OK
button-overwrite = Korvaa
button-save-as = Tallenna nimellä…
dialog-unsaved-title = Tallennetaanko muutokset tiedostoon { $name }?
dialog-unsaved-body = Muutokset menetetään, jos et tallenna niitä.
dialog-conflict-title = Tiedosto { $name } on muuttunut levyllä
dialog-conflict-body = Toinen ohjelma muokkasi tiedostoa sen avaamisen jälkeen. Korvaaminen poistaa ne muutokset.
dialog-readonly-title = Tiedosto { $name } on vain luku -tilassa
dialog-readonly-body = Tallenna kopio toisella nimellä.
error-open-encoding = { $name } ei ole UTF-8- tai UTF-16-tekstiä. Sitä ei avattu, joten sen sisältö on ennallaan. Valitse merkistö kohdasta Tiedosto > Avaa uudelleen merkistöllä.
error-save = Tiedostoa { $name } ei voitu tallentaa: { $reason }
menu-window = Ikkuna
edit-cut = Leikkaa
edit-copy = Kopioi
edit-paste = Liitä
app-quit = Lopeta Sanain
app-hide = Kätke Sanain
app-hide-others = Kätke muut
app-show-all = Näytä kaikki
window-minimize = Pienennä
window-zoom = Zoomaa
status-recovered = Edellisen istunnon tallentamattomat muutokset palautettiin
status-recovery-failed = Palautuskopiota ei voitu tallentaa: { $reason }
edit-duplicate-line = Monista rivi
edit-delete-line = Poista rivi
edit-move-line-up = Siirrä riviä ylös
edit-move-line-down = Siirrä riviä alas
file-close-tab = Sulje välilehti
file-reopen-tab = Avaa suljettu välilehti uudelleen
window-next-tab = Seuraava välilehti
window-prev-tab = Edellinen välilehti
tab-close = Sulje välilehti
edit-find = Etsi…
edit-find-next = Etsi seuraava
edit-find-prev = Etsi edellinen
edit-replace = Korvaa…
find-placeholder = Etsi
replace-placeholder = Korvaa tekstillä
find-match-case = Sama kirjainkoko
find-whole-word = Kokonainen sana
find-regex = Säännöllinen lauseke
find-no-results = Ei tuloksia
find-count = { $total ->
    [one] 1 osuma
   *[other] { $total } osumaa
    }
find-count-current = { $current }/{ $total }
find-prev = Edellinen osuma (Vaihto+Enter)
find-next = Seuraava osuma (Enter)
find-close = Sulje (Esc)
replace-one = Korvaa
replace-all = Korvaa kaikki
status-replaced = { $count ->
    [one] 1 osuma korvattiin
   *[other] { $count } osumaa korvattiin
    }
language-plain = Pelkkä teksti
menu-view = Näytä
app-settings = Asetukset…
view-zoom-in = Lähennä
view-zoom-out = Loitonna
view-zoom-reset = Todellinen koko
settings-title = Asetukset
settings-font-size = Fonttikoko
settings-tab-width = Sarkaimen leveys
settings-insert-spaces = Lisää välilyöntejä sarkainnäppäimellä
settings-theme = Ulkoasu
settings-theme-system = Järjestelmän mukaan
settings-theme-light = Vaalea
settings-theme-dark = Tumma
settings-restore-session = Avaa edellisen istunnon tiedostot
settings-language = Kieli
settings-language-system = Järjestelmän mukaan
settings-language-note = Kielen vaihto tulee voimaan, kun Sanain käynnistetään uudelleen.
settings-save-failed = Asetuksia ei voitu tallentaa: { $reason }
view-command-palette = Komentopaletti…
palette-placeholder = Kirjoita komento
view-markdown-source = Markdown-lähde
view-markdown-split = Markdown-lähde ja esikatselu
view-markdown-preview = Markdown-esikatselu
preview-too-large = Asiakirja on liian suuri esikatseltavaksi.
preview-mermaid-unsupported = Mermaid-kaavioita ei vielä piirretä; näytetään lähdekoodi.
menu-tools = Työkalut
tools-json-validate = Tarkista JSON
tools-json-format = Muotoile JSON
tools-json-minify = Tiivistä JSON
tools-sort-lines = Lajittele rivit
tools-sort-lines-desc = Lajittele rivit laskevasti
tools-unique-lines = Poista toistuvat rivit
tools-trim-trailing = Poista rivien lopun välilyönnit
tools-upper-case = Isot kirjaimet
tools-lower-case = Pienet kirjaimet
tools-to-lf = Muunna rivinvaihdot LF-muotoon
tools-to-crlf = Muunna rivinvaihdot CRLF-muotoon
tools-json-valid = JSON on kelvollinen
tools-done = Valmis. Kumoa palauttaa muutoksen.
tools-no-change = Ei muutettavaa
tools-json-error = Virheellinen JSON rivillä { $line }, sarake { $column }: { $reason }
json-reason-unterminated-string = merkkijono jää sulkematta
json-reason-invalid-u-escape = virheellinen \u-koodi
json-reason-invalid-escape = virheellinen escape-merkintä
json-reason-control-character-in-string = ohjausmerkki merkkijonossa
json-reason-invalid-number = virheellinen luku
json-reason-unexpected-character = odottamaton merkki
json-reason-unexpected-content-after-the-value = ylimääräistä sisältöä arvon jälkeen
json-reason-unexpected-comma = odottamaton pilkku
json-reason-expected-a-string-key = odotettiin avainta merkkijonona
json-reason-expected = odotettiin kaksoispistettä
json-reason-expected-or-a-closing-bracket = odotettiin pilkkua tai sulkevaa sulkua
json-reason-expected-a-value = odotettiin arvoa
json-reason-empty-document = tyhjä asiakirja
json-reason-unexpected-end-of-input = syöte loppui kesken
json-reason-too-deeply-nested-to-format = liian syvä sisäkkäisyys muotoiltavaksi
tools-compare = Vertaa…
compare-need-two = Avaa toinen välilehti vertailua varten.
compare-pick-title = Vertaa tiedostoa { $name } tiedostoon
compare-count = { $count ->
    [0] Ei eroja
    [one] 1 muutos
   *[other] { $count } muutosta
    }
compare-close = Sulje vertailu
compare-to-left = Kopioi tämä muutos vasemmalle
compare-to-right = Kopioi tämä muutos oikealle
compare-stale = Asiakirja muuttui vertailun aikana. Vertailu päivitettiin; yritä uudelleen.
view-structured = Rakenteinen näkymä (CSV, JSON, XML, YAML)
csv-delimiter = Erotin
csv-rows = { $count ->
    [one] 1 rivi
   *[other] { $count } riviä
    }
csv-unterminated = Sulkematon lainausmerkki rivillä { $line }
csv-truncated = Näytetään ensimmäiset 200 000 riviä
json-more = … { $count } lisää
dialog-open-link-title = Avataanko linkki selaimessa?
button-open = Avaa
link-blocked = Tämäntyyppistä linkkiä ei avata turvallisuussyistä.
link-not-found = Linkitettyä tiedostoa ei löydy: { $target }
enc-windows-1252 = Länsieurooppalainen
enc-iso-8859-15 = Länsieurooppalainen
enc-windows-1250 = Keskieurooppalainen
enc-iso-8859-2 = Keskieurooppalainen
enc-windows-1251 = Kyrillinen
enc-koi8-r = Kyrillinen
enc-windows-1253 = Kreikkalainen
enc-windows-1254 = Turkkilainen
enc-windows-1255 = Heprealainen
enc-windows-1256 = Arabialainen
enc-windows-1257 = Balttilainen
enc-windows-874 = Thai
enc-shift_jis = Japanilainen
enc-euc-jp = Japanilainen
enc-gb18030 = Kiina, yksinkertaistettu
enc-big5 = Kiina, perinteinen
enc-euc-kr = Korealainen
file-reopen-encoding = Avaa uudelleen merkistöllä…
file-save-encoding = Tallenna merkistöllä…
encoding-reopen-title = Avaa uudelleen merkistöllä
encoding-save-title = Tallenna merkistöllä
encoding-not-exact = Tällä merkistöllä tiedostoa ei voi avata muuttamatta sitä: { $reason }
encoding-unencodable = Merkkiä ”{ $character }” ei voi tallentaa merkistöllä { $encoding }. Valitse toinen merkistö tai muuta tekstiä; mitään ei kirjoitettu.
encoding-save-first = Tallenna tai kumoa muutokset ennen kuin avaat toisella merkistöllä.
image-remote = Etäkuvia ei ladata.
image-blocked = Vain tämän asiakirjan kansiossa olevat kuvat näytetään.
image-unsupported = Kuvamuotoa ei tueta.
image-too-large = Kuva on liian suuri näytettäväksi.
image-unreadable = Kuvaa ei voitu lukea.
settings-theme-high-contrast = Suuri kontrasti
view-fold = Laskosta
view-unfold = Avaa laskos
view-unfold-all = Avaa kaikki laskokset
settings-shortcuts = Pikanäppäimet
settings-keymap-preset = Pohja
settings-keymap-default = Sanain
settings-keymap-notepadpp = Notepad++
settings-shortcut-change = Vaihda
settings-shortcut-reset = Palauta
settings-shortcut-press = Paina pikanäppäintä… (Esc peruu)
settings-shortcut-moved = Pikanäppäin siirrettiin komennolta:
edit-find-in-folder = Etsi kansiosta…
folder-search-placeholder = Etsi kansiosta
folder-search-run = Hae
folder-search-stop = Pysäytä
folder-search-no-folder = Valitse kansio
folder-search-summary = { $count ->
    [one] 1 osuma
   *[other] { $count } osumaa
    } { $files ->
    [one] 1 tiedostossa
   *[other] { $files } tiedostossa
    }
app-check-updates = Tarkista päivitykset…
update-checking = Tarkistetaan päivityksiä…
update-up-to-date = Sanain { $version } on uusin versio.
update-no-releases = Sanainista ei ole vielä julkaistu versioita.
update-failed = Päivityksiä ei voitu tarkistaa: { $reason }
update-invalid = odottamaton vastaus
update-available-title = Sanain { $version } on saatavilla
update-available-body = Avaa lataussivu hakeaksesi sen. Sanain ei lataa eikä asenna mitään itse.
update-open-page = Avaa lataussivu
update-later = Myöhemmin
settings-check-updates = Tarkista päivitykset kerran päivässä
settings-check-updates-detail = Kysyy github.comista uusinta julkaisua. Mitään muuta ei lähetetä.
file-local-history = Paikallinen historia…
file-clear-history = Tyhjennä paikallinen historia…
history-title = Tiedoston { $name } paikallinen historia
history-open = Avaa
history-restore = Palauta
history-empty = Tallennettuja versioita ei vielä ole. Jokaisesta tallennuksesta säilytetään kopio.
history-unreadable = Tätä versiota ei voitu lukea.
history-just-now = Juuri nyt
history-minutes-ago = { $count ->
    [one] 1 minuutti sitten
   *[other] { $count } minuuttia sitten
    }
history-hours-ago = { $count ->
    [one] 1 tunti sitten
   *[other] { $count } tuntia sitten
    }
history-days-ago = { $count ->
    [one] 1 päivä sitten
   *[other] { $count } päivää sitten
    }
history-purge-title = Poistetaanko koko paikallinen historia?
history-purge-body = Kaikkien tiedostojen tallennetut kopiot poistetaan tältä tietokoneelta. Itse tiedostoihin ei kosketa.
history-purge-confirm = Poista historia
history-purged = Paikallinen historia poistettiin.
settings-local-history = Säilytä tallennettujen tiedostojen paikallinen historia
settings-local-history-detail = Säilyttää enintään 50 versiota tiedostoa kohden 30 päivän ajan, vain tällä tietokoneella.
file-open-log = Avaa lokikatseluun…
log-lines = { $count ->
    [one] 1 rivi
   *[other] { $count } riviä
    }
log-indexing = Luetaan… { $percent } %
log-matches = { $count ->
    [one] 1 osuva rivi
   *[other] { $count } osuvaa riviä
    }
log-filter-hint = Suodata rivejä
log-bad-pattern = Virheellinen haku
log-follow = Seuraa
log-close = Sulje katselu
log-opened-large = { $name } on suuri, joten se avattiin vain luettavaksi lokikatseluun.
tab-pin = Kiinnitä välilehti
tab-unpin = Irrota välilehti
edit-add-cursor-above = Lisää kohdistin yläpuolelle
edit-add-cursor-below = Lisää kohdistin alapuolelle
view-word-wrap = Rivitys
status-wrap-on = Pitkät rivit rivitetään nyt ikkunan levyiseksi.
status-wrap-off = Pitkiä rivejä ei enää rivitetä.
settings-word-wrap = Rivitä pitkät rivit
edit-go-to-line = Siirry riville…
edit-go-to-heading = Siirry otsikkoon…
goto-line-hint = Rivi tai rivi:sarake
goto-line-note = { $count ->
    [one] Asiakirjassa on 1 rivi.
   *[other] Asiakirjassa on { $count } riviä.
    }
goto-line-invalid = Anna rivinumero ja halutessasi :sarake.
goto-heading-hint = Suodata otsikoita kirjoittamalla
goto-heading-none = Tässä asiakirjassa ei ole otsikoita.
format-xml = XML
format-yaml = YAML
tools-tree-error = Virheellinen { $format } rivillä { $line }, sarakkeessa { $column }: { $reason }
edit-select-next-occurrence = Valitse seuraava esiintymä
