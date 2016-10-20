// Wraps libfringe with some unsafeness to be able
// to use a Yielder outside its function.

use fringe::{Generator, Stack};
use fringe::generator::Yielder;
use core::intrinsics::transmute;

pub struct Group<'a, I, O, S: Stack> where I: Send + 'a, O: Send + 'a {
    generator: Generator<'a, I, O, S>,
    yielder: &'a Yielder<I, O>,
}

unsafe fn zero<T: Sized>() -> T {
    ::core::ptr::read(Vec::with_capacity(1).as_ptr())
}

impl<'a, I, O, S: Stack> Group<'a, I, O, S> where I: Send + 'a, O: Send + 'a, S: Stack {

    pub unsafe fn new<'b, F>(f: F, stack: S) -> Group<'b, I, O, S> where F: FnOnce() + Send + 'b {
        // Alternative to the raw pointers and zero(.) is to pass `yielder` with
        // .suspend(.). That still requires a transmute due to life-times. Not worth
        // the hassel...
        let mut yielder_ptr: &usize = &0;
        let mut yielder_usize: usize = transmute(yielder_ptr);
        let mut gen = Generator::unsafe_new(stack, move |yielder, _| {
            *transmute::<_, *mut usize>(yielder_usize) = transmute(yielder);
            //info!("inner yielder at 0x{:x}", *transmute(yielder_usize));
            yielder.suspend(zero());
            f();
        });
        
        gen.resume(zero());
        info!("got yielder at 0x{:x}", *yielder_ptr);
        Group { generator: gen, yielder: transmute(*yielder_ptr)}
    }
    
    // Unsafe because needs to be called in the right thread...
    pub unsafe fn resume(&mut self, i: I) -> Option<O> {
        self.generator.resume(i)        
    }
    
    // Unsafe because needs to be called in the right thread...
    pub unsafe fn suspend(&self, o: O) -> I {
        info!("suspending to yielder at 0x{:x}", self.yielder as *const Yielder<_, _> as usize);
        self.yielder.suspend(o)
    }

}