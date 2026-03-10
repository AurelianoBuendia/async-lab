#![allow(unused_variables)]

mod f1_racer;

use crate::f1_racer::F1Racer;


#[tokio::main]
async fn main() {
    println!("I am alive!");
    let number_of_laps = 10;
    let pilots = vec![
        "Louis Hamilton",
        "Max Verstappen",
        "Gabriel Bortoleto",
        "Lando Norris",
        "Oscar Piastri",
        "Charles Leclerc",
        "Kimi Antonelli",
    ];
    let mut racers: Vec<F1Racer> = Vec::new();
    for pilot in &pilots {
        let racer = F1Racer::new(
            String::from(*pilot),
            number_of_laps,
        );
        racers.push(racer);
    }
    let mut handles: Vec<tokio::task::JoinHandle<u8>> = Vec::new();
    for racer in racers {
        let handle = tokio::task::spawn(racer);
        handles.push(handle);
    };
    let mut outputs: Vec<u8> = Vec::with_capacity(handles.len());
    for handle in handles {
        outputs.push(handle.await.unwrap());
    }
    for i in 0..pilots.len() {
        println!("Best lap time for {} was {}.", pilots[i], outputs[i]);
    }
}
