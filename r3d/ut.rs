use std::libc;
use std::vec_ng::Vec;
use std::libc::c_void;

/*new file*/  

unsafe fn byte_ofs_ref<'a,X,Y,I:Int=int>(base:&'a X, ofs:I)->&'a Y {
	&*byte_ofs_ptr(base,ofs)
}

/// return a raw ptr to a different type at a byte offset from the given base object reference
unsafe fn byte_ofs_ptr<'a,X,Y,I:Int=int>(base:&'a X, ofs:I)->*Y {
	(base as *_ as *u8).offset( ofs.to_int().unwrap() ) as *Y
}

trait ToVoidPtr {
	/// Get a void pointer for the contents of a collection
	unsafe fn as_void_ptr(&self)->*c_void;
	/// Get a void pointer for the contents of a collection, with a byte offset
	unsafe fn byte_ofs<I:Int>(&self, ofs:I)->*c_void;
}
impl<T> ToVoidPtr for Vec<T> {
	unsafe fn as_void_ptr(&self)->*c_void {
		self.get(0) as *_ as *c_void
	}
	unsafe fn byte_ofs<I:Int>(&self,ofs:I)->*c_void {
		self.as_void_ptr().offset(ofs.to_int().unwrap())
	}
}
impl<'a,T> ToVoidPtr for &'a T {
	unsafe fn as_void_ptr(&self)->*c_void {
		// NOTE special handling of self, self here is &&T, we deref to get &T
		(*self) as *_ as *c_void
	}	
	unsafe fn byte_ofs<I:Int>(&self,ofs:I)->*c_void {
		// NOTE special handling of self, self here is &&T, we deref to get &T
		(*self as *_ as *u8).offset(ofs.to_int().unwrap()) as *c_void
	}
}
impl ToVoidPtr for *c_void {
	unsafe fn as_void_ptr(&self)->*c_void {
		// NOTE special handling of self, self here is &&T, we deref to get &T
		*self
	}	
	unsafe fn byte_ofs<I:Int>(&self,ofs:I)->*c_void {
		// NOTE special handling of self, self here is &&T, we deref to get &T
		(*self as *u8).offset(ofs.to_int().unwrap()) as *c_void
	}
}
