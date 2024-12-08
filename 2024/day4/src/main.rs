use std::env;
use std::fs;

static XMAS: &'static [char] = &['X', 'M', 'A', 'S'];

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please enter file path for data set");
    }

    println!("Part 1 : {}", pt1(&args[1]));
    println!("Part 2 : {}", pt2(&args[1]));
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in fs::read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }

    result
}

fn look_for_christmas(lines : &Vec<String>, line: usize, index : usize, next_char : usize, direction : i64) -> bool {
    // println!("Line: {} LineNo: {} index: {} Chr: {} direction: {}", lines[line], line, index, XMAS[next_char], direction);
    if direction == 7 {
        if lines[line].chars().nth(index).unwrap() == XMAS[next_char] {
            if XMAS[next_char] == 'S' {
                return true;
            } else {
                return look_for_christmas(lines, line - 1, index - 1, next_char + 1 , direction);
            }
        } else {
            return false;
        }
    }

    if direction == 8 {
        if lines[line].chars().nth(index).unwrap() == XMAS[next_char] {
            if XMAS[next_char] == 'S' {
                return true;
            } else {
                return look_for_christmas(lines, line - 1, index, next_char + 1 , direction);
            }
        } else {
            return false;
        }
    }
    
    if direction == 9 {
        if lines[line].chars().nth(index).unwrap() == XMAS[next_char] {
            if XMAS[next_char] == 'S' {
                return true;
            } else {
                return look_for_christmas(lines, line - 1, index + 1, next_char + 1 , direction);
            }
        } else {
            return false;
        }
    }

    if direction == 6 {
        if lines[line].chars().nth(index).unwrap() == XMAS[next_char] {
            if XMAS[next_char] == 'S' {
                return true;
            } else {
                return look_for_christmas(lines, line, index + 1, next_char + 1 , direction)
            }
        } else {
            return false;
        }
    }
    
    if direction == 4 {
        if lines[line].chars().nth(index).unwrap() == XMAS[next_char] {
            if XMAS[next_char] == 'S' {
                return true;
            } else {
                return look_for_christmas(lines, line, index - 1, next_char + 1 , direction)
            }
        } else {
            return false;
        }
    }

    if direction == 1 {
        if lines[line].chars().nth(index).unwrap() == XMAS[next_char] {
            if XMAS[next_char] == 'S' {
                return true;
            } else {
                return look_for_christmas(lines, line + 1, index - 1, next_char + 1 , direction);
            }
        } else {
            return false;
        }
    }
    
    if direction == 2 {
        if lines[line].chars().nth(index).unwrap() == XMAS[next_char] {
            if XMAS[next_char] == 'S' {
                return true;
            } else {
                return look_for_christmas(lines, line + 1, index, next_char + 1 , direction);
            }
        } else {
            return false;
        }
    }
    
    if direction == 3 {
        if lines[line].chars().nth(index).unwrap() == XMAS[next_char] {
            if XMAS[next_char] == 'S' {
                return true;
            } else {
                return look_for_christmas(lines, line + 1, index + 1, next_char + 1 , direction);
            }
        } else {
            return false;
        }
    }

    return false;
}

fn look_for_x_mas(lines : &Vec<String>, line: usize, index : usize) -> bool {
    if lines[line - 1].chars().nth(index - 1).unwrap() == 'M' && lines[line + 1].chars().nth(index + 1).unwrap() == 'S' {
        if lines[line - 1].chars().nth(index + 1).unwrap() == 'M' && lines[line + 1].chars().nth(index - 1).unwrap() == 'S' || 
            lines[line - 1].chars().nth(index + 1).unwrap() == 'S' && lines[line + 1].chars().nth(index - 1).unwrap() == 'M' {
            return true;
        }
    } else if lines[line - 1].chars().nth(index - 1).unwrap() == 'S' && lines[line + 1].chars().nth(index + 1).unwrap() == 'M' {
        if lines[line - 1].chars().nth(index + 1).unwrap() == 'M' && lines[line + 1].chars().nth(index - 1).unwrap() == 'S' || 
           lines[line - 1].chars().nth(index + 1).unwrap() == 'S' && lines[line + 1].chars().nth(index - 1).unwrap() == 'M' {
            return true;
        }
    }

    return false;
}

fn pt1(filename : &str) -> i32{
    let lines = read_lines(filename);
    let mut i : usize = 0;
    let mut total = 0;

    while i < lines.len() {
        let width = lines[i].chars().count();
        let mut ch = 0;
        while ch < lines[i].chars().count() {
            if lines[i].chars().nth(ch).unwrap() == 'X' {
                if i >= 3 && ch <= width - 4 {
                    if look_for_christmas(&lines, i - 1, ch + 1, 1, 9) {total += 1;}
                }

                if i >= 3 {
                    if look_for_christmas(&lines, i - 1 , ch, 1, 8) {total += 1;}
                }

                if i >= 3 && ch >= 3 { 
                    if look_for_christmas(&lines, i - 1, ch - 1, 1, 7) {total += 1;}
                }

                if ch <= width - 4 {
                    if look_for_christmas(&lines, i, ch + 1, 1, 6) {total += 1;}
                }

                if ch >= 3 {
                    if look_for_christmas(&lines, i, ch - 1, 1, 4) {total += 1;}
                }
                
                if i <= lines.len() - 4 && ch <= width - 4 {
                    if look_for_christmas(&lines, i + 1, ch + 1, 1, 3) {total += 1;}
                }

                if i <= lines.len() - 4 {
                    if look_for_christmas(&lines, i + 1, ch, 1, 2) {total += 1;}
                }

                if i <= lines.len() - 4 && ch >= 3 {
                    if look_for_christmas(&lines, i + 1, ch - 1, 1, 1) {total += 1;}
                }
            }

            ch += 1;
        }
        i += 1;
    }

    return total;
}

fn pt2(filename : &str) -> i32 {
    let lines = read_lines(filename);

    let mut i : usize = 0;
    let mut total = 0;

    while i < lines.len() {
        let width = lines[i].chars().count();
        let mut ch = 0;
        while ch < lines[i].chars().count() {
            if i > 0 && i < lines[i].chars().count() - 1 && ch > 0 && ch < width - 1 {
                if lines[i].chars().nth(ch).unwrap() == 'A' {
                    if look_for_x_mas(&lines, i, ch) {
                        total += 1;
                    }
                }
            }
         
            ch += 1;
        }

        i += 1;
    }

    return total;
}