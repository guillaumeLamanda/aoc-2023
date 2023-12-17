use pathfinding::directed::dijkstra;
use pathfinding::matrix::Matrix;

use advent_of_code::map::Position;

pub fn part_one(input: &str) -> Option<u32> {
    let matrix = Matrix::from_rows(input.lines().map(|line| {
        line.chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<_>>()
    }))
    .unwrap();

    let max_move = 3;
    let min_move = 1;
    find_path(matrix, max_move, min_move)
}

fn find_path(matrix: Matrix<u32>, max_move: i32, min_move: i32) -> Option<u32> {
    let end_position = Position::from((matrix.rows - 1, matrix.columns - 1));
    dijkstra::dijkstra(
        &(Position::from((0, 0)), (0, 0), 0),
        |&(pos, (dr, dc), moves_count)| {
            let mut next = Vec::with_capacity(3);
            let mut extend_next_possibilities = |dir, moves_count| {
                next.extend(
                    &matrix
                        .move_in_direction(pos.into(), dir)
                        .map(|t| ((Position::from(t), dir, moves_count), matrix[t])),
                );
            };
            if moves_count < max_move {
                extend_next_possibilities((dr, dc), moves_count + 1);
            }
            if moves_count >= min_move {
                extend_next_possibilities((-dc, -dr), 1);
                extend_next_possibilities((dc, dr), 1);
            } else if moves_count == 0 {
                extend_next_possibilities((1, 0), 1);
                extend_next_possibilities((0, 1), 1);
            }
            next
        },
        |&(pos, _, l)| pos == end_position && l >= min_move,
    )
    .map(|(_, cost)| cost)
}

pub fn part_two(input: &str) -> Option<u32> {
    let matrix = Matrix::from_rows(input.lines().map(|line| {
        line.chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<_>>()
    }))
    .unwrap();

    find_path(matrix, 10, 4)
}

advent_of_code::main!(17);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 17));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 17));
        assert_eq!(result, Some(94));
    }
}
