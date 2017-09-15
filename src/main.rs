
extern crate csv;
extern crate serde;


use std::io;
use std::io::Write;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::process;

#[macro_use] extern crate prettytable;
use prettytable::Table;
use prettytable::row::Row;
use prettytable::cell::Cell;
use prettytable::format;

extern crate term;

use serde::ser::{Serialize, Serializer, SerializeStruct};

#[derive(Debug, Clone)]
struct Vm {
    hostname: String,
    beschreibung: String,
    ip: String,
    frei: bool,
}


impl Serialize for Vm {
    // Eigene serialize Funktion, damit das Feld 'frei' nicht mit in der csv landet.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut state = serializer.serialize_struct("Vm", 3)?;
        state.serialize_field("hostname", &self.hostname)?;
        state.serialize_field("beschreibung", &self.beschreibung)?;
        state.serialize_field("ip", &self.ip)?;
        state.end()
    }
}

fn generiere_tabelle(vms: &Vec<Vm>) {
    let mut table = Table::new();
    let mut i = 0;
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    
    table.set_titles(row![b => "ID", "Hostname", "Beschreibung", "IP"]); //Kopfzeile
    for vm in vms {
        if !vm.frei {
            table.add_row(Row::new(vec![
            Cell::new(&format!("{}", i)),
            Cell::new(&vm.hostname),
            Cell::new(&vm.beschreibung),
            Cell::new(&vm.ip)]));
        }
        i += 1;
    }
    
    table.printstd();
}

fn lade_csv(file_path: OsString) -> Result<Vec<Vm>, Box<Error>> {
    let mut vms = Vec::new();
    
    let file = File::open(file_path)?; // Datei lesend öffnen
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false) // Es gibt keine Kopfzeile
        .delimiter(b';') // Simikolon ist das Trennzeichen
        .comment(Some(b'#')) // Ignoriere Zeilen mit einem '#' am Anfang
        .from_reader(file);
        
    for result in rdr.records() {
        let record = result?;
        
        let hostname = &record[0];
        let beschreibung = &record[1];
        let ip = &record[2];
        
        let vm = Vm {
            hostname: hostname.to_string(),
            beschreibung: beschreibung.to_string(),
            ip: ip.to_string(),
            frei: match beschreibung.as_ref() {
                "" => true,
                _  => false,
            },
        };
        vms.push(vm);  
    }
    
    Ok(vms)
}

fn speichere_csv(file_path: OsString, vms: &Vec<Vm>) -> Result<(), Box<Error>> {
    let file = File::create(&file_path)?; // Datei schreibend öffnen
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(false)
        .delimiter(b';')
        .from_writer(&file);

    for vm in vms {
        wtr.serialize(vm)?;
    }

    wtr.flush()?;
    Ok(())
}

fn abfrage_id(maxid: i32) -> Option<i32> {
    io::stdout().flush().unwrap();
    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).expect("failed to read line");

    return match input_text.trim().parse::<i32>() {
        Ok(id) => {
            if id >= maxid {
                println!("Bitte gib eine gültige ID an.");
                return None;
            }
            return Some(id);
        },
        _ => {
            println!("Bitte gib eine gültige ID an.");
            None
        }
    };
}

