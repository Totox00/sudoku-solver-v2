use crate::{board::Board, defaults::default_region_bounds};

pub fn format(board: &Board) -> Option<String> {
    if board.is_solved() {
        return format_solved(board);
    }

    let (rwidth, rheight) = default_region_bounds();
    let mut out = String::new();
    out.push('╔');
    for i in 0..rheight {
        if i > 0 {
            out.push('╦');
        }
        for j in 0..rwidth {
            if j > 0 {
                out.push('╤');
            }
            out.push_str("═".repeat(rwidth).as_str());
        }
    }
    out.push('╗');

    for region_row in 0..rwidth {
        if region_row > 0 {
            out.push_str("\n╠");
            for i in 0..rheight {
                if i > 0 {
                    out.push('╬');
                }
                for j in 0..rwidth {
                    if j > 0 {
                        out.push('╪');
                    }
                    out.push_str("═".repeat(rwidth).as_str());
                }
            }
            out.push('╣');
        }
        for cell_row in 0..rheight {
            if cell_row > 0 {
                out.push_str("\n╟");
                for i in 0..rheight {
                    if i > 0 {
                        out.push('╫');
                    }
                    for j in 0..rwidth {
                        if j > 0 {
                            out.push('┼');
                        }
                        out.push_str("─".repeat(rwidth).as_str());
                    }
                }
                out.push('╢');
            }
            for digit_row in 0..rheight {
                out.push_str("\n║");
                for region_col in 0..rheight {
                    if region_col > 0 {
                        out.push('║');
                    }
                    for cell_col in 0..rwidth {
                        if cell_col > 0 {
                            out.push('│');
                        }
                        for digit_col in 1..=rwidth {
                            out.push(
                                if board.get_cell_coords(cell_row + region_row * rheight, cell_col + region_col * rwidth)? & 1 << (digit_col + digit_row * rwidth) > 0 {
                                    #[allow(clippy::cast_possible_truncation)]
                                    char::from_digit((digit_col + digit_row * rwidth) as u32, 10).unwrap_or(' ')
                                } else {
                                    ' '
                                },
                            );
                        }
                    }
                }
                out.push('║');
            }
        }
    }

    out.push_str("\n╚");
    for i in 0..rheight {
        if i > 0 {
            out.push('╩');
        }
        for j in 0..rwidth {
            if j > 0 {
                out.push('╧');
            }
            out.push_str("═".repeat(rwidth).as_str());
        }
    }
    out.push('╝');

    Some(out)
}

fn format_solved(board: &Board) -> Option<String> {
    let (rwidth, rheight) = default_region_bounds();
    let mut out = String::new();
    out.push('╔');
    for i in 0..rheight {
        if i > 0 {
            out.push('╤');
        }
        out.push_str("═".repeat(rwidth).as_str());
    }
    out.push('╗');

    for region_row in 0..rheight {
        if region_row > 0 {
            out.push_str("\n╟");
            for i in 0..rwidth {
                if i > 0 {
                    out.push('┼');
                }
                out.push_str("─".repeat(rwidth).as_str());
            }

            out.push('╢');
        }
        for cell_row in 0..rheight {
            out.push_str("\n║");

            for region_col in 0..rwidth {
                if region_col > 0 {
                    out.push('│');
                }
                for cell_col in 0..rwidth {
                    out.push(char::from_digit(board.get_cell_coords(cell_row + region_row * rheight, cell_col + region_col * rwidth)?.trailing_zeros(), 10).unwrap_or(' '));
                }
            }

            out.push('║');
        }
    }

    out.push_str("\n╚");
    for i in 0..rheight {
        if i > 0 {
            out.push('╧');
        }
        out.push_str("═".repeat(rwidth).as_str());
    }
    out.push('╝');

    Some(out)
}
