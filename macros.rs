#[macro_escape];

/*new file*/  


#[macro_export]
macro_rules! logi{
	($($arg:tt)*)=>( ::log_print(5, format!("{:s}:{:u}: ",file!(),line!())+format!($($arg)*)))
}
#[macro_export]
macro_rules! logw{
	($($arg:tt)*)=>( ::log_print(6, format!("{:s}:{:u}: ",file!(),line!())+format!($($arg)*)))
}


// debug macro: just print the value of an expression, at a specific location
#[macro_export]
macro_rules! logi{
	($($arg:tt)*)=>( ::log_print(5, format!("{:s}:{:u}: ",file!(),line!())+format!($($arg)*)))
}
#[macro_export]
macro_rules! logw{
	($($arg:tt)*)=>( ::log_print(6, format!("{:s}:{:u}: ",file!(),line!())+format!($($arg)*)))
}

// debug macro: just print the value of an expression, at a specific location
#[macro_export]
macro_rules! dump{ ($($a:expr),*)=>
    (   {   let mut txt=format!("{:s}:{:u}: ",file!(), line!());
            $( { txt=txt.append(
                 format!(" {:s}={:?}",  stringify!($a),$a)+";")
                }
            );*;
            ::log_print(5, txt);
        }
    )
}
/*
verify - executes an expression, asserts about the result, and returns it to containing scope.
examples
	verify!(glCreateShader(..)); // fails if it returns 0
	verify!(glCreateShader(..) isnt 0) fails if it returns 0
	verify!(glCreateShader(..) isnt 0) fails if it returns 0
	ptr=verify!(malloc(1024) isnt 0 as *void)  ptr = result of allocation, fails if nullptr

*/
#[macro_export]
macro_rules! verify{
	// no expect value given - just verify its non zero
	( $main_expr:expr)=>(
		{
			let val=$main_expr;
			assert!(val!=0);
			val
		}
	);
	// verify the value is one of several expected values..
	( $main_expr:expr is $($expected_results:expr),*)=>(
		{
			let val=$main_expr;
			assert!($(val==$expected_results ||)* false, stringify!($main_expr)+"=="+val.to_str());
			val
		}
	);
	// verify the value isn't one of several unexpected values
	( $main_expr:expr isnt $($unwanted_result:expr),*)=>(
		{
			let val=$main_expr;
			$(assert!(val!=$unwanted_result, stringify!($main_expr)+"=="+stringify!($unwanted_result)))+;
			val
		}
	)
}

// Define a structure with a scala style constructor.
macro_rules! def_new{
	( struct $struct_name:ident($($arg_name:ident:$arg_type:ident),*) {$($field_name:ident:$field_type:ident=$field_init_expr:expr),*} )=>(
		mod $struct_name {
			pub struct $struct_name {
			$( $field_name: $field_type,)*
			}
			pub fn dump() {
				$(::std::io::println(stringify!($field_name)+":"+stringify!($field_type)); )*
			}
			pub fn new($($arg_name:$arg_type),*)->$struct_name {
				$struct_name {
					$($field_name: $field_init_expr),*
				}
			}
		}
	)
}

/// Defines a vertex structure with embedded  attribute index annotations & GL type enums; 
/// generates an corresponding function to set gl vertex attribute data.
/// TODO: change that to create a data-table.
macro_rules! def_vertex_format{
	( struct $layout_name:ident {
			$($element:ident : [$elem_type:ident($elem_enum:expr),..$elem_count:expr]( $elem_index:expr)  ),*  
		}
	)=>(
//		mod $layout_name {
			pub struct $layout_name {
				$( $element: [$elem_type ,.. $elem_count],)*
			}
			impl $layout_name {
				pub fn set_vertex_attrib() {
					use r3d::rawglbinding::{GLuint,GLfloat,GLsizei,glVertexAttribPointer,glEnableVertexAttribArray};
					use r3d::gl_h_consts::{GL_FLOAT,GL_FALSE};
					use std::intrinsics::size_of;
					use std::libc::c_void;
					$( unsafe {
							let base_vertex = 0 as *$layout_name;
							glEnableVertexAttribArray($elem_index as GLuint);
							glVertexAttribPointer(
								$elem_index as GLuint, 
								$elem_count,
								$elem_enum,	// todo: type -> GL type.
								GL_FALSE, 
								size_of::<$layout_name>() as GLsizei,
								&(*base_vertex).$element as *GLfloat as *c_void,
							);
						}
					);*
				}
			}
//		}	
	)
}

macro_rules! def_vertex_attrib(
	( enum $attrib_group_name:ident { $($attr_name:ident),* } ) =>(
		enum $attrib_group_name {
			$($attr_name),*
		}
		fn bind_vertex_attribs(prog:GLuint) {
			use r3d::rawglbinding::{GLuint,GLfloat,GLsizei,glBindAttribLocation};
			unsafe {
				$(glBindAttribLocation(prog, $attr_name as GLuint, c_str( stringify!($attr_name) ) );
				)*
			}
		}
	)
)


pub fn test() {
	def_new!{
		struct MyStruct(x:int,y:int,z:int) {
			foo:int=x+y+z,
			bar:f32=0.0
		}
	};
// TODO: Vertex layout macro to make a struct, and 'glVertexAttribPointer' calls, seriealizer ..
//	vertex_layout!(MyVertex{pos:[f32=GL_FLOAT,..3] = 0 });
	let f1=MyStruct::MyStruct{foo:1, bar:2.0};
	let f2=MyStruct::new(2,3,4);
	MyStruct::dump();
	dump!(f1);
	dump!(f2);

}
