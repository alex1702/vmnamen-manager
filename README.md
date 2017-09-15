# vmnamen-manager

[![Build Status](https://travis-ci.org/alex1702/vmnamen-manager.svg?branch=master)](https://travis-ci.org/alex1702/vmnamen-manager)

Ein kleines Programm um die VMNamen per csv Datei zu verwalten.



Das Skript verwaltet die VM-Namen die in der csv Datei sind.
Funktionen:
- hinzufügen einer neuen VM solange ein hostname frei ist (es wird der nächst freie Hostname verwendet)
- löschen einer VM (hostname wird auf frei verfügbar gesetzt)
- ändern der Beschreibung und IP einer VM
- speichern der Datei

Es wird Rust und Corgo gebraucht um das Projekt zu bauen. Zum ausführen des Programes werden keine weiteren Abhängigkeiten benötigt.

## Bauen

Zum bauen einfach 
```bash
cargo build --release
```
ausführen. Die ausführbare Datei landet dann in *target/release* .

## Ausführen

Die ausführbare Datei nimmt ein Argument entgegen und dass ist die csv Datei inklusive Pfad.

```bash
./vmnamen-manager test.csv
```

Die Datei wird nicht erstellt und muss vorher vorhanden sein.
Sie muss alle verfügbaren Hostnamen schon enthalten

Beispiel: (siehe auch test.csv)
```csv
srv1;Mailserver;127.0.0.1
srv2;Webserver;127.0.0.2
testvm;Test Maschine;127.0.0.5
srv3;;
srv4;;
```

## Debugen

Zum bauen der debug version einfach

```bash
cargo build
```
ausführen. Die ausführbare Datei landet dann in *target/debug* .

## Direktes ausführen

Zum ausführen der debug version mit cargo einfach

```bash
cargo run test.csv
```
ausführen. Die csv-Datei muss direkt mit angegeben werden.




