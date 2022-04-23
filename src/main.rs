pub mod products;
pub mod recipes;
pub mod spheres;

use num_traits::FromPrimitive;
use products::ProductRecipes;

use crate::products::PRODUCTS;

fn main() {
    println!(
        r#"Select product recipe: 
        NaquiumTesseract1 = 0,
        NaquiumTesseract2 = 1,
        WormholeData = 2,
        NaquiumProcessor1 = 3,
        NaquiumProcessor2 = 4,
        SpaceDilation1 = 5,
        SpaceDilation2 = 6,
        SpaceFolding1 = 7,
        SpaceFolding2 = 8,
        SpaceInjection1 = 9,
        SpaceInjection2 = 10,
        SpaceWarping1 = 11,
        SpaceWarping2 = 12,
        Singularity1 = 13,
        Singularity2 = 14,
        All = ALL
        "#
    );
    let mut recipe = String::new();
    std::io::stdin()
        .read_line(&mut recipe)
        .expect("Failed to read line");

    println!("Maximum recursion depth: ");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let maximum_depth = input
        .trim()
        .parse()
        .expect("Expected number of recursion depth (u8)");

    println!("Maximum recipe multiplier: ");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let max_multiplier = input
        .trim()
        .parse()
        .expect("Expected number of recursion depth (u8)");

    if "all".eq_ignore_ascii_case(recipe.trim()) {
        for i in 0..15 {
            let product_recipe: ProductRecipes = FromPrimitive::from_usize(i).unwrap();
            println!("{product_recipe:?}");
            balance_recipe(product_recipe, maximum_depth, max_multiplier);
        }
    } else {
        let product_recipe: ProductRecipes =
            FromPrimitive::from_usize(recipe.trim().parse().expect("Not an integer"))
                .expect("Invalid recipe");
        balance_recipe(product_recipe, maximum_depth, max_multiplier);
    }
}

fn balance_recipe(product_recipe: ProductRecipes, maximum_depth: u8, max_multiplier: i16) {
    'outer: for i in 1..=maximum_depth {
        for multiplier in 1..=max_multiplier {
            let mut spheres = PRODUCTS[product_recipe as usize];
            spheres *= multiplier;
            if spheres.clone().balance(i, multiplier) {
                break 'outer;
            }
        }
    }
}
