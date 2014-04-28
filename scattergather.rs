#![feature(default_type_params)]
use std::iter::RandomAccessIterator;

pub trait ScatterInto<'a, T,INDEX=uint> {
	fn scatter_into<SRC:Iterator<(INDEX,T)>> (&mut self, src:SRC);
	fn scatter_by_indices<IDXS:Iterator<INDEX>, TS:Iterator<T>> (&mut self, indices:IDXS,values:TS) { self.scatter_into(indices.zip(values) ) }
}

pub trait ScatterIntoGrow<'a, T,INDEX=uint> {
	/// Scatter values into a collection, and grow it if its not large enough
	fn scatter_into_grow<SRC:Iterator<(INDEX,T)>> (&mut self, src:SRC, default_value:&T);
}

// TODO - implement for slice...
impl<'a, T:Clone,INDEX:Int> ScatterInto<'a, T,INDEX> for Vec<T> {
	fn scatter_into<SRC:Iterator<(&'a INDEX,&'a T)>> (&mut self, mut src:SRC) {
		for (index,value) in src {
			let i=index.to_uint().unwrap();
			*(self.get_mut(i))=value.clone();
		}
	}
}
impl<'a, T:Clone,INDEX:Int> ScatterIntoGrow<'a, T,INDEX> for Vec<T> {
	fn scatter_into_grow<SRC:Iterator<(&'a INDEX,&'a T)> >(&mut self, mut src:SRC, default_value:&T) {
		for (index,value) in src {
			let i=index.to_uint().unwrap();
			let len=self.len();
			if i>= len {
				self.grow( 1+i-len, default_value);
			}
			*(self.get_mut(i))=value.clone();
		}
	}
}

pub trait ScatterToVec<'a, T> : Iterator<&'a T> {
	fn scatter_to_vec<I:Int,IS:Iterator<&'a I>>(self, indices:IS, default_value:&T)->Vec<T>;
}

impl<'a, T:Clone, TS:Iterator<&'a T> > ScatterToVec<'a, T> for TS {
	/// Take an iterator of values, scatter by an iterator of indices creating a new vector.
	fn scatter_to_vec<I:Int,IS:Iterator<&'a I>>(self,  indices:IS, default_value:&T)->Vec<T> {
		let mut result = Vec::new();
		let foo : ::std::iter::Zip<IS, TS> = indices.zip(self);
		result.scatter_into_grow(foo, default_value);
		result.shrink_to_fit();
		result
	}

}
trait ScatterValuesToVec<'a, I:Int> : Iterator<&'a I> {
	fn scatter_values_to_vec<T:Clone,TS:Iterator<&'a T>>(self, values:TS, default_value:&T)->Vec<T>;
}

impl<'a, I:Int, IS:Iterator<&'a I> > ScatterValuesToVec<'a, I> for IS {
	/// Take an iterator of values, scatter by an iterator of indices creating a new vector.
	fn scatter_values_to_vec<T:Int,TS:Iterator<&'a T>>(self, values:TS, default_value:&T)->Vec<T> {
		let mut result = Vec::new();
		let foo : ::std::iter::Zip<IS, TS> = self.zip(values);
		result.scatter_into_grow(foo, default_value);
		result.shrink_to_fit();
		result
	}

}



struct IterGather<'a, TS, INDICES> {
	indices: INDICES,	// todo: parameterize index.
	values: TS,		// any collection with a .index() or .get(), whichever it becomes.
}

impl<'a,T, TS:RandomAccessIterator<T>, INDICES:Iterator<&'a uint>> Iterator<T> for IterGather<'a, TS, INDICES> {
	fn next(&mut self)->Option<T> {
		match self.indices.next() {
			None=>None,
			Some(index)=>{
				self.values.idx(*index)
			}
		}
	}
}

pub trait Gather<'a, T,I=uint> : RandomAccessIterator<T> {
	/// yields an iterator that is the values of self permuted by indices from the source iterator  (look up table, shuffle, permute)
	fn gather<IS:Iterator<I>>(self, indices: IS )->IterGather<'a,  Self,IS>;
}

impl<'a, T,I, TS: RandomAccessIterator<T> > Gather<'a,T,I> for TS
{
	fn gather<IS:Iterator<I>>(self, iter_indices: IS)->IterGather<'a, TS, IS> {
		IterGather{ indices:iter_indices, values:self,  }
	}
}

pub trait GatherFrom<'a, I:Int,T,TS:RandomAccessIterator<T>> : Iterator<&'a I> {
	/// Given an iterator of indices, lookup values from a given random-acess iterator, yield an iterator over the results
	fn gather_from<'a>(self, values: TS )->IterGather<'a,  TS,Self> {
		values.gather(self)
	}
}

