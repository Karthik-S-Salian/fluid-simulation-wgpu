// pub trait Program {
//     fn dispatch(&self);
// }


fn vec_items_mem_size<T>(&vector:Vec<T>)->usize{
    vector.len()*std::mem::size_of::<T>()
}