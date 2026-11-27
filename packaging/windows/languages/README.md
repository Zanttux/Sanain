# Inno Setup translations

The installer's own texts in the 25 languages besides English that Text³'s
interface has (locales/). Copied unchanged from the Inno Setup source
repository, tag `is-6_7_1` (the compiler version on the CI runner):
https://github.com/jrsoftware/issrc/tree/is-6_7_1/Files/Languages
(Greek, Indonesian, Romanian, Vietnamese and both Chinese files are from
its `Unofficial` folder). Licensed under the Inno Setup License
(LICENSE-InnoSetup.txt); the translators are named in each file.

Kept in the repository so the installer languages do not depend on which
translations the installed compiler ships. Texts Text³ adds (task names)
are in `[CustomMessages]` of `../text3.iss`.
