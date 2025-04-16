#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    X,
    O,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TicTacField {
    board: [[Option<Player>; 3]; 3],
    current_player: Player,
}
impl Default for TicTacField {
    fn default() -> Self {
        TicTacField {
            board: [[None; 3]; 3],
            current_player: Player::X,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GameState {
    WinX,
    WinY,
    WinBoth,
    #[default]
    GameOn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    InvalidMove,
    GameOver,
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidMove => write!(f, "Invalid move"),
            Error::GameOver => write!(f, "Game over"),
        }
    }
}

pub fn analyze(field: &TicTacField) -> GameState {
    let mut win_x = false;
    let mut win_y = false;
    // Check rows
    for i in 0..3 {
        win_x |= field.board[i] == [Some(Player::X); 3];
        win_y |= field.board[i] == [Some(Player::O); 3];
    }
    // Check columns
    for j in 0..3 {
        win_x |= [field.board[0][j], field.board[1][j], field.board[2][j]] == [Some(Player::X); 3];
        win_y |= [field.board[0][j], field.board[1][j], field.board[2][j]] == [Some(Player::O); 3];
    }
    // Check diagonals
    win_x |= [field.board[0][0], field.board[1][1], field.board[2][2]] == [Some(Player::X); 3];
    win_y |= [field.board[0][0], field.board[1][1], field.board[2][2]] == [Some(Player::O); 3];
    win_x |= [field.board[0][2], field.board[1][1], field.board[2][0]] == [Some(Player::X); 3];
    win_y |= [field.board[0][2], field.board[1][1], field.board[2][0]] == [Some(Player::O); 3];
    if win_x && win_y {
        GameState::WinBoth
    } else if win_x {
        GameState::WinX
    } else if win_y {
        GameState::WinY
    } else {
        GameState::GameOn
    }
}

pub fn make_move(
    field: &TicTacField,
    x: u32,
    y: u32,
    player: Player,
) -> Result<TicTacField, Error> {
    if x > 2 || y > 2 {
        return Err(Error::InvalidMove);
    }
    if field.board[x as usize][y as usize].is_some() {
        return Err(Error::InvalidMove);
    }
    if field.current_player != player {
        return Err(Error::InvalidMove);
    }
    if analyze(field) != GameState::GameOn {
        return Err(Error::GameOver);
    }
    let mut new_field = field.clone();
    new_field.board[x as usize][y as usize] = Some(player);
    new_field.current_player = match player {
        Player::X => Player::O,
        Player::O => Player::X,
    };
    Ok(new_field)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze() {
        let mut field = TicTacField::default();
        field.board[0][0] = Some(Player::X);
        field.board[0][1] = Some(Player::X);
        field.board[0][2] = Some(Player::X);
        field.board[1][2] = Some(Player::O);
        field.board[2][2] = Some(Player::O);
        assert_eq!(analyze(&field), GameState::WinX);

        field = TicTacField::default();
        field.board[0][0] = Some(Player::X);
        field.board[0][1] = Some(Player::X);
        field.board[2][0] = Some(Player::X);
        field.board[2][1] = Some(Player::X);
        field.board[1][0] = Some(Player::O);
        field.board[1][1] = Some(Player::O);
        field.board[1][2] = Some(Player::O);
        assert_eq!(analyze(&field), GameState::WinY);

        field = TicTacField::default();
        field.board[0][0] = Some(Player::X);
        field.board[1][1] = Some(Player::X);
        field.board[2][2] = Some(Player::X);
        field.board[0][1] = Some(Player::O);
        field.board[1][2] = Some(Player::O);
        assert_eq!(analyze(&field), GameState::WinX);

        field = TicTacField::default();
        field.board[0][0] = Some(Player::X);
        field.board[0][1] = Some(Player::X);
        field.board[0][2] = Some(Player::X);
        field.board[1][0] = Some(Player::O);
        field.board[1][1] = Some(Player::O);
        field.board[1][2] = Some(Player::O);
        assert_eq!(analyze(&field), GameState::WinBoth);

        field = TicTacField::default();
        assert_eq!(analyze(&field), GameState::GameOn);
    }

    #[test]
    fn test_make_move() {
        let mut field = TicTacField::default();
        field.board[0][0] = Some(Player::X);
        field.board[0][1] = Some(Player::X);
        field.board[0][2] = Some(Player::X);
        field.board[1][0] = Some(Player::O);
        field.board[1][2] = Some(Player::O);
        field.current_player = Player::O;
        assert_eq!(make_move(&field, 1, 1, Player::O), Err(Error::GameOver));

        field = TicTacField::default();
        field.board[0][0] = Some(Player::X);
        field.board[0][1] = Some(Player::X);
        field.board[1][0] = Some(Player::O);
        field.board[1][2] = Some(Player::O);
        assert_eq!(
            make_move(&field, 1, 1, Player::X),
            Ok(TicTacField {
                board: [
                    [Some(Player::X), Some(Player::X), None],
                    [Some(Player::O), Some(Player::X), Some(Player::O)],
                    [None, None, None]
                ],
                current_player: Player::O
            })
        );
        assert_eq!(make_move(&field, 0, 1, Player::X), Err(Error::InvalidMove));
        assert_eq!(make_move(&field, 1, 1, Player::O), Err(Error::InvalidMove));
        assert_eq!(make_move(&field, 0, 3, Player::X), Err(Error::InvalidMove));
    }
}
