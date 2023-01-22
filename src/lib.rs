pub struct TreeGrid{
    rows: Vec<Vec<u32>>
}

impl TreeGrid{
    pub fn new(input: &str) -> Option<Self>{
        let new = Self{
            rows: input
                    .split('\n')
                    .filter(|x| !x.is_empty())
                    .map(|row| row
                                .chars()
                                .map(|c| c.to_digit(10).unwrap_or_else(|| panic!("{c}")))
                                .collect())
                    .collect()
        };

        // Check that all rows are same length to ensure it is a proper grid
        if new.rows.windows(2).all(|w| w[0].len() == w[1].len()){
            return Some(new);
        }else{
            return None;
        }
    }

    /// Counts and returns the number of trees visible from outside
    /// # Examples
    /// ```
    /// use advent_of_code_2022_08::TreeGrid;
    /// let tg = TreeGrid::new(
    ///     concat!(
    ///         "30373\n",
    ///         "25512\n",
    ///         "65332\n",
    ///         "33549\n",
    ///         "35390"
    /// )).expect("This should produce a valid TreeGrid");
    /// assert_eq!(tg.count_visible(), 21);
    /// ```
    pub fn count_visible(&self) -> u32{
        let mut n = 0;
        let nrows = self.rows.len();
        let ncols = self.rows[0].len();
        for r in 0..nrows{
            for c in 0..ncols{
                if r == 0 || r == nrows-1 || c == 0 || c == ncols-1{
                    n += 1;
                }else{
                    let height = self.rows[r][c];

                    let left = (0..c).map(|n| self.rows[r][n]);
                    let right = (c+1..ncols).map(|n| self.rows[r][n]);
                    let above = (0..r).map(|n| self.rows[n][c]);
                    let below = (r+1..nrows).map(|n| self.rows[n][c]);
                    if left.filter(|h| *h >= height).count() == 0
                    || right.filter(|h| *h >= height).count() == 0
                    || above.filter(|h| *h >= height).count() == 0
                    || below.filter(|h| *h >= height).count() == 0
                    {
                        n += 1;
                    }
                }
            }
        }
        return n;
    }
}
