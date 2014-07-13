// Conversion Operator
// uses 2way inference and indirection traits
// to implement a generic conversion,
// requires an explicit '.to()' call but infers from context

pub trait To<T> {fn to(&self)->T;}
pub trait IntTo {fn int_to(x:&int)->Self;}
pub trait UIntTo {fn uint_to(x:&uint)->Self;}
pub trait U32To {fn u32_to(x:&u32)->Self;}
pub trait I32To {fn i32_to(x:&i32)->Self;}
pub trait F32To {fn f32_to(x:&f32)->Self;}
pub trait F64To {fn f64_to(x:&f64)->Self;}
pub trait BoolTo {fn bool_to(x:&bool)->Self;}
pub trait U8To {fn u8_to(x:&u8)->Self;}

impl<T:IntTo> To<T> for int {
	fn to(&self)->T { IntTo::int_to(self) }
}
impl<T:UIntTo> To<T> for uint {
	fn to(&self)->T { UIntTo::uint_to(self) }
}
impl<T:U32To> To<T> for u32 {
	fn to(&self)->T { U32To::u32_to(self) }
}
impl<T:I32To> To<T> for i32 {
	fn to(&self)->T { I32To::i32_to(self) }
}
impl<T:F32To> To<T> for f32 {
	fn to(&self)->T { F32To::f32_to(self) }
}
impl<T:F64To> To<T> for f64 {
	fn to(&self)->T { F64To::f64_to(self) }
}
impl<T:U8To> To<T> for u8 {
	fn to(&self)->T { U8To::u8_to(self) }
}
impl<T:BoolTo> To<T> for bool {
	fn to(&self)->T { BoolTo::bool_to(self) }
}

// U32..
impl U32To for u8 {
	fn u32_to(x:&u32)->u8 { *x as u8}
}
impl U32To for i32 {
	fn u32_to(x:&u32)->i32 { *x as i32}
}
impl U32To for u32 {
	fn u32_to(x:&u32)->u32 { *x as u32}
}
impl U32To for f32 {
	fn u32_to(x:&u32)->f32 { *x as f32}
}

impl I32To for u8 {
	fn i32_to(x:&i32)->u8 { *x as u8}
}
impl I32To for i32 {
	fn i32_to(x:&i32)->i32 { *x as i32}
}
impl I32To for u32 {
	fn i32_to(x:&i32)->u32 { *x as u32}
}
impl I32To for f32 {
	fn i32_to(x:&i32)->f32 { *x as f32}
}

impl F32To for u8 {
	fn f32_to(x:&f32)->u8 { *x as u8}
}
impl F32To for int {
	fn f32_to(x:&f32)->int { *x as int}
}
impl F32To for uint {
	fn f32_to(x:&f32)->uint { *x as uint}
}
impl F32To for i32 {
	fn f32_to(x:&f32)->i32 { *x as i32}
}
impl F32To for u32 {
	fn f32_to(x:&f32)->u32 { *x as u32}
}
impl F32To for f32 {
	fn f32_to(x:&f32)->f32 { *x as f32}
}
impl F32To for f64 {
	fn f32_to(x:&f32)->f64 { *x as f64}
}

impl F64To for int {
	fn f64_to(x:&f64)->int { *x as int}
}
impl F64To for i32 {
	fn f64_to(x:&f64)->i32 { *x as i32}
}
impl F64To for u32 {
	fn f64_to(x:&f64)->u32 { *x as u32}
}
impl F64To for f32 {
	fn f64_to(x:&f64)->f32 { *x as f32}
}
impl F64To for f64 {
	fn f64_to(x:&f64)->f64 { *x as f64}
}

impl IntTo for u8 {
	fn int_to(x:&int)->u8 { *x as u8}
}
impl IntTo for i32 {
	fn int_to(x:&int)->i32 { *x as i32}
}
impl IntTo for u32 {
	fn int_to(x:&int)->u32 { *x as u32}
}
impl IntTo for f32 {
	fn int_to(x:&int)->f32 { *x as f32}
}
impl IntTo for f64 {
	fn int_to(x:&int)->f64 { *x as f64}
}
impl IntTo for int {
	fn int_to(x:&int)->int { *x as int}
}
impl IntTo for uint {
	fn int_to(x:&int)->uint { *x as uint}
}

impl UIntTo for u8 {
	fn uint_to(x:&uint)->u8 { *x as u8}
}
impl UIntTo for int {
	fn uint_to(x:&uint)->int { *x as int}
}
impl UIntTo for uint {
	fn uint_to(x:&uint)->uint { *x as uint}
}
impl UIntTo for u32 {
	fn uint_to(x:&uint)->u32 { *x as u32}
}
impl UIntTo for i32 {
	fn uint_to(x:&uint)->i32 { *x as i32}
}



// example use..
pub fn sum<R, A:Add<A,R>+To<B>, B:To<A> >(a:A,b:B)->R{
	a+b.to()
}
