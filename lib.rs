#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod player {
    #[ink(storage)]
    pub struct Player {}

    impl Player {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        /// This function will be called during every game round.
        ///
        /// The function returns a `Some((x, y))` coordinate of the pixel which you
        /// want to color.
        ///
        /// # Notes
        ///
        /// The function signature `&mut self` is so that you can retain state
        /// in the contract's storage if you want to.
        ///
        /// The function can be named as you like, but it always needs to have
        /// a defined selector of `0`.
        #[ink(message, selector = 0)]
        pub fn your_turn(&self) -> Option<(u32, u32)> {
            // TODO: return the turn you want to make (x, y)
            Some((0, 0))
        }
    }
}
