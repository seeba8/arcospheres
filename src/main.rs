pub mod products;
pub mod recipes;
pub mod spheres;

use num_traits::FromPrimitive;
use products::ProductRecipes;

use crate::products::PRODUCTS;

fn main() {
    let mut input = String::new();
    
    println!(
        r#"Select product recipe: 
        NaquiumTesseract1 = 0, // > 12
        NaquiumTesseract2 = 1, // > 12
        WormholeData = 2, // [2, 4, 6, 8]
        NaquiumProcessor1 = 3, // > 12
        NaquiumProcessor2 = 4, // > 12
        SpaceDilation1 = 5, // [1, 3, 4, 4, 5, 7]
        SpaceDilation2 = 6, // [2, 4, 6, 7, 7, 8]
        SpaceFolding1 = 7, // [5, 6, 7, 8]
        SpaceFolding2 = 8, // [5, 5, 6, 6, 7, 7, 7, 7, 8, 8, 8, 8]
        SpaceInjection1 = 9, // [1, 3, 5, 7, 8, 8]
        SpaceInjection2 = 10, // > 12
        SpaceWarping1 = 11, // [3, 4, 5, 6]
        SpaceWarping2 = 12, // [1, 2, 3, 4]
        Singularity1 = 13, // [3, 7]
        Singularity2 = 14, // [2, 6]
        "#
    );
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let product_recipe: ProductRecipes =
        FromPrimitive::from_usize(input.trim().parse().expect("Not an integer"))
            .expect("Invalid recipe");
    let mut spheres = PRODUCTS[product_recipe as usize].clone();
    println!("Recursion Depth: ");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    println!("Recipe numbers are one-based. Result is not necessarily the shortest recipe");
    spheres.balance(
        input
            .trim()
            .parse()
            .expect("Expected number of recursion depth (u8)"),
    );
    println!("Finished");
}

#[cfg(test)]
mod tests {
    use crate::{recipes::FOLDING_RECIPES, spheres::Spheres};

    #[test]
    fn it_applies_recipe() {
        let mut spheres = Spheres::default();
        spheres += &FOLDING_RECIPES[0];
        assert_eq!(
            Spheres {
                zeta: 0,
                theta: 1,
                gamma: 0,
                omega: -1,
                lambda: -1,
                xi: 1,
                epsilon: 0,
                phi: 0,
            },
            spheres
        );

        spheres += &FOLDING_RECIPES[0];
        assert_eq!(
            Spheres {
                zeta: 0,
                theta: 2,
                gamma: 0,
                omega: -2,
                lambda: -2,
                xi: 2,
                epsilon: 0,
                phi: 0,
            },
            spheres
        );

        spheres -= &FOLDING_RECIPES[0];
        assert_eq!(
            Spheres {
                zeta: 0,
                theta: 1,
                gamma: 0,
                omega: -1,
                lambda: -1,
                xi: 1,
                epsilon: 0,
                phi: 0,
            },
            spheres
        );
        spheres -= &FOLDING_RECIPES[0];
        assert_eq!(Spheres::default(), spheres);
    }

    #[test]
    fn it_balances_simple() {
        let mut spheres = Spheres::default();
        spheres -= &FOLDING_RECIPES[6];
        spheres -= &FOLDING_RECIPES[2];
        spheres.balance(2);
    }

    #[test]
    fn it_stops_if_no_result() {
        let mut spheres = Spheres::default();
        spheres -= &FOLDING_RECIPES[6];
        spheres -= &FOLDING_RECIPES[2];
        spheres.balance(1);
    }
}
