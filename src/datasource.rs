// TODO parse example data, mock real time stream from device using sleeps

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};

#[derive(Debug, Clone)]
pub struct Data {
    pub timestamp: u64,
    pub acc: AccData,
    pub mag: MagData,
}

#[derive(Debug, Clone)]
pub struct AccData {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone)]
pub struct MagData {
    pub x: f64,
    pub y: f64,
    pub z: f64,
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

pub async fn stream_file(path: &impl AsRef<Path>) -> impl Stream<Item = Data> {
    let (tx, rx) = mpsc::channel::<Data>(10);

    // let x = path.to_owned();
    let p = PathBuf::from(path.as_ref());
    tokio::spawn(async move {
        use std::io::prelude::*;
        let file = File::open(p).unwrap();
        let reader = BufReader::new(file);

        let mut prev = 0;
        for line in reader.lines() {
            let line = line.unwrap();
            let line = line.trim();
            if line.len() == 0 {
                continue;
            }
            let data: Data = line.into();

            if prev != 0 {
                tokio::time::sleep(tokio::time::Duration::from_millis(data.timestamp - prev)).await;
            }
            prev = data.timestamp;
            tx.send(data).await.unwrap();
        }
    });

    ReceiverStream::new(rx)
}

// TODO later stream from tailing a file or from some /dev/ttyUSB

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;
    use tokio_stream::StreamExt;

    const TEST_FILE: &str = "test-input-short.csv";

    #[tokio::test]
    async fn test_stream_file() {
        let mut s = stream_file(&TEST_FILE).await;
        let start = Instant::now();
        while let Some(x) = s.next().await {
            println!("received data {x:?} at {}ms", start.elapsed().as_millis());
        }
    }
}
