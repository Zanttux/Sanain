# Draft translation; native review required before release.
app-name = Text³
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
    [yes] ● { $name } — Text³
   *[no] { $name } — Text³
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
app-quit = Lopeta Text³
app-hide = Kätke Text³
app-hide-others = Kätke muut
app-show-all = Näytä kaikki
window-minimize = Pienennä
window-zoom = Zoomaa
status-recovered = Edellisen istunnon tallentamattomat muutokset palautettiin
status-recovery-failed = Palautuskopiota ei voitu tallentaa: { $reason }
status-hot-exit-failed = Tallentamattomia muutoksia ei voitu säilyttää seuraavaan käynnistykseen ({ $reason }), joten niistä kysytään.
status-hot-exit-too-large = { $name } on liian suuri säilytettäväksi seuraavaan käynnistykseen, joten siitä kysytään.
status-restored-changed = { $name } muuttui levyllä sen jälkeen, kun tallentamattomat muutoksesi säilytettiin. Tekstisi on välilehdellä, eikä tiedostoon ole koskettu.
status-restored-missing = Tiedostoa { $name } ei enää ole levyllä. Sen tallentamaton teksti on tallessa välilehdellä; tallentaminen luo tiedoston uudelleen.
edit-duplicate-line = Monista rivi
edit-delete-line = Poista rivi
edit-move-line-up = Siirrä riviä ylös
edit-move-line-down = Siirrä riviä alas
file-close-tab = Sulje välilehti
file-reopen-tab = Avaa suljettu välilehti uudelleen
window-next-tab = Seuraava välilehti
window-prev-tab = Edellinen välilehti
tab-close = Sulje välilehti
tab-missing-on-disk = Ei löydy levyltä
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
settings-hot-exit = Muista tallentamattomat muutokset suljettaessa
settings-hot-exit-detail = Sulkeminen ei kysy jokaisesta tallentamattomasta välilehdestä. Tallentamattomat ja nimettömät välilehdet palaavat tekstineen seuraavalla käynnistyskerralla, vaikka edellisen istunnon tiedostojen avaaminen olisi pois päältä. Yksittäisen välilehden sulkeminen kysyy edelleen.
settings-language = Kieli
settings-language-system = Järjestelmän mukaan
settings-language-note = Kielen vaihto tulee voimaan, kun Text³ käynnistetään uudelleen.
settings-save-failed = Asetuksia ei voitu tallentaa: { $reason }
view-command-palette = Komentopaletti…
palette-placeholder = Kirjoita komento
view-markdown-source = Markdown-lähde
view-markdown-split = Markdown-lähde ja esikatselu
view-markdown-preview = Markdown-esikatselu
preview-too-large = Asiakirja on liian suuri esikatseltavaksi.
preview-mermaid-unsupported-type = Kaaviotyyppiä ei tueta, joten näytetään lähdekoodi.
preview-mermaid-unsupported-syntax = Kaaviossa on merkintöjä, joita ei osata piirtää, joten näytetään lähdekoodi.
preview-mermaid-too-large = Kaavio on liian suuri piirrettäväksi, joten näytetään lähdekoodi.
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
csv-edit-cell = Muokkaa solua
csv-insert-row-below = Lisää rivi alle
csv-delete-row = Poista rivi
csv-read-only = Tiedosto on vain luettavissa, joten taulukkoa ei voi muokata
json-more = … { $count } lisää
json-edit-value = Muokkaa arvoa
json-rename-key = Nimeä avain uudelleen
json-add-after = Lisää perään
json-add-inside = Lisää sisään
json-delete-node = Poista
json-read-only = Tiedosto on vain luettavissa, joten puuta ei voi muokata
json-invalid-value = Arvo ei ole kelvollista JSONia
json-duplicate-key = Objektissa on jo samanniminen avain
xml-edit-value = Muokkaa arvoa
xml-edit-text = Muokkaa tekstiä
xml-rename = Nimeä uudelleen
xml-add-attribute = Lisää attribuutti
xml-add-child = Lisää alielementti
xml-add-after = Lisää perään
xml-delete = Poista
xml-invalid-name = Nimi ei kelpaa XML-nimeksi
xml-duplicate-attribute = Elementillä on jo samanniminen attribuutti
xml-not-well-formed = Muutoksen jälkeen XML ei olisi kelvollista
yaml-not-preserved = Muutos muuttaisi YAMLin muitakin kohtia
yaml-key-too-long = Avain voi olla enintään 1024 merkkiä pitkä
yaml-read-only-flow = Sulkeiden sisään kirjoitettua (flow-tyylistä) rakennetta ei voi vielä muokata puussa
yaml-read-only-anchor = Arvoja, joilla on ankkuri (&), ei voi muokata puussa
yaml-read-only-tag = Arvoja, joilla on tagi (!), ei voi muokata puussa
yaml-read-only-alias = Aliaksia (*) ei voi muokata puussa
yaml-read-only-block-scalar = Tekstilohkoja (| tai >) ei voi vielä muokata puussa
yaml-read-only-complex-key = Monimutkaisia avaimia ei voi muokata puussa
yaml-read-only-document = Puussa voi muokata vain ensimmäistä dokumenttia
yaml-read-only-layout = Kohta jakaa rivinsä muun sisällön kanssa, joten sitä ei voi muuttaa tässä
yaml-read-only-only-child = Ainoaa kohtaa ei voi poistaa
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
settings-keymap-default = Text³
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
update-up-to-date = Text³ { $version } on uusin versio.
update-no-releases = Text³:sta ei ole vielä julkaistu versioita.
update-failed = Päivityksiä ei voitu tarkistaa: { $reason }
update-invalid = odottamaton vastaus
update-available-title = Text³ { $version } on saatavilla
update-available-body = Avaa lataussivu hakeaksesi sen. Text³ ei lataa eikä asenna mitään itse.
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
edit-goto-anything = Siirry mihin tahansa…
goto-anything-hint = Tiedoston nimi tai :rivi, @symboli, #teksti
goto-anything-line = Siirry riville { $line }
goto-anything-open = auki
goto-anything-scanning = Haetaan tiedostoja kansiosta { $folder }…
goto-anything-scanned = Tiedostoja kansiossa { $folder }: { $count }
goto-anything-limit = Kansiosta { $folder } näytetään vain ensimmäiset { $max } tiedostoa.
goto-anything-no-folder = Tallenna tiedosto tai käytä Etsi kansiosta -toimintoa, niin kansion tiedostot näkyvät tässä.
goto-anything-symbols-none = Tässä asiakirjassa ei ole symboleita.
goto-anything-matches-none = Tästä asiakirjasta ei löytynyt osumia.
format-xml = XML
format-yaml = YAML
tools-tree-error = Virheellinen { $format } rivillä { $line }, sarakkeessa { $column }: { $reason }
edit-select-next-occurrence = Valitse seuraava esiintymä
tools-record-macro = Aloita tai lopeta makron tallennus
tools-play-macro = Toista makro
macro-recording = Makroa tallennetaan
macro-recorded = { $count ->
    [one] Makro tallennettu: 1 vaihe.
   *[other] Makro tallennettu: { $count } vaihetta.
    }
