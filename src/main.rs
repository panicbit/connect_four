use std::io;
use std::ops;

#[derive(Copy,Clone,PartialEq,Eq)]
enum Piece {
    X,
    O,
}

impl Piece {
    fn char(&self) -> char {
        match self {
            Piece::X => 'X',
            Piece::O => 'O',
        }
    }
}

impl ops::Not for Piece {
    type Output = Piece;

    fn not(self) -> Piece {
        match self {
            Piece::X => Piece::O,
            Piece::O => Piece::X,
        }
    }
}

struct Field([[Option<Piece>; Field::WIDTH]; Field::HEIGHT]);

impl Field {
    const HEIGHT: usize = 6;
    const WIDTH: usize = 7;

    pub fn new() -> Self {
        Field(Default::default())
    }

    pub fn insert(&mut self, x: usize, piece: Piece) -> Result<(), String> {
        if x >= Field::WIDTH {
            return Err(format!("Invalid column '{}'", x))?;
        }

        for row in self.0.iter_mut().rev() {
            if row[x].is_none() {
                row[x] = Some(piece);
                return Ok(());
            }
        }

        Err(format!("Column '{}' is already full", x))
    }

    pub fn print(&self) {
        for pos in 0..Field::WIDTH {
            print!(" {}", pos);
        }

        println!();

        for (y, row) in self.0.iter().enumerate() {
            if y > 0 {
                print!("├");

                for pos in 0..(2*Field::WIDTH-1) {
                    if pos % 2 == 0 {
                        print!("─");
                    } else {
                        print!("┼");
                    }
                }

                println!("┤");
            }

            for piece in row.iter() {
                print!("│");

                let piece = piece.map(|piece| piece.char()).unwrap_or(' ');

                print!("{}", piece);
            }

            println!("│");
        }

        print!("└");

        for pos in 0..(2*Field::WIDTH-1) {
            if pos % 2 == 0 {
                print!("─");
            } else {
                print!("┴");
            }
        }

        println!("┘");

        for pos in 0..Field::WIDTH {
            print!(" {}", pos);
        }

        println!();
    }

    fn is_full(&self) -> bool {
        self.0.iter().all(|row|
            row.iter().all(|piece| piece.is_some())
        )
    }
}

struct Game {
    field: Field,
    current_player: Piece,
}

impl Game {
    const REQUIRED_PIECES_TO_WIN: usize = 4;

    pub fn new() -> Self {
        Self {
            field: Field::new(),
            current_player: Piece::X,
        }
    }

    pub fn prompt_for_column(&self) -> usize {
        loop {
            println!("Enter column (0-{})", Field::WIDTH-1);

            let mut line = String::new();

            io::stdin().read_line(&mut line).expect("Could not read line");

            let line = line.trim();

            match line.parse::<usize>() {
                Err(_) => println!("Invalid column '{}'", line),
                Ok(n) if n >= Field::WIDTH => println!("Invalid column '{}'", n),
                Ok(n) => return n,
            }
        }
    }

    pub fn get_piece(&self, x: isize, y: isize) -> Option<Piece> {
        if x < 0 || y < 0 {
            return None;
        }

        let x = x as usize;
        let y = y as usize;

        self.field.0.get(y).and_then(|row| row.get(x)).and_then(|&piece| piece)
    }

    pub fn won(&self) -> bool {
        let dirs = [(0, 1), (1, 1), (1, 0), (1, -1)];

        for x in 0..Field::WIDTH as isize {
            for y in 0..Field::WIDTH as isize {
                'dirs: for (xd, yd) in &dirs {
                    for n in 0..Self::REQUIRED_PIECES_TO_WIN as isize {
                        let x = x + n * xd;
                        let y = y + n * yd;

                        if self.get_piece(x, y) != Some(self.current_player) {
                            continue 'dirs;
                        }
                    }

                    return true;
                }
            }
        }
        
        false
    }

    pub fn run(&mut self) {
        loop {
            self.field.print();
            println!("Turn of player '{}'", self.current_player.char());

            loop {
                let col = self.prompt_for_column();

                match self.field.insert(col, self.current_player) {
                    Err(e) => println!("{}", e),
                    Ok(()) => break,
                }
            }

            if self.won() {
                self.field.print();
                println!("Player '{}' wins!", self.current_player.char());
                break;
            }

            if self.field.is_full() {
                self.field.print();
                println!("Draw!");
                break;
            }

            self.current_player = !self.current_player;
        }
    }
}

fn main() {
    Game::new().run();
}
