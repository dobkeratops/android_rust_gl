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
macro_rules! def_struct{
	( $struct_name:ident($($arg_name:ident:$arg_type:ident),*) {$($field_name:ident:$field_type:ident=$field_init_expr:expr),*} )=>(
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


macro_rules! vertex_layout{
	($layout_name:ident{
			$($element:ident: [$elem_t:ident,..$elem_count:expr] = $elem_index:expr ),*  
		}
	)=>{
		mod layout_name {
			struct $layout_name {
				$( $element: [$elem_t ,.. $elem_dim] ),*
			}
			fn set_gl_attrib() {
				$( unsafe {
						let base_vertex = 0 as *$layout_name;
						glVertexAttribPointer(
							$index as GLuint, 
							$elem_dum,
							$elem_type,	// todo: type -> GL type.
							GL_FALSE, 
							$layout_name::size_of::<>(),
							&(*base_vertex).$element as *GL_FLOAT as *std::libc::c_void,
						);
					}
				);*
			}
		}
	}
}

pub fn test() {
	def_struct!(
		MyStruct(x:int) {
			foo:int=x,
			bar:f32=0.0
		}
	);
// TODO: Vertex layout macro to make a struct, and 'glVertexAttribPointer' calls, seriealizer ..
	vertex_layout!(MyVertex{pos:[GL_FLOAT,..3] = 0; });
	let f1=MyStruct::MyStruct{foo:1, bar:2.0};
	let f2=MyStruct::new(2);
	MyStruct::dump();
	dump!(f1);
	dump!(f2);

}
