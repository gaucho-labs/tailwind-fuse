#[allow(unused_variables)]
pub trait TailwindInstance {
    /// New tailwind instance
    fn boxed(self) -> Box<dyn TailwindInstance>
    where
        Self: Sized,
        Self: 'static,
    {
        Box::new(self)
    }

    /// Unique ID for the instance. Used to determine if two instances collide.
    fn collision_id(&self) -> &'static str;
    /// All IDs that this instance collides with. Does not include [`Self::collision_id()`]
    fn get_collisions(&self) -> Vec<&'static str>;
}
