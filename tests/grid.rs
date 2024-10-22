use connect::*;

macro_rules! grid_tests {
    ($grid_name:literal, $grid_type:ident) => {
        paste::item! {
            #[test]
            fn [< test_no_space_ $grid_name >]() {
                let mut grid = $grid_type::new();
                for _ in 0..6 {
                    assert_eq!(Ok(Status::OnGoing), grid.drop(0));
                }
                assert!(!grid.has_space(0));
                assert_eq!(Err(NoSpaceError()), grid.drop(0));
            }

            // todo check more win states (to check e.g. diagonals)
            #[test]
            fn [< test_win_ $grid_name >]() {
                let mut grid = $grid_type::new();
                let mut i = 0;
                for _ in 0..6 {
                    if i % 2 == 0 {
                        assert_eq!(Ok(Status::OnGoing), grid.drop(0));
                    } else {
                        assert_eq!(Ok(Status::OnGoing), grid.drop(1));
                    }
                    i += 1;
                }
                assert_eq!(Ok(Status::Win(Player::P1)), grid.drop(0));
            }

            #[test]
            fn [< test_draw_ $grid_name >]() {
                let mut grid = $grid_type::new();
                for x in [0, 2, 4] {
                    let mut i = 0;
                    for _ in 0..3 {
                        if i % 2 == 0 {
                            assert_eq!(Ok(Status::OnGoing), grid.drop(x));
                            assert_eq!(Ok(Status::OnGoing), grid.drop(x + 1));
                            assert_eq!(Ok(Status::OnGoing), grid.drop(x));
                            assert_eq!(Ok(Status::OnGoing), grid.drop(x + 1));
                        } else {
                            assert_eq!(Ok(Status::OnGoing), grid.drop(x + 1));
                            assert_eq!(Ok(Status::OnGoing), grid.drop(x));
                            assert_eq!(Ok(Status::OnGoing), grid.drop(x + 1));
                            assert_eq!(Ok(Status::OnGoing), grid.drop(x));
                        }
                        i += 1;
                    }
                }
                for _ in 0..5 {
                    assert_eq!(Ok(Status::OnGoing), grid.drop(6));
                }
                assert_eq!(Ok(Status::Draw), grid.drop(6));
            }
        }
    };
}

pub type StandardGenericGrid = GenericGrid<7, 6, 4>;

grid_tests!("generic", StandardGenericGrid);
grid_tests!("bitboard", BitboardGrid);
