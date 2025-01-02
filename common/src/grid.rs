type Point = crate::point::Point<usize>;

pub struct Grid<T> {
    data: Vec<T>,
    pub rows: usize,
    pub columns: usize,
}

impl<T> Grid<T> {
    pub fn construct(input: &str, mapper: impl Fn(char) -> T) -> Grid<T> {
        let columns = if input.is_empty() {
            0
        } else {
            input.lines().next().unwrap().len()
        };
        Grid {
            data: input
                .lines()
                .flat_map(|line| line.chars())
                .map(mapper)
                .collect(),
            rows: input.lines().count(),
            columns,
        }
    }

    pub fn from_2d_array(data: Vec<Vec<T>>) -> Grid<T> {
        let rows = data.len();
        let columns = data.first().map(|row| row.len()).unwrap_or(0);
        let data = data.into_iter().flatten().collect();
        Grid {
            data,
            rows,
            columns,
        }
    }

    pub fn manual_construct(data: Vec<T>, rows: usize, columns: usize) -> Grid<T> {
        Grid {
            data,
            rows,
            columns,
        }
    }

    fn point_to_index(&self, point: Point) -> Option<usize> {
        if point.0 >= self.rows || point.1 >= self.columns {
            return None;
        }
        Some(point.0 * self.columns + point.1)
    }

    pub fn get_ref(&self, point: Point) -> Option<&T> {
        self.data.get(self.point_to_index(point)?) // .copied()
    }
    pub fn get_i32_ref(&self, position: (i32, i32)) -> Option<&T> {
        if position.0 >= self.rows as i32
            || position.0 < 0
            || position.1 >= self.columns as i32
            || position.1 < 0
        {
            return None;
        }
        self.data
            .get((position.0 as usize) * self.columns + (position.1 as usize))
    }
    pub fn set(&mut self, point: Point, val: T) -> Option<()> {
        if let Some(idx) = self.point_to_index(point) {
            self.data[idx] = val;
            Some(())
        } else {
            None
        }
    }
    pub fn row_ref(&self, row: usize) -> Option<&[T]> {
        if row >= self.rows {
            return None;
        }
        let start = row * self.columns;
        let end = start + self.columns;
        Some(&self.data[start..end])
    }
    pub fn column_ref(&self, column: usize) -> Option<Vec<&T>> {
        if column >= self.columns {
            return None;
        }

        Some(
            self.data
                .iter()
                .skip(column)
                .step_by(self.columns)
                .collect(),
        )
    }
    pub fn print(&self)
    where
        T: std::fmt::Display,
    {
        for i in 0..self.rows {
            for j in 0..self.columns {
                let val = self.get_ref((i, j).into()).unwrap();
                print!("{val}");
            }
            println!();
        }
    }
}

impl<T> Grid<T>
where
    T: Copy,
{
    pub fn iter(&self) -> impl Iterator<Item = (Point, T)> + '_ {
        self.data
            .iter()
            .enumerate()
            .map(|(idx, c)| ((idx / self.columns, idx % self.columns).into(), *c))
    }
    pub fn get(&self, point: Point) -> Option<T> {
        self.get_ref(point).copied()
    }

    pub fn get_i32(&self, position: (i32, i32)) -> Option<T> {
        self.get_i32_ref(position).copied()
    }

    pub fn row(&self, row: usize) -> Option<Vec<T>> {
        if row >= self.rows {
            return None;
        }
        let start = row * self.columns;
        let end = start + self.columns;
        Some(self.data[start..end].to_vec())
    }

    pub fn column(&self, column: usize) -> Option<Vec<T>> {
        if column >= self.columns {
            return None;
        }

        Some(
            self.data
                .iter()
                .skip(column)
                .step_by(self.columns)
                .copied()
                .collect(),
        )
    }
}

impl<T> Grid<T>
where
    T: PartialEq,
{
    pub fn search(&self, needle: T) -> Option<Point> {
        self.data.iter().enumerate().find_map(|(idx, c)| {
            if *c == needle {
                return Some((idx / self.columns, idx % self.columns).into());
            }
            None
        })
    }

    pub fn search_all(&self, needle: T) -> Vec<Point> {
        self.data
            .iter()
            .enumerate()
            .filter_map(|(idx, c)| {
                if *c == needle {
                    return Some((idx / self.columns, idx % self.columns).into());
                }
                None
            })
            .collect()
    }
}
