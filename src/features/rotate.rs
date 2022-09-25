//! This module contains a [`Rotate`] primitive which can be used in order to rotate [`Table`].
//!
//! It's also possible to transpose the table at the point of construction.
//! See [`Builder::index`].
//!
//! # Example
//!
//! ```
//! use tabled::{Rotate, TableIteratorExt};
//!
//! let data = [[1, 2, 3], [4, 5, 6]];
//!
//! let table = data.table().with(Rotate::Left).to_string();
//!
//! assert_eq!(
//!     table,
//!     concat!(
//!         "+---+---+---+\n",
//!         "| 2 | 3 | 6 |\n",
//!         "+---+---+---+\n",
//!         "| 1 | 2 | 5 |\n",
//!         "+---+---+---+\n",
//!         "| 0 | 1 | 4 |\n",
//!         "+---+---+---+",
//!     )
//! );
//! ```
//!
//! [`Table`]: crate::Table
//! [`Builder::index`]: crate::builder::Builder::index

use papergrid::records::{Records, Resizable};

use crate::{Table, TableOption};

/// Rotate can be used to rotate a table by 90 degrees.
#[derive(Debug)]
pub enum Rotate {
    /// Rotate [`Table`] to the left.
    ///
    /// [`Table`]: crate::Table
    Left,
    /// Rotate [`Table`] to the right.
    ///
    /// [`Table`]: crate::Table
    Right,
    /// Rotate [`Table`] to the top.
    ///
    /// So the top becames the bottom.
    ///
    /// [`Table`]: crate::Table
    Top,
    /// Rotate [`Table`] to the bottom.
    ///
    /// So the top becames the bottom.
    ///
    /// [`Table`]: crate::Table
    Bottom,
}

impl<R> TableOption<R> for Rotate
where
    R: Records + Resizable,
{
    fn change(&mut self, table: &mut Table<R>) {
        let (count_rows, count_cols) = table.shape();
        let records = table.get_records_mut();
        match self {
            Self::Left => {
                {
                    let n = std::cmp::max(count_rows, count_cols);
                    for _ in count_rows..n {
                        records.push_row();
                    }

                    for _ in count_cols..n {
                        records.push_column();
                    }
                }

                for col in 0..count_cols {
                    for row in col..count_rows {
                        records.swap((col, row), (row, col));
                    }
                }

                for row in 0..count_cols / 2 {
                    records.swap_row(row, count_cols - row - 1);
                }

                {
                    let n = std::cmp::max(count_rows, count_cols);
                    for (shift, row) in (count_rows..n).enumerate() {
                        let row = row - shift;
                        records.remove_column(row);
                    }

                    for (shift, col) in (count_cols..n).enumerate() {
                        let col = col - shift;
                        records.remove_row(col);
                    }
                }
            }
            Self::Right => {
                {
                    let n = std::cmp::max(count_rows, count_cols);
                    for _ in count_rows..n {
                        records.push_row();
                    }

                    for _ in count_cols..n {
                        records.push_column();
                    }
                }

                for col in 0..count_cols {
                    for row in col..count_rows {
                        records.swap((col, row), (row, col));
                    }
                }

                for col in 0..count_rows / 2 {
                    records.swap_column(col, count_rows - col - 1);
                }

                {
                    let n = std::cmp::max(count_rows, count_cols);
                    for (shift, row) in (count_rows..n).enumerate() {
                        let row = row - shift;
                        records.remove_column(row);
                    }

                    for (shift, col) in (count_cols..n).enumerate() {
                        let col = col - shift;
                        records.remove_row(col);
                    }
                }
            }
            Self::Bottom => {
                for row in 0..count_rows / 2 {
                    for col in 0..count_cols {
                        let last_row = count_rows - row - 1;
                        records.swap((last_row, col), (row, col));
                    }
                }
            }
            Self::Top => Self::Bottom.change(table),
        }
    }
}
