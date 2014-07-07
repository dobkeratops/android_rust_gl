use std::num::div_rem;
use std::mem::transmute;
//#[deriving(Show)]
//pub type Int3=[int,..3];
pub struct Int3 {i:int,j:int,k:int}

//#[deriving(Show)]
pub struct Vec3d<T>{
	size:Int3,
	data:Vec<T>,
}

struct Vec3dIter<'a,T> {
	src:&'a Vec3d<T>,
	i:int,
	j:int,
	k:int,
	begin:Int3,end:Int3
	
}

fn lin_index_sub(size:Int3, (i,j,k):(int,int,int))->int {
	i+size.i*(j+size.j*k)
}

impl<'a,T> Vec3d<T> {
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

impl<'a,T> Vec3dIter<'a,T> {
	fn next(&'a self)->Option<((int,int,int),&'a T)> {
		unsafe {
			let mself:&mut Vec3dIter<'a,T>= transmute(self);
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

type Array4x4x4<T>=[[[T,..4],..4],..4];

// ah, this is as far as we got last time before ..
enum Tile4<T> {
	Fill(T),				// single value across 4x4
	Detail(Box<Array4x4x4<T>>)	// mutated stuff
}


impl<T:Clone+Copy> Clone for Tile4<T> {
	fn clone(&self)->Tile4<T> {
		match *self {
			Fill(ref v)=>Fill(v.clone()),
			Detail(ref b)=>{
				let new_tile=**b;
				Detail(box new_tile )
			}
		}
	}
}


struct TiledArray4<T> {
	tiles:Vec3d<Tile4<T>>,
}
fn fill_array_4<T:Clone>(v:&T)->[T,..4]{ [v.clone(),v.clone(),v.clone(),v.clone()] }
// grr. clone for [T,..4]
// can't impl on intrinsic type, outside of ours.
impl<T:Clone+Copy> TiledArray4<T> {
	fn get<'a>(&'a self,(i,j,k):(int,int,int))->&'a T {
		let (ii,jj,kk)=(i&3,j&3,k&3);
		let tile:&Tile4<T>=self.tiles.get((i/4,j/4,k/4));
		match *tile{
			Fill(ref fill_value)	=> fill_value,
			Detail(ref tile)=> &tile[kk as uint][jj as uint][ii as uint],
		}
	}
	fn set(&mut self, (i,j,k):(int,int,int), new_val:&T) {
		let (ii,jj,kk)=((i&3) as uint,(j&3) as uint,(k&3) as uint);
		let tile:&mut Tile4<T>=self.tiles.get_mut((i/4,j/4,k/4));
	// if the old tile isn't allocated, update it..
		match *tile{
			Detail(ref mut tile)=> {
				tile[kk][jj][ii]=new_val.clone();
				return
			},
			_=>{},
		}

		let old_val=match *tile { Fill(ref old_val)=>old_val.clone(),_=>new_val.clone()};
		let mut new_row:[T,..4]=[old_val, ..4];//fill_array_4(new_val);
		let mut new_rowcol:[[T,..4],..4]=[new_row, ..4];
		let mut new_tile:[[[T,..4],..4],..4]=[new_rowcol, ..4];
		new_tile[kk][jj][ii]=new_val.clone();
		
		*tile=Detail(box new_tile);
	}
	/// Step through tiles, any that are homogenous are reverted to 'fill'
	fn compact(&mut self) {
	}
	fn from_val((numi,numj,numk):(int,int,int), init_val:&T)->TiledArray4<T> {
		TiledArray4::<T>{
			tiles:Vec3d::<Tile4<T>>::from_fn(((numi+3)/4,(numj+3)/4,(numk+3)/4), |(_,_,_)|Fill(init_val.clone()) )
		}
	}
}

impl<'a, T> Vec3d<T> {
	fn iter(&'a self)->Vec3dIter<'a,T>{
//		self.iter_range([0,0,0],self.size)
		Vec3dIter::<T> {
			i:0,j:0,k:0,
			begin:Int3{i:0,j:0,k:0},
			end:self.size,
			src:self
		}		
	}
	fn iter_range(&'a self, from:Int3, to:Int3)->Vec3dIter<'a,T> {
		Vec3dIter::<T> {
			i:from.i,j:from.j,k:from.k,
			begin:from,
			end:to,
			src:self
		}		
	}
}
								   
impl<T:Clone> Vec3d<T> {
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
	fn from_fn((si,sj,sk):(int,int,int),generate:|(int,int,int)|->T)->Vec3d<T> {
		Vec3d::<T>{
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
	fn map<Y:Clone=T>(&self, f:|(int,int,int),&T|->Y)->Vec3d<Y> {
		Vec3d::from_fn((self.size.i,self.size.j,self.size.k),
				|ijk|f(ijk,self.get(ijk)))
	}

	fn fold<R=()>(&self,
						initial_val:R,
						f:|(int,int,int),&T,R|->R
						)->R {
		let mut acc=initial_val;
		for k in range(0,self.size.k) {
			for j in range(0,self.size.j) {
				for i in range(0,self.size.i) {
					acc=f((i,j,k), self.get((i,j,k)),acc)
				}
			}
		}
		acc
	}

	fn fold_mut<R=()>(&mut self,
						initial_val:R,
						f:|(int,int,int),&mut T,R|->R
						)->R {
		let mut acc=initial_val;
		for k in range(0,self.size.k) {
			for j in range(0,self.size.j) {
				for i in range(0,self.size.i) {
					acc=f((i,j,k), self.get_mut((i,j,k)),acc)
				}
			}
		}
		acc
	}
}


#[cfg(run)]
fn main() {
	
	std::io::println("Array3d Test\n");	
	let foo=Vec3d::from_fn((4,3,2), |(i,j,k)|i+j*10+k*100);
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
	foo.fold(0u32,
		|acc,(i,j,k),val|{
			println!("array[{},{},{}]={}",i,j,k,val);
			acc
		}
	);
	let mut ta= TiledArray4::<int>::from_val((8,4,4),&0i);
	ta.set((1,3,2),&999);
	ta.set((6,2,1),&777);

	for k in range(0,4){
		for j in range(0,4){
			print!("(j,k)={}{} ",j,k);
			for i in range(0,8){
				print!("{} ", ta.get((i,j,k)));
			}
			println!("----");
		}
	}
}






