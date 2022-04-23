use num_derive::FromPrimitive;

use crate::spheres::Spheres;

#[derive(Debug, FromPrimitive)]
pub enum ProductRecipes {
    NaquiumTesseract1 = 0, // > 12
    NaquiumTesseract2 = 1, // > 11
    WormholeData = 2, // [2, 4, 6, 8]
    NaquiumProcessor1 = 3, // > 11
    NaquiumProcessor2 = 4, // > 10
    SpaceDilation1 = 5, // [1, 3, 4, 4, 5, 7]
    SpaceDilation2 = 6, // [2, 4, 6, 7, 7, 8]
    SpaceFolding1 = 7, // [5, 6, 7, 8]
    SpaceFolding2 = 8, // > 10
    SpaceInjection1 = 9, // [1, 3, 5, 7, 8, 8]
    SpaceInjection2 = 10, // > 10
    SpaceWarping1 = 11, // [3, 4, 5, 6]
    SpaceWarping2 = 12, // [1, 2, 3, 4]
    Singularity1 = 13, // [3, 7]
    Singularity2 = 14, // [2, 6]
}

pub static PRODUCTS: [Spheres; 15] = [
    Spheres {
        // NaquiumTesseract1
        zeta: -1,
        theta: 1,
        gamma: 0,
        omega: 0,
        lambda: -1,
        xi: -1,
        epsilon: 1,
        phi: 1,
    },
    Spheres {
        // NaquiumTesseract2
        zeta: -4,
        theta: 0,
        gamma: 4,
        omega: 4,
        lambda: -4,
        xi: -4,
        epsilon: 0,
        phi: 4,
    },
    Spheres {
        // WormholeData
        zeta: -1,
        theta: 1,
        gamma: -1,
        omega: 1,
        lambda: -1,
        xi: 1,
        epsilon: -1,
        phi: 1,
    },
    Spheres {
        // Processor1
        zeta: -1,
        theta: -1,
        gamma: -1,
        omega: -1,
        lambda: 5,
        xi: 1,
        epsilon: -1,
        phi: -1,
    },
    Spheres {
        // Processor2
        zeta: -1,
        theta: -1,
        gamma: -1,
        omega: -1,
        lambda: 5,
        xi: 1,
        epsilon: -1,
        phi: -1,
    },
    Spheres {
        // SpaceDilation1
        zeta: -2,
        theta: 0,
        gamma: 0,
        omega: -2,
        lambda: 4,
        xi: 0,
        epsilon: 0,
        phi: 0,
    },
    Spheres {
        // SpaceDilation2
        zeta: -2,
        theta: 0,
        gamma: 0,
        omega: -2,
        lambda: 0,
        xi: 0,
        epsilon: 0,
        phi: 4,
    },
    Spheres {
        // SpaceFolding1
        zeta: 2,
        theta: 2,
        gamma: 0,
        omega: 0,
        lambda: -2,
        xi: -2,
        epsilon: 0,
        phi: 0,
    },
    Spheres {
        // SpaceFolding2
        zeta: 4,
        theta: 4,
        gamma: 0,
        omega: 0,
        lambda: -4,
        xi: -4,
        epsilon: 4,
        phi: 4,
    },
    Spheres {
        // SpaceInjection1
        zeta: 0,
        theta: -2,
        gamma: -2,
        omega: 0,
        lambda: 0,
        xi: 0,
        epsilon: 4,
        phi: 0,
    },
    Spheres {
        // SpaceInjection2
        zeta: 2,
        theta: -1,
        gamma: -1,
        omega: 0,
        lambda: 0,
        xi: 0,
        epsilon: 0,
        phi: 0,
    },
    Spheres {
        // SpaceWarping1
        zeta: 2,
        theta: 2,
        gamma: 0,
        omega: 0,
        lambda: 0,
        xi: 0,
        epsilon: -2,
        phi: -2,
    },
    Spheres {
        // SpaceWarping2
        zeta: 0,
        theta: 0,
        gamma: 2,
        omega: 2,
        lambda: 0,
        xi: 0,
        epsilon: -2,
        phi: -2,
    },
    Spheres {
        // Singularity1
        zeta: 1,
        theta: -1,
        gamma: 1,
        omega: -1,
        lambda: 0,
        xi: 0,
        epsilon: 0,
        phi: 0,
    },
    Spheres {
        // Singularity2
        zeta: 0,
        theta: 0,
        gamma: 0,
        omega: 0,
        lambda: -1,
        xi: 1,
        epsilon: -1,
        phi: 1,
    },
];
