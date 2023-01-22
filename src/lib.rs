pub struct TreeGrid{
    rows: Vec<Vec<u32>>
}

impl TreeGrid{
    pub fn new(input: &str) -> Option<Self>{
        let new = Self{
            rows: input.split('\n').map(|row| row.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect()
        };

        // Check that all rows are same length to ensure it is a proper grid
        if new.rows.windows(2).all(|w| w[0] == w[1]){
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
    /// )).unwrap();
    /// assert_eq!(tg.count_visible(), 21);
    /// ```
    pub fn count_visible(&self) -> u32{
        todo!();
    }
}

