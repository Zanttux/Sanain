# Machine translation by Claude (AI); review by a native speaker required before release.
app-name = Sanain
menu-file = Dosya
file-open = Aç…
file-save = Kaydet
status-tabs = { $count ->
    [one] Bir sekme
   *[other] { $count } sekme
    }
error-open = { $name } açılamadı.
menu-edit = Düzen
file-new = Yeni Sekme
file-save-as = Farklı Kaydet…
edit-undo = Geri Al
edit-redo = Yinele
edit-select-all = Tümünü Seç
untitled = Adsız
window-title = { $dirty ->
    [yes] ● { $name } — Sanain
   *[no] { $name } — Sanain
    }
status-position = Sat { $line }, Süt { $column }
status-selections = { $count ->
    [one] 1 seçim
   *[other] { $count } seçim
    }
status-saved = { $name } kaydedildi
encoding-utf8 = UTF-8
encoding-utf8-bom = UTF-8 (BOM ile)
encoding-utf16le = UTF-16 LE
encoding-utf16be = UTF-16 BE
line-ending-lf = LF
line-ending-crlf = CRLF
line-ending-cr = CR
line-ending-mixed = Karışık satır sonları
editor-accessible-name = Metin düzenleyici
button-save = Kaydet
button-dont-save = Kaydetme
button-cancel = İptal
button-ok = Tamam
button-overwrite = Üzerine Yaz
button-save-as = Farklı Kaydet…
dialog-unsaved-title = { $name } dosyasındaki değişiklikler kaydedilsin mi?
dialog-unsaved-body = Kaydetmezseniz değişiklikleriniz kaybolacak.
dialog-conflict-title = { $name } diskte değişti
dialog-conflict-body = Siz açtıktan sonra başka bir program bu dosyayı değiştirdi. Üzerine yazmak bu değişiklikleri siler.
dialog-readonly-title = { $name } salt okunur
dialog-readonly-body = Bunun yerine farklı bir adla bir kopya kaydedin.
error-open-encoding = { $name } UTF-8 veya UTF-16 metin değil. Açılmadı, bu nedenle içeriği değişmedi. Kodlamasını seçmek için Dosya > Kodlamayla Yeniden Aç komutunu kullanın.
error-save = { $name } kaydedilemedi: { $reason }
menu-window = Pencere
edit-cut = Kes
edit-copy = Kopyala
edit-paste = Yapıştır
app-quit = Sanain'den Çık
app-hide = Sanain'i Gizle
app-hide-others = Diğerlerini Gizle
app-show-all = Tümünü Göster
window-minimize = Simge Durumuna Küçült
window-zoom = Yakınlaştır
status-recovered = Önceki oturumdaki kaydedilmemiş değişiklikler geri yüklendi
status-recovery-failed = Kurtarma kopyası kaydedilemedi: { $reason }
edit-duplicate-line = Satırı Çoğalt
edit-delete-line = Satırı Sil
edit-move-line-up = Satırı Yukarı Taşı
edit-move-line-down = Satırı Aşağı Taşı
file-close-tab = Sekmeyi Kapat
file-reopen-tab = Kapatılan Sekmeyi Yeniden Aç
window-next-tab = Sonraki Sekme
window-prev-tab = Önceki Sekme
tab-close = Sekmeyi kapat
edit-find = Bul…
edit-find-next = Sonrakini Bul
edit-find-prev = Öncekini Bul
edit-replace = Değiştir…
find-placeholder = Bul
replace-placeholder = Şununla değiştir
find-match-case = Büyük/küçük harf eşleştir
find-whole-word = Tam sözcük
find-regex = Normal ifade
find-no-results = Sonuç yok
find-count = { $total ->
    [one] 1 eşleşme
   *[other] { $total } eşleşme
    }
find-count-current = { $current } / { $total }
find-prev = Önceki eşleşme (Shift+Enter)
find-next = Sonraki eşleşme (Enter)
find-close = Kapat (Esc)
replace-one = Değiştir
replace-all = Tümünü Değiştir
status-replaced = { $count ->
    [one] 1 eşleşme değiştirildi
   *[other] { $count } eşleşme değiştirildi
    }
