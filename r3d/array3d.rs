use std::num::div_rem;
use std::mem::transmute;
//#[deriving(Show)]
//pub type Int3=[int,..3];
pub struct Int3 {i:int,j:int,k:int}

//#[deriving(Show)]
pub struct Array3d<T>{
	size:Int3,
	data:Vec<T>,
}

struct Array3dIter<'a,T> {
	src:&'a Array3d<T>,
	i:int,
	j:int,
	k:int,
	begin:Int3,end:Int3
	
}

fn lin_index_sub(size:Int3, (i,j,k):(int,int,int))->int {
	i+size.i*(j+size.j*k)
}

impl<'a,T> Array3d<T> {
	fn linear_index(&self,ijk:(int,int,int))->int {
		lin_index_sub(self.size, ijk)
	}
	pub fn get(&'a self,ijk:(int,int,int))->&'a T {
		self.data.get(self.linear_index(ijk) as uint)
	}
	pub fn get_mut(&'a mut self,ijk:(int,int,int))->&'a mut T {
		let idx=self.linear_index(ijk);
		self.data.get_mut(idx as uint)
	}
}

impl<'a,T> Array3dIter<'a,T> {
	fn next(&'a self)->Option<((int,int,int),&'a T)> {
		unsafe {
			let mself:&mut Array3dIter<'a,T>= transmute(self);
			if mself.k>=mself.end.k {return None;}
			let ret=((mself.i,mself.j,mself.k), mself.src.get((mself.i,mself.j,mself.k)));
			mself.i+=1;
			if mself.i>=mself.end.i{
				mself.i=mself.begin.i;
				mself.j+=1;
				if mself.j>=mself.end.j{
					mself.j=mself.begin.j;
					mself.k+=1
				}
			}
			return Some(ret);
		}
	}	
}

impl<'a, T> Array3d<T> {
	fn iter(&'a self)->Array3dIter<'a,T>{
//		self.iter_range([0,0,0],self.size)
		Array3dIter::<T> {
			i:0,j:0,k:0,
			begin:Int3{i:0,j:0,k:0},
			end:self.size,
			src:self
		}		
	}
	fn iter_range(&'a self, from:Int3, to:Int3)->Array3dIter<'a,T> {
		Array3dIter::<T> {
			i:from.i,j:from.j,k:from.k,
			begin:from,
			end:to,
			src:self
		}		
	}
}

impl<T:Clone> Array3d<T> {
	fn set_val(&mut self,ijk:(int,int,int),v:&T) {
		*self.get_mut(ijk)=v.clone();
	}
	fn set(&mut self,ijk:(int,int,int),f:||->T) {
		*self.get_mut(ijk)=f();
	}
	fn update<Y>(&mut self, ijk:(int,int,int), f:|&mut T|->Y)->Y {
		f(self.get_mut(ijk))
	}
	fn linear_idx(&self,(i,j,k):(int,int,int))->int{
		(k*self.size.k+j)*self.size.j+i
	}
	fn size(&self)->Int3 {
		self.size
	}
	fn from_fn((si,sj,sk):(int,int,int),generate:|(int,int,int)|->T)->Array3d<T> {
		Array3d::<T>{
			size:Int3{i:si,j:sj,k:sk},
			// the functional syntax of rust is a WHOLE lot more pleasing.
			// todo: low-level, dodging the divides!
			data:Vec::<T>::from_fn(
				(si*sj*sk) as uint,
				|x|{
					let (k,ij)=div_rem(x as uint,(si*sj) as uint);
					let (j,i)=div_rem(ij,si as uint);
					generate((i as int,j as int,k as int))
				}
			)
		}
	}
	fn for_each<R>(&self,
						initial_val:R,
						f:|R,(int,int,int),&T|->R
						)->R {
		let mut acc=initial_val;
		for k in range(0,self.size.k) {
			for j in range(0,self.size.j) {
				for i in range(0,self.size.i) {
					acc=f(acc, (i,j,k), self.get((i,j,k)))
				}
			}
		}
		acc
	}

	fn for_each_mut<R>(&mut self,
						initial_val:R,
						f:|R,(int,int,int),&mut T|->R
						)->R {
		let mut acc=initial_val;
		for k in range(0,self.size.k) {
			for j in range(0,self.size.j) {
				for i in range(0,self.size.i) {
					acc=f(acc, (i,j,k), self.get_mut((i,j,k)))
				}
			}
		}
		acc
	}
}


#[cfg(run)]
fn main() {
	std::io::println("Array3d Test\n");	
	let foo=Array3d::from_fn((4,3,2), |(i,j,k)|i+j*10+k*100);
	println!("3d array iterator")
	{
		let mut it=foo.iter();
		loop {
			match it.next() {
				Some(x)=>println!("{}",x),
				None=>break
			}
		}
	}
	println!("foreach ...")
	foo.for_each(0u32,
		|acc,(i,j,k),val|{
			println!("array[{},{},{}]={}",i,j,k,val);
			acc
		}
	);
}
