#[feature(macro_rules)];
use std::{cast};

// emulating C++ classes using Traits as vtables, could the safety be improved?

pub trait Foo {
	fn foo(&self,i:int);
}

pub struct CppClass<RawDataStruct,TraitInterface> {
	priv vtable_ptr: *(),	// you must not change this :(
							// it is a ptr to the vtable
	dat:RawDataStruct
}


impl<Struct,Trait> Deref<Struct> for  CppClass<Struct,Trait> {
	fn deref<'a> (&'a self)->&'a Struct { &self.dat }
}
impl<Struct,Trait> DerefMut<Struct> for  CppClass<Struct,Trait> {
	fn deref_mut<'a> (&'a mut self)->&'a mut Struct { &mut self.dat }
}
/*
// TODO - we can't get this to work.
impl<Struct,T> Deref<T> for  CppClass<Struct,&'static T> {
	fn deref<'a> (&'a self)->&'a T { //&'a self.as_trait() 
		unsafe {cast::transmute::<_,&'a T>( (self.vtable_ptr, &self.dat) )}
	}
}
impl<Struct,T> DerefMut<T> for  CppClass<Struct,&'static T> {
	fn deref_mut<'a> (&'a mut self)->&'a mut T { //&'a self.as_trait() 
		unsafe {cast::transmute::<_,&'a mut T>( (self.vtable_ptr, &mut self.dat) )}
	}
}
*/
//impl<Struct,Trait> DerefMut<Trait> for  CppClass<Struct,Trait> {
//	fn deref_mut<'a> (&'a mut self)->&'a mut Struct { &mut self.data }
//}

impl<Struct,Trait>  CppClass<Struct,Trait> {
	pub fn as_trait<'a>(&'a self)->Trait  {
	    unsafe {
			cast::transmute::<_,Trait>( (self.vtable_ptr, &self.dat) )
		}
	}
	pub fn data<'a>(&'a self)->&'a Struct { &'a self.dat }
	pub fn data_mut<'a>(&'a mut self)->&'a mut Struct { &'a mut self.dat }
}



macro_rules! new_class {
	
	($tname:ty for $sname:ident { $($field:ident : $val:expr),* })=>{
		// todo: we can't make initializer list work for arbitrary type,
		// so add another rule to accept a constructor function for non ident type.
		CppClass::<$sname,& $tname> {
			vtable_ptr:unsafe {
				let null_obj=cast::transmute::<_,&'static $sname>(0);
				let (vt,_):(*u8,*u8)=
					cast::transmute(null_obj as &'static $tname);
				cast::transmute(vt)
			},

			dat: $sname { $($field: $val),* }
		}
	}
}

struct Banana { x:int }
impl Foo for Banana {
	fn foo(&self,i:int) {
		println!("hello from banana.foo {:?} {:?} {:?}",i, self.x, i+self.x);
	}
}


fn main() {
	let mut b :CppClass<Banana,&Foo> = new_class!( Foo for Banana  {x:10} );
	b.dat.x=10;
	// can you write to 'vtable_ptr' unsafely?

	b.as_trait().foo(3);



}