impl<'a, T,I:Int, IS:Iterator<&'a I>,TS:RandomAccessIterator<T> >  GatherFrom<'a, I,T,TS> for IS
{
}


//values.scatter(indices)
//indices.scatter_from(values)


fn write_and_grow<T:Clone, I:Int>(mut dst:&Vec<T>,at_index:I,value:&T, default:&T) {
	if at_index.to_uint().unwrap() >= dst.len() {
		dst.grow(at_index.to_uint().unwrap() +1 - dst.len(), default);
	}
	*dst.get_mut(at_index.to_uint().unwrap()) = value.clone();
}


pub struct BucketVec<I,T> {
	first_per_j:Vec<I>,
	num_per_j:Vec<I>,
	values:Vec<T>
}
impl<I,T> BucketVec<I,T> {
	pub fn get_num_buckets(&self)->uint{self.first_per_j.len()}
	pub fn get_num(&self, j:uint)->uint { *self.num_per_j.get(j)}
	pub fn get(&self,j:uint, i:uint)->&T {
		self.values.get( *self.first_per_j.get(j) + i)
	}
	pub fn iter(&self)->BucketVecIterBucket<I,T> {
		return BucketVecIterBucket{
			src:self, curr_j:0
		}
	}
}
pub struct BucketVecIterBucket<'a, I,T> {
	src:&'a BucketVec<I,T>, curr_j:uint
}
impl<'a, I,T> BucketVecIterBucket<'a, I,T> {
	fn next(&'a mut self)->Option<&'a [T]> {
		if self.curr_j >= self.get_num_buckets() {
			None
		} else {
			let x0=self.first_per_j.get(self.curr_j);
			let x1=x0+self.num_per_j.get(self.curr_j);
			self.curr_j+=1;
			Some( self.values.slice(x0,x1))
		}
	}
}


/// Scatter values into buckets, with minimal allocations, by prepasses that count.
fn scatter_into_buckets<I:Int,T:Clone>(scatter_indices:Vec<I>, values:Vec<T>, default:&T)-> BucketVec<I,I> {
	// TODO: version of this which scatters indices to create gather indices,
	// then yields an iterator-of-iterators over the src values

	// 'j' = bucket index; 
	let mut max_j=0;
	// find the larget output index..
	for x in scatter_indices.iter() {
		max_j = ::std::cmp::max(max_j,x);
	}

	// count the number of elements per output bucket
	let mut num_per_j=Vec::new(max_j,0);
	for x in scatter_indices.iter() {
		*num_per_j.get_mut(x) +=1 ;
	}

	// allocate base index per bucket ...
	let mut first_per_j=Vec::new(max_j,0);
	let mut main_index=0;
	for x in range(0,max_j) {
		*first_per_j.get(x)=main_index;
		let write_index=main_index;
		main_index+=*num_per_j.get(x);
		*num_per_j.get_mut(x)=write_index;	// we re-use this index
	}

	// scatter the values into the output vector.. TODO:this wants to be unsafe - we avoid allocating temporary.
	let mut values: Vec<T> =Vec::from_elem(max_j, default);
	for (i,ref v) in scatter_indices.iter().zip(values.iter()) {
		let x=*num_per_j.get_mut(i);
		*values.get_mut(x) = v.clone();
		*num_per_j.get_mut(i)=x+1;
	}
	// reset 'num' indices
	for i in range(0,max_j) {
		*num_per_j.get_mut(i) -= *num_per_j.get(i);
	}

	// construct the buckets object..
	// TODO write out a permute of source indices aswell.
	BucketVec{
		first_per_j:first_per_j,
		num_per_j:num_per_j,
		values:values
	}
	
}


fn main() {
	let indices=vec!(3u,2u,1u,0u);
	let indices2=vec!(7u,6u,3u,0u);
	let src=vec!("zero","one","two","three");
	let result: Vec<&'static str> =Vec::from_elem(4,&"");
	std::io::print("Output of scatter:\n");
	println!("{}",result);

	let mut result2: Vec<&'static str> =Vec::new();
	result2.scatter_into_grow( indices2.iter().zip(src.iter()), &&"empty" );
	std::io::print("Output of scatter_grow:\n");
	println!("{}",result2);

	std::io::print("Scatter Iterator:\n");
	for (i,x) in src.iter().scatter_to_vec(indices2.iter(),&&"-").iter().enumerate() {
		println!("scatter result {}={}",i,x);
	}

	std::io::print("Gather iterator:\n");

	for x in src.iter().gather(indices.iter()) {
		println!("{}", x);
	}
	for x in indices.iter().gather_from(src.iter()) {
		println!("{}", x);
	}
	std::io::print("\n");
}
