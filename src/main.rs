use std::num::NonZeroUsize;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CatalanError {
    #[error("cannot calculate result for that big input")]
    InputTooBig,
}

type Result<T> = std::result::Result<T, CatalanError>;

const MAX: usize = 1000;
const MODULO: usize = 100_000_007;

#[derive(Debug, Clone)]
struct Catalan {
    values: [Option<NonZeroUsize>; MAX],
}

impl Default for Catalan {
    fn default() -> Self {
        let mut values: [Option<NonZeroUsize>; MAX] = [None; MAX];
        values[0] = NonZeroUsize::new(1);
        values[1] = NonZeroUsize::new(1);
        Self { values }
    }
}

impl Catalan {
    pub fn get(&mut self, n: usize) -> Result<usize> {
        if n >= MAX {
            return Err(CatalanError::InputTooBig);
        }
        Ok(self._get(n))
    }

    fn _get(&mut self, n: usize) -> usize {
        if let Some(value) = self.values[n] {
            return value.into();
        }
        // We do not need to worry about n being 0 or 1 because of the Default implementation
        let value: usize = (0..n).fold(0, |mut v, k| -> usize {
            let left = self._get(k);
            let right = self._get(n - 1 - k);
            let term = usize::from(left) * usize::from(right) % MODULO;
            v += term;
            v %= MODULO;
            v
        });
        self.values[n] = NonZeroUsize::new(value);
        value
    }
}

fn main() {
    let mut catalan = Catalan::default();
    let input = [0, 1, 2, 3, 4, 5, 6, 7, 100, 200, 500, 999, 1000];
    let output: Vec<usize> = input
        .into_iter()
        .filter_map(|i| catalan.get(i).ok())
        .collect();

    dbg!(output);
}
