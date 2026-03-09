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
    for pilot in pilots {
        let racer = F1Racer::new(
            String::from(pilot),
            number_of_laps,
        );
        racers.push(racer);
    }
    for racer in racers {
        let name = &racer.name.clone();
        let best_time = racer.await;
        println!("Best time lap of {} was {}.", name, best_time);
    }
}
