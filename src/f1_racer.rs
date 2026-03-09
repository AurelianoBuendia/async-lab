#![allow(dead_code)]

use rand::prelude::*;
use std::future::Future;
use std::pin::Pin;
use tokio::time::Duration;

pub struct F1Racer {
    pub name: String,
    pub completed_laps: usize,
    pub number_of_laps: usize,
    pub best_lap_time: u8,
    pub lap_times: Vec<u8>,
    pub race_state: RaceState,
}

pub enum RaceState {
    Start,
    Running(Pin<Box<dyn Future<Output = ()>>>),
    Finished,
}

impl F1Racer {
    pub fn new(name: String, number_of_laps: usize) -> Self {
        F1Racer {
            name,
            completed_laps: 0,
            number_of_laps,
            best_lap_time: 255,
            lap_times: vec![255; number_of_laps],
            race_state: RaceState::Start,
        }
    }
}

pub struct F1RacerFuture {
    race_state: RaceState,
}

async fn do_lap(current_lap_time: u64) {
    tokio::time::sleep(Duration::from_millis(current_lap_time)).await;
}

impl std::future::Future for F1Racer {
    type Output = u8; // The future returns the best lap time for a given racer

    fn poll(mut self: std::pin::Pin<&mut Self>,
            cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        loop {
            match &mut self.race_state {
                RaceState::Start => {
                    let mut rng = rand::rng();
                    let current_lap_time = rng.random_range(10..255);
                    let completed_laps = self.completed_laps;
                    self.lap_times[completed_laps] = current_lap_time;
                    if current_lap_time < self.best_lap_time {
                        self.best_lap_time = current_lap_time;
                    }
                    println!("LAP {} completed with time {} by: {}",
                             self.completed_laps,
                             self.lap_times[self.completed_laps],
                             self.name);
                    self.completed_laps += 1;
                    if self.completed_laps > self.number_of_laps {
                        self.race_state = RaceState::Finished;
                        cx.waker().wake_by_ref();
                        return std::task::Poll::Pending;
                    }
                    let inner_future = Box::pin(do_lap(current_lap_time as u64));
                    self.race_state = RaceState::Running(inner_future);
                    cx.waker().wake_by_ref();
                    return std::task::Poll::Pending;
                }
                RaceState::Running(inner_future) => {
                    match inner_future.as_mut().poll(cx) {
                        std::task::Poll::Pending => {
                            cx.waker().wake_by_ref();
                            return std::task::Poll::Pending;
                        }
                        std::task::Poll::Ready(()) => {
                            if self.completed_laps >= self.number_of_laps {
                                self.race_state = RaceState::Finished;
                            } else {
                                self.race_state = RaceState::Start;
                            }
                            cx.waker().wake_by_ref();
                            return std::task::Poll::Pending;
                        }
                    }
                }
                RaceState::Finished => {
                    println!("Executed on thread with ID: {:?}.", std::thread::current().id());
                    return std::task::Poll::Ready(self.best_lap_time);
                }
            }
        }
    }
}
