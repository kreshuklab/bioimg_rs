// use std::{time::Instant, sync::Mutex};


pub fn run_task(target: impl FnOnce() + 'static + Send){
    #[cfg(not(target_arch = "wasm32"))]
    std::thread::Builder::new()
        .name("model_builder_background_task".into())
        .spawn(target)
        .expect("Could not spawn a thread");
}

// pub struct GenerationalMutex<T>(Mutex<(T, Instant)>);

// impl<T> GenerationalMutex<T>{
//     pub fn new(value: T) -> Self{
//         Self(
//             Mutex::new( (value, Instant::now()) )
//         )
//     }

//     pub fn generation(&self) -> Instant{
//         self.0.lock().unwrap().1
//     }

//     pub fn set_if_not_stale(&self, value: T, generation: Instant) {
//         let mut guard = self.0.lock().unwrap();
//         if guard.1 == generation{
//             (*guard).0 = value
//         }
//     }

//     pub fn lock(&self) -> &T{
//         let a = self.0.lock().unwrap();
//         &a.0
//     }
// }