language-plain = Düz Metin
menu-view = Görünüm
app-settings = Ayarlar…
view-zoom-in = Yakınlaştır
view-zoom-out = Uzaklaştır
view-zoom-reset = Gerçek Boyut
settings-title = Ayarlar
settings-font-size = Yazı tipi boyutu
settings-tab-width = Tab genişliği
settings-insert-spaces = Tab tuşuna basıldığında boşluk ekle
settings-theme = Görünüm
settings-theme-system = Sistemle aynı
settings-theme-light = Açık
settings-theme-dark = Koyu
settings-restore-session = Önceki oturumdaki dosyaları yeniden aç
settings-language = Dil
settings-language-system = Sistemle aynı
settings-language-note = Dil değişiklikleri Sanain yeniden başlatıldıktan sonra uygulanır.
settings-save-failed = Ayarlar kaydedilemedi: { $reason }
view-command-palette = Komut Paleti…
palette-placeholder = Bir komut yazın
view-markdown-source = Markdown Kaynağı
view-markdown-split = Markdown Kaynağı ve Önizleme
view-markdown-preview = Markdown Önizleme
preview-too-large = Bu belge önizleme için çok büyük.
preview-mermaid-unsupported-type = Diyagram türü desteklenmiyor; kaynak gösteriliyor.
preview-mermaid-unsupported-syntax = Diyagram çizilemeyen bir sözdizimi kullanıyor; kaynak gösteriliyor.
preview-mermaid-too-large = Diyagram çizilemeyecek kadar büyük; kaynak gösteriliyor.
menu-tools = Araçlar
tools-json-validate = JSON'u Doğrula
tools-json-format = JSON'u Biçimlendir
tools-json-minify = JSON'u Küçült
tools-sort-lines = Satırları Sırala
tools-sort-lines-desc = Satırları Azalan Sırada Sırala
tools-unique-lines = Yinelenen Satırları Kaldır
tools-trim-trailing = Sondaki Boşlukları Kırp
tools-upper-case = Büyük Harf
tools-lower-case = Küçük Harf
tools-to-lf = Satır Sonlarını LF'ye Dönüştür
tools-to-crlf = Satır Sonlarını CRLF'ye Dönüştür
tools-json-valid = JSON geçerli
tools-done = Tamamlandı. Geri Al ile geri alınabilir.
tools-no-change = Değiştirilecek bir şey yok
tools-json-error = Satır { $line }, sütun { $column } konumunda geçersiz JSON: { $reason }
json-reason-unterminated-string = sonlandırılmamış dize
json-reason-invalid-u-escape = geçersiz \u kaçış dizisi
json-reason-invalid-escape = geçersiz kaçış dizisi
json-reason-control-character-in-string = dizede denetim karakteri
json-reason-invalid-number = geçersiz sayı
json-reason-unexpected-character = beklenmeyen karakter
json-reason-unexpected-content-after-the-value = değerden sonra beklenmeyen içerik
json-reason-unexpected-comma = beklenmeyen virgül
json-reason-expected-a-string-key = dize anahtarı bekleniyordu
json-reason-expected = ':' bekleniyordu
json-reason-expected-or-a-closing-bracket = ',' veya kapanış ayracı bekleniyordu
json-reason-expected-a-value = değer bekleniyordu
json-reason-empty-document = boş belge
json-reason-unexpected-end-of-input = beklenmeyen giriş sonu
json-reason-too-deeply-nested-to-format = biçimlendirilemeyecek kadar derin iç içe
tools-compare = Şununla Karşılaştır…
compare-need-two = Karşılaştırmak için başka bir sekme açın.
compare-pick-title = { $name } şununla karşılaştır
compare-count = { $count ->
    [0] Fark yok
    [one] 1 değişiklik
   *[other] { $count } değişiklik
    }
compare-close = Karşılaştırmayı Kapat
compare-to-left = Bu değişikliği sola kopyala
compare-to-right = Bu değişikliği sağa kopyala
compare-stale = Karşılaştırma sırasında bir belge değişti. Karşılaştırma yenilendi; yeniden deneyin.
view-structured = Yapılandırılmış Görünüm (CSV, JSON, XML, YAML)
csv-delimiter = Ayırıcı
csv-rows = { $count ->
    [one] 1 satır
   *[other] { $count } satır
    }