tools-compare-saved = Vertaa tallennettuun versioon
tools-compare-clipboard = Vertaa leikepöytään
compare-saved-label = { $name } (levyllä)
compare-clipboard-label = Leikepöytä
compare-saved-failed = Tallennettua tiedostoa ei voitu lukea: { $reason }
compare-clipboard-empty = Leikepöydällä ei ole tekstiä vertailtavaksi.
error-save-disk-full = Tiedostoa { $name } ei tallennettu: levy on täynnä. Levyllä oleva tiedosto ei muuttunut, ja tekstisi on yhä tallessa. Vapauta tilaa tai tallenna muualle valitsemalla Tallenna nimellä.
error-save-permission = Tiedostoa { $name } ei tallennettu: sinulla ei ole oikeutta kirjoittaa siihen sijaintiin. Levyllä oleva tiedosto ei muuttunut. Tallenna muualle valitsemalla Tallenna nimellä.
error-save-read-only-volume = Tiedostoa { $name } ei tallennettu: levy on vain luku -tilassa. Levyllä oleva tiedosto ei muuttunut. Tallenna muualle valitsemalla Tallenna nimellä.
replace-preview-title = { $count ->
    [one] Korvataanko 1 osuma?
   *[other] Korvataanko { $count } osumaa?
    }
replace-preview-confirm = { $count ->
    [one] Korvaa 1
   *[other] Korvaa { $count }
    }
