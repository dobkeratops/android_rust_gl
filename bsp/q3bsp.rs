
type V3f=(f32,f32,f23);
type V2f=(f32,f32);
type V3i=(i32,i32,i32);
type V2i=(i32,i32);
type V3b=(u8,u8,u8);
type V2b=(u8,u8);


struct Q3BspFileHeader {
	magic:[u8,..4],
	version:i32,
	entities:Q3DirEntry<Entities>,
	textures:Q3DirEntry<Texture>,
	planes:Q3DirEntry<Plane>,
	nodes:Q3DirEntry<Node>,
	leafs:Q3DirEntry<Leaf>,
	leaf_faces:Q3DirEntry<LeafFaces>,
	models:Q3DirEntry<Models>,
	brushes:Q3DirEntry<Brush>,
	brushsides:Q3DirEntry<BrushSides>,
	vertexes:Q3DirEntry<Vertex>,
	meshverts:Q3DirEntry<MeshVerts>,
	effects:Q3DirEntry<Effects>,
	faces:Q3DirEntry<Faces>,
	lightmaps:Q3DirEntry<LightMaps>,
	lightvols:Q3DirEntry<LightVols>,
	visdata:Q3DirEntry<VisData>
}

struct Q3Bsp<'a> {
	blob:&'a [u8],
	entities:&'a[Entities]
	textures:&'a[Textures],
	planes:&'a[Planes],
	nodes:&'a[Nodes],
	leafs:&'a[Leafs],
	leaf_faces:&'a[LeafFaces],
	models:&'a[Models],
	brushes:&'a[Brushes],
	brushsides:&'a[BrushSides],
	vertexes:&'a[Vertex],
	meshverts:&'a[MeshVerts],
	effects:&'a [Effects],
	faces:&'a [Faces],
	lightmaps:&'a [LightMaps],
	lightvols:&'a [LightVols],
	visdata:&'a [VisData],
}


type LeafFace=i32;
type LeafBrush=i32;
struct Leaf {
	cluster:i32,
	area:i32,
	mins:V3i,
	maxs:V3i,
	leaf_face:i32,
	n_leaf_faces:i32,
	leaf_brush:i32,
	n_leaf_brushes:i32,
}
struct Model {
	min:V3f,
	max:V3f,
	face:i32,
	n_faces:i32,
	brush:i32,
	n_brushes:i32,
}
struct Brush {
	brushside:i32,
	n_brushsides:i32,
	texture:i32,
}
struct BrushSide {
	plane:i32,
	texture;i32,
}
struct Vertex {
	position:V3f,
	texcoord:V2f,
	lightmap:V2f,
	normal:V3f,
	color:u32,
}
type MeshVert=i32;	// "offset relative to first vertex of corresponding face"

struct Effect {
	name:[u8,..64],
	brush:i32,
	unknown:i32
}

enum FaceType {
	FT_Polygon=1i32,
	FT_Patch=2i32,
	FT_Mesh=3i32,
	FT_BillBoard=4i32
}

struct Face {
	texture:i32,
	effect:i32,
	type:i32,
	vertex:i32,
	n_vertexes:i32,
	meshvert:i32,
	n_meshverts:i32,
	lightmap_index:i32,
	lightmap_start:V2i,
	lightmap_size:V2i,
	lightmap_origin:V3f,
	lightmap_s:V3f,
	lightmap_t:V3f,
	normal:V3f,
	size:V2i
}





// todo: we want to encode this as an SH
struct LightVol {
	ambient:V3b,
	directional:V3b,
	dir:V2b,
}
struct VisData {
	n_vecs:i32,
	sz_vecs:i32,
	vecs:[u8,..1],	// actual length is given by DirEntry, also n_vecs*sz_vecs. its uncompressed pvs..
}

struct DirEntry<T,H> {
	ofs:i32,
	length:i32
}
type Q3DirEntry<T>=DirEntry<T,Q3BspHeader>;

fn cast_slice<'a,T,H>(entire_blob:&[u8], H DEntry<T,H>)->&'a [T] {
}

fn fixup_q3bsp<'a> (&'a [u8])->Q3Bsp {
	
	// *** [2] Create slices, fix them up.

}





