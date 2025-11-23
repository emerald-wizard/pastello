// This file reconstructs the nested module structure from the flat files
// generated in src/pb/.

pub mod runecraftstudios {
    pub mod pastello {
        pub mod auth {
            pub mod session {
                pub mod v1 {
                    include!("runecraftstudios.pastello.auth.session.v1.rs");
                }
            }
        }
        pub mod game {
            pub mod puzzle {
                pub mod v1 {
                    include!("runecraftstudios.pastello.game.puzzle.v1.rs");
                }
            }
            pub mod trivia {
                pub mod v1 {
                    include!("runecraftstudios.pastello.game.trivia.v1.rs");
                }
            }
            pub mod session {
                pub mod v1 {
                    include!("runecraftstudios.pastello.game.session.v1.rs");
                }
            }
            pub mod types {
                pub mod v1 {
                    include!("runecraftstudios.pastello.game.types.v1.rs");
                }
            }
        }
        pub mod web {
            pub mod auth {
                pub mod v1 {
                    include!("runecraftstudios.pastello.web.auth.v1.rs");
                }
            }
            pub mod game {
                pub mod v1 {
                    include!("runecraftstudios.pastello.web.game.v1.rs");
                }
            }
        }
    }
}