use super::*;

#[cfg(test)]
mod tests {
    use super::Moves;

    #[test]
    fn test_board_size() {
        assert_eq!(super::Board::default().size(), 4);
    }

    #[test]
    fn test_move_right() {
        let mut b = super::Board { array : [[1, 0, 0, 0],
                                            [0, 1, 0, 0],
                                            [0, 0, 1, 0],
                                            [1, 0, 0, 0]]};
        assert_eq!(b.move_right(), true);
        assert_eq!(b.array, [[0, 0, 0, 1],
                             [0, 0, 0, 1],
                             [0, 0, 0, 1],
                             [0, 0, 0, 1]]);

        let mut b = super::Board { array : [[0, 0, 2, 1],
                                            [0, 0, 0, 1],
                                            [0, 0, 0, 1],
                                            [0, 0, 0, 0]]};
        assert_eq!(b.move_right(), false);
        assert_eq!(b.array, [[0, 0, 2, 1],
                             [0, 0, 0, 1],
                             [0, 0, 0, 1],
                             [0, 0, 0, 0]]);

        let mut b = super::Board { array : [[2, 2, 0, 0],
                                            [4, 0, 4, 0],
                                            [4, 0, 0, 2],
                                            [8, 8, 8, 8]]};
        assert_eq!(b.move_right(), true);
        assert_eq!(b.array, [[0, 0, 0, 4],
                             [0, 0, 0, 8],
                             [0, 0, 4, 2],
                             [0, 0, 16, 16]]);

        let mut b = super::Board { array : [[1, 2, 3, 4],
                                            [4, 4, 4, 0],
                                            [4, 4, 2, 2],
                                            [0, 0, 0, 0]]};
        assert_eq!(b.move_right(), true);
        assert_eq!(b.array, [[1, 2, 3, 4],
                             [0, 0, 4, 8],
                             [0, 0, 8, 4],
                             [0, 0, 0, 0]]);
    }

    #[test]
    fn test_move_left() {
        let mut b = super::Board { array : [[0, 0, 0, 1],
                                            [0, 0, 1, 0],
                                            [0, 1, 0, 0],
                                            [0, 0, 0, 1]]};
        assert_eq!(b.move_left(), true);
        assert_eq!(b.array, [[1, 0, 0, 0],
                             [1, 0, 0, 0],
                             [1, 0, 0, 0],
                             [1, 0, 0, 0]]);

        let mut b = super::Board { array : [[1, 2, 0, 0],
                                            [1, 0, 0, 0],
                                            [1, 0, 0, 0],
                                            [0, 0, 0, 0]]};
        assert_eq!(b.move_left(), false);
        assert_eq!(b.array, [[1, 2, 0, 0],
                             [1, 0, 0, 0],
                             [1, 0, 0, 0],
                             [0, 0, 0, 0]]);

        let mut b = super::Board { array : [[2, 2, 0, 0],
                                            [4, 0, 4, 0],
                                            [4, 0, 0, 2],
                                            [8, 8, 8, 8]]};
        assert_eq!(b.move_left(), true);
        assert_eq!(b.array, [[4, 0, 0, 0],
                             [8, 0, 0, 0],
                             [4, 2, 0, 0],
                             [16, 16, 0, 0]]);

        let mut b = super::Board { array : [[1, 2, 3, 4],
                                            [4, 4, 4, 0],
                                            [2, 2, 4, 4],
                                            [0, 0, 0, 0]]};
        assert_eq!(b.move_left(), true);
        assert_eq!(b.array, [[1, 2, 3, 4],
                             [8, 4, 0, 0],
                             [4, 8, 0, 0],
                             [0, 0, 0, 0]]);
    }

    #[test]
    fn test_move_up() {
        let mut b = super::Board { array : [[0, 0, 0, 1],
                                            [0, 1, 0, 0],
                                            [0, 0, 1, 0],
                                            [1, 0, 0, 2]]};
        assert_eq!(b.move_up(), true);
        assert_eq!(b.array, [[1, 1, 1, 1],
                             [0, 0, 0, 2],
                             [0, 0, 0, 0],
                             [0, 0, 0, 0]]);

        let mut b = super::Board { array : [[1, 2, 3, 0],
                                            [2, 0, 0, 0],
                                            [0, 0, 0, 0],
                                            [0, 0, 0, 0]]};
        assert_eq!(b.move_up(), false);
        assert_eq!(b.array, [[1, 2, 3, 0],
                             [2, 0, 0, 0],
                             [0, 0, 0, 0],
                             [0, 0, 0, 0]]);

        let mut b = super::Board { array : [[2, 0, 4, 4],
                                            [2, 4, 0, 4],
                                            [0, 0, 0, 4],
                                            [0, 4, 2, 4]]};
        assert_eq!(b.move_up(), true);
        assert_eq!(b.array, [[4, 8, 4, 8],
                             [0, 0, 2, 8],
                             [0, 0, 0, 0],
                             [0, 0, 0, 0]]);

        let mut b = super::Board { array : [[1, 4, 2, 0],
                                            [2, 4, 2, 0],
                                            [3, 4, 4, 0],
                                            [4, 0, 4, 0]]};
        assert_eq!(b.move_up(), true);
        assert_eq!(b.array, [[1, 8, 4, 0],
                             [2, 4, 8, 0],
                             [3, 0, 0, 0],
                             [4, 0, 0, 0]]);
    }

