#![macro_escape]

macro_rules! logi{
	($($arg:tt)*)=>( ::log_print(5, format!("{:s}:{:u}:\t{:s}",file!(),line!(),format!($($arg)*).as_slice()).as_slice()))
}

macro_rules! logw{
	($($arg:tt)*)=>( ::log_print(6, format!("{:s}:{:u}:\t{:s}",file!(),line!(),format!($($arg)*).as_slice())).as_slice())
}


// debug macro: just print the value of an expression, at a specific location

macro_rules! logi{
	($($arg:tt)*)=>( ::log_print(5, format!("{:s}:{:u}:\t{:s}",file!(),line!(),format!($($arg)*).as_slice()).as_slice()))
}


macro_rules! logw{
	($($arg:tt)*)=>( ::log_print(6, format!("{:s}:{:u}:\t{:s}",file!()  ,line!(),format!($($arg)*).as_slice()).as_slice()))
}

// debug macro: just print the value of an expression, at a specific location

macro_rules! dump{ ($($a:expr),*)=>
    (   {   let mut txt=format!("{:s}:{:u}:\t",file!(), line!());
            $( { txt=txt.append(
                 format!(" {:s}={:?};",  stringify!($a),$a).as_slice() )
                }               );*;
            ::log_print(5, txt.as_slice());
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
			assert!($(val==$expected_results ||)* false, stringify!($main_expr).append("==").append(val.to_str()));
			val
		}
	);
	// verify the value isn't one of several unexpected values
	( $main_expr:expr isnt $($unwanted_result:expr),*)=>(
		{
			let val=$main_expr;
			$(assert!(val!=$unwanted_result, format!("{}{}{}",stringify!($main_expr),"==",stringify!($unwanted_result)) ))+;
			val
		}
	)
}

// Define a structure with a scala style constructor.
macro_rules! def_new{
	( struct $struct_name:ident($($arg_name:ident:$arg_type:ident),*) {$($field_name:ident:$field_type:ident=$field_init_expr:expr),*} )=>(
		mod $struct_name {
			pub struct $struct_name {
			$( pub $field_name: $field_type,)*
			}
			pub fn dump() {
				$(::std::io::println(format!("{}{}{}",stringify!($field_name),":",stringify!($field_type)).as_slice()); )*
			}
			pub fn new($($arg_name:$arg_type),*)->$struct_name {
				$struct_name {
					$($field_name: $field_init_expr),*
				}
			}
		}
	)
}

macro_rules! forr {
	( ($($it:ident=$it_init:expr),* ; $condition:expr ; $($inc:expr),*)  $($e:expr);*)=>
	{
		{
			$(let mut $it=$it_init);*;
			while $condition {
				let ret={$($e);*};
				$($inc);*;
			}
		}
		
	}
}



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

	forr!{(i=0u; i<10; i+=1)
		dump!(i)
	}

}
