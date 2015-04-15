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
trait Coord {
	fn to_pair(&self, z: usize) -> (usize,usize);
	fn from_pair(&self, x: usize, y: usize) -> usize;
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
		let mut out = Field::new(self.x_, self.y_, self.f_, self.p_);
		for (y,_) in self.cell_.iter() {
			out.cell_.insert(y, self.rand_cell(t.clone(),p));
		}
		out
	}
	fn step(&self) -> Field {
		let mut out = Field::new(self.x_, self.y_, self.f_, self.p_);
		println!("Stepping");
		for (y,c) in self.cell_.iter() {
			println!("Step");
			out.cell_.insert(y,
				match (c, self.to_pair(y)) {
					(&Cell::Tree, (x,y)) if self.has_burning_neighbour(x,y) => Cell::Burning.clone(),
					(&Cell::Empty, (_,_)) => self.rand_cell(Cell::Tree, self.p_),
					_ => {println!(".");c.clone()}
				}
			);
		}
		out
	}
	// */
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
							if out_of_bounds {None}
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
impl Coord for Field {
	fn to_pair(&self, z: usize) -> (usize,usize) {
		let y = z/self.x_;
		let x = z%self.x_;
		(x,y)
	}
	fn from_pair(&self, x: usize, y: usize ) -> usize {
		y * self.x_ + x
	}
}
#[cfg(test)]
mod test {
use super::Field;
use super::Cell;
use super::Coord;

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
#[test]
fn to_pair_calculates() {
	let f = Field::new(10,10, 0.05, 0.001);
	assert_eq!( (9,0), f.to_pair(9));
	assert_eq!( (5,4), f.to_pair(45));
	assert_eq!( (9,9), f.to_pair(99));
}
#[test]
fn from_pair_calculates() {
	let f = Field::new(10,10, 0.05, 0.001);
	assert_eq!(9, f.from_pair( 9,0 ));
	assert_eq!(54, f.from_pair( 4,5 ));
	assert_eq!(99, f.from_pair( 9,9 ));
}
#[test]
fn center_burns_all() {
	let mut f = Field::new(3,3, 0.05, 0.001).populate(Cell::Tree, 1 as f32);
	f.set(&1,&1, Cell::Burning);
	let n = f.step();
	assert!(n.cell_.iter().all(|(c,f)| c==n.from_pair(1,1) || *f == Cell::Burning));
}
#[test]
fn fire_burns_out() {
	let f = Field::new(3,3, 0.05, 0.001).populate(Cell::Burning, 1 as f32).step();
	assert!(f.cell_.iter().all(|(_,f)| *f == Cell::Empty));
}
#[test]
fn empties_sprout_trees() {
	let f = Field::new(3,3, 1.0, 0.0).populate(Cell::Empty, 1 as f32).step();
	assert!(f.cell_.iter().all(|(_,f)| *f == Cell::Tree));
}


// */
}