csv-unterminated = { $line }. satırda başlayan tırnak kapatılmamış
csv-truncated = İlk 200.000 satır gösteriliyor
csv-edit-cell = Hücreyi düzenle
csv-insert-row-below = Alta satır ekle
csv-delete-row = Satırı sil
csv-read-only = Salt okunur dosya: tablo düzenlenemez
json-more = … { $count } tane daha
dialog-open-link-title = Bu bağlantı web tarayıcınızda açılsın mı?
button-open = Aç
link-blocked = Güvenlik nedeniyle bu tür bağlantılar açılmaz.
link-not-found = Bağlantılı dosya bulunamadı: { $target }
enc-windows-1252 = Batı Avrupa
enc-iso-8859-15 = Batı Avrupa
enc-windows-1250 = Orta Avrupa
enc-iso-8859-2 = Orta Avrupa
enc-windows-1251 = Kiril
enc-koi8-r = Kiril
enc-windows-1253 = Yunanca
enc-windows-1254 = Türkçe
enc-windows-1255 = İbranice
enc-windows-1256 = Arapça
enc-windows-1257 = Baltık
enc-windows-874 = Tayca
enc-shift_jis = Japonca
enc-euc-jp = Japonca
enc-gb18030 = Çince, Basitleştirilmiş
enc-big5 = Çince, Geleneksel
enc-euc-kr = Korece
file-reopen-encoding = Kodlamayla Yeniden Aç…
file-save-encoding = Kodlamayla Kaydet…
encoding-reopen-title = Kodlamayla Yeniden Aç
encoding-save-title = Kodlamayla Kaydet
encoding-not-exact = Bu kodlama dosyayı değiştirmeden açamaz: { $reason }
encoding-unencodable = “{ $character }” karakteri { $encoding } olarak kaydedilemez. Başka bir kodlama seçin veya metni değiştirin; hiçbir şey yazılmadı.
encoding-save-first = Başka bir kodlamayla yeniden açmadan önce değişikliklerinizi kaydedin veya geri alın.
image-remote = Uzak görüntüler yüklenmez.
image-blocked = Yalnızca bu belgenin klasöründeki görüntüler gösterilir.
image-unsupported = Desteklenmeyen görüntü biçimi.
image-too-large = Görüntü gösterilemeyecek kadar büyük.
image-unreadable = Görüntü okunamadı.
settings-theme-high-contrast = Yüksek karşıtlık
view-fold = Daralt
view-unfold = Genişlet
view-unfold-all = Tümünü Genişlet
settings-shortcuts = Klavye kısayolları
settings-keymap-preset = Hazır ayar
settings-keymap-default = Sanain
settings-keymap-notepadpp = Notepad++
settings-shortcut-change = Değiştir
settings-shortcut-reset = Sıfırla
settings-shortcut-press = Bir kısayola basın… (Esc iptal eder)
settings-shortcut-moved = Kısayol şuradan taşındı:
edit-find-in-folder = Klasörde Bul…
folder-search-placeholder = Klasörde bul
folder-search-run = Ara
folder-search-stop = Durdur
folder-search-no-folder = Klasör seç
folder-search-summary = { $count ->
    [one] 1 eşleşme
   *[other] { $count } eşleşme
    }, { $files ->
    [one] 1 dosyada
   *[other] { $files } dosyada
    }
app-check-updates = Güncellemeleri Denetle…
update-checking = Güncellemeler denetleniyor…
update-up-to-date = Sanain { $version } en yeni sürümdür.
update-no-releases = Henüz yayımlanmış bir Sanain sürümü yok.
update-failed = Güncellemeler denetlenemedi: { $reason }
update-invalid = beklenmeyen yanıt
update-available-title = Sanain { $version } kullanılabilir
update-available-body = Edinmek için indirme sayfasını açın. Sanain kendiliğinden hiçbir şey indirmez veya yüklemez.
update-open-page = İndirme Sayfasını Aç
update-later = Daha Sonra
settings-check-updates = Güncellemeleri günde bir kez denetle
settings-check-updates-detail = En son sürümü github.com'a sorar. Başka hiçbir şey gönderilmez.
file-local-history = Yerel Geçmiş…
file-clear-history = Yerel Geçmişi Temizle…
history-title = { $name } dosyasının yerel geçmişi
history-open = Aç
history-restore = Geri Yükle
history-empty = Henüz kaydedilmiş sürüm yok. Her kaydedişinizde bir kopya saklanır.
history-unreadable = Bu sürüm okunamadı.
history-just-now = Az önce
history-minutes-ago = { $count ->
    [one] 1 dakika önce
   *[other] { $count } dakika önce
    }
history-hours-ago = { $count ->
    [one] 1 saat önce
   *[other] { $count } saat önce
    }
