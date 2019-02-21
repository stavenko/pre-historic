use amethyst::ecs::{ Component, DenseVecStorage };

#[derive(Hash, PartialEq, Eq)]
pub enum Race {
  Euro,
  Afro,
  Asian,
  Indean,
}

#[derive(Hash, PartialEq, Eq)]
pub enum Complex {
  Skinny,
  Obese,
  Athletic
}

#[derive(Hash, PartialEq, Eq)]
pub enum Sex {
  Male,
  Female
}


#[derive(Hash, PartialEq, Eq)]
pub enum Spece {
  Wolf(Sex),
  Human(Sex, Race, Complex)
}

impl Component for Spece {
  type Storage = DenseVecStorage<Self>;
}
