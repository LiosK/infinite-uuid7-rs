use std::{io, io::Write as _, time};

use rand::rngs::ThreadRng;
use uuid7::V7Generator;

fn main() -> io::Result<()> {
    let mut out = io::BufWriter::new(io::stdout());
    eprintln!("Print stats every 10 seconds....");

    let mut g: V7Generator<ThreadRng> = Default::default();
    let mut counters: Counters = Default::default();
    let mut ts_prev_print = get_timestamp();
    loop {
        let ts = get_timestamp();
        counters.count += 1;
        let id = g.generate_or_abort_core(ts, 1).unwrap_or_else(|| {
            counters.clock_rollback += 1;
            g.generate_or_reset_core(ts, 1)
        });

        out.write_all(id.encode().as_bytes()).unwrap();
        out.write_all(&[b'\n']).unwrap();

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

#[derive(Debug, Default)]
struct Counters {
    count: isize,
    clock_rollback: isize,
}

impl Counters {
    fn print_stats(&self) {
        eprintln!(
            "Total {}, ClockRollback {}",
            self.count, self.clock_rollback
        );
    }

    fn reset(&mut self) {
        self.count = 0;
        self.clock_rollback = 0;
    }
}
