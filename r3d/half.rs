// Half precision float

pub struct Half(u16);

// conversions

pub trait ToHalf {
	fn to_half(&self)->Half;
}

impl ToHalf for f32 {
	fn to_half(&self)->Half{ fail!();Half(0)}
}

impl ToPrimitive for Half {
	fn to_f32(&self)->Option<f32> {fail!(); None}
	fn to_f64(&self)->Option<f64> {fail!(); None}

	fn to_u8(&self)->Option<u8> {fail!(); self.to_uint().unwrap().to_u8()}
	fn to_u16(&self)->Option<u16> {fail!(); self.to_uint().unwrap().to_u16()}
	fn to_u32(&self)->Option<u32> {fail!(); self.to_uint().unwrap().to_u32()}
	fn to_u64(&self)->Option<u64> {fail!(); None}
	fn to_uint(&self)->Option<uint> {fail!(); None}

	fn to_i8(&self)->Option<i8> {fail!(); self.to_int().unwrap().to_i8()}
	fn to_i16(&self)->Option<i16> {fail!(); self.to_int().unwrap().to_i16()}
	fn to_i32(&self)->Option<i32> {fail!(); self.to_int().unwrap().to_i32()}
	fn to_i64(&self)->Option<i64> {fail!(); None}
	fn to_int(&self)->Option<int> {fail!(); None}
}

// Dont bother with arithmetic, its a storage format only.
