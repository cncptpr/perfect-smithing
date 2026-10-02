use std::{cmp::max, fmt::Display};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Matrix<T> {
    pub(crate) values: Box<[T]>,
    pub(crate) size: usize,
}

impl<T: Copy + Display> Display for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut colum_widths = vec![0; self.size];
        for (column, max_width) in colum_widths.iter_mut().enumerate() {
            for i in 0..self.size {
                *max_width = max(
                    format!("{}", self.get_expect(i as i64, column as i64)).len(),
                    *max_width,
                )
            }
        }

        for i in 0..self.size {
            for (j, width) in colum_widths.iter().copied().enumerate() {
                write!(f, "{:>width$}  ", self.get_expect(i as i64, j as i64))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T: Copy + Display> Matrix<T> {
    pub(crate) fn new(size: usize, initial_value: T) -> Self {
        Self {
            values: vec![initial_value; size * size].into_boxed_slice(),
            size,
        }
    }

    pub(crate) fn set(&mut self, i: i64, j: i64, value: impl Into<T>) {
        if i < 0 || j < 0 || i as usize >= self.size || j as usize >= self.size {
            return;
        }
        let index = j as usize * self.size + i as usize;
        self.values[index] = value.into();
    }

    pub(crate) fn get(&self, i: i64, j: i64) -> Option<T> {
        if i < 0 || j < 0 || i as usize >= self.size || j as usize >= self.size {
            return None;
        }
        let index = j as usize * self.size + i as usize;
        Some(self.values[index])
    }

    pub(crate) fn get_expect(&self, i: i64, j: i64) -> T {
        self.get(i, j).expect("Call to get_unsafe must succeed")
    }
}
