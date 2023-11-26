use crate::board::Board;

#[allow(dead_code)]
pub fn test_perft_speed() {
    println!("Starting perft speed test");
    let mut board = Board::empty_board();
    board.from_startpos();

    let now = std::time::Instant::now();
    let res = board.perft(6, 6, true);
    let elapsed = now.elapsed();
    let elapsed_ms = elapsed.as_millis();
    println!("{} nodes in {} ms", res, elapsed_ms);
}