history-days-ago = { $count ->
    [one] 1 gün önce
   *[other] { $count } gün önce
    }
history-purge-title = Tüm yerel geçmiş silinsin mi?
history-purge-body = Tüm dosyaların kaydedilmiş kopyaları bu bilgisayardan kaldırılır. Dosyalarınızın kendisi değiştirilmez.
history-purge-confirm = Geçmişi Sil
history-purged = Yerel geçmiş silindi.
settings-local-history = Kaydedilen dosyaların yerel geçmişini sakla
settings-local-history-detail = Dosya başına en fazla 50 sürümü 30 gün boyunca yalnızca bu bilgisayarda saklar.
file-open-log = Günlük Görüntüleyicide Aç…
log-lines = { $count ->
    [one] 1 satır
   *[other] { $count } satır
    }
log-indexing = Okunuyor… %{ $percent }
log-matches = { $count ->
    [one] 1 eşleşen satır
   *[other] { $count } eşleşen satır
    }
log-filter-hint = Satırları filtrele
log-bad-pattern = Geçersiz desen
log-follow = Takip Et
log-close = Görüntüleyiciyi Kapat
log-opened-large = { $name } büyük olduğu için günlük görüntüleyicide salt okunur olarak açıldı.
tab-pin = Sekmeyi Sabitle
tab-unpin = Sekmenin Sabitlemesini Kaldır
edit-add-cursor-above = Yukarıya İmleç Ekle
edit-add-cursor-below = Aşağıya İmleç Ekle
view-word-wrap = Sözcük Kaydırma
status-wrap-on = Uzun satırlar artık pencere genişliğinde kaydırılıyor.
status-wrap-off = Uzun satırlar artık kaydırılmıyor.
settings-word-wrap = Uzun satırları kaydır
edit-go-to-line = Satıra Git…
edit-go-to-heading = Başlığa Git…
goto-line-hint = Satır veya satır:sütun
goto-line-note = { $count ->
    [one] Belgede 1 satır var.
   *[other] Belgede { $count } satır var.
    }
goto-line-invalid = Bir satır numarası girin; isteğe bağlı olarak ardından :sütun ekleyin.
goto-heading-hint = Başlıkları filtrelemek için yazın
goto-heading-none = Bu belgede başlık yok.
format-xml = XML
format-yaml = YAML
tools-tree-error = Satır { $line }, sütun { $column } konumunda geçersiz { $format }: { $reason }
edit-select-next-occurrence = Sonraki Eşleşmeyi Seç
tools-record-macro = Makro Kaydını Başlat veya Durdur
tools-play-macro = Makroyu Oynat
macro-recording = Makro kaydediliyor
macro-recorded = { $count ->
    [one] Makro kaydedildi: 1 adım.
   *[other] Makro kaydedildi: { $count } adım.
    }
tools-compare-saved = Kaydedilmiş Sürümle Karşılaştır
tools-compare-clipboard = Panoyla Karşılaştır
compare-saved-label = { $name } (diskte)
compare-clipboard-label = Pano
compare-saved-failed = Kaydedilmiş dosya okunamadı: { $reason }
compare-clipboard-empty = Panoda karşılaştırılacak metin yok.
error-save-disk-full = { $name } kaydedilmedi: disk dolu. Diskteki dosya değişmedi ve metniniz hâlâ burada. Biraz yer açın veya başka bir yere kaydetmek için Farklı Kaydet'i kullanın.
error-save-permission = { $name } kaydedilmedi: oraya yazma izniniz yok. Diskteki dosya değişmedi. Başka bir yere kaydetmek için Farklı Kaydet'i kullanın.
error-save-read-only-volume = { $name } kaydedilmedi: disk salt okunur. Diskteki dosya değişmedi. Başka bir yere kaydetmek için Farklı Kaydet'i kullanın.
replace-preview-title = { $count ->
    [one] 1 eşleşme değiştirilsin mi?
   *[other] { $count } eşleşme değiştirilsin mi?
    }
replace-preview-confirm = { $count ->
    [one] 1 tanesini değiştir
   *[other] { $count } tanesini değiştir
    }
