#[feature(macro_rules)];
use std::{cast};

// emulating C++ classes using Traits as vtables, could the safety be improved?

pub trait Foo {
	fn foo(&self,i:int);
}

pub struct CppClass<RawDataStruct,TraitInterface> {
	priv vtable_ptr: *(),	// you must not change this :(
							// it is a ptr to the vtable
	data:RawDataStruct
}

impl<Struct,Trait>  CppClass<Struct,Trait> {
	pub fn deref<'a>(&'a self)->&'a Struct { &self.data }
	pub fn as_trait(self)->Trait  {
	    unsafe {
			cast::transmute::<_,Trait>( (self.vtable_ptr, &self.data) )
		}
	}
}



macro_rules! new_class {
	
	($tname:ty for $sname:ident { $($field:ident : $val:expr),* })=>{
		// todo: we can't make initializer list work for arbitrary type,
		// so add another rule to accept a constructor function for non ident type.
		CppClass::<$sname,$tname> {
			vtable_ptr:unsafe {
				let null_obj=cast::transmute::<_,&'static $sname>(0);
				let (vt,_):(*u8,*u8)=
					cast::transmute(null_obj as &'static $tname);
				cast::transmute(vt)
			},

			data: $sname { $($field: $val),* }
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
	let mut b :CppClass<Banana,&'static Foo> = new_class!( Foo for Banana  {x:10} );
	b.data.x=10;
	// can you write to 'vtable_ptr' unsafely?

	b.as_trait().foo(3);



}

