



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
type LeafFace=i32;
type LeafBrush=i32;
struct Leaf {
	cluster:i32,
	area:i32,
	mins:(i32,i32,i32),
	maxs:(i32,i32,i32),
	leaf_face:i32,
	n_leaf_faces:i32,
	leaf_brush:i32,
	n_leaf_brushes:i32,
}
struct Model {
	min:(f32,f32,f32),
	max:(f32,f32,f32),
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
	position:(f32,f32,f32),
	texcoord:(f32,f32),
	lightmap:(f32,f32),
	normal:(f32,f32,f32),
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
	lightmap_start:(i32,i32),
	lightmap_size:(i32,i32),
	lightmap_origin:(f32,f32,f32),
	lightmap_s:(f32,f32,f32),
	lightmap_t:(f32,f32,f32),
	normal:(f32,f32,f32),
	size:(i32,i32)
}

// todo: we want to encode this as an SH
struct LightVol {
	ambient:(u8,u8,u8),
	directional:(u8,u8,u8),
	dir:(u8,u8),
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

fn load_q3bsp(filename:&Path) {
	
}





