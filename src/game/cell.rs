
pub struct Cell {
    pub positionX : usize,
    pub positionY : usize,
    pub isAlive : bool,
}

impl Cell {
    pub fn new(positionX: usize, positionY: usize) -> Self {
        Cell {
            positionX: positionX,
            positionY: positionY,
            isAlive: false,
        }
    }

    pub fn isAlive() -> bool {
        isAlive;
    }
}
