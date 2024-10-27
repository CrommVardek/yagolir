

pub fn DoesCellRemainAlive(cell: Cell, adjacentCells: Vec<Cell> ) -> Result<boolean, Error> {
    if(!cell.isAlive) {
        return Err("cell is not supposed to be dead here")
    }

    let currentNumberOfAdjacentCellsAlive: u8 = 0;

    for ac in adjacentCells.iter() {
        if(ac.isAlive) {
            currentNumberOfAdjacentCellsAlive += 1;
        }
    }

    return Ok(currentNumberOfAdjacentCellsAlive >= 2);
}

pub fn DoesCellBecomeAlive(cell: Cell, adjacentCells: Vec<Cell> ) -> Result<boolean, Error> {
    if(cell.isAlive) {
        return Err("cell is not supposed to be alive here")
    }

    let currentNumberOfAdjacentCellsAlive: u8 = 0;

    for ac in adjacentCells.iter() {
        if(ac.isAlive) {
            currentNumberOfAdjacentCellsAlive += 1;
        }
    }

    return Ok(currentNumberOfAdjacentCellsAlive >= 3);
}