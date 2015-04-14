#![feature(collections)]

extern crate rand;

use std::cmp::Eq;
use std::cmp::PartialEq;
use std::collections::VecMap;
use rand::{random, Open01};

#[derive(Debug,PartialEq,Eq,Clone)]//,PartialOrd,Ord)]
enum Cell {
	Empty,
	Tree,
	Burning,
}
type FieldType = VecMap<Cell>;
struct Field {
	cell_: FieldType,
	p_: f32,
	f_: f32,
	x_: usize,
	y_: usize,
	empty_: Cell,
}
impl Field {
	fn new(x: usize, y: usize, f: f32, p: f32) -> Field {
		Field {
			cell_ : FieldType::with_capacity(y*x),
			p_: p,
			f_: f,
			x_: x,
			y_: y,
			empty_: Cell::Empty,
		}
	}
	fn populate(self, t: Cell, p: f32) -> Field {
		let mut out = Field {
			cell_ : FieldType::with_capacity(self.y_*self.x_),
			p_: self.p_,
			f_: self.f_,
			x_: self.x_,
			y_: self.y_,
			empty_: Cell::Empty,
		};
		for (y,_) in  self.cell_.iter() {
			out.cell_.insert(y, self.rand_cell(t.clone(),p));
		}
		out
	}
	pub fn set(&mut self, x: &usize, y: &usize, c: Cell) {
		self.cell_.insert( y*self.x_ + *x, c );
	}
	pub fn get(&self, x: &usize, y: &usize) -> &Cell {
		let cell = *y * self.x_ + *x;
		if let Some(ref r) = self.cell_.get( &cell)  {
			return r
		}
		return &self.empty_
	}
	fn cells(&self) -> Vec<(usize,usize)> {
		let mut v: Vec<(usize, usize)> = Vec::new();
		for i in 0..self.x_ {
			for j in 0..self.y_ {
				v.push( (i, j) );
			}
		}
		v
	}
	fn neighbours<'a>(&self, x: usize, y: usize, r: &'a Vec<i16>) -> Vec<(usize,usize)> {
		let mut v: Vec<(&i16, &i16)> = Vec::new();
		for i in r {
			for j in r {
				v.push( (&i , &j ) );
			}
		}
		v.iter()
			.filter_map( |&pair| match pair {
						(&0,&0) => None,
						(&a,&b)  => {
							let xa = x as i16 + a;
							let yb = y as i16 + b;
							let out_of_bounds =
								xa < 0 || xa >= self.x_ as i16 ||
								yb < 0 || yb >= self.y_ as i16;
							if (out_of_bounds) {None}
							else {
								Some((xa as usize, yb as usize))
							}
						}
					}).collect()
	}
	pub fn has_burning_neighbour(&self, x: usize, y: usize) -> bool {
		let xs = vec![-1,0,1];
		self.neighbours(x,y,&xs).iter().any(|&(x,y)|{
				match self.get(&x,&y) {
				 &Cell::Burning => true,
				 _ => false
				}
			})
	}
	fn rand_cell(&self, c: Cell, p: f32) -> Cell
	{
		match  random::<Open01<f32>>() {
			Open01(val) if val < p => c,
			_=> Cell::Empty,
		}
	}
}
#[cfg(test)]
mod test {
use super::Field;
use super::Cell;

#[test]
fn field_functions() {
	let mut f = Field::new(10,10, 0.05, 0.001);
	f.set(&1,&2, Cell::Tree);
	assert_eq!(&Cell::Tree, f.get(&1,&2))
}


#[test]
fn neighbours() {
	let f = Field::new(10,10, 0.05, 0.001);

	let r = vec![-1,0,1];

	let c = f. neighbours(5,5,&r);
	for i in c.iter() {
		let &(a,b) = i;
		//println!("stuff: {} {}",a,b)
	}

	assert_eq!(8, f. neighbours(5,5,&r).len());

	assert_eq!(5, f.neighbours(0,2,&r).len());
	assert_eq!(5, f.neighbours(0,1,&r).len());
	assert_eq!(5, f.neighbours(1,0,&r).len());
	assert_eq!(3, f.neighbours(0,0,&r).len());

	assert_eq!(3, f. neighbours(9,9,&r).len());
	assert_eq!(3, f.neighbours(9,0,&r).len());
	assert_eq!(3, f.neighbours(0,9,&r).len());

	assert_eq!(5, f.neighbours(1,9,&r).len());
	assert_eq!(5, f.neighbours(9,1,&r).len());

}
#[test]
fn neighbour_isnt_burning() {
	let f = Field::new(10,10, 0.05, 0.001);
	for i in -1..1 {
		for j in -1..1 {
			assert_eq!(false, f.has_burning_neighbour(1+i, 2+j));
		}
	}
}
#[test]
fn neighbour_is_burning() {
	let mut f = Field::new(10,10, 0.05, 0.001);
	f.set(&1,&2, Cell::Burning);
	let r = vec![-1,0,1];
	for (x,y) in f.neighbours(1,2,&r) {
		if !f.has_burning_neighbour(x,y) {
			assert!(false);
		}
	}
}
#[test]
fn rand_1_always_does() {
	let f = Field::new(10,10, 0.05, 0.001);
	assert_eq!(Cell::Tree, f.rand_cell(Cell::Tree, 1 as f32));
}
#[test]
fn rand_0_never_does() {
	let f = Field::new(10,10, 0.05, 0.001);
	assert_eq!(Cell::Empty, f.rand_cell(Cell::Tree, 0 as f32));
}
#[test]
fn all_cells() {
	let f = Field::new(3,3, 0.05, 0.001);
	assert_eq!(9, f.cells().len());
}
#[test]
fn populate_does() {
	let f = Field::new(9,9, 0.05, 0.001).populate(Cell::Tree, 1 as f32);
	assert!(f.cells().iter().any(|&(x,y)|*f.get(&x,&y) != Cell::Tree));
}
// */
}
