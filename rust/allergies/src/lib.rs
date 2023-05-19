pub struct Allergies(u32);

macro_rules! enum_iter {
    ($name: ident { $($variant: ident),* } ) => {
        #[derive(Debug, PartialEq, Eq, Clone)]
        pub enum $name {
            $( $variant, )*
        }

        impl $name {
            fn variants() -> Vec<$name> {
                vec![$( $name::$variant, )*]
            }
        }
    };
}

enum_iter!(Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats
});

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies(score)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        (self.0 & 1_u32 << allergen.clone() as u32) != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        Allergen::variants()
            .iter()
            .filter_map(|allergen| self.is_allergic_to(allergen).then_some(allergen.clone()))
            .collect()
    }
}