    #[test]
    fn test_move_down() {
        let mut b = super::Board { array : [[0, 0, 0, 1],
                                            [0, 1, 0, 0],
                                            [0, 0, 1, 0],
                                            [1, 0, 0, 2]]};
        assert_eq!(b.move_down(), true);
        assert_eq!(b.array, [[0, 0, 0, 0],
                             [0, 0, 0, 0],
                             [0, 0, 0, 1],
                             [1, 1, 1, 2]]);

        let mut b = super::Board { array : [[0, 0, 0, 0],
                                            [3, 0, 0, 0],
                                            [2, 0, 0, 0],
                                            [1, 2, 3, 0]]};
        assert_eq!(b.move_down(), false);
        assert_eq!(b.array, [[0, 0, 0, 0],
                             [3, 0, 0, 0],
                             [2, 0, 0, 0],
                             [1, 2, 3, 0]]);

        let mut b = super::Board { array : [[2, 0, 4, 4],
                                            [2, 4, 0, 4],
                                            [0, 0, 0, 4],
                                            [0, 4, 2, 4]]};
        assert_eq!(b.move_down(), true);
        assert_eq!(b.array, [[0, 0, 0, 0],
                             [0, 0, 0, 0],
                             [0, 0, 4, 8],
                             [4, 8, 2, 8]]);

        let mut b = super::Board { array : [[1, 4, 4, 0],
                                            [2, 4, 4, 0],
                                            [3, 4, 2, 0],
                                            [4, 0, 2, 0]]};
        assert_eq!(b.move_down(), true);
        assert_eq!(b.array, [[1, 0, 0, 0],
                             [2, 0, 0, 0],
                             [3, 4, 8, 0],
                             [4, 8, 4, 0]]);
    }

    #[test]
    fn test_move_around() {
        let mut b = super::Board { array : [[1, 2, 0, 0],
                                            [3, 4, 0, 0],
                                            [0, 0, 0, 0],
                                            [0, 0, 0, 0]]};
        assert_eq!(b.move_right(), true);
        assert_eq!(b.move_right(), false);
        assert_eq!(b.array, [[0, 0, 1, 2],
                             [0, 0, 3, 4],
                             [0, 0, 0, 0],
                             [0, 0, 0, 0]]);

        assert_eq!(b.move_down(), true);
        assert_eq!(b.move_down(), false);
        assert_eq!(b.array, [[0, 0, 0, 0],
                             [0, 0, 0, 0],
                             [0, 0, 1, 2],
                             [0, 0, 3, 4]]);

        assert_eq!(b.move_left(), true);
        assert_eq!(b.move_left(), false);
        assert_eq!(b.array, [[0, 0, 0, 0],
                             [0, 0, 0, 0],
                             [1, 2, 0, 0],
                             [3, 4, 0, 0]]);

        assert_eq!(b.move_up(), true);
        assert_eq!(b.move_up(), false);
        assert_eq!(b.array, [[1, 2, 0, 0],
                             [3, 4, 0, 0],
                             [0, 0, 0, 0],
                             [0, 0, 0, 0]]);
    }

    #[test]
    fn test_is_won() {
        let mut b = super::Board { array : [[2, 0, 0, 0],
                                            [0, 0, 0, 0],
                                            [0, 0, 0, 0],
                                            [0, 0, 0, 0]]};
        assert_eq!(b.is_won(), false);
        b.move_right();
        assert_eq!(b.is_won(), false);
        b.move_down();
        assert_eq!(b.is_won(), false);
        b.move_right();
        assert_eq!(b.is_won(), false);
        b.move_left();
        assert_eq!(b.is_won(), false);

        let b = super::Board { array : [[2048, 0, 0, 0],
                                            [0, 0, 0, 0],
                                            [0, 0, 0, 0],
                                            [0, 0, 0, 0]]};
        assert_eq!(b.is_won(), true);

        let mut b = super::Board { array : [[0, 1024, 1024, 0],
                                            [0, 0, 64, 64],
                                            [1, 1, 1, 1],
                                            [0, 0, 0, 0]]};
        b.move_right();
        assert_eq!(b.is_won(), true);
    }
}