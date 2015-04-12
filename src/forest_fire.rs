#![feature(collections)]

use std::cmp::Eq;
use std::cmp::PartialEq;
use std::collections::VecMap;

#[derive(Debug,PartialEq,Eq)]//,PartialOrd,Ord,Clone)]
enum Cell {
	Empty,
	Tree,
	Burning,
}
type FieldType = VecMap<Cell>;
type RowType = VecMap<Cell>;
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
	pub fn set(&mut self, x: &usize, y: &usize, c: Cell) {
		self.cell_.insert( y*self.x_ + *x, c );
	}
	pub fn get(&mut self, x: &usize, y: &usize) -> &Cell {
		let cell = *y * self.x_ + *x;
		if let Some(ref r) = self.cell_.get( &cell)  {
			return r
		}
		return &self.empty_
	}
	//fn neighbours<'a>(&self, x: usize, y: usize, r: &'a Vec<i16>) -> Vec<(&'a usize,&'a usize)> {
	fn neighbours<'a>(&self, x: usize, y: usize, r: &'a Vec<i16>) -> Vec<(usize,usize)> {
		let mut v: Vec<(&i16, &i16)> = Vec::new();
		for i in r {
			for j in r {
				v.push( (&i , &j ) );
			}
		}
		v.iter()
			.filter( |&pair| match pair {
						&(&0,&0) => false,
							&(&a,&b) if a<0 || b < 0 => (( x as i16) + a ) >= 0 && ((y as i16) + b)  >= 0,
							//&(&a,&b) => ( x  + a ) < self.x_ && (y + b)  < self.y_,
						_ => true
						})
			.map(|&(&a,&b)| (
			     (x as i16 + a) as usize
				,(y as i16 + b) as usize)
		).collect()
	}
	/*pub fn has_burning_neighbour(self, x: usize, y: usize) -> bool {
		let xs = vec![&-1,&0,&1];
		self.neighbours(x,y,&xs).iter().any(|&(&x,&y)| self.get(&x,&y) == &Cell::Burning)
	}*/

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
	let s = f. neighbours(5,5,&r);
	let c = f. neighbours(0,0,&r);
	for i in c.iter() {
		let &(a,b) = i;
		println!("stuff: {} {}",a,b)
	}
	assert_eq!(8, f. neighbours(5,5,&r).len());
	assert_eq!(5, f.neighbours(0,2,&r).len());
	assert_eq!(5, f.neighbours(0,1,&r).len());
	assert_eq!(5, f.neighbours(1,0,&r).len());
	assert_eq!(3, f.neighbours(0,0,&r).len());
	assert_eq!(5, f.neighbours(9,2,&r).len());
	//assert_eq!(3, f. neighbours(0,0,&r).len());

}
/*
#[test]
fn neighbour_is_burning() {
	let mut f = Field::new(10,10, 0.05, 0.001);
	f.set(&1,&2, Cell::Burning);
	for i in -1..1 {
		for j in -1..1 {
			assert_eq!(!(i==0&&j==0), f.has_burning_neighbour(1+i, 2+j));
		}
	}
}
// */
}
