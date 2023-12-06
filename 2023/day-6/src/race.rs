pub struct Race {
    time: u64,
    record_distance: i64,
}

impl Race {
    pub fn new(time: u64, record_distance: i64) -> Self {
        Self {
            time,
            record_distance,
        }
    }

    fn quadratic_roots(&self) -> (f64, f64) {
        let roots = (
            (*&self.time as f64
                - ((self.time.pow(2) as i64 - 4 * self.record_distance) as f64).sqrt())
                / 2 as f64,
            (*&self.time as f64
                + ((self.time.pow(2) as i64 - 4 * self.record_distance) as f64).sqrt())
                / 2 as f64,
        );

        roots
    }

    pub fn num_wins(&self) -> u64 {
        let (lower, upper): (f64, f64) = self.quadratic_roots();
        let first_win = (lower + 1 as f64).floor() as u64;
        let last_win = (upper - 1 as f64).ceil() as u64;

        if first_win < last_win {
            return last_win - first_win + 1;
        } else {
            return 0;
        }
    }
}
