use std::{io,mem,raw,cast};


struct BlobContainingSlices<'a>  {
	data:~[u8],
	subset1: &'a [u32],
	subset2: &'a [u16],
}

fn slice_within_as<TO,FROM>(owner:&[FROM], start:uint, len_in_new_units:uint)->&[TO] {
	let len_bytes = len_in_new_units * mem::size_of::<TO>();
	let owner_bytes = owner.len()*mem::size_of::<FROM>();
	let ofs_bytes = start * mem::size_of::<FROM>();
	println!("{} {} {} {}", len_bytes, ofs_bytes, owner_bytes, len_bytes+ofs_bytes)
	assert!(ofs_bytes>=0 && ofs_bytes<= owner_bytes && (ofs_bytes+len_bytes)<=owner_bytes);

	let slice=raw::Slice::<TO> {
		data:&owner[start]as*_ as*TO,
		len:len_in_new_units
	};
	unsafe {
		cast::transmute(slice)
	}
	
}

impl<'a> BlobContainingSlices<'a> {
	fn new()->BlobContainingSlices<'a> {
		let blob=~[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15];
		// todo - as_slice_at_bytes_as<T>(owning_blob, byte_offset, length)
		// safe by virtue of assert on size vs end..
		let slice1:&[u32]=slice_within_as(blob,4,2);
		let slice2:&[u16]=slice_within_as(blob,4,2);
		// todo - can we infer the type from subset decl, cast::transmute loses the info
		unsafe {
			BlobContainingSlices {
				data: blob,
				subset1: cast::transmute(slice1),
				subset2: cast::transmute(slice2),
			}
		}
	}
}



fn main() {
	let a=BlobContainingSlices::new();

	println!( "enclosing owner object {:?}",a.data);
	println!( "size of data  element {}",mem::size_of_val(&a.data[0]));
	println!( "size of subset1 element {}",mem::size_of_val(&a.subset1[0]));
	println!( "size of subset2 element {}",mem::size_of_val(&a.subset2[0]));
	println!( "first subset1[0],[1] {:x} {:x}",a.subset1[0],a.subset1[1]);
	println!( "second subset2[0],[1] {:x} {:x}",a.subset2[0],a.subset2[1]);
	println!( "whole subset1 {:?}",a.subset1);
	println!( "whole subset2 {:?}",a.subset2);

	let mut f=io::File::open(&Path::new("foo.txt"));
	let x= f.read_le_f32().unwrap_or(0.0);
	let x = f.read_le_f32().unwrap_or(0.0);
	io::println("%x");
}


