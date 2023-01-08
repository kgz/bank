

// pub fn route() {
//     println!("route")
// }
// // macro_rules! route {
// //     ($path:expr) => {
// //         println!("route: {}", $path);
        
// //     };
// // }
// // #[macro_use]
// // pub(crate) use route;
// mod inner  {
//     pub fn route() {
//         println!("admin route")
//     }

//     pub(crate) use route;
// }

// #[inner::route]
#[allow(dead_code)]
pub fn admin_test() -> String{

    return "admin_test".to_string();

}
