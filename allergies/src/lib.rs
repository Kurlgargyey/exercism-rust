pub struct Allergies(u32);

#[derive(Debug, PartialEq, Eq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

use Allergen::*;

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies(score)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let allergens: Vec<Allergen> = vec![
            Eggs,
            Peanuts,
            Shellfish,
            Strawberries,
            Tomatoes,
            Chocolate,
            Pollen,
            Cats
        ];
        let allergen_value = (2u32).pow(
            allergens
                .iter()
                .position(|a| a == allergen)
                .unwrap() as u32
        );

        self.0 & allergen_value > 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let allergens: Vec<Allergen> = vec![
            Eggs,
            Peanuts,
            Shellfish,
            Strawberries,
            Tomatoes,
            Chocolate,
            Pollen,
            Cats
        ];
        let mut result_vec = vec![];

        for allergen in allergens {
            if self.is_allergic_to(&allergen) {
                result_vec.push(allergen);
            }
        }
        result_vec
    }
}