replace-preview-more = Vain 200 ensimmäistä muuttuvaa riviä näytetään.
replace-preview-stale = Asiakirja muuttui, joten mitään ei korvattu. Valitse Korvaa kaikki uudelleen.
encoding-detected = Avattiin merkistöllä { $encoding } (tunnistettu tiedoston sisällöstä).
encoding-suggested = Tiedoston sisällön perusteella ehdotettu: { $encoding }. Tarkista esikatselu ennen avaamista.
app-about = Tietoja Text³:sta
menu-help = Ohje
about-version = Versio { $version }
about-license = Vapaa ja avoin lähdekoodi Mozilla Public License 2.0 -lisenssillä.
about-ai = Kehitetty tekoälyn avulla (Anthropicin Claude) ylläpitäjän johdolla.
about-notices = Kolmansien osapuolten komponentit lisensseineen on lueteltu tiedostossa THIRD_PARTY_NOTICES, joka on mukana jokaisessa julkaisussa.
about-source = Lähdekoodi ja palautteet GitHubissa
tools-xml-validate = Tarkista XML
tools-yaml-validate = Tarkista YAML
tools-markup-valid = Kelvollinen { $format }.
file-open-recent = Avaa viimeaikainen…
file-save-all = Tallenna kaikki
file-revert = Palauta tallennettu versio
recent-hint = Suodata viimeaikaisia tiedostoja kirjoittamalla
recent-none = Viimeaikaisia tiedostoja ei vielä ole.
revert-title = Hylätäänkö tiedoston { $name } muutokset?
revert-body = Tiedosto ladataan uudelleen levyltä. Tallentamattomat muutokset menetetään, eikä niitä voi kumota.
revert-confirm = Hylkää ja lataa uudelleen
revert-done = { $name } ladattiin uudelleen levyltä.
status-saved-all = { $count ->
    [0] Ei tallennettavaa.
    [one] 1 tiedosto tallennettiin.
   *[other] { $count } tiedostoa tallennettiin.
    }
edit-toggle-comment = Kommentoi rivit / poista kommentointi
edit-indent = Sisennä rivit
edit-outdent = Poista rivien sisennys
comment-unsupported = Tällä tiedostotyypillä ei ole rivikommentteja.
tab-close-others = Sulje muut välilehdet
tab-close-all = Sulje kaikki välilehdet
file-copy-path = Kopioi tiedostopolku
file-reveal-macos = Näytä Finderissa
file-reveal-windows = Näytä Resurssienhallinnassa
file-reveal-other = Näytä tiedostonhallinnassa
status-path-copied = Tiedostopolku kopioitiin.
tabs-kept-unsaved = { $count ->
    [one] 1 välilehti, jossa on tallentamattomia muutoksia, jätettiin auki.
   *[other] { $count } välilehteä, joissa on tallentamattomia muutoksia, jätettiin auki.
    }
status-indent-tabs = Sarkaimet
status-indent-spaces = Välilyönnit: { $count }
find-searching = Haetaan…
compare-ignore-whitespace = Ohita tyhjätila
tools-compare-file = Vertaa tiedostoon…
compare-file-failed = Vertailu tiedostoon { $name } ei onnistunut: { $reason }
cli-help =
    Käyttö:
      text3 [TIEDOSTO…]            avaa tiedostot
      text3 --diff VASEN OIKEA     vertaa kahta tiedostoa rinnakkain
      text3 --version              näytä versio
      text3 --help                 näytä tämä ohje
    Kirjoita -- ennen tiedostonimiä, jotka alkavat viivalla.
