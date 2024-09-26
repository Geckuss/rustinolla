# Ympäristön valmistelu

## Rust ja SDL2

- **Lataa ja asenna Rust [täältä](https://www.rust-lang.org/tools/install)**
- **Lataa käyttöjärjestelmällesi sopiva SDL2 [täältä](https://github.com/libsdl-org/SDL/releases/tag/release-2.30.6)**

### Windows - 'SDL2-devel-2.30.6-VC.zip'

- **Kopioi** kansion *SDL2-devel-2.30.6-VC\SDL2-2.30.6\lib\x64* sisältö (neljä tiedostoa) kohteeseen *C:\Users\omanimi\\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib*

### Mac - 'SDL2-2.30.6.dmg'

- **Avaa** lataamasi tiedosto, ja **kopioi** 'SDL2.framework' kansioon 'Macintosh HD/Library/Frameworks/'

- **Muokkaa** tiedosto *Cargo.toml* muotoon:

        [package]
        ...
        build = "build.rs"

        ...

        [dependencies.sdl2]
        version = "*"
        features = ["use_mac_framework"]

- **Luo** projektin juurikansioon tiedosto *build.rs* ja **kopioi** tämä sinne:

        fn main() {
            println!("cargo:rustc-link-search=framework=/Library/Frameworks")
        }

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

Cargo on Rustin mukana tuleva työkalu, joka tekee projektien hallinnasta, kääntämisestä ja ajamisesta erittäin helppoa. Kun olet asentanut kaiken, voit ajaa tämän projektin terminaalista komennolla **cargo run**. Jatkossa ohjelman ajaminen onnistuu yksinkertaisesti avaamalla konsolin (Ctrl/Cmd + ö), painamalla ylänuolta ja enteriä.
