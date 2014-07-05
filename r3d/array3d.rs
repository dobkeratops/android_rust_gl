
type int3=[int,..3];
pub struct Array3dS<T>{
	size:int3,
	data:Vec3<T>,
}

pub trait Array3d<T> {
	fn get_mut(&self, ijk:int3)->&mut T;
	fn get(&self),ijk:int3)->&T;
	fn set(&mut self,ijk:int3,c:&T) {
		*self.get_mut(ijk)=c;
	}
	fn update<Y>(&mut self, ijk:int3, f:|&mut T|->Y) {
		f(self.get_mut(ijk))
	}
	fn linear_idx([i,j,k]:int3)->int{
		(k*self.size[2]+j)*self.size[1]+i
	}
	fn size(&self)->int3;
	fn resize(&mut self,int3,generate:|int3|->T);
	fn for_each<U,R>(&self,in:R |src:R,ijk:int3,cell:&mut T|->R) {
		let acc=in;
		let [ni,nj,nk]=self.size();
		for k in range(0,nk) {
			for j in range(0,nj) {
				for i in range(0,ni) {
					f([i,j,k], self.get_mut[i,j,k])
				}
			}
		}
	}
	fn map
}

impl<T> Array3d<T> for Array3dS{
	fn get_mut(&mut self,ijk:int3)->&mut U {
		self.data.get_mut(self.linear_idx(ijk))
	}
	fn get(&self,ijk:int3)->&U {
		self.data.get(self.linear_idx(ijk))
	}
	fn size(&self)->int3{self.size}
}
