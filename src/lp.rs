/// A basic Loop structure that holds `start_pos` and `end_pos`.
/// Used to identify loops in the program and describe them.
/// `end_pos` is public and can be mutated.
/// `start_pos` can only be set once at creation.
pub struct Loop {
    start_pos: usize,
    pub end_pos: usize
}

impl Loop {
    pub fn new(pos: usize) -> Loop {
        Loop { start_pos: pos, end_pos: 0 }
    }

    pub fn start_pos(&self) -> &usize {
        &self.start_pos
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_construct_new_loop() {
        let lp = Loop::new(10);

        assert_eq!(10, *lp.start_pos());
    }
}