cli-unknown-option = Tuntematon valitsin { $option }.
cli-diff-needs-two = --diff tarvitsee kaksi tiedostoa: text3 --diff VASEN OIKEA
window-move-tab-left = Siirrä välilehti vasemmalle
window-move-tab-right = Siirrä välilehti oikealle
edit-skip-occurrence = Ohita ja valitse seuraava esiintymä
tools-play-macro-times = Toista makro useita kertoja…
macro-times-hint = Toistokerrat
macro-times-note = 1–{ $max }
macro-times-invalid = Anna luku väliltä 1–{ $max }.
view-outline = Jäsennys
outline-empty = Dokumentissa ei ole otsikoita.
outline-close = Sulje jäsennys
folder-replace-placeholder = Korvaa tekstillä
folder-replace-run = Korvaa tiedostoissa…
replace-files-title = { $count ->
    [one] Korvataanko 1 osuma { $files ->
        [one] 1 tiedostossa
       *[other] { $files } tiedostossa
    }?
   *[other] Korvataanko { $count } osumaa { $files ->
        [one] 1 tiedostossa
       *[other] { $files } tiedostossa
    }?
}
replace-files-body = Välilehdellä auki olevat tiedostot muutetaan siellä, ja muutoksen voi kumota; tallenna ne itse. Muut tiedostot tallennetaan heti, ja niiden aiempi sisältö säilyy paikallisessa historiassa, jos se on käytössä. Jos tiedosto on sillä välin muuttunut levyllä, se ohitetaan.
replace-files-confirm = Korvaa
replace-files-done = { $count ->
    [one] Korvattiin 1 osuma
   *[other] Korvattiin { $count } osumaa
} { $files ->
    [one] 1 tiedostossa
   *[other] { $files } tiedostossa
} ({ $open } auki välilehdillä, ei tallennettu).
replace-files-failed = { $failed ->
    [one] 1 tiedostoa ei muutettu:
   *[other] { $failed } tiedostoa jäi muuttamatta; ensimmäinen:
} { $first }
image-outside = Kuva on dokumentin kansion ulkopuolella.
image-allow = Näytä tämän kansion kuvat
cli-help-integration =
    Liittäminen tiedostonhallintaan (vain nykyiselle käyttäjälle):
      text3 --register            lisää Text³ "Avaa sovelluksella" -valikkoon (Linux, Windows)
      text3 --unregister          poista se sieltä
