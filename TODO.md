# Tehtävät

## 1. Pelitilan nollaaminen: perusteet

Ohjelman main-funktiosta löytyvässä main loopissa pysähdytään odottamaan käyttäjän syötteitä. Syötteet tallennetaan muuttujaan *engine_event: EngineEvent* ja match-blokissa määritellään toiminnot EngineEvent-enumin variantin mukaan. Tehtävänäsi on lisätä peliin toiminto, jonka avulla peli voidaan aloittaa alusta ilman uudelleenkäynnistystä.

Tehtävän olet suorittanut onnistuneesti, kun välilyönnin painaminen aiheu/ttaa peliruudukon tyhjenemisen. Lisäksi problems-valikosta häviää varoitus *method 'reset' is never used.

1. Lisää rivi tiedostossa *engine.rs* olevaan metodiin *pub fn poll(&mut self) -> EngineEvent*, joka saa sen palauttamaan enum-variantin

        EngineEvent::Clear

    käyttäjän painaessa välilyöntiä.
2. Lisää pelin main-looppiin rivi, joka vastaa pelitilan nollaamisesta poll-funktion palauttaessa Clear-komennon. Voit käyttää tähän GameState:n metodia **reset()**;

## 2. Voittajan selvittäminen

### 2.1 Voiton tunnistaminen: algoritmi

Rustinolla tarvitsee kyvyn pelin voittajan selvittämiseen. Helpointa on määritellä pelitilaa mallintavalle **GameState** -structille metodi tätä varten. Tehtävän olet suorittanut onnistuneesti, kun pelin voittajan ollessa selvillä terminaaliin tulostuu voittajan nimi ("X voitti!" tai "O voitti!").

1. Toteuta tiedostosta *game_state.rs* löytyvä metodi *pub fn has_a_winner(&self) -> Option&lt;Player&gt;*. Metodi palauttaa ns. Option-tyypin, joka on Rustissa käytetty vaihtoehto NULL-arvolle. Kun voittaja on selvillä, metodi palauttaa:

        Some(Player::X) tai Some(Player::O)

    Mikäli voittaja ei vielä ole selvillä, palauttaa metodi yksinkertaisesti arvon:

        None

### 2.2 Voittavan rivin merkintä: perusteet

Kun voittaja on saatu selville, olisi sen merkitseminen peli-ikkunaan suotavaa. Tätä varten tiedostossa *game_state.rs* on olemassa valmis funktio fn draw_bar(..), joka piirtää viivan annetun alku- ja loppupisteen välille. Pisteet ovat ruutujen indeksejä, joten jos voittava rivi olisi esimerkiksi vasemmasta yläkulmasta oikeaan alakulmaan, funktion syötteen tulisi olla:

        draw_bar((0, 0), (2, 2), canvas)

1. Keksi keino hankkia tieto näistä indekseistä ja piirrä keltainen viiva niiden mukaan.

## 3. Oman kuvion piirtäminen: renderöinti

Pohjaprojektissa pelaajat piirretään erivärisinä palloina ja tähän pitäisi saada muutos. Tiedoston *game_state.rs* osiosta *Apufunktiot* löytyy funktio *draw_circle( .. ) -> Result<..>, joka huolehtii ympyröiden piirtämisestä. Funktio ottaa syötteinä pallon keskikohdan, säteen ja kanvaasin, jolle pallo on tarkoitus piirtää. Kanvaasi on koko ikkunan kokoinen ja sen koordinaatisto näyttää tältä:

        (0, 0) ————————————————> x
            |                  (WIDTH, 0)
            |
            |
            |
            |
            |
            |
            |
            V (0, HEIGHT)      (WIDTH, HEIGHT)
            y

1. Keksi oma toteutus funktiolle *draw_x( .. )*. Päätä itse, millaisen kuvion haluan piirtää ympyrän kaveriksi. Voit kokeilla esim:

- Laatikko (helppo)
- Pystyssä seisova risti (keskivaikea)
- X-kirjain (vaikea)
- Yksisarvinen (legendaarinen)

Yksittäisten pikselien värittäminen tapahtuu metodilla *canvas.draw_point(point: P)*, mutta käytössäsi on muitakin metodeja, joista voi olla hyötyä:

- draw_line(P1: P, P2: P) - piirtää viivan annettujen pisteiden välille.
- draw_rect(rect: Rect) - piirtää nelikulmion ääriviivat.
- fill_rect(rect: Rect) - piirtää ja värittää nelikulmion.

## 4. Ympyrän parantelu: renderöinti

Tällä hetkellä ympyrä piirtyy sinisenä pallona. Ideaalisesti haluaisimme piirtää O-kirjaimen. Voimme määritellä tiedostossa *game_state.rs" olevaan funktioon *draw_circle( .. )* minimietäisyyden, jota lähempänä olevat pikselit jätetään värittämättä. Tehtävä on onnistunut, kun ympyrän sijasta O:n vuorolla ruudukkoon ilmestyy O-kirjain.

1. Muokkaa funktiota *draw_circle( .. )* piirtämään ympyrän sijasta O-kirjain.