replace-preview-more = Yalnızca değişen ilk 200 satır gösteriliyor.
replace-preview-stale = Belge değişti; hiçbir şey değiştirilmedi. Tümünü Değiştir'i yeniden çalıştırın.
encoding-detected = { $encoding } olarak açıldı (dosya içeriğinden algılandı).
encoding-suggested = Dosya içeriğine göre önerilen: { $encoding }. Açmadan önce önizlemeyi kontrol edin.
app-about = Sanain Hakkında
menu-help = Yardım
about-tagline = Kelimelerin gücü.
about-version = Sürüm { $version }
about-license = Mozilla Public License 2.0 kapsamında özgür ve açık kaynaklıdır.
about-ai = Yapay zekâ yaptı. İnsanlar yönetiyor. Topluluk geliştiriyor.
about-notices = Üçüncü taraf bileşenler ve lisansları, her sürüme dahil olan THIRD_PARTY_NOTICES dosyasında listelenmiştir.
about-source = GitHub’da kaynak kodu ve sorunlar
tools-xml-validate = XML'i Doğrula
tools-yaml-validate = YAML'ı Doğrula
tools-markup-valid = { $format } geçerli
file-open-recent = Son Kullanılanı Aç…
file-save-all = Tümünü Kaydet
file-revert = Kaydedilmiş Sürüme Döndür
recent-hint = Son kullanılan dosyaları süzmek için yazın
recent-none = Henüz son kullanılan dosya yok.
revert-title = { $name } dosyasındaki değişiklikler atılsın mı?
revert-body = Dosya diskten yeniden yüklenir. Kaydedilmemiş değişiklikler kaybolur ve geri alınamaz.
revert-confirm = At ve Yeniden Yükle
revert-done = { $name } diskten yeniden yüklendi.
status-saved-all = { $count ->
    [0] Kaydedilecek bir şey yok.
    [one] 1 dosya kaydedildi.
   *[other] { $count } dosya kaydedildi.
    }
edit-toggle-comment = Satır Yorumunu Aç/Kapat
edit-indent = Satırları Girintile
edit-outdent = Satır Girintisini Azalt
comment-unsupported = Bu dosya türünde satır yorumu yok.
tab-close-others = Diğer Sekmeleri Kapat
tab-close-all = Tüm Sekmeleri Kapat
file-copy-path = Dosya Yolunu Kopyala
file-reveal-macos = Finder’da Göster
file-reveal-windows = Dosya Gezgini’nde Göster
file-reveal-other = Dosya Yöneticisinde Göster
status-path-copied = Dosya yolu kopyalandı.
tabs-kept-unsaved = { $count ->
    [one] Kaydedilmemiş değişiklikleri olan 1 sekme açık bırakıldı.
   *[other] Kaydedilmemiş değişiklikleri olan { $count } sekme açık bırakıldı.
    }
status-indent-tabs = Sekmeler
status-indent-spaces = Boşluklar: { $count }
find-searching = Aranıyor…
compare-ignore-whitespace = Boşlukları yok say
tools-compare-file = Dosyayla Karşılaştır…
compare-file-failed = { $name } ile karşılaştırılamadı: { $reason }
cli-help =
    Kullanım:
      sanain [DOSYA…]          dosyaları aç
      sanain --diff SOL SAĞ    iki dosyayı yan yana karşılaştır
      sanain --version         sürümü göster
      sanain --help            bu yardımı göster
    Tire ile başlayan dosya adlarından önce -- kullanın.
cli-unknown-option = Bilinmeyen seçenek { $option }.
cli-diff-needs-two = --diff iki dosya gerektirir: sanain --diff SOL SAĞ
window-move-tab-left = Sekmeyi Sola Taşı
window-move-tab-right = Sekmeyi Sağa Taşı
edit-skip-occurrence = Sonraki Eşleşmeye Atla
tools-play-macro-times = Makroyu Birden Çok Kez Oynat…
macro-times-hint = Tekrar sayısı
macro-times-note = 1 ile { $max } arası
macro-times-invalid = 1 ile { $max } arasında bir sayı girin.
view-outline = Ana Hat
outline-empty = Bu belgede başlık yok.
outline-close = Ana Hattı Kapat
folder-replace-placeholder = Şununla değiştir
folder-replace-run = Dosyalarda Değiştir…
replace-files-title = { $files ->
    [one] 1 dosyadaki
   *[other] { $files } dosyadaki
    } { $count ->
    [one] 1 eşleşme
   *[other] { $count } eşleşme
    } değiştirilsin mi?
