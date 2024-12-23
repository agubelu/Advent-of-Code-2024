#![allow(dead_code)]

use std::marker::PhantomData;
use std::ops::{Index, IndexMut};
use std::fmt::Display;
use std::iter::Enumerate;
use std::slice::Iter;

use num_traits::{PrimInt, Unsigned};

use super::coords::Coords2D;

/** A 2D-like structure backed by a Vec */
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Matrix<T: Copy + PartialEq> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

pub struct VecMaxIndexedIter<'a, T: Copy + PartialEq, I: PrimInt> {
    _typ: PhantomData<I>,
    iter: Enumerate<Iter<'a, T>>,
    mat: &'a Matrix<T>
}

impl<T: Copy + PartialEq> Matrix<T> {
    pub fn new(width: usize, height: usize, default: T) -> Self {
        let data = vec![default; width * height];
        Self { width, height, data }
    }

    pub fn from_data(width: usize, height: usize, data: Vec<T>) -> Self {
        assert_eq!(data.len(), width * height);
        Self { width, height, data }
    }

    pub fn map_from_str(string: &str, mapper: fn(char) -> T) -> Self {
        let width = string.lines().next().unwrap().len();
        let data: Vec<_> = string.chars().filter(|ch| !ch.is_whitespace()).map(mapper).collect();
        Self{ width, height: data.len() / width, data }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get_row(&self, row: usize) -> Vec<T>  {
        assert!(row < self.height());
        self.data[self.width() * row .. self.width() * (row + 1)].to_vec()
    }

    pub fn get_col(&self, col: usize) -> Vec<T> {
        assert!(col < self.width());
        (0..self.height()).map(|i| self.data[i * self.width() + col]).collect()
    }

    pub fn enumerate<I: PrimInt>(&self) -> VecMaxIndexedIter<T, I> {
        VecMaxIndexedIter::new(self)
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = T> + 'a {
        self.data.iter().copied()
    }

    pub fn is_in_bounds<I: PrimInt>(&self, pos: Coords2D<I>) -> bool {
        let x = pos.x.to_i64().unwrap();
        let y = pos.y.to_i64().unwrap();

        let bound_x = self.width() as i64;
        let bound_y = self.height() as i64;

        x >= 0 && y >= 0 && x < bound_x && y < bound_y
    }

    pub fn get_or<I: PrimInt + Display>(&self, pos: Coords2D<I>, default: T) -> T {
        if self.is_in_bounds(pos) {
            self[pos]
        } else {
            default
        }
    }

    pub fn rotate_right(&mut self) {
        let mut data = vec![];

        for x in 0..self.width() {
            data.extend(self.get_col(x).into_iter().rev());
        }

        *self = Self { data, width: self.height(), height: self.width() };
    }

    pub fn rotate_left(&mut self) {
        let mut data = vec![];

        for x in (0..self.width()).rev() {
            data.extend(self.get_col(x));
        }

        *self = Self { data, width: self.height(), height: self.width() };
    }

    pub fn index(&self, x: usize, y: usize) -> usize {
        assert!(x < self.width(), "x index out of bounds: {x} but width is {}", self.width());
        assert!(y < self.height(), "y index out of bounds: {y} but height is {}", self.height());
        y * self.width + x
    }

    pub fn coords<I: PrimInt>(&self, index: usize) -> Coords2D<I> {
        (I::from(index % self.width).unwrap(), I::from(index / self.width).unwrap()).into()
    }

    pub fn find<I: PrimInt>(&self, elem: T) -> Option<Coords2D<I>> {
        // Find the first position of the provided element, if it exists.
        self.enumerate().find(|&x| x.1 == elem).map(|x| x.0)
    }

    pub fn find_all<'a, I: PrimInt + 'a>(&'a self, elem: T) -> impl Iterator<Item = Coords2D<I>> + 'a {
        self.enumerate().filter(move |&x| x.1 == elem).map(|x| x.0)
    }

}

impl Matrix<char> {
    pub fn from_str(string: &str) -> Self {
        let width = string.lines().next().unwrap().len();
        let data: Vec<char> = string.chars().filter(|ch| !ch.is_whitespace()).collect();
        Self{ width, height: data.len() / width, data }
    }

    pub fn get_or_dot<I: PrimInt + Display>(&self, pos: Coords2D<I>) -> char {
        self.get_or(pos, '.')
    }
}

impl<T: PrimInt + Unsigned> Matrix<T> {
    pub fn map_digits(string: &str) -> Self {
        Self::map_from_str(string, |ch| T::from(ch.to_digit(10).unwrap()).unwrap())
    }
}

impl<T, I> Index<(I, I)> for Matrix<T>
where T: Copy + PartialEq,
      I: PrimInt + Display
{
    type Output = T;

    fn index(&self, (x, y): (I, I)) -> &Self::Output {
        let i = self.index(
            x.to_usize().unwrap_or_else(|| panic!("X index not valid: {x}")),
            y.to_usize().unwrap_or_else(|| panic!("Y index not valid: {y}"))
        );
        &self.data[i]
    }
}

impl<T, I> IndexMut<(I, I)> for Matrix<T>
where T: Copy + PartialEq,
      I: PrimInt + Display
{
    fn index_mut(&mut self, (x, y): (I, I)) -> &mut Self::Output {
        let i = self.index(
            x.to_usize().unwrap_or_else(|| panic!("X index not valid: {x}")),
            y.to_usize().unwrap_or_else(|| panic!("Y index not valid: {y}"))
        );
        &mut self.data[i]
    }
}

impl<T, I> Index<Coords2D<I>> for Matrix<T>
where T: Copy + PartialEq,
      I: PrimInt + Display
{
    type Output = T;

    fn index(&self, Coords2D { x, y }: Coords2D<I>) -> &Self::Output {
        &self[(x, y)]
    }
}

impl<T, I> IndexMut<Coords2D<I>> for Matrix<T>
where T: Copy + PartialEq,
      I: PrimInt + Display
{

    fn index_mut(&mut self, Coords2D { x, y }: Coords2D<I>) -> &mut Self::Output {
        &mut self[(x, y)]
    }
}

impl <'a, T: Copy + PartialEq, I: PrimInt> VecMaxIndexedIter<'a, T, I> {
    pub fn new(mat: &'a Matrix<T>) -> Self {
        let iter = mat.data.iter().enumerate();
        Self { mat, iter, _typ: PhantomData }
    }
}

impl<'a, T: Copy + PartialEq, I: PrimInt> Iterator for VecMaxIndexedIter<'a, T, I> {
    type Item = (Coords2D<I>, T);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(i, x)| (self.mat.coords(i), *x))
    }
}

impl<T: Copy + PartialEq + Display> Display for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height() {
            for x in 0..self.width() {
                write!(f, "{}", self[(x, y)])?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}