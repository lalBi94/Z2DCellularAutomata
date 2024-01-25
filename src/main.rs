use std::thread;
use std::time::Duration;

fn clear_cons() -> () {
    print!("\x1B[2J\x1B[1;1H");
}

#[derive(Debug, Copy, Clone)]
enum State {
    ALIVE,
    DEAD,
    BORDER,
}

#[derive(Debug, Copy, Clone)]
enum Neighbors {
    TopLeft,
    TopUp,
    TopRight,
    LEFT,
    RIGHT,
    BottomLeft,
    BottomDown,
    BottomRight,
}

#[derive(Debug, Copy, Clone)]
struct GOLCellularAutomata {
    state: State,
}

impl GOLCellularAutomata {
    fn new(state: State) -> GOLCellularAutomata {
        GOLCellularAutomata { state: state }
    }

    fn get_state(self: &Self) -> State {
        self.state
    }

    fn change_state(self: &mut Self, by: State) -> () {
        self.state = by
    }
}

#[derive(Debug, Clone)]
struct GOLContainer {
    game: Vec<Vec<GOLCellularAutomata>>,
}

impl GOLContainer {
    fn new(list: Option<Vec<Vec<State>>>) -> GOLContainer {
        match list {
            Some(v) => {
                let mut container: Vec<Vec<GOLCellularAutomata>> = Vec::new();

                for i in 0..v.len() {
                    let mut j_container: Vec<GOLCellularAutomata> = Vec::new();

                    for j in 0..v[i].len() {
                        j_container.push(GOLCellularAutomata::new(v[i][j]))
                    }

                    container.push(j_container);
                }

                GOLContainer { game: container }
            }
            None => GOLContainer { game: Vec::new() },
        }
    }

    fn run(self: &mut Self, step: usize) -> () {
        let mut now: usize = 1;

        loop {
            println!("(Step. {}) CRTL+C to quit.\n", now);

            for i in 1..self.game.len() - 1 {
                for j in 1..self.game[i].len() - 1 {
                    let current: GOLCellularAutomata = self.game[i][j];

                    let neighbors: Vec<(Neighbors, GOLCellularAutomata)> = vec![
                        (Neighbors::TopRight, self.game[i - 1][j + 1]),
                        (Neighbors::TopUp, self.game[i - 1][j]),
                        (Neighbors::TopLeft, self.game[i - 1][j - 1]),
                        (Neighbors::LEFT, self.game[i][j - 1]),
                        (Neighbors::RIGHT, self.game[i][j + 1]),
                        (Neighbors::BottomLeft, self.game[i + 1][j - 1]),
                        (Neighbors::BottomDown, self.game[i + 1][j]),
                        (Neighbors::BottomRight, self.game[i + 1][j + 1]),
                    ];

                    let infos: (usize, usize) = self.get_info(neighbors);

                    match current.get_state() {
                        State::ALIVE => {
                            if infos.0 == 2 || infos.0 == 3 {
                                self.game[i][j].change_state(State::ALIVE)
                            } else {
                                self.game[i][j].change_state(State::DEAD)
                            }

                            print!("{}", "██");
                        }
                        State::DEAD => {
                            if infos.0 == 3 {
                                self.game[i][j].change_state(State::ALIVE)
                            } else {
                                self.game[i][j].change_state(State::DEAD)
                            }

                            print!("{}", "  ");
                        }
                        _ => (),
                    }
                }

                println!();
            }

            if now == step && step > 0 {
                return;
            } else {
                now += 1;
            }

            thread::sleep(Duration::from_millis(600));
            clear_cons();
        }
    }

    fn get_info(self: &Self, of: Vec<(Neighbors, GOLCellularAutomata)>) -> (usize, usize) {
        let mut alive: usize = 0;
        let mut dead: usize = 0;

        for i in 0..of.len() {
            match of[i].1.get_state() {
                State::ALIVE => alive += 1,
                State::DEAD => dead += 1,
                State::BORDER => (),
            }
        }

        (alive, dead)
    }
}

fn main() {
    let mut container: GOLContainer = GOLContainer::new(Some(vec![
        vec![State::BORDER, State::BORDER, State::BORDER, State::BORDER, State::BORDER, State::BORDER, State::BORDER, State::BORDER, State::BORDER, State::BORDER, State::BORDER, State::BORDER, State::BORDER, State::BORDER, State::BORDER, State::BORDER, State::BORDER, State::BORDER, ],
        vec![State::BORDER, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::ALIVE, State::ALIVE, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::BORDER,],
        vec![State::BORDER, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::BORDER,],
        vec![State::BORDER, State::DEAD, State::DEAD, State::ALIVE, State::DEAD, State::DEAD, State::ALIVE, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::ALIVE, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::BORDER,],
        vec![State::BORDER, State::DEAD, State::DEAD, State::ALIVE, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::BORDER,],
        vec![
            State::BORDER, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::ALIVE, State::DEAD, State::ALIVE, State::ALIVE, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::ALIVE, State::BORDER,
        ],
        vec![
            State::BORDER, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::ALIVE, State::DEAD, State::ALIVE, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::BORDER,
        ],
        vec![
            State::BORDER, State::DEAD, State::DEAD, State::ALIVE, State::DEAD, State::DEAD, State::DEAD, State::ALIVE, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::BORDER,
        ],
        vec![
            State::BORDER, State::DEAD, State::ALIVE, State::DEAD, State::DEAD, State::DEAD, State::ALIVE, State::ALIVE, State::DEAD, State::DEAD, State::ALIVE, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::BORDER,
        ],
        vec![
            State::BORDER, State::DEAD, State::ALIVE, State::ALIVE, State::ALIVE, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::ALIVE, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::BORDER,
        ],
        vec![
            State::BORDER, State::ALIVE, State::ALIVE, State::ALIVE, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::ALIVE, State::ALIVE, State::DEAD, State::ALIVE, State::DEAD, State::DEAD, State::DEAD, State::ALIVE, State::DEAD, State::BORDER,
        ],
        vec![
            State::BORDER, State::ALIVE, State::ALIVE, State::ALIVE, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::ALIVE, State::ALIVE, State::DEAD, State::ALIVE, State::DEAD, State::DEAD, State::DEAD, State::ALIVE, State::DEAD, State::BORDER,
        ],
        vec![
            State::BORDER, State::BORDER, State::BORDER, State::BORDER, State::BORDER, State::BORDER, State::BORDER, State::BORDER, State::BORDER, State::BORDER, State::BORDER, State::BORDER, State::BORDER, State::BORDER, State::BORDER, State::BORDER, State::BORDER, State::BORDER,
        ],
        vec![
            State::BORDER, State::ALIVE, State::ALIVE, State::ALIVE, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::ALIVE, State::ALIVE, State::DEAD, State::ALIVE, State::DEAD, State::DEAD, State::DEAD, State::ALIVE, State::DEAD, State::BORDER,
        ],
        vec![
            State::BORDER, State::ALIVE, State::ALIVE, State::ALIVE, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::ALIVE, State::ALIVE, State::DEAD, State::ALIVE, State::DEAD, State::DEAD, State::DEAD, State::ALIVE, State::DEAD, State::BORDER,
        ],
    ]));

    container.run(0);
}