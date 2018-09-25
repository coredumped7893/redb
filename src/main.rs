extern crate rand;
extern crate chrono;

use std::time::SystemTime;

use rand::Rng;

use chrono::{NaiveDate, Datelike, NaiveDateTime};

const ELEMENTS: usize = 1000*1000*1000;
const MAX_PASSENGERS: usize = 10;

fn query1() {
    let elements = ELEMENTS / 64;
    let mut data: Vec<i64> = vec![0; elements];
    for i in 0..elements {
        data[i] = rand::thread_rng().gen();
    }

    let start = SystemTime::now();

    let ones = data.iter()
        .map(|x| x.count_ones())
        .fold(0, |sum, x| sum + x);

    let end = SystemTime::now();
    let duration = end.duration_since(start)
        .expect("Time went backwards");
    println!("Query 1 result: {}", ones);
    println!("Query 1 duration: {:.1}ms", duration.as_secs() as f32 * 1000.0 +
        (duration.subsec_nanos() as f32 / 1000.0 / 1000.0));
}

fn query2() {
    let mut num_passengers: Vec<u8> = vec![0; ELEMENTS];
    let mut total_fare: Vec<f32> = vec![0.0; ELEMENTS];
    for i in 0..ELEMENTS {
        num_passengers[i] = rand::thread_rng().gen_range(0, 10);
        total_fare[i] = rand::thread_rng().gen_range(1.0, 100.0);
    }

    let mut sums: [f32; MAX_PASSENGERS] = [0.0; MAX_PASSENGERS];
    let mut counts: [u32; MAX_PASSENGERS] = [0; MAX_PASSENGERS];

    let start = SystemTime::now();

    for i in 0..ELEMENTS {
        sums[num_passengers[i] as usize] += total_fare[i];
        counts[num_passengers[i] as usize] += 1;
    }

    let end = SystemTime::now();
    let duration = end.duration_since(start)
        .expect("Time went backwards");
    println!("Query 2 result:");
    for i in 0..MAX_PASSENGERS {
        println!("{}: {}", i, sums[i] / counts[i] as f32);
    }
    println!("Query 2 duration: {:?}ms", duration.as_secs() * 1000 + (duration.subsec_nanos() / 1000 / 1000) as u64);
}

fn query3() {
    let mut num_passengers: Vec<u8> = vec![0; ELEMENTS];
    let mut pickup_timestamp: Vec<NaiveDateTime> = vec![NaiveDate::from_ymd(2001, 1, 1).and_hms(1, 1, 1); ELEMENTS];
    for i in 0..ELEMENTS {
        num_passengers[i] = rand::thread_rng().gen_range(0, 10);
        pickup_timestamp[i] = NaiveDateTime::from_timestamp(rand::thread_rng().gen_range(1, (2018 - 1970)*365*24*60*60), 0);
    }

    let mut counts: [u32; MAX_PASSENGERS * (2018 - 1970)] = [0; MAX_PASSENGERS * (2018 - 1970)];

    let start = SystemTime::now();

    for i in 0..ELEMENTS {
        let passengers = num_passengers[i] as usize;
        let year = (pickup_timestamp[i].date().year() - 1970) as usize;
        counts[(MAX_PASSENGERS * year + passengers) as usize] += 1;
    }

    let end = SystemTime::now();
    let duration = end.duration_since(start)
        .expect("Time went backwards");
    println!("Query 3 (first 10) results:");
    for i in 0..10 {
        let passengers = i % MAX_PASSENGERS;
        let year = i / MAX_PASSENGERS + 1970;
        println!("{}, {}: {}", passengers, year, counts[i]);
    }
    println!("Query 3 duration: {:?}ms", duration.as_secs() * 1000 + (duration.subsec_nanos() / 1000 / 1000) as u64);
}

fn query4() {
    let mut num_passengers: Vec<u8> = vec![0; ELEMENTS];
    let mut pickup_timestamp: Vec<NaiveDateTime> = vec![NaiveDate::from_ymd(2001, 1, 1).and_hms(1, 1, 1); ELEMENTS];
    let mut trip_distance: Vec<f32> = vec![0.0; ELEMENTS];
    for i in 0..ELEMENTS {
        num_passengers[i] = rand::thread_rng().gen_range(0, 10);
        pickup_timestamp[i] = NaiveDateTime::from_timestamp(rand::thread_rng().gen_range(1, (2018 - 1970)*365*24*60*60), 0);
        trip_distance[i] = rand::thread_rng().gen_range(0.1, 100.0);
    }

    let mut counts: [u32; MAX_PASSENGERS * (2018 - 1970) * 100] = [0; MAX_PASSENGERS * (2018 - 1970) * 100];

    let start = SystemTime::now();

    for i in 0..ELEMENTS {
        let passengers = num_passengers[i] as usize;
        let year = (pickup_timestamp[i].date().year() - 1970) as usize;
        let distance = trip_distance[i] as usize;
        counts[(MAX_PASSENGERS * 100 * year + 100 * passengers + distance) as usize] += 1;
    }

    let end = SystemTime::now();
    let duration = end.duration_since(start)
        .expect("Time went backwards");
    println!("Query 4 (first 10) result:");
    for i in 0..10 {
        let distance = i % 100;
        let passengers = (i % MAX_PASSENGERS * 100) / 100;
        let year = i / (MAX_PASSENGERS * 100) + 1970;
        println!("{}, {}, {}: {}", passengers, year, distance, counts[i]);
    }
    println!("Query 4 duration: {:?}ms", duration.as_secs() * 1000 + (duration.subsec_nanos() / 1000 / 1000) as u64);
}

fn main() {
    query1();
    query2();
    query3();
    query4();
}
