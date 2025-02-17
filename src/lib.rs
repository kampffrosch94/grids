use glam::IVec2;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::{Index, IndexMut};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Grid<T> {
    pub data: Vec<T>,
    pub width: i32,
    pub height: i32,
}

impl<T: Clone> Grid<T> {
    pub fn new(width: i32, height: i32, value: T) -> Self {
        Self {
            data: vec![value; (width * height) as usize],
            width,
            height,
        }
    }

    pub fn filled_with<F: FnMut(i32, i32) -> T>(width: i32, height: i32, mut f: F) -> Self {
        let mut data = Vec::with_capacity((width * height) as usize);

        for y in 0..height {
            for x in 0..width {
                data.push(f(x, y));
            }
        }

        Self {
            data,
            width,
            height,
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn is_valid(&self, coord: glam::IVec2) -> bool {
        coord.x >= 0 && coord.x < self.width() && coord.y >= 0 && coord.y < self.height()
    }

    pub fn get(&self, x: i32, y: i32) -> &T {
        &self[(x, y)]
    }

    pub fn get_mut(&mut self, x: i32, y: i32) -> &mut T {
        &mut self[(x, y)]
    }

    pub fn get_clamped(&self, x: i32, y: i32) -> &T {
        let x = x.clamp(0, self.width - 1);
        let y = y.clamp(0, self.height - 1);

        self.get(x, y)
    }

    pub fn get_clamped_v(&self, v: glam::IVec2) -> &T {
        let x = v.x;
        let y = v.y;

        let x = x.clamp(0, self.width - 1);
        let y = y.clamp(0, self.height - 1);

        self.get(x, y)
    }

    pub fn v_clamped(&self, v: glam::Vec2) -> &T {
        let x = v.x as i32;
        let y = v.y as i32;

        let x = x.clamp(0, self.width - 1);
        let y = y.clamp(0, self.height - 1);

        self.get(x, y)
    }

    pub fn get_clamped_mut(&mut self, x: i32, y: i32) -> &mut T {
        let x = x.clamp(0, self.width - 1);
        let y = y.clamp(0, self.height - 1);

        self.get_mut(x, y)
    }

    pub fn iter_rect(&self, min: IVec2, max: IVec2) -> impl Iterator<Item = (i32, i32, &T)> {
        let mut coords = vec![];

        for x in min.x..max.x {
            for y in min.y..max.y {
                coords.push((x, y));
            }
        }

        coords.into_iter().map(|(x, y)| (x, y, &self[(x, y)]))
    }

    pub fn clone_rect(&self, min: IVec2, max: IVec2) -> Grid<T> {
        let dims = max - min;
        let mut result = Grid::new(dims.x, dims.y, self[(0i32, 0i32)].clone());

        for x in 0..dims.x {
            for y in 0..dims.y {
                result[(x, y)] = self[(x + min.x, y + min.y)].clone();
            }
        }

        result
    }

    pub fn paste_grid(&mut self, offset: IVec2, other: &Grid<T>) {
        for x in 0..other.width() {
            for y in 0..other.height() {
                self[(offset.x + x, offset.y + y)] = other[(x, y)].clone();
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (i32, i32, &T)> {
        self.data.iter().enumerate().map(|(i, v)| {
            let i = i as i32;
            let x = i % self.width;
            let y = i / self.width;

            (x, y, v)
        })
    }

    pub fn iter_values(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    pub fn into_iter_values(self) -> impl Iterator<Item = T> {
        self.data.into_iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (i32, i32, &mut T)> {
        self.data.iter_mut().enumerate().map(|(i, v)| {
            let i = i as i32;
            let x = i % self.width;
            let y = i / self.height;

            (x, y, v)
        })
    }

    pub fn iter_values_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut()
    }

    pub fn iter_coords(&self) -> impl Iterator<Item = (glam::IVec2, &T)> {
        self.iter().map(|(x, y, v)| (glam::IVec2::new(x, y), v))
    }

    pub fn iter_coords_mut(&mut self) -> impl Iterator<Item = (glam::IVec2, &mut T)> {
        self.iter_mut().map(|(x, y, v)| (glam::IVec2::new(x, y), v))
    }

    pub fn coords(&self) -> Vec<glam::IVec2> {
        self.iter()
            .map(|(x, y, _)| glam::IVec2::new(x, y))
            .collect::<Vec<_>>()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn size(&self) -> IVec2 {
        IVec2::new(self.width(), self.height())
    }

    pub fn pack(fill: T, grids: Vec<Grid<T>>) -> Grid<T> {
        assert!(grids.len() > 0);

        let mut size = IVec2::ZERO;

        for grid in grids.iter() {
            size.x += grid.width();
            size.y = size.y.max(grid.height());
        }

        let mut result = Grid::new(size.x, size.y, fill.clone());

        let mut offset = IVec2::ZERO;

        for grid in grids.iter() {
            for (x, y, val) in grid.iter() {
                result[offset + IVec2::new(x, y)] = val.clone();
            }

            offset += IVec2::new(grid.width(), 0);
        }

        result
    }

    /// Returns an iterator over the rows of the grid.
    ///
    /// ```
    /// use grids::Grid;
    ///
    /// let mut grid = Grid::new(3, 2, 0);
    ///
    /// grid[(0, 0)] = 1;
    /// grid[(1, 0)] = 2;
    /// grid[(2, 0)] = 3;
    /// grid[(0, 1)] = 5;
    /// grid[(1, 1)] = 6;
    /// grid[(2, 1)] = 7;
    ///
    /// let mut row_iter = grid.row_iter();
    /// assert_eq!(row_iter.next().unwrap().cloned().collect::<Vec<_>>(), vec![1, 2, 3]);
    /// assert_eq!(row_iter.next().unwrap().cloned().collect::<Vec<_>>(), vec![5, 6, 7]);
    /// assert!(row_iter.next().is_none()
    /// )
    ///
    /// ```
    pub fn row_iter(&self) -> impl Iterator<Item = core::slice::Iter<'_, T>> {
        (0..self.height).map(move |y| {
            let start = (y * self.width) as usize;
            let end = start + self.width as usize;
            self.data[start..end].iter()
        })
    }
}

impl<T: Clone> Index<(i32, i32)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (i32, i32)) -> &Self::Output {
        &self.data[(x + y * self.width) as usize]
    }
}

impl<T: Clone> IndexMut<(i32, i32)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (i32, i32)) -> &mut Self::Output {
        &mut self.data[(x + y * self.width) as usize]
    }
}

// impl<T: Clone> Index<(u32, u32)> for Grid<T> {
//     type Output = T;
//
//     fn index(&self, (x, y): (u32, u32)) -> &Self::Output {
//         &self.data[(x as i32 + y as i32 * self.width) as usize]
//     }
// }
//
// impl<T: Clone> IndexMut<(u32, u32)> for Grid<T> {
//     fn index_mut(&mut self, (x, y): (u32, u32)) -> &mut Self::Output {
//         &mut self.data[(x as i32 + y as i32 * self.width) as usize]
//     }
// }

impl<T: Clone> Index<glam::IVec2> for Grid<T> {
    type Output = T;

    fn index(&self, index: glam::IVec2) -> &Self::Output {
        &self[(index.x, index.y)]
    }
}

impl<T: Clone> IndexMut<glam::IVec2> for Grid<T> {
    fn index_mut(&mut self, index: glam::IVec2) -> &mut Self::Output {
        &mut self[(index.x, index.y)]
    }
}

impl<T: Clone> Index<glam::UVec2> for Grid<T> {
    type Output = T;

    fn index(&self, index: glam::UVec2) -> &Self::Output {
        &self[(index.x as i32, index.y as i32)]
    }
}

impl<T: Clone> IndexMut<glam::UVec2> for Grid<T> {
    fn index_mut(&mut self, index: glam::UVec2) -> &mut Self::Output {
        &mut self[(index.x as i32, index.y as i32)]
    }
}

#[test]
fn test_stuff() {
    let mut grid = Grid::new(3, 2, 0);
    grid[(0, 1)] = 5;

    assert_eq!(grid[glam::IVec2::new(1, 0)], 0);
    assert_eq!(grid[glam::UVec2::new(0, 1)], 5);

    assert_eq!(
        grid.into_iter_values().collect::<Vec<_>>(),
        vec![0, 0, 0, 5, 0, 0]
    );
}

#[test]
fn readme_test() {
    let mut grid = Grid::new(3, 2, 0); // A 3x2 grid filled with zeros.
    grid[(0, 1)] = 5;

    // Accessing using glam::IVec2.
    assert_eq!(grid[glam::IVec2::new(1, 0)], 0);
    // Accessing using glam::UVec2.
    assert_eq!(grid[glam::UVec2::new(0, 1)], 5);

    // Converting grid to a Vec.
    assert_eq!(
        grid.into_iter_values().collect::<Vec<_>>(),
        vec![0, 0, 0, 5, 0, 0]
    );
}

#[test]
fn test_row_iter_empty_grid() {
    let grid: Grid<i32> = Grid::new(0, 0, 0);
    let mut row_iter = grid.row_iter();

    assert!(row_iter.next().is_none());
}

#[test]
fn test_row_iter_single_row() {
    let grid = Grid::new(3, 1, 42);
    let mut row_iter = grid.row_iter();

    if let Some(row) = row_iter.next() {
        let row: Vec<_> = row.cloned().collect();
        assert_eq!(row, vec![42, 42, 42]);
    } else {
        panic!("Expected one row, but got none");
    }

    assert!(row_iter.next().is_none());
}

#[test]
fn test_row_iter_multiple_rows() {
    let grid = Grid::filled_with(3, 3, |x, y| x + y);
    let mut row_iter = grid.row_iter();

    #[rustfmt::skip]
    let expected_rows = vec![
        vec![0, 1, 2],
        vec![1, 2, 3],
        vec![2, 3, 4]
    ];

    for (i, expected_row) in expected_rows.iter().enumerate() {
        if let Some(row) = row_iter.next() {
            let row: Vec<_> = row.cloned().collect();
            assert_eq!(&row, expected_row, "Row {} did not match", i);
        } else {
            panic!("Expected more rows, but got none");
        }
    }

    assert!(row_iter.next().is_none(), "Expected no more rows");
}
