#![feature(globs)]
#![allow(unused_attribute)]
#![feature(default_type_params)]
#![feature(macro_rules)]
#![allow(dead_code)]
#![allow(unused_variable)]
#![allow(unreachable_code)]

use super::vecmath::Vec4;
use super::vecmath::Vec3;

// todo: f32->f16 conversion, Vec3<f32>->Vec3<f16>  Vec4<f32> -> Vec4<f16>

pub trait Pack {
	fn pack(&self)->u32 {self.pack8888()}

	fn pack8888(&self)->u32;
	fn pack_u10u10u10u2(&self)->u32;
	fn pack_s10s10s10s2(&self)->u32;

	fn pack1555(&self)->u16;
	fn pack565(&self)->u16;
	fn pack4444(&self)->u16;
}

// pixel formats
impl Pack for  Vec4<f32> {
	fn pack8888(&self)->u32 {
		let scale = 255.0f32;
		let r=(self.x*scale) as u32;
		let g=(self.y*scale) as u32;
		let b=(self.z*scale) as u32;
		let a=(self.w*scale) as u32;
		return r|(g<<8)|(b<<16)|(a<<24);
	}
	fn pack_u10u10u10u2(&self)->u32 {
		let scale = 1023.0f32;
		let r=(self.x*scale) as u32;
		let g=(self.y*scale) as u32;
		let b=(self.z*scale) as u32;
		let a=(self.w*3.0f32) as u32;
		return r|(g<<10)|(b<<20)|(a<<30);
	}
	fn pack_s10s10s10s2(&self)->u32 {
		let scale = 511.0f32;
		let r=((self.x+1.0f32)*scale) as u32;
		let g=((self.y+1.0f32)*scale) as u32;
		let b=((self.z+1.0f32)*scale) as u32;
		let a=(self.w*3.0f32) as u32;
		return r|(g<<10)|(b<<20)|(a<<30);
	}
	fn pack1555(&self)->u16 {
		let scale = 31.0f32;
		let r=(self.x*scale) as u32;
		let g=(self.y*scale) as u32;
		let b=(self.z*scale) as u32;
		let a=(self.w) as u32;
		return (r|(g<<5)|(b<<10)|(a<<15)) as u16;
	}
	fn pack565(&self)->u16 {
		let scale = 31.0f32;
		let r=(self.x*31.0f32) as u32;
		let g=(self.y*63.0f32) as u32;
		let b=(self.z*31.0f32) as u32;
		return (r|(g<<5)|(b<<10)) as u16;
	}
	fn pack4444(&self)->u16 {
		let scale = 15.0f32;
		let r=(self.x*scale) as u16;
		let g=(self.y*scale) as u16;
		let b=(self.z*scale) as u16;
		let a=(self.w*scale) as u16;
		return r|(g<<4)|(b<<8)|(a<<12);
	}
}
//32bit packed values=default
pub trait UnPack<V> {
	fn unpack(self)->V{self.unpack8888()}
	fn unpack8888(self)->V;
	fn unpack1010102(self)->V;
}
pub trait UnPack16<V> {
	fn unpack1555(self)->V;
	fn unpack565(self)->V;
	fn unpack4444(self)->V;
}

//32bit formats
impl UnPack<Vec4<f32>> for u32 {
	fn unpack8888(self)->Vec4<f32> {
		let r=(self &255) as f32;
		let g=((self>>8) &255 )as f32;
		let b=((self>>16) &255) as f32;
		let a=((self>>24) &255) as f32;
		let scale = 1.0f32/255.0f32;
		return Vec4::new(r*scale,g*scale,b*scale,a*scale);
	}
	fn unpack1010102(self)->Vec4<f32> {
		let r=(self &1023) as f32;
		let g=((self>>10) &1023 )as f32;
		let b=((self>>20) &1023) as f32;
		let a=((self>>30) &3) as f32;
		let scale = 1.0f32/1023.0f32;
		return Vec4::new(r*scale,g*scale,b*scale,a*1.0f32/3.0f32);
	}
}
// 16bit formats
impl UnPack16<Vec4<f32>> for u16 {
	fn unpack1555(self)->Vec4<f32> {

		let r=(self &31) as f32 * 1.0f32/31.0f32;
		let g=((self>>5) &31 )as f32 * 1.0f32/31.0f32;
		let b=((self>>10) &31) as f32 * 1.0f32/31.0f32;
		let a=((self>>15) &1) as f32;
		return Vec4::new(r,g,b,a);
	}
	fn unpack565(self)->Vec4<f32> {
		let r=(self &31) as f32 * 1.0f32/31.0f32;
		let g=((self>>5) &63 )as f32 * 1.0f32/63.0f32;
		let b=((self>>11) &31) as f32 * 1.0f32/31.0f32;
		return Vec4::new(r,g,b,1.0f32);
	}
	fn unpack4444(self)->Vec4<f32> {
		let r=(self &15) as f32 * 1.0f32/15.0f32;
		let g=((self>>4) &15 )as f32 * 1.0f32/15.0f32;
		let b=((self>>8) &15) as f32 * 1.0f32/15.0f32;
		let a=((self>>12) &15) as f32 * 1.0f32/15.0f32;
		return Vec4::new(r,g,b,a);
	}
}
