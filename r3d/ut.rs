pub use super::common::*;

/*new file*/  

/// return a reference to a different type at a byte offset from the given base object reference
unsafe fn byte_ofs_ref<'a,X,Y=X,I:Int=int>(base:&'a X, ofs:I)->&'a Y {
	&*byte_ofs_ptr(base,ofs)
}
/// return a raw ptr to a different type at a byte offset from the given base object reference
unsafe fn byte_ofs_ptr<'a,FROM,TO=u8,I:Int=int>(base:&'a FROM, ofs:I)->*const TO {
	byte_ofs(base as *const _, ofs)
}
/// offsets a raw pointer by a byte amount, and changes type based on return value inference.
unsafe fn byte_ofs<'a,FROM,TO=u8,I:Int=int>(base:*const FROM, ofs:I)->*const TO {
	(base as *const u8).offset( ofs.to_int().unwrap() ) as *const TO
}

trait ToVoidPtr {
	/// Get a void pointer for the contents of a collection
	unsafe fn as_void_ptr(&self)->*const c_void;
	/// Get a void pointer for the contents of a collection, with a byte offset
	unsafe fn byte_ofs<I:Int>(&self, ofs:I)->*const c_void;
}
impl<T> ToVoidPtr for Vec<T> {
	unsafe fn as_void_ptr(&self)->*const c_void {
		&self[0] as *const _ as *const c_void
	}
	unsafe fn byte_ofs<I:Int>(&self,ofs:I)->*const c_void {
		self.as_void_ptr().offset(ofs.to_int().unwrap())
	}
}
impl<'a,T> ToVoidPtr for &'a T {
	unsafe fn as_void_ptr(&self)->*const c_void {
		// NOTE special handling of self, self here is &&T, we deref to get &T
		(*self) as *const _ as *const c_void
	}	
	unsafe fn byte_ofs<I:Int>(&self,ofs:I)->*const c_void {
		// NOTE special handling of self, self here is &&T, we deref to get &T
		(*self as *const _ as *const u8).offset(ofs.to_int().unwrap()) as *const c_void
	}
}
impl ToVoidPtr for *const c_void {
	unsafe fn as_void_ptr(&self)->*const c_void {
		// NOTE special handling of self, self here is &&T, we deref to get &T
		*self
	}	
	unsafe fn byte_ofs<I:Int>(&self,ofs:I)->*const c_void {
		// NOTE special handling of self, self here is &&T, we deref to get &T
		(*self as *const u8).offset(ofs.to_int().unwrap()) as *const c_void
	}
}

pub unsafe fn as_void_ptr<T>(ptr:&T)->*const c_void {
	ptr as *const T as *const c_void
}              

pub unsafe fn c_str(s:&str)->*const c_char {
	s.to_c_str().unwrap()
}

pub fn array8_from_fn<T:Copy>(f:|uint|->T)->[T,..8] {
	unsafe {
		let mut v=::std::mem::uninitialized::<[T,..8]>();
		for i in range(0,8) { v[i]=f(i); } 
		v
	}
}

pub fn array4_from_fn<T:Copy>(f:|uint|->T)->[T,..4] {
	unsafe {
		let mut v=::std::mem::uninitialized::<[T,..4]>();
		for i in range(0,4) { v[i]=f(i); } 
		v
	}
}
pub fn array16_from_fn<T:Copy>(f:|uint|->T)->[T,..16] {
	unsafe {
		let mut v=::std::mem::uninitialized::<[T,..16]>();
		for i in range(0,16) { v[i]=f(i); } 
		v
	}
}







