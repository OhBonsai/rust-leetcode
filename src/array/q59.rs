//! 给定一个正整数 n，生成一个包含 1 到 n2 所有元素，且元素按顺时针顺序螺旋排列的正方形矩阵。
//!
//! 示例:
//!
//! 输入: 3 输出: [ [ 1, 2, 3 ], [ 8, 9, 4 ], [ 7, 6, 5 ] ]


enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Solution ();
impl Solution {
    pub fn rotate_matrix(n: u8) -> Vec<Vec<u8>> {
        let mut dir = Direction::Right;
        let mut x = 0;
        let mut y = 0;
        let mut level = 0;

        let mut result = vec![vec![1; n as usize]; n as usize];
        for i in 1..n*n+1 {
            match dir {
                Direction::Up => {
                    println!("UP: {:?} {:?}", y, x);
                    result[y][x] = i;
                    if y == level + 1 {
                        dir = Direction::Right;
                        level += 1;
                        x += 1;
                    } else {
                        y -= 1;
                    }
                },
                Direction::Down => {
                    println!("Down: {:?} {:?}", y, x);
                    result[y][x] = i;
                    if y == (n as usize  -1 -level)  {
                        dir = Direction::Left;
                        x -= 1;
                    } else {
                        y += 1;
                    }
                },
                Direction::Left => {
                    println!("Left: {:?} {:?}", y, x);
                    result[y][x] = i;
                    if x == level as usize {
                        dir = Direction::Up;
                        y -= 1;
                    } else {
                        x -= 1;
                    }

                },
                Direction::Right => {
                    println!("Right: {:?} {:?}", y, x);
                    result[y][x] = i;
                    if x == (n as usize - 1 - level) as usize {
                        dir = Direction::Down;
                        y += 1;
                    } else {
                        x += 1;
                    }
                },
            }

        }


        result

    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn common_test() {
        let v = Solution::rotate_matrix(5);

        for vv in v.iter() {
            for i in vv.iter() {
                print!("{:?} ", i)
            }
            println!("  ")
        }

        println!("{:?}", v);


    }
}