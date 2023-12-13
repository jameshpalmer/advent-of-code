#[derive(Clone)]
pub struct Sequence {
    terms: Vec<i32>,
}

impl Sequence {
    pub fn new(terms: Vec<i32>) -> Sequence {
        Sequence { terms }
    }

    fn len(&self) -> usize {
        self.terms.len()
    }

    fn extrapolate(&self) -> Sequence {
        let terms: Vec<i32> = self.terms[1..]
            .iter()
            .zip(self.terms[..self.len() - 1].iter())
            .map(|(&b, &v)| b - v)
            .collect();
        Sequence { terms }
    }

    pub fn get_next_term(&self) -> i32 {
        let mut curr_seq = self.clone();
        let mut last_term_sum = 0;

        while !curr_seq.terms.iter().all(|&term| term == 0) {
            last_term_sum += curr_seq.terms.last().unwrap();
            curr_seq = curr_seq.extrapolate()
        }

        return last_term_sum;
    }

    pub fn get_prev_term(&self) -> i32 {
        let mut curr_seq = self.clone();
        let mut first_terms = Vec::new();

        while !curr_seq.terms.iter().all(|&term| term == 0) {
            let terms_first = *curr_seq.terms.first().unwrap();
            first_terms.push(terms_first);
            curr_seq = curr_seq.extrapolate();
        }

        first_terms
            .iter()
            .enumerate()
            .fold(0, |acc, (index, &value)| {
                acc + if index % 2 == 0 { value } else { -value }
            })
    }
}
