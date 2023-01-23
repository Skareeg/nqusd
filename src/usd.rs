use std::sync::{Weak, Arc};

pub struct UsdScene {
    pub layers: Vec<Arc<UsdLayer>>,
}

pub struct UsdLayer {
    pub scene: Weak<UsdScene>,
}

pub struct UsdPrim {
}

pub struct UsdAttribute {
}

pub struct UsdRelationship {
}