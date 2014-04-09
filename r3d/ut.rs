use std::vec::Vec;
use libc::c_void;

/*new file*/  

/// return a reference to a different type at a byte offset from the given base object reference
unsafe fn byte_ofs_ref<'a,X,Y=X,I:Int=int>(base:&'a X, ofs:I)->&'a Y {
	&*byte_ofs_ptr(base,ofs)
}
/// return a raw ptr to a different type at a byte offset from the given base object reference
unsafe fn byte_ofs_ptr<'a,FROM,TO=u8,I:Int=int>(base:&'a FROM, ofs:I)->*TO {
	byte_ofs(base as *_, ofs)
}
/// offsets a raw pointer by a byte amount, and changes type based on return value inference.
unsafe fn byte_ofs<'a,FROM,TO=u8,I:Int=int>(base:*FROM, ofs:I)->*TO {
	(base as *u8).offset( ofs.to_int().unwrap() ) as *TO
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



