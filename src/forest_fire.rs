#![feature(collections)]

extern crate rand;

use std::collections::VecMap;
use rand::{random, Open01};

#[derive(Debug,PartialEq,Eq,Clone)]//,PartialOrd,Ord)]
enum Cell {
	Empty,
	Tree,
	Burning,
}
trait ToChar {
	fn to_char(&self) -> char;
}
impl ToChar for Cell {
	fn to_char(&self) -> char {
		match *self {
			Cell::Empty => ' ',
			Cell::Tree => '|',
			Cell::Burning => 'X',
		}
	}
}

type FieldType = VecMap<Cell>;
struct Field {
	cell_: FieldType,
	p_: f32,
	f_: f32,
	x_: usize,
	y_: usize,
}

impl Field {
	fn empty(x: usize, y: usize, f: f32, p: f32) -> Field {
		Field {
			cell_: FieldType::with_capacity(y*x),
			f_: f,
			p_: p,
			x_: x,
			y_: y,
		}
	}

	pub fn new(x: usize, y: usize, f: f32, p: f32) -> Field {
		fn fill(p: usize, mut field: FieldType) -> FieldType {
			field.insert(p, Cell::Empty);
			match p {
				0 => field,
				p => fill(p-1, field)
			}
		}
		let mut out = Field::empty(x,y,f,p);
		out.cell_ = fill(x*y-1, out.cell_);
		out
	}

	fn populate(self, t: Cell, p: f32) -> Field {
		Field {
			cell_ : self.cell_.iter().map(|(key,_)| (key, self.rand_cell(t.clone(),p)) ).collect(),
			p_: self.p_,
			f_: self.f_,
			x_: self.x_,
			y_: self.y_,
		}
	}
	fn step(self) -> Field {
		let mut out = Field::empty(self.x_, self.y_, self.f_, self.p_);
		for (y,c) in self.cell_.iter() {
			out.cell_.insert(y,
				match (c, self.to_pair(y)) {
					(&Cell::Tree, (x,y)) if self.has_burning_neighbour(x,y) => Cell::Burning,
					(&Cell::Tree,    _)  if self.auto_ignites() => Cell::Burning,
					(&Cell::Empty,   _) => self.rand_cell(Cell::Tree, self.f_),
					(&Cell::Burning, _) => self.rand_cell(Cell::Empty, self.p_),
					(c,              _) => c.clone()
				}
			);
		}
		out
	}

	#[cfg(test)]
	fn set(&mut self, x: &usize, y: &usize, c: Cell) {
		let num = self.from_pair(x,y);
		self.cell_.insert( num, c );
	}
	pub fn get(&self, x: &usize, y: &usize) -> &Cell {
		let cell = *y * self.x_ + *x;
		if let Some(ref r) = self.cell_.get( &cell)  {
			return r
		}
		println!("trying for {} {}", *x, *y);
		panic!();
	}
	fn neighbours<'a>(&self, x: usize, y: usize, r: &'a Vec<i16>) -> Vec<(usize,usize)> {
		fn bounded(v: i16, upper:usize) -> Option<usize> {
			match v >= 0 && v < upper as i16  {
				true => Some(v as usize),
				false => None,
			}
		}
		fn repos(i: usize, offset: i16) -> i16 {
			i as i16 + offset
		}

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
							match (bounded(repos(x,a), self.x_), bounded(repos(y,b), self.y_)) {
								(Some(x), Some(y)) => Some( (x,y) ),
								_ => None,
							}
						}
					}).collect()
	}
	pub fn has_burning_neighbour(&self, x: usize, y: usize) -> bool {
		let xs = vec![-1,0,1];
		self.neighbours(x,y,&xs).iter().any(|&(x,y)| self.get(&x,&y) == &Cell::Burning )
	}
	fn auto_ignites(&self) -> bool
	{
		let Open01(val) = random::<Open01<f32>>();
		val < self.p_
	}
	fn rand_cell(&self, c: Cell, p: f32) -> Cell
	{
		match  random::<Open01<f32>>() {
			Open01(val) if val < p => c,
			_=> Cell::Empty,
		}
	}
	fn to_string(&self) -> String {
		let mut out = String::with_capacity((1+self.x_)*self.y_+2);
		for (c,f) in self.cell_.iter() {
			out.push( f.to_char());
			if 0 == (1+c) % self.x_ {
				out.push('\n');
			}
		}
		out
	}
	fn to_pair(&self, z: usize) -> (usize,usize) {
		let y = z/self.x_;
		let x = z%self.x_;
		(x,y)
	}
	fn from_pair(&self, x: &usize, y: &usize ) -> usize {
		y * self.x_ + x
	}
}

#[cfg(not(test))]
fn main() {
	let mut field = Field::new(70,50, 0.1, 0.001).populate(Cell::Tree, 0.3);
	for i in 1..200 {
		field = field.step();
		println!("run {}\n{}", i, field.to_string());
		std::thread::sleep_ms(500);
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

	assert_eq!(8, f. neighbours(5,5,&r).len());

	assert_eq!(5, f.neighbours(0,2,&r).len());
	assert_eq!(5, f.neighbours(0,1,&r).len());
	assert_eq!(5, f.neighbours(1,0,&r).len());
	assert_eq!(3, f.neighbours(0,0,&r).len());

	assert_eq!(3, f.neighbours(9,9,&r).len());
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
	assert_eq!(9, f.cell_.len());
}
#[test]
fn populate_does() {
	let f = Field::new(9,9, 0.0, 0.0).populate(Cell::Tree, 1 as f32);
	assert_eq!(81, f.cell_.len());
	assert!(f.cell_.iter().all(|(_,c)|*c == Cell::Tree));
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
	assert_eq!(9, f.from_pair( &9,&0 ));
	assert_eq!(54, f.from_pair( &4,&5 ));
	assert_eq!(99, f.from_pair( &9,&9 ));
}
#[test]
fn center_burns_all() {
	let mut f = Field::new(3,3, 0.05, 0.001).populate(Cell::Tree, 1 as f32);
	f.set(&1,&1, Cell::Burning);
	let n = f.step();
	assert_eq!(9, n.cell_.len());
	assert!(n.cell_.iter().all(|(c,f)| c==n.from_pair(&1,&1) || *f == Cell::Burning));
}
#[test]
fn fire_burns_out() {
	let f = Field::new(3,3, 0.0, 0.0).populate(Cell::Burning, 1 as f32).step();
	assert!(f.cell_.iter().all(|(_,c)| *c == Cell::Empty));
}
#[test]
fn empties_sprout_trees() {
	let f = Field::new(3,3, 1.0, 0.0).populate(Cell::Empty, 1 as f32).step();
	assert!(!f.cell_.iter().any(|(_,c)| *c == Cell::Empty));
}
#[test]
fn fire_starts_on_its_own() {
	let f = Field::new(3,3, 0.0, 1.0).populate(Cell::Tree, 1 as f32).step();
	assert!(f.cell_.iter().all(|(_,c)| *c == Cell::Burning));
}
#[test]
fn to_string() {
	let mut f = Field::new(3,3, 0.0, 0.0).populate(Cell::Tree, 1 as f32);
	f.set( &0,&0, Cell::Burning);
	let n = f.step();
	assert_eq!(" X|\nXX|\n|||\n", n.to_string());
}

}