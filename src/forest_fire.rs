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
}
