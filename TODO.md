# Tehtävät

## 1. Pelitilan nollaaminen (Helppo)

Ohjelman main-funktiosta löytyvässä main loopissa tiedustellaan moottorilta käyttäjän syötteitä. Syötteet tallennetaan muuttujaan *engine_event: EngineEvent* ja match-blokissa määritellään toiminnot EngineEvent-enumin variantin mukaan. Tehtävän olet suorittanut onnistuneesti, kun välilyönnin painaminen aiheuttaa peliruudukon tyhjenemisen.

- [ ] Lisää rivi tiedossa *engine.rs* olevaan funktioon *pub fn poll(&mut self) -> EngineEvent*, joka saa funktion palauttamaan enum-variantin *EngineEvent::Clear* käyttäjän painaessa välilyöntiä. Tämän tehtyäsi Problems-paneelista katoaa varoitus *Event 'Clear' is never constructed*
- [ ] Lisää pelin main-looppiin rivi, joka vastaa pelitilan nollaamisesta poll-funktion palauttaessa Clear-komento. Ainekset tämän suorittamiseen löydät tutkimalla main-looppia.
