use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    pub data: Vec<T>,
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone + Debug,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

impl Grid<char> {
    pub fn parse(input: &str) -> Self {
        let raw = input
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let width = raw[0].len();
        let height = raw.len();
        let flat = raw.iter().flatten().map(|&c| c).collect::<Vec<_>>();

        Grid {
            width,
            height,
            data: flat,
        }
    }

    pub fn from_rows(rows: Vec<Vec<char>>) -> Self {
        let width = rows.len();
        let height = rows[0].len();

        Grid {
            width,
            height,
            data: rows.iter().flatten().map(|&c| c).collect::<Vec<_>>(),
        }
    }

    pub fn from_cols(columns: Vec<Vec<char>>) -> Self {
        Grid::from_rows(transpose(columns))
    }
}

impl<T: Copy + PartialEq + Debug> Grid<T> {
    pub fn row(&self, row_idx: usize) -> Option<Vec<&T>> {
        if row_idx >= self.height {
            None
        } else {
            Some(
                self.data
                    .iter()
                    .skip(row_idx * self.width)
                    .take(self.width)
                    .collect::<Vec<_>>(),
            )
        }
    }

    pub fn col(&self, col_idx: usize) -> Vec<&T> {
        if col_idx >= self.width {
            panic!(
                "Attempted to get column with index {} while grid width is {} (indidices are 0-indexed)",
                col_idx, self.width
            );
        }

        self.data
            .iter()
            .skip(col_idx)
            .step_by(self.width)
            .collect::<Vec<_>>()
    }

    pub fn columns(&self) -> Vec<Vec<&T>> {
        (0..self.width)
            .map(|index| self.col(index))
            .collect::<Vec<_>>()
    }

    pub fn rows(&self) -> Vec<Vec<&T>> {
        self.data
            .chunks(self.width)
            .map(|chunk| chunk.iter().collect::<Vec<_>>())
            .collect::<Vec<_>>()
    }

    /**
     * Mutate the grid by adding a row of element T at the given index
     */
    pub fn insert_row(&mut self, row_idx: usize, element: T) {
        (0..self.width).for_each(|offset| self.data.insert(row_idx * self.width + offset, element));
        self.height += 1;
    }

    /**
     * Mutate the grid by adding a column of element T at the given index
     */
    pub fn insert_col(&mut self, col_idx: usize, element: T) {
        (0..self.height).for_each(|offset| {
            self.data
                .insert(col_idx + (self.width * offset) + offset, element)
        });
        self.width += 1;
    }

    pub fn to_vec(&self) -> Vec<Vec<T>> {
        self.data
            .windows(self.width)
            .step_by(self.width)
            .map(|row| row.to_vec())
            .collect::<Vec<_>>()
    }

    /**
     * Returns the coordinate of the first match
     */
    pub fn find(&self, needle: T) -> Option<(usize, usize)> {
        self.data
            .iter()
            .position(|&d| d == needle)
            .map(|index| (index % self.width, index / self.width))
    }

    /**
     * Returns the coordinates for all matches
     */
    pub fn find_all(&self, needle: T) -> Vec<(usize, usize)> {
        self.data
            .iter()
            .enumerate()
            .filter_map(|(index, &d)| {
                if d == needle {
                    Some((index % self.width, index / self.width))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }
}
