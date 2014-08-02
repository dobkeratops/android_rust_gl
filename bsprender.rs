use r3d::*;
use bsp::*;

// renderable version of a bsp..
pub struct BspRender {
	bsp:Box<Blob<BspHeader>>,
	textures:Vec<GLuint>,
//	tris_per_tex:Vec<Vec<uint>>,
	extents:Extents<Vec3<f32>>,
	centre:Vec3,
	radius:f32
	// vertex/index arrays would go here
}
impl BspRender {
	pub fn new(bsp:Box<Blob<BspHeader>>)->BspRender {
		let mut tex_array=Vec::new();

		bsp.visit_textures( &mut |i,_|{
				let (tx,img)=bsp.get_texture_image(i); 
				let txsize=(tx.width as u32,tx.height as u32);
				let txi=create_texture((tx.width,tx.height), &img,8);
				tex_array.push(txi);
			}
		);
		let (ext,c,r)=bsp.extents();

		// Sort triangles per texture...
		BspRender{
			bsp:bsp,
			textures:tex_array,
			extents:ext,
			centre:c,
			radius:r
		}
	}
}

impl Render for BspRender {
	fn render(&self) {
		let mut curr_texid=0xffff;
		draw_begin(GL_TRIANGLES);
		self.bsp.visit_triangles(
			&mut |_,(v0,v1,v2),(_,txinfo),(_,plane),(face_id,_)| {
				unsafe {
					let txi=txinfo.miptex as uint;
					let (usize,vsize)=self.bsp.get_texture_size(txi);
					let fu=usize as f32; let fv=vsize as f32;
					if curr_texid!=txi {
						draw_end();
						draw_set_texture(0,*self.textures.get(txi));
						curr_texid=txi;
						draw_begin(GL_TRIANGLES);
					}
			
					fn applytx<'a>(tx:&'a TexInfo,(us,vs):(f32,f32),v:&'a BspVec3)->(&'a BspVec3,(f32,f32)){
						let s=1.0f32/64.0f32;
						//let rpos=v.swap_yz();
						(v, ((v3dot(&tx.axis_s,v)+tx.ofs_s)/us,(v3dot(&tx.axis_t,v)+tx.ofs_t)/vs) )
					}
					let scale=1.0f32;//1.0f32/2000.0f32;
					draw_tri_tex_xzy(applytx(txinfo,(fu,fv),v0),applytx(txinfo,(fu,fv),v1),applytx(txinfo,(fu,fv),v2), 0xffffff,scale)
				}
			}
		);
		draw_end();
	}
}
