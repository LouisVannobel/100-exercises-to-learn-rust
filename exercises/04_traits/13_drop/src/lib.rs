// TODO: implement a so-called "Drop bomb": a type that panics when dropped
//  unless a certain operation has been performed on it.
//  You can see the expected API in the tests below.

pub enum BombState {
    Armed,
    Defused,
}

pub struct DropBomb {
    state: BombState,
}

impl DropBomb {
    pub fn new() -> Self {
        DropBomb { state: BombState::Armed }
    }

    pub fn defuse(&mut self) {
        self.state = BombState::Defused;
    }
}

impl Drop for DropBomb {
    fn drop(&mut self) {
        if let BombState::Armed = self.state {
            panic!("Boom!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_drop_bomb() {
        let bomb = DropBomb::new();
        // The bomb should panic when dropped
    }

    #[test]
    fn test_defused_drop_bomb() {
        let mut bomb = DropBomb::new();
        bomb.defuse();
        // The bomb should not panic when dropped
        // since it has been defused
    }
}
