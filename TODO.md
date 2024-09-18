# Tehtävät

## 1. Pelitilan nollaaminen: perusteet

Ohjelman main-funktiosta löytyvässä main loopissa pysähdytään odottamaan käyttäjän syötteitä. Syötteet tallennetaan muuttujaan *engine_event: EngineEvent* ja match-blokissa määritellään toiminnot EngineEvent-enumin variantin mukaan. Tehtävänäsi on lisätä peliin toiminto, jonka avulla peli voidaan aloittaa alusta ilman uudelleenkäynnistystä.

Tehtävän olet suorittanut onnistuneesti, kun välilyönnin painaminen aiheu/ttaa peliruudukon tyhjenemisen.

1. Lisää rivi tiedostossa *engine.rs* olevaan metodiin *pub fn poll(&mut self) -> EngineEvent*, joka saa sen palauttamaan enum-variantin

        EngineEvent::Clear

    käyttäjän painaessa välilyöntiä.
2. Lisää pelin main-looppiin rivi, joka vastaa pelitilan nollaamisesta poll-funktion palauttaessa Clear-komennon. Voit käyttää tähän GameState:n metodia **reset()**;

## 2. Voittajan selvittäminen: algoritmi

Rustinolla tarvitsee kyvyn pelin voittajan selvittämiseen. Helpointa on määritellä pelitilaa mallintavalle **GameState** -structille metodi tätä varten. Tehtävän olet suorittanut onnistuneesti, kun pelin voittajan ollessa selvillä terminaaliin tulostuu voittajan nimi ("X voitti!" tai "O voitti!").

1. Toteuta tiedostosta *game_state.rs* löytyvä metodi *pub fn has_a_winner(&self) -> Option&lt;Player&gt;*

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