cli-registered = Text³ lisättiin "Avaa sovelluksella" -valikkoon. { $place }
cli-unregistered = Text³ poistettiin "Avaa sovelluksella" -valikosta.
cli-register-failed = Tiedostonhallinnan liitosta ei voitu muuttaa: { $reason }
cli-register-macos = macOS:ssä mitään ei tarvitse rekisteröidä: kun Text³.app on Ohjelmat-kansiossa, Finder tarjoaa sitä Avaa sovelluksella -valikossa.
settings-tab-general = Yleiset
settings-tab-editor = Editori
settings-tab-appearance = Ulkoasu
settings-shortcut-search = Hae komentoa tai näppäintä
settings-font = Fontti
settings-font-default = Sisäänrakennettu tasaväli
settings-font-loading = Haetaan asennettuja fontteja…
settings-font-missing = Fonttia { $font } ei löytynyt; käytetään sisäänrakennettua fonttia.
settings-theme-more = Lisää teemoja
settings-theme-user-note = Omat teemat: .theme-tiedostot kansiossa { $folder } (ks. docs/THEMES.md).
language-auto = Tunnista automaattisesti
language-choose = Valitse tämän välilehden kieli
tools-xml-format = Muotoile XML
tools-save-macro = Tallenna makro nimellä…
tools-play-saved-macro = Toista tallennettu makro…
tools-delete-saved-macro = Poista tallennettu makro…
macro-save-hint = Makron nimi
macro-save-note = Samanniminen tallennettu makro korvataan.
macro-save-empty = Ei tallennettavaa: tallenna ensin makro.
macro-saved = Makro tallennettu: { $name }
macro-deleted = Makro poistettu: { $name }
macro-name-invalid = Anna nimi, jossa on 1–{ $max } merkkiä yhdellä rivillä.
macro-save-too-large = Makro on liian pitkä tallennettavaksi.
macro-save-full = Tallennettuja makroja voi olla enintään { $max }. Poista ensin jokin.
macro-save-failed = Makrojen tallentaminen epäonnistui: { $error }
macro-saved-hint = Suodata tallennettuja makroja kirjoittamalla
macro-saved-none = Tallennettuja makroja ei vielä ole.
settings-shortcut-import-npp = Tuo Notepad++:sta…
settings-shortcut-import-npp-hint = Valitse Notepad++:n shortcuts.xml. Windowsissa se on kansiossa %AppData%\Notepad++\shortcuts.xml.
settings-shortcut-import-npp-done = Tuotiin Notepad++:sta: { $imported }. Ohitettiin: { $skipped }. Ristiriitoja jäljellä: { $conflicts }.
settings-shortcut-import-npp-failed = Tiedoston { $name } tuominen epäonnistui: { $reason }
settings-shortcut-import-npp-too-large = Tiedosto on yli 1 Mt.
settings-shortcut-import-npp-invalid = Tiedosto ei ole kelvollinen Notepad++:n shortcuts.xml.
view-split-right = Jaa oikealle
view-split-down = Jaa alas
view-close-pane = Sulje ruutu
view-focus-next-pane = Siirry seuraavaan ruutuun
view-focus-prev-pane = Siirry edelliseen ruutuun
view-focus-next-region = Siirry seuraavaan alueeseen
view-focus-prev-region = Siirry edelliseen alueeseen
status-pane-limit = Ruutuja on jo enimmäismäärä ({ $max }).
a11y-csv-grid = CSV-taulukko
a11y-grid-cell = Rivi { $row }, sarake { $column }: { $name }
a11y-grid-cell-plain = Rivi { $row }, sarake { $column }
a11y-grid-column-header = Sarakkeen { $column } otsikko
a11y-grid-row-header = Rivi { $row }
a11y-edit-field = Muokataan: { $item }
a11y-json-tree = JSON-puu
a11y-xml-tree = XML-puu
a11y-yaml-tree = YAML-puu
a11y-tree-item = Taso { $level }
a11y-tree-item-expanded = Taso { $level }, laajennettu
a11y-tree-item-collapsed = Taso { $level }, supistettu
a11y-tree-object = Objekti, jäseniä: { $count }
a11y-tree-list = Lista, alkioita: { $count }
a11y-tree-element = Elementti { $name }
a11y-tree-attribute = Attribuutti { $name }
a11y-tree-text = Teksti
a11y-pane = Ruutu { $pane }, ruutuja yhteensä { $count }: { $name }
a11y-pane-active = Ruutu { $pane }, ruutuja yhteensä { $count }: { $name }, aktiivinen
a11y-pane-editor = Tekstieditori, ruutu { $pane }, ruutuja yhteensä { $count }: { $name }
a11y-mermaid-flowchart = Vuokaavio. Solmuja: { $nodes }, yhteyksiä: { $links }.
a11y-mermaid-sequence = Sekvenssikaavio. Osallistujia: { $participants }, viestejä: { $messages }.
a11y-mermaid-state = Tilakaavio. Tiloja: { $states }, siirtymiä: { $transitions }.
a11y-mermaid-class = Luokkakaavio. Luokkia: { $classes }, suhteita: { $relations }.
a11y-shortcut-change = Vaihda komennon { $command } pikanäppäin
a11y-shortcut-change-current = Vaihda komennon { $command } pikanäppäin, nyt { $keys }
a11y-shortcut-reset = Palauta komennon { $command } pikanäppäin
status-line-ending-choose = Vaihda tiedoston rivinvaihdot
status-encoding-choose = Avaa uudelleen tai tallenna toisella merkistökoodauksella
status-indent-choose = Sisennysasetukset
a11y-settings-sections = Asetusten osiot
a11y-tab-strip = Avoimet tiedostot
a11y-tab-unsaved = { $name }, tallentamattomia muutoksia
migration-copied = Sanainin asetukset ja tiedot kopioitiin Text³:een. Sanainin kansiot jätettiin ennalleen.

# Character and word count (status bar)
# How one thousand is written with digit grouping: the characters between
# 1 and 000 separate groups of three digits.
number-thousand = 1{ "\u00A0" }000
count-status = Merkit: { $chars } · Sanat: { $words }
count-status-selection = Merkit: { $chars } / { $total-chars } · Sanat: { $words } / { $total-words }
count-choose = Merkki-, sana-, rivi- ja tavumäärät
count-a11y-pending = Merkkejä ja sanoja ei ole vielä laskettu. Avaa, niin ne lasketaan.
count-heading-selection = Valinta
count-heading-document = Asiakirja
count-characters = Merkit
count-characters-no-spaces = Merkit (ilman välilyöntejä)
count-words = Sanat
count-lines = Rivit
count-bytes = Tavut ({ $encoding })
count-unencodable = Kaikkia merkkejä ei voi tallentaa koodauksella { $encoding }.
count-counting = Lasketaan…
settings-show-counts = Näytä merkki- ja sanamäärä tilarivillä
