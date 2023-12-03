use std::fmt;

pub struct Array2d<T>
where
    T: Copy,
{
    vec: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T: Copy> Array2d<T> {
    pub fn new(col_capacity: usize, row_capacity: usize, fill_value: T) -> Array2d<T> {
        let mut v = Vec::new();
        v.resize_with(col_capacity * row_capacity, || fill_value);

        Array2d {
            vec: v,
            width: col_capacity,
            height: row_capacity,
        }
    }

    pub fn insert(&mut self, element: T, row: usize, col: usize) {
        self.vec[row * self.height + col] = element;
    }

    pub fn get(&self, row: usize, col: usize) -> Option<T> {
        if col >= self.width || row >= self.height {
            return None;
        }

        Some(self.vec[row * self.height + col])
    }

    pub fn get_s(&self, row: i32, col: i32) -> Option<T> {
        if col < 0 || row < 0 {
            return None;
        }

        let row_s: usize = row.try_into().ok()?;
        let col_s: usize = col.try_into().ok()?;
        if col_s >= self.width || row_s >= self.height {
            return None;
        }
        Some(self.vec[row_s * self.height + col_s])
    }
}

impl<T: fmt::Debug + Copy> fmt::Debug for Array2d<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Array2d {{\n")?;

        for row in 0..self.height {
            write!(f, " [")?;
            for col in 0..self.width {
                if col > 0 {
                    write!(f, ", ")?
                }
                write!(f, "{:?}", self.get(row, col).unwrap())?;
            }
            write!(f, "]\n")?;
        }
        write!(f, "}}")
    }
}