replace-files-body = Bir sekmede açık olan dosyalar orada değiştirilir ve değişiklik geri alınabilir; bunları kendiniz kaydedin. Diğer dosyalar şimdi kaydedilir ve önceki içerikleri, etkinse yerel geçmişte saklanır. Bu sırada diskte değişen bir dosya atlanır.
replace-files-confirm = Değiştir
replace-files-done = { $files ->
    [one] 1 dosyada
   *[other] { $files } dosyada
    } { $count ->
    [one] 1 eşleşme
   *[other] { $count } eşleşme
    } değiştirildi (sekmelerde açık: { $open }, kaydedilmedi).
replace-files-failed = { $failed ->
    [one] 1 dosya değiştirilmedi:
   *[other] { $failed } dosya değiştirilmedi; ilki:
    } { $first }
image-outside = Bu görüntü belgenin klasörünün dışında.
image-allow = Bu klasördeki görüntüleri göster
cli-help-integration =
    Dosya yöneticisi tümleştirmesi (yalnızca geçerli kullanıcı):
      sanain --register           Sanain'i "Birlikte Aç" menüsüne ekle (Linux, Windows)
      sanain --unregister         yeniden kaldır
cli-registered = Sanain "Birlikte Aç" menüsüne eklendi. { $place }
cli-unregistered = Sanain "Birlikte Aç" menüsünden kaldırıldı.
cli-register-failed = Dosya yöneticisi tümleştirmesi değiştirilemedi: { $reason }
cli-register-macos = macOS'ta hiçbir şeyin kaydedilmesi gerekmez: Sanain.app Uygulamalar klasörüne konduğunda Finder onu "Birlikte Aç" menüsünde sunar.
settings-tab-general = Genel
settings-tab-editor = Düzenleyici
settings-tab-appearance = Görünüm
settings-shortcut-search = Komut veya tuş ara
settings-font = Yazı tipi
settings-font-default = Yerleşik eş aralıklı
settings-font-loading = Yüklü yazı tipleri aranıyor…
settings-font-missing = { $font } yazı tipi bulunamadı; yerleşik yazı tipi kullanılıyor.
settings-theme-more = Diğer temalar
settings-theme-user-note = Kendi temalarınız: { $folder } klasöründeki .theme dosyaları (bkz. docs/THEMES.md).
language-auto = Otomatik Algıla
language-choose = Bu sekmenin dilini seçin
tools-xml-format = XML'i Biçimlendir
tools-save-macro = Makroyu Kaydet…
tools-play-saved-macro = Kayıtlı Makroyu Oynat…
tools-delete-saved-macro = Kayıtlı Makroyu Sil…
macro-save-hint = Makro adı
macro-save-note = Aynı adlı kayıtlı makro değiştirilir.
macro-save-empty = Kaydedilecek bir şey yok: önce bir makro kaydedin.
macro-saved = Makro kaydedildi: { $name }
macro-deleted = Makro silindi: { $name }
macro-name-invalid = Tek satırda 1 ile { $max } karakter arasında bir ad girin.
macro-save-too-large = Bu makro kaydedilemeyecek kadar uzun.
macro-save-full = En fazla { $max } makro kaydedilebilir. Önce birini silin.
macro-save-failed = Makrolar kaydedilemedi: { $error }
macro-saved-hint = Kayıtlı makroları süzmek için yazın
macro-saved-none = Henüz kayıtlı makro yok.
settings-shortcut-import-npp = Notepad++'tan içe aktar…
settings-shortcut-import-npp-hint = Notepad++ shortcuts.xml dosyasını seçin. Windows'ta %AppData%\Notepad++\shortcuts.xml konumundadır.
settings-shortcut-import-npp-done = Notepad++'tan içe aktarılan: { $imported }. Atlanan: { $skipped }. Kalan çakışma: { $conflicts }.
settings-shortcut-import-npp-failed = { $name } içe aktarılamadı: { $reason }
settings-shortcut-import-npp-too-large = Dosya 1 MiB'den büyük.
settings-shortcut-import-npp-invalid = Dosya geçerli bir Notepad++ shortcuts.xml dosyası değil.
view-split-right = Sağa Böl
view-split-down = Aşağı Böl
view-close-pane = Bölmeyi Kapat
view-focus-next-pane = Sonraki Bölmeye Odaklan
view-focus-prev-pane = Önceki Bölmeye Odaklan
status-pane-limit = Bölme sınırına ulaşıldı (en fazla { $max }).
