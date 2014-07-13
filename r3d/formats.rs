#![feature(globs)]
#![allow(unused_attribute)]
#![feature(default_type_params)]
#![feature(macro_rules)]
#![allow(dead_code)]
#![allow(unused_variable)]
#![allow(unreachable_code)]

use super::vecmath::Vec4;
use super::vecmath::Vec3;
use super::vecmath::*;

// todo: f32->f16 conversion, Vec3<f32>->Vec3<f16>  Vec4<f32> -> Vec4<f16>
type rgba8888=u32;

pub trait To8888 {
	fn to_8888(&self)->u32;
}
impl To8888 for Vec4<f32> { fn to_8888(&self)->u32{ self.pack() }}
impl To8888 for u32 { fn to_8888(&self)->u32{ *self }}
impl To8888 for (int,int,int,int) {
	fn to_8888(&self)->u32{
		let (r,g,b,a)=*self;
		(r|(g<<8)|(b<<16)|(a<<24)) as u32
	}
}
impl To8888 for (f32,f32,f32,f32) {
	fn to_8888(&self)->u32{
		Vec4::<f32>::from_tuple(*self).to_8888()
	}
}

pub trait Pack {
	fn pack(&self)->u32 {self.pack_8888()}

	fn pack_8888(&self)->u32;
	fn pack_s8s8s8s8(&self)->u32;
	fn pack_u10u10u10u2(&self)->u32;
	fn pack_s10s10s10u2(&self)->u32;

	fn pack_1555(&self)->u16;
	fn pack_565(&self)->u16;
	fn pack_4444(&self)->u16;
}

// pixel formats
fn pack_components_sub(vec:&Vec4<f32>,ox:f32,oy:f32,oz:f32,ow:f32,fx:f32,fy:f32,fz:f32,fw:f32, sx:uint,sy:uint,sz:uint,sw:uint)->u32 {
	let Vec4(ref x,ref y,ref z,ref w)=*vec;
	let xx=((*x+ox) * fx) as  u32;
	let yy=((*y+oy) * fy) as  u32;
	let zz=((*z+oz) * fz) as  u32;
	let ww=((*w+ow) * fw) as  u32;
	(xx<<sx)|(yy<<sy)|(zz<<sz)|(ww<<sw)
}
fn pack_components(vec:&Vec4<f32>,fx:f32,fy:f32,fz:f32,fw:f32, sx:uint,sy:uint,sz:uint,sw:uint)->u32 {
	pack_components_sub(vec,0.0f32,0.0f32,0.0f32,0.0f32, fx,fy,fz,fw, sx,sy,sz,sw)
}
fn pack_components_signed_xyz(vec:&Vec4<f32>,fx:f32,fy:f32,fz:f32,fw:f32, sx:uint,sy:uint,sz:uint,sw:uint)->u32 {
	pack_components_sub(vec,1.0f32,1.0f32,1.0f32,0.0f32, fx,fy,fz,fw, sx,sy,sz,sw)
}
fn pack_components_signed(vec:&Vec4<f32>,fx:f32,fy:f32,fz:f32,fw:f32, sx:uint,sy:uint,sz:uint,sw:uint)->u32 {
	pack_components_sub(vec,1.0f32,1.0f32,1.0f32,1.0f32, fx,fy,fz,fw, sx,sy,sz,sw)
}

impl Pack for  Vec4<f32> {
	fn pack_8888(&self)->u32 {
		let scale = 255.0f32;
		return pack_components(self, scale,scale,scale,scale, 0,8,16,24);
	}
	fn pack_s8s8s8s8(&self)->u32 {
		let scale = 127.0f32;
		return pack_components_signed(self, scale,scale,scale,scale, 0,8,16,24);
	}
	fn pack_u10u10u10u2(&self)->u32 {
		let scale = 1023.0f32;
		return pack_components(self, scale,scale,scale,3.0f32, 0,10,20,30);
	}
	fn pack_s10s10s10u2(&self)->u32 {
		let s = 511.0f32;
		return pack_components_signed_xyz(self, s,s,s,3.0f32, 0,10,20,30)
	}
	fn pack_1555(&self)->u16 {
		let s = 31.0f32;
		return pack_components(self, s,s,s,1.0f32, 0,5,10,15) as u16;
	}
	fn pack_565(&self)->u16 {
		let scale = 31.0f32;
		return pack_components(self, 31.0f32, 63.0f32, 31.0f32, 0.0f32, 0,5,10,16) as u16;

	}
	fn pack_4444(&self)->u16 {
		let s = 15.0f32;
		return pack_components(self, s,s,s,s, 0,4,8,112) as u16;
	}
}
// todo: less cut-paste
//32bit packed values=default
pub trait UnPack<V> {
	fn unpack(self)->V{self.unpack_8888()}
	fn unpack_8888(self)->V;
	fn unpack_1010102(self)->V;
	fn unpack_s10s10s10u2(self)->V;
}
pub trait UnPack16<V> {
	fn unpack_1555(self)->V;
	fn unpack_565(self)->V;
	fn unpack_4444(self)->V;
}

//32bit formats
impl UnPack<Vec4<f32>> for u32 {
	fn unpack_8888(self)->Vec4<f32> {
		let r=(self &255) as f32;
		let g=((self>>8) &255 )as f32;
		let b=((self>>16) &255) as f32;
		let a=((self>>24) &255) as f32;
		let scale = 1.0f32/255.0f32;
		return Vec4(r*scale,g*scale,b*scale,a*scale);
	}
	fn unpack_1010102(self)->Vec4<f32> {
		let r=(self &1023) as f32;
		let g=((self>>10) &1023 )as f32;
		let b=((self>>20) &1023) as f32;
		let a=((self>>30) &3) as f32;
		let scale = 1.0f32/1023.0f32;
		return Vec4(r*scale,g*scale,b*scale,a*1.0f32/3.0f32);
	}
	fn unpack_s10s10s10u2(self)->Vec4<f32> {
		let r=(self &1023) as f32;
		let g=((self>>10) &1023 )as f32;
		let b=((self>>20) &1023) as f32;
		let a=((self>>30) &3) as f32;
		let scale = 1.0f32/511.0f32;
		return Vec4(r*scale-1.0f32,g*scale-1.0f32,b*scale-1.0f32,a*1.0f32/3.0f32);
	}
}
// 16bit formats
impl UnPack16<Vec4<f32>> for u16 {
	fn unpack_1555(self)->Vec4<f32> {

		let r=(self &31) as f32 * 1.0f32/31.0f32;
		let g=((self>>5) &31 )as f32 * 1.0f32/31.0f32;
		let b=((self>>10) &31) as f32 * 1.0f32/31.0f32;
		let a=((self>>15) &1) as f32;
		return Vec4(r,g,b,a);
	}
	fn unpack_565(self)->Vec4<f32> {
		let r=(self &31) as f32 * 1.0f32/31.0f32;
		let g=((self>>5) &63 )as f32 * 1.0f32/63.0f32;
		let b=((self>>11) &31) as f32 * 1.0f32/31.0f32;
		return Vec4(r,g,b,1.0f32);
	}
	fn unpack_4444(self)->Vec4<f32> {
		let r=(self &15) as f32 * 1.0f32/15.0f32;
		let g=((self>>4) &15 )as f32 * 1.0f32/15.0f32;
		let b=((self>>8) &15) as f32 * 1.0f32/15.0f32;
		let a=((self>>12) &15) as f32 * 1.0f32/15.0f32;
		return Vec4(r,g,b,a);
	}
}
