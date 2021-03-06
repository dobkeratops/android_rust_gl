extern crate rustrt;
extern crate core;
extern crate sync;

use std::task;
use std::io;
use std::mem;
use std::cmp;
use rustrt::thread;
use rustrt::thread::Thread;
use std::io::timer::Timer;
use core::atomics;
use sync::{Mutex,Arc};


// data-parallel.
// Run the given closure across worker tasks for 0..n, ideally task one per core.
// they use a shared atomic to count the index up. avoids spaning N actual tasks
// when they will have shared data (?)

fn num_worker_cores()->int{4}
#[deriving(Send)]
fn par_for_n(num:int,f:|int|) {
	// TODO: do we need to manage a task-pool, or is the rust runtime sufficient?
	// dont make more tasks than needed. dont spawn task on core for just one shot
	let num_workers=cmp::min(std::rt::default_sched_threads() as int,(num/2)); 
	// TODO: Give that we block on the tasks,
	// could these be stack objects.
	// TODO: task failiure would lock this up :(
	//
	// is there a way to express task lifetime with raii?
	// then it could all be safe code

	let mut job_index = Arc::new(Mutex::new(num));
	let mut active_workers = Arc::new(Mutex::new(num_workers));
	unsafe{
		io::println("BEGIN PARALLEL JOBS{");
		let pf:int=mem::transmute (&f);  // cant send raw pointer to closure
		for i in range(0,num_workers) {	
			let mut my_job_index=job_index.clone();
			let mut p_active_workers=active_workers.clone();
			let the_proc=proc() // todo- want raii based..
			{
				loop{
					let job=
					{
						//TODO helper function, .predec() ??
						let mut lji=my_job_index.lock(); 
						let ji=*lji-1;
						*lji=ji;
						ji
					};
					if job<0{
						*p_active_workers.lock()-=1;
						break;
					}
					println!("thread {} has job {}..",i,job);
					unsafe {
						let ff:*|int| = mem::transmute(pf);
						(*ff)(job);
					}
					// done.
				}
			};
			if i<(num_workers-1){	// 0..n-1 , spawn background task 
				task::spawn(the_proc);
				//io::println("1");
				//let tsk=(box std::rt::task::Task::new()).run(the_proc);
				//io::println("2");
			}
			else{
				io::println("3");
				the_proc();	// n-1:  we run it on this task.
				io::println("4");
			}
			io::println("5");
		}
		// wait for the tasks to complete.
		// TODO - instead of this manual counting, can we sync on a number of tasks?
		io::println("main thread waiting for workers..");
		loop {
			if *active_workers.lock()<=0 { break }
		}
		io::println("}ALL JOBS DONE");
	}
}

// todo: stuff these in a par:: module surely.
// parallel from_fn
// todo: parallel map
// todo: parallel filter_map - 2pass/ordered.
// todo: parallel map-reduce.
// todo: parallel find_best(ts,score_fn)->(&T,score)
// etc..

fn par_for<T>(ts:&mut Vec<T>, f:|&T|) {
	fail!("todo");
	par_for_n(ts.len()as int, |i|{
		f(ts.get(i as uint));
	});
}
fn par_map<T,U>(src:&Vec<T>, out:&mut Vec<U>, f:|&T|->U) {
	fail!("todo");
	// todo: it needs EMPLACE BACK ?
//	out.resize(src.len());
//	par_for_n(src.len() as int, |i|{
//		*out.get_mut(i as uint) = f(src.get(i));
//	});
}
fn par_from_fn<T>(out:&mut Vec<T>,num:int, f:|int|->T) {
	fail!("todo");
	// todo: it needs EMPLACE BACK ?
//	out.resize(num);
//	par_for_n(num, |i|{
//		*out.get_mut(i as uint) = f(i);
//	});
}
fn par_filter_map<T,U,X>(src:&Vec<T>, out:&mut Vec<U>,pred:|&T|->(bool,X), f:|&T,&X|->U) {
	fail!("todo");
}

//fn par_map_reduce(

// test main..
fn main() {
	std::io::println("parallel test");
	let mut timer=io::timer::Timer::new().unwrap();

	let mut n=0;
	while n<10 {
		io::println("render\n");
		//todo: block until the job is done
			//todo:todo: sequential jobs
			// todo: add to joblist q;
		par_for_n(
				10,
				|i:int|->(){
					println!("doing job {}",i);
					io::timer::sleep((100+ (i*127)+if i==8{2000}else{0}) as u64);
				}
		);

		io::timer::sleep(1200);
		n+=1;
	};
	std::io::println("done");
}
