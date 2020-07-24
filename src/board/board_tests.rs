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
                                            [1, 0, 0, 0],
                                            [1, 0, 0, 0],
                                            [1, 0, 0, 0]]};
        assert_eq!(b.move_right(), true);
        assert_eq!(b.array, [[0, 0, 0, 1],
                             [0, 0, 0, 1],
                             [0, 0, 0, 1],
                             [0, 0, 0, 1]]);

        let mut b = super::Board { array : [[0, 0, 0, 1],
                                            [0, 0, 0, 1],
                                            [0, 0, 0, 1],
                                            [0, 0, 0, 1]]};
        assert_eq!(b.move_right(), false);
        assert_eq!(b.array, [[0, 0, 0, 1],
                             [0, 0, 0, 1],
                             [0, 0, 0, 1],
                             [0, 0, 0, 1]]);

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
                                            [0, 0, 0, 1],
                                            [0, 0, 0, 1],
                                            [0, 0, 0, 1]]};
        assert_eq!(b.move_left(), true);
        assert_eq!(b.array, [[1, 0, 0, 0],
                             [1, 0, 0, 0],
                             [1, 0, 0, 0],
                             [1, 0, 0, 0]]);

        let mut b = super::Board { array : [[1, 0, 0, 0],
                                            [1, 0, 0, 0],
                                            [1, 0, 0, 0],
                                            [1, 0, 0, 0]]};
        assert_eq!(b.move_left(), false);
        assert_eq!(b.array, [[1, 0, 0, 0],
                             [1, 0, 0, 0],
                             [1, 0, 0, 0],
                             [1, 0, 0, 0]]);

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
}