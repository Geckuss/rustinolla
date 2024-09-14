# Ympäristön valmistelu

## Rust ja SDL2

- **Lataa ja asenna Rust [täältä](https://www.rust-lang.org/tools/install)**
- **Lataa käyttöjärjestelmällesi sopiva SDL2 [täältä](https://github.com/libsdl-org/SDL/releases/tag/release-2.30.6)**

### Windows - 'SDL2-devel-2.30.6-VC.zip'

- **Kopioi** kansion *SDL2-devel-2.30.6-VC\SDL2-2.30.6\lib\x64* sisältö (neljä tiedostoa) kohteeseen *C:\Users\omanimi\\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib*

### Mac

todo: SDL2 asennusohjeet Macille (eroaako applen ja intelin prossuilla?)

## Visual Studio Code

- **Asenna** rust-analyzer Extensions-valikosta

### Vapaaehtoisia (mutta suositeltuja) workflow-ohjeita

#### Lisää terminaali ja *Problems*-paneeli pikanäppäimiin

1. Avaa vasemmasta yläkulmasta: File &rarr; Preferences &rarr; Keyboard Shortcuts.
2. Etsi hakukentästä asetus "View: Toggle Problems"
3. Aseta pikanäppäimeksi **Ctrl/Cmd + ä**.
4. Toista sama asetukselle "View: Toggle Terminal" ja aseta pikanäppäimeksi  **Ctrl/Cmd + ö**.

#### Siirrä terminaali ja *Problems*-paneeli näytön oikeaan reunaan

## Cargo

Cargo on Rustin mukana tuleva työkalu, joka tekee projektien hallinan, kääntämisen ja suorittamisen erittäin helpoksi. Kun olet asentanut kaiken, voit ajaa tämän projektin terminaalista komennolla **cargo run**.
