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
type FieldType = VecMap<VecMap<Cell>>;
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
		fn populate_field(x:usize, width: usize, height: usize, mut field: FieldType) -> FieldType {
			match (height, width) {
			(0,_) => field,
			(h,w) => {
					field.insert(h-1, RowType::with_capacity(x));
					populate_field(x, h-1,w, field)
				}
			}
		}
		Field {
			cell_ : populate_field(x, x,y, FieldType::with_capacity(y)),
			p_: p,
			f_: f,
			x_: x,
			y_: y,
			empty_: Cell::Empty,
		}
	}
	pub fn set(&mut self, x: &usize, y: &usize, c: Cell) {
		if let Some(ref mut r) = self.cell_.get_mut(y)
		{
			r.insert(*x,c);
		}
	}
	pub fn get(&mut self, x: &usize, y: &usize) -> &Cell {
		if let Some(ref r) = self.cell_.get(y) {
			if let Some(ref r) = r.get(x)  {
				return r
			}
		}
		return &self.empty_
	}
	//fn neighbours<'a>(&self, x: usize, y: usize, r: &'a Vec<i16>) -> Vec<(&'a usize,&'a usize)> {
	fn neighbours<'a>(&self, x: usize, y: usize, r: &'a Vec<i16>) -> Vec<(usize,usize)> {
		let mut v: Vec<(usize, usize)> = Vec::new();
		for i in -1..1 {
			for j in -1..1 {
				v.push(i,j);
			}
		}

		v.iter()
			.filter( |pair| match pair {
						&(&0,&0) => false,
						&(&a,&b) if ((x as i16) + a ) < 0 || ((y as i16) + b)  <0 => false,
						_ => true
						})
			.map(|(a,b)| (
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
	for i in r.iter().zip(r.iter()) {
		let (&a,&b) = i;
		println!("full: {} {}",a,b)
	}
	for i in s.iter() {
		let &(a,b) = i;
		println!("stuff: {} {}",a,b)
	}
	assert_eq!(8, s.len());
	let c = f. neighbours(0,0,&r);
	assert_eq!(5, c.len());
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
