use std::ops::{Index, IndexMut};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[derive(Clone)]
pub struct Grid<T> {
    data: Vec<T>,
    width: usize,
}

impl<T> Grid<T> {
    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.data.len() / self.width
    }
}

impl<T: Default + Clone> Grid<T> {
    pub fn new(width: usize, height: usize) -> Grid<T> {
        Grid {
            data: vec![Default::default(); width * height],
            width: width,
        }
    }
}

impl<T: Clone> Grid<T> {
    pub fn new_filled_with(thing: T, width: usize, height: usize) -> Grid<T> {
        Grid {
            data: vec![thing; width * height],
            width: width,
        }
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &T {
        &self.data[x + y * self.width]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut T {
        &mut self.data[x + y * self.width]
    }
}

