use z2d_cellular_automata::gol_container::GOLContainer;
use z2d_cellular_automata::state::State;

fn main() {
    let mut container: GOLContainer = GOLContainer::new(None);
    container.push_line(vec![State::DEAD, State::DEAD, State::ALIVE, State::DEAD, State::ALIVE, State::DEAD, State::ALIVE, State::DEAD, State::ALIVE, State::DEAD, State::DEAD, State::DEAD], false);
    container.push_line(vec![State::DEAD, State::DEAD, State::ALIVE, State::DEAD, State::ALIVE, State::DEAD, State::DEAD, State::DEAD, State::ALIVE, State::DEAD, State::DEAD, State::DEAD], false);
    container.push_line(vec![State::DEAD, State::DEAD, State::ALIVE, State::DEAD, State::ALIVE, State::DEAD, State::DEAD, State::ALIVE, State::ALIVE, State::DEAD, State::DEAD, State::DEAD], false);
    container.push_line(vec![State::DEAD, State::DEAD, State::ALIVE, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::ALIVE, State::DEAD, State::DEAD, State::DEAD], false);
    container.push_line(vec![State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::ALIVE, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD], false);
    container.push_line(vec![State::DEAD, State::DEAD, State::ALIVE, State::DEAD, State::ALIVE, State::ALIVE, State::ALIVE, State::DEAD, State::ALIVE, State::DEAD, State::DEAD, State::DEAD], false);
    container.push_line(vec![State::DEAD, State::DEAD, State::ALIVE, State::DEAD, State::DEAD, State::DEAD, State::ALIVE, State::DEAD, State::ALIVE, State::DEAD, State::DEAD, State::DEAD], false);
    container.push_line(vec![State::DEAD, State::DEAD, State::ALIVE, State::DEAD, State::ALIVE, State::DEAD, State::ALIVE, State::DEAD, State::ALIVE, State::DEAD, State::DEAD, State::DEAD], false);
    container.push_line(vec![State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::ALIVE, State::DEAD, State::ALIVE, State::DEAD, State::DEAD, State::DEAD], false);
    container.push_line(vec![State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::ALIVE, State::DEAD, State::ALIVE, State::DEAD, State::ALIVE, State::DEAD, State::DEAD, State::DEAD], false);
    container.push_line(vec![State::ALIVE, State::DEAD, State::DEAD, State::ALIVE, State::ALIVE, State::DEAD, State::ALIVE, State::DEAD, State::ALIVE, State::DEAD, State::DEAD, State::DEAD], false);
    container.push_line(vec![State::DEAD, State::DEAD, State::ALIVE, State::DEAD, State::DEAD, State::DEAD, State::ALIVE, State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::DEAD], false);
    container.push_line(vec![State::DEAD, State::DEAD, State::DEAD, State::DEAD, State::ALIVE, State::ALIVE, State::ALIVE, State::DEAD, State::ALIVE, State::DEAD, State::DEAD, State::DEAD], false);
    container.push_line(vec![State::DEAD, State::DEAD, State::ALIVE, State::DEAD, State::DEAD, State::DEAD, State::ALIVE, State::ALIVE, State::ALIVE, State::DEAD, State::DEAD, State::DEAD], false);
    container.push_line(vec![State::DEAD, State::DEAD, State::ALIVE, State::DEAD, State::ALIVE, State::DEAD, State::ALIVE, State::DEAD, State::ALIVE, State::DEAD, State::DEAD, State::DEAD], false);
    container.push_line(vec![State::DEAD, State::DEAD, State::ALIVE, State::DEAD, State::DEAD, State::DEAD, State::ALIVE, State::DEAD, State::ALIVE, State::DEAD, State::DEAD, State::DEAD], false);
    container.push_line(vec![State::DEAD, State::DEAD, State::ALIVE, State::DEAD, State::ALIVE, State::DEAD, State::ALIVE, State::DEAD, State::ALIVE, State::DEAD, State::DEAD, State::DEAD], false);
    container.push_line(vec![State::ALIVE, State::DEAD, State::ALIVE, State::DEAD, State::ALIVE, State::DEAD, State::ALIVE, State::ALIVE, State::ALIVE, State::DEAD, State::DEAD, State::DEAD], false);
    container.push_line(vec![State::DEAD, State::DEAD, State::ALIVE, State::DEAD, State::ALIVE, State::DEAD, State::ALIVE, State::DEAD, State::ALIVE, State::DEAD, State::DEAD, State::DEAD], false);
    container.push_line(vec![State::DEAD, State::DEAD, State::ALIVE, State::DEAD, State::ALIVE, State::DEAD, State::ALIVE, State::ALIVE, State::ALIVE, State::DEAD, State::DEAD, State::DEAD], false);
    container.push_line(vec![State::DEAD, State::DEAD, State::ALIVE, State::DEAD, State::ALIVE, State::DEAD, State::ALIVE, State::DEAD, State::ALIVE, State::DEAD, State::DEAD, State::DEAD], false);
    container.push_line(vec![State::DEAD, State::DEAD, State::ALIVE, State::DEAD, State::ALIVE, State::DEAD, State::ALIVE, State::DEAD, State::ALIVE, State::DEAD, State::DEAD, State::DEAD], true);

    container.run(0, 10);
}