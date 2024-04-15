// use std::{time::Instant, sync::Mutex};


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
