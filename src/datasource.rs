// TODO parse example data, mock real time stream from device using sleeps

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Debug)]
struct Data {
    timestamp: u64,
    acc: AccData,
    mag: MagData,
}

#[derive(Debug)]
struct AccData {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug)]
struct MagData {
    x: f64,
    y: f64,
    z: f64,
}

impl From<&str> for Data {
    fn from(value: &str) -> Self {
        let mut x = value.trim().split(",");

        let timestamp = x.next().unwrap().parse().unwrap();
        let acc = AccData {
            x: x.next().unwrap().parse().unwrap(),
            y: x.next().unwrap().parse().unwrap(),
            z: x.next().unwrap().parse().unwrap(),
        };
        let mag = MagData {
            x: x.next().unwrap().parse().unwrap(),
            y: x.next().unwrap().parse().unwrap(),
            z: x.next().unwrap().parse().unwrap(),
        };

        Data {
            timestamp,
            acc,
            mag,
        }
    }
}

async fn stream_file(path: &impl AsRef<Path>) -> bool {
    use std::io::prelude::*;
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut prev = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.trim();
        if line.len() == 0 {
            continue;
        }
        let data: Data = line.into();
        println!("{data:?}");

        if prev != 0 {
            tokio::time::sleep(tokio::time::Duration::from_millis(data.timestamp - prev)).await;
        }
        prev = data.timestamp;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FILE: &str = "test-input-short.csv";

    #[tokio::test]
    async fn test_stream_file() {
        stream_file(&TEST_FILE).await;
    }
}
