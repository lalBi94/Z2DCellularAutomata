use crate::state::State;
use crate::golcellular_automata::GOLCellularAutomata;
use crate::neighbors::Neighbors;
use crate::zterm::clear_cons;
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct GOLContainer {
    game: Vec<Vec<GOLCellularAutomata>>,
}

impl GOLContainer {
    pub fn new(list: Option<Vec<Vec<State>>>) -> GOLContainer {
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

    pub fn push_line(self: &mut Self, line: Vec<State>, is_last: bool) -> () {
        if self.game.len() == 0 {
            println!("{} to add.", line.len()+2);
            self.game.push(GOLContainer::gen_border_lines(line.len() + 2));
        } else {
            let mut stock: Vec<GOLCellularAutomata> = Vec::new();
            stock.push(GOLCellularAutomata::new(State::BORDER));

            for i in 0..line.len() {
                stock.push(GOLCellularAutomata::new(line[i]));
            }

            stock.push(GOLCellularAutomata::new(State::BORDER));

            self.game.push(stock);
        }

        if is_last {
            self.game.push(GOLContainer::gen_border_lines(self.game[0].len()));
        }
    }

    pub fn run(self: &mut Self, step: usize, speed_milis: u64) -> () {
        let mut now: usize = 1;

        loop {
            println!("CRTL+C to quit.\n(Step. {}) {}ms\n", now, speed_milis);

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

            let stats: (usize, usize) = self.gen_stat();
                println!("[ALIVE: {}, DEAD: {}]", stats.0, stats.1);

            if now == step && step > 0 {
                return;
            } else {
                now += 1;
            }

            thread::sleep(Duration::from_millis(speed_milis));
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

    fn gen_border_lines(n: usize) -> Vec<GOLCellularAutomata> {
        let mut stock: Vec<GOLCellularAutomata> = Vec::new();

        for _ in 0..n {
            stock.push(GOLCellularAutomata::new(State::BORDER));
        }

        stock
    }

    fn gen_stat(self: &Self) -> (usize, usize) {
        let mut dead: usize = 0;
        let mut alive: usize = 0;

        for i in 0..self.game.len() {
            for j in 0..self.game[i].len() {
                match self.game[i][j].get_state() {
                    State::ALIVE => alive += 1,
                    State::DEAD => dead += 1,
                    _ => ()
                }
            }
        }

        (alive, dead)
    }
}