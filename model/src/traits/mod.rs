use std::fmt::Display;

#[allow(unused_variables)]
pub trait TailwindInstance: Display {
    /// New tailwind instance
    fn boxed(self) -> Box<dyn TailwindInstance>
    where
        Self: Sized,
        Self: 'static,
    {
        Box::new(self)
    }

    fn collision_id(&self) -> String;
    fn get_collisions(&self) -> Vec<String>;
}
