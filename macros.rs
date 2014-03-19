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

