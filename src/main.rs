use std::{io, io::Write as _, time};

use rand::rngs::ThreadRng;
use uuid7::gen7::{Generator, Status};

fn main() -> io::Result<()> {
    let mut out = io::BufWriter::new(io::stdout());
    eprintln!("Print stats every 10 seconds....");

    let mut g: Generator<ThreadRng> = Default::default();
    let mut counters: Counters = Default::default();
    let mut ts_prev_print = get_timestamp();
    loop {
        let ts = get_timestamp();
        let (id, status) = g.generate_core(ts);

        out.write_all(id.encode().as_bytes()).unwrap();
        out.write_all(&[b'\n']).unwrap();

        counters.count(status);
        if ts >= ts_prev_print + 10_000 {
            counters.print_stats();
            counters.reset();
            ts_prev_print = ts;
        }
    }
}

#[inline(always)]
fn get_timestamp() -> u64 {
    time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .expect("clock may have gone backwards")
        .as_millis() as u64
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
struct Counters {
    count: isize,
    new_timestamp: isize,
    counter_inc: isize,
    timestamp_inc: isize,
    clock_rollback: isize,
}

impl Counters {
    fn count(&mut self, status: Status) {
        self.count += 1;
        match status {
            Status::NewTimestamp => self.new_timestamp += 1,
            Status::CounterInc => self.counter_inc += 1,
            Status::TimestampInc => self.timestamp_inc += 1,
            Status::ClockRollback => self.clock_rollback += 1,
        }
    }

    fn print_stats(&self) {
        eprintln!(
            "Total {}, NewTimestamp {}, CounterInc {}, TimestampInc {}, ClockRollback {}",
            self.count,
            self.new_timestamp,
            self.counter_inc,
            self.timestamp_inc,
            self.clock_rollback
        );
    }

    fn reset(&mut self) {
        self.count = 0;
        self.new_timestamp = 0;
        self.counter_inc = 0;
        self.timestamp_inc = 0;
        self.clock_rollback = 0;
    }
}
