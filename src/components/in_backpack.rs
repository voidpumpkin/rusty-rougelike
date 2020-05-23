use specs::{Component, DenseVecStorage, Entity};

#[derive(Component, Debug, Clone)]
pub struct InBackpack {
    pub owner: Entity,
}