fn abfrage_entscheidung(entscheidung: &mut String) {
    *entscheidung = String::new();
    print!("Was möchten Sie tun? (c = hinzufügen, d = löschen, e = ändern, w = speichern und beenden, q = beenden ohne zu speichern) ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(entscheidung).expect("Failed to read line");
    entscheidung.truncate(1); //String auf ein Zeichen begrenzen
}


fn finde_freie_vm(vms: &Vec<Vm>, id: &mut i32) -> bool {
    *id = 0;
    for vm in vms {
        if vm.frei {
            break;
        }
        *id += 1;
    }
    if *id == (vms.len() as i32) {
        return false;
    }
    println!("Noch verfügbare Hostnamen nach dem erstellen {}.", (vms.len() as i32 - (*id + 1)));
    true
}

fn run() -> Result<(), Box<Error>> {
    let reader = io::stdin();
    let mut entscheidung = String::new();
    
    let mut vms = match lade_csv(get_first_arg()?) {
        Ok(vms) => vms,
        Err(e) => {
            println!("FEHLER: {}", e);
            Vec::new()
        }
    };
    
    generiere_tabelle(&vms);
    
    abfrage_entscheidung(&mut entscheidung);
    while entscheidung != "q" {
        match entscheidung.as_ref() {
            "c" => { // Erstelle neue VM zum nächst freien Hostnamen
                let mut freie_id = 0;
                
                if !finde_freie_vm(&vms, &mut freie_id) {
                    println!("Kein freier Hostname mehr verfügbar.");
                    abfrage_entscheidung(&mut entscheidung);
                    continue;
                }
                
                println!("Hostname der neuen VM: {}", vms[freie_id as usize].hostname);
                
                let mut beschreibung = String::new();
                let mut ip = String::new();
                
                print!("Beschreibung: ");
                io::stdout().flush().unwrap();
                reader.read_line(&mut beschreibung).expect("failed to read line");
                beschreibung.pop();
                if beschreibung != "" {
                    vms[freie_id as usize].beschreibung = beschreibung;
                }
                
                print!("IP: ");
                io::stdout().flush().unwrap();
                reader.read_line(&mut ip).expect("failed to read line");
                ip.pop();
                if ip != "" {
                    vms[freie_id as usize].ip = ip;
                }
                
                vms[freie_id as usize].frei = false;
                
            },
            "d" => { // Lösche VM mit der ID
                print!("Welche VM möchten Sie löschen? (Angabe der ID) ");
                match abfrage_id(vms.len() as i32) {
                    Some(id) => {
                        vms[id as usize].beschreibung = "".to_string();
                        vms[id as usize].ip = "".to_string();
                        vms[id as usize].frei = true;
                    }
                    _ => continue,
                }
            },
            "e" => { // Editiere Vm mit der ID
                print!("Welche VM möchten Sie editieren? (Angabe der ID) ");
                match abfrage_id(vms.len() as i32) {
                    Some(id) => {
                        println!("Bearbeite Vm {} :", vms[id as usize].hostname);
                        
                        let mut beschreibung = String::new();
                        let mut ip = String::new();
                        
                        print!("Beschreibung[{}] (Drücke Enter um die Beschreibung nicht zu ändern.) ", vms[id as usize].beschreibung);
                        io::stdout().flush().unwrap();
                        reader.read_line(&mut beschreibung).expect("failed to read line");
                        beschreibung.pop();
                        if beschreibung != "" {
                            vms[id as usize].beschreibung = beschreibung;
                        }
                        
                        print!("IP[{}] (Drücke Enter um die IP nicht zu ändern.) ", vms[id as usize].ip);
                        io::stdout().flush().unwrap();
                        reader.read_line(&mut ip).expect("failed to read line");
                        ip.pop();
                        if ip != "" {
                            vms[id as usize].ip = ip;
                        }
                    }
                    _ => {}
                }
            },
            "w" => { // Schreibe csv Datei
                print!("Speichere Datei ab...");
                io::stdout().flush().unwrap();
                // Speichere Datei wieder ab.
                match speichere_csv(get_first_arg()?, &vms) {
                    Ok(_) => println!("ok!"),
                    Err(e) => println!("failed! {}", e),
                }
                
                return Ok(());
            }
            "q" => return Ok(()),
            _ => println!("Unbekannter Befehl."),
        }
        
        generiere_tabelle(&vms); // Drucke die Tabelle neu auf der Konsole
        abfrage_entscheidung(&mut entscheidung);
    }

    Ok(())
}

/// Returns the first positional argument sent to this process. If there are no
/// positional arguments, then this returns an error.
fn get_first_arg() -> Result<OsString, Box<Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("Zu verwendene CSV Datei angeben.")),
        Some(file_path) => Ok(file_path),
    }
}

fn main() {
    
    println!("VM-Namen Manager v0.1.0");
    println!("=======================");
    println!();
    
    if let Err(err) = run() {
        println!("{:?}", err);
        process::exit(1);
    }
}
