// 874. Walking Robot Simulation
// Easy

// A robot on an infinite grid starts at point (0, 0) and faces north.  The robot can receive one of three possible types of commands:

// -2: turn left 90 degrees
// -1: turn right 90 degrees
// 1 <= x <= 9: move forward x units
// Some of the grid squares are obstacles. 

// The i-th obstacle is at grid point (obstacles[i][0], obstacles[i][1])

// If the robot would try to move onto them, the robot stays on the previous grid square instead (but still continues following the rest of the route.)

// Return the square of the maximum Euclidean distance that the robot will be from the origin.

 

// Example 1:

// Input: commands = [4,-1,3], obstacles = []
// Output: 25
// Explanation: robot will go to (3, 4)
// Example 2:

// Input: commands = [4,-1,4,-2,4], obstacles = [[2,4]]
// Output: 65
// Explanation: robot will be stuck at (1, 4) before turning left and going to (1, 8)
 

// Note:

// 0 <= commands.length <= 10000
// 0 <= obstacles.length <= 10000
// -30000 <= obstacle[i][0] <= 30000
// -30000 <= obstacle[i][1] <= 30000
// The answer is guaranteed to be less than 2 ^ 31.

pub struct Solution {}

use std::collections::HashSet;
impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        if commands.len() == 0 {
            return 0;
        }

        let mut hash_set: HashSet<(i32, i32)> = HashSet::new();
        for obstacle in obstacles.clone().into_iter() {
            hash_set.insert((obstacle[0], obstacle[1]));
        }

        let (mut cur_x, mut cur_y) = (0, 0);
        let mut ans = 0;
        // NOTE 注意题目指定从(0,0)开始，哪怕有障碍物在(0, 0)
        // dir = 0..=4 => Up, Left, Down, Right
        let dirs = [(0, 1), (-1, 0), (0, -1), (1, 0)];
        let mut cur_dir = 0;
        for &command in commands.iter() {
            match command {
                -2 => {
                    cur_dir = (cur_dir + 1) % 4;
                }
                -1 => {
                    cur_dir = (cur_dir + 3) % 4;
                }
                1..=9 => {
                    let (x_off, y_off) = dirs[cur_dir as usize];
                    let mut steps = command;
                    while steps > 0 {
                        if hash_set.get(&(cur_x + x_off, cur_y + y_off)).is_none() {
                            cur_x = cur_x + x_off;
                            cur_y = cur_y + y_off;
                        } else {
                            break;
                        }
                        steps -= 1;
                    }
                    ans = i32::max(ans, cur_x * cur_x + cur_y * cur_y);
                },
                _ => {}
            }
        }
        ans
    }
}

// TODO 这代码有个测试案例通过不了，还需要分析原因
// use std::collections::HashMap;
// impl Solution {
//     pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
//         if commands.len() == 0 {
//             return 0;
//         }

//         // 构建两个 HashMap 分别保存x, y上的障碍点
//         let mut x_obstacles_map: HashMap<i32, Vec<i32>> = HashMap::new();
//         let mut y_obstacles_map: HashMap<i32, Vec<i32>> = HashMap::new();
//         for obstacle in obstacles.clone().into_iter() {
//             let x_counter = x_obstacles_map.entry(obstacle[0]).or_insert(Vec::new());
//             (*x_counter).push(obstacle[1]);

//             let y_counter = y_obstacles_map.entry(obstacle[1]).or_insert(Vec::new());
//             (*y_counter).push(obstacle[0]);
//         }

//         let mut ans = 0;
//         // NOTE 注意题目指定从(0,0)开始，哪怕有障碍物在(0, 0)
//         let (mut cur_x, mut cur_y) = (0, 0);
//         // dir = 0..=4 => Up, Left, Down, Right
//         let mut dir = 0;
//         for &command in commands.iter() {
//             match command {
//                 -2 => {
//                     dir = (dir + 1 + 4) % 4;
//                 }
//                 -1 => {
//                     dir = (dir - 1 + 4) % 4;
//                 }
//                 1..=9 => match dir {
//                     0 => {
//                         if let Some(array) = x_obstacles_map.get_mut(&cur_x) {
//                             let mut arr: Vec<i32> = (*array)
//                                 .clone()
//                                 .into_iter()
//                                 .filter(|&a| a > cur_y)
//                                 .collect();
//                             if arr.len() == 0 {
//                                 cur_y += command;
//                             } else {
//                                 arr.sort();
//                                 if cur_y + command >= arr[0] {
//                                     cur_y = arr[0] - 1;
//                                 } else {
//                                     cur_y += command;
//                                 }
//                             }
//                         } else {
//                             cur_y += command;
//                         }
//                         ans = i32::max(ans, cur_x * cur_x + cur_y * cur_y);
//                     }
//                     1 => {
//                         if let Some(array) = y_obstacles_map.get_mut(&cur_y) {
//                             let mut arr: Vec<i32> = (*array)
//                                 .clone()
//                                 .into_iter()
//                                 .filter(|&a| a < cur_x)
//                                 .collect();
//                             if arr.len() == 0 {
//                                 cur_x -= command;
//                             } else {
//                                 arr.sort();
//                                 if cur_x - command <= arr[0] {
//                                     cur_x = arr[0] + 1;
//                                 } else {
//                                     cur_x -= command;
//                                 }
//                             }
//                         } else {
//                             cur_x -= command;
//                         }
//                         ans = i32::max(ans, cur_x * cur_x + cur_y * cur_y);
//                     }
//                     2 => {
//                         if let Some(array) = x_obstacles_map.get_mut(&cur_x) {
//                             let mut arr: Vec<i32> = (*array)
//                                 .clone()
//                                 .into_iter()
//                                 .filter(|&a| a < cur_y)
//                                 .collect();
//                             if arr.len() == 0 {
//                                 cur_y -= command;
//                             } else {
//                                 arr.sort();
//                                 if cur_y - command <= arr[0] {
//                                     cur_y = arr[0] + 1;
//                                 } else {
//                                     cur_y -= command;
//                                 }
//                             }
//                         } else {
//                             cur_y -= command;
//                         }
//                         ans = i32::max(ans, cur_x * cur_x + cur_y * cur_y);
//                     }
//                     3 => {
//                         if let Some(array) = y_obstacles_map.get_mut(&cur_y) {
//                             let mut arr: Vec<i32> = (*array)
//                                 .clone()
//                                 .into_iter()
//                                 .filter(|&a| a > cur_x)
//                                 .collect();
//                             if arr.len() == 0 {
//                                 cur_x += command;
//                             } else {
//                                 arr.sort();
//                                 if cur_x + command >= arr[0] {
//                                     cur_x = arr[0] - 1;
//                                 } else {
//                                     cur_x += command;
//                                 }
//                             }
//                         } else {
//                             cur_x += command;
//                         }
//                         ans = i32::max(ans, cur_x * cur_x + cur_y * cur_y);
//                     }
//                     _ => {
//                         panic!("I'm passible!");
//                     }
//                 },
//                 _ => {}
//             }
//         }
//         println!("cur_x: {}, cur_y: {}", cur_x, cur_y);
//         ans
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn robot_sim_test() {
        assert_eq!(Solution::robot_sim(vec![4, -1, 3], vec![]), 25);
        assert_eq!(
            Solution::robot_sim(vec![4, -1, 4, -2, 4], vec![vec![2, 4]]),
            65
        );
        assert_eq!(Solution::robot_sim(vec![], vec![vec![0, 0]]), 0);
        assert_eq!(
            Solution::robot_sim(
                vec![-2, -1, 8, 9, 6],
                vec![
                    vec![-1, 3],
                    vec![0, 1],
                    vec![-1, 5],
                    vec![-2, -4],
                    vec![5, 4],
                    vec![-2, -3],
                    vec![5, -1],
                    vec![1, -1],
                    vec![5, 5],
                    vec![5, 2]
                ]
            ),
            0
        );
        assert_eq!(
            Solution::robot_sim(
                vec![2, 2, 5, -1, -1],
                vec![
                    vec![-3, 5],
                    vec![-2, 5],
                    vec![3, 2],
                    vec![5, 0],
                    vec![-2, 0],
                    vec![-1, 5],
                    vec![5, -3],
                    vec![0, 0],
                    vec![-4, 4],
                    vec![-3, 4]
                ]
            ),
            81
        );
        assert_eq!(
            Solution::robot_sim(
                vec![
                    1, 2, -2, 5, -1, -2, -1, 8, 3, -1, 9, 4, -2, 3, 2, 4, 3, 9, 2, -1, -1, -2, 1,
                    3, -2, 4, 1, 4, -1, 1, 9, -1, -2, 5, -1, 5, 5, -2, 6, 6, 7, 7, 2, 8, 9, -1, 7,
                    4, 6, 9, 9, 9, -1, 5, 1, 3, 3, -1, 5, 9, 7, 4, 8, -1, -2, 1, 3, 2, 9, 3, -1,
                    -2, 8, 8, 7, 5, -2, 6, 8, 4, 6, 2, 7, 2, -1, 7, -2, 3, 3, 2, -2, 6, 9, 8, 1,
                    -2, -1, 1, 4, 7
                ],
                vec![
                    vec![-57, -58],
                    vec![-72, 91],
                    vec![-55, 35],
                    vec![-20, 29],
                    vec![51, 70],
                    vec![-61, 88],
                    vec![-62, 99],
                    vec![52, 17],
                    vec![-75, -32],
                    vec![91, -22],
                    vec![54, 33],
                    vec![-45, -59],
                    vec![47, -48],
                    vec![53, -98],
                    vec![-91, 83],
                    vec![81, 12],
                    vec![-34, -90],
                    vec![-79, -82],
                    vec![-15, -86],
                    vec![-24, 66],
                    vec![-35, 35],
                    vec![3, 31],
                    vec![87, 93],
                    vec![2, -19],
                    vec![87, -93],
                    vec![24, -10],
                    vec![84, -53],
                    vec![86, 87],
                    vec![-88, -18],
                    vec![-51, 89],
                    vec![96, 66],
                    vec![-77, -94],
                    vec![-39, -1],
                    vec![89, 51],
                    vec![-23, -72],
                    vec![27, 24],
                    vec![53, -80],
                    vec![52, -33],
                    vec![32, 4],
                    vec![78, -55],
                    vec![-25, 18],
                    vec![-23, 47],
                    vec![79, -5],
                    vec![-23, -22],
                    vec![14, -25],
                    vec![-11, 69],
                    vec![63, 36],
                    vec![35, -99],
                    vec![-24, 82],
                    vec![-29, -98],
                    vec![-50, -70],
                    vec![72, 95],
                    vec![80, 80],
                    vec![-68, -40],
                    vec![65, 70],
                    vec![-92, 78],
                    vec![-45, -63],
                    vec![1, 34],
                    vec![81, 50],
                    vec![14, 91],
                    vec![-77, -54],
                    vec![13, -88],
                    vec![24, 37],
                    vec![-12, 59],
                    vec![-48, -62],
                    vec![57, -22],
                    vec![-8, 85],
                    vec![48, 71],
                    vec![12, 1],
                    vec![-20, 36],
                    vec![-32, -14],
                    vec![39, 46],
                    vec![-41, 75],
                    vec![13, -23],
                    vec![98, 10],
                    vec![-88, 64],
                    vec![50, 37],
                    vec![-95, -32],
                    vec![46, -91],
                    vec![10, 79],
                    vec![-11, 43],
                    vec![-94, 98],
                    vec![79, 42],
                    vec![51, 71],
                    vec![4, -30],
                    vec![2, 74],
                    vec![4, 10],
                    vec![61, 98],
                    vec![57, 98],
                    vec![46, 43],
                    vec![-16, 72],
                    vec![53, -69],
                    vec![54, -96],
                    vec![22, 0],
                    vec![-7, 92],
                    vec![-69, 80],
                    vec![68, -73],
                    vec![-24, -92],
                    vec![-21, 82],
                    vec![32, -1],
                    vec![-6, 16],
                    vec![15, -29],
                    vec![70, -66],
                    vec![-85, 80],
                    vec![50, -3],
                    vec![6, 13],
                    vec![-30, -98],
                    vec![-30, 59],
                    vec![-67, 40],
                    vec![17, 72],
                    vec![79, 82],
                    vec![89, -100],
                    vec![2, 79],
                    vec![-95, -46],
                    vec![17, 68],
                    vec![-46, 81],
                    vec![-5, -57],
                    vec![7, 58],
                    vec![-42, 68],
                    vec![19, -95],
                    vec![-17, -76],
                    vec![81, -86],
                    vec![79, 78],
                    vec![-82, -67],
                    vec![6, 0],
                    vec![35, -16],
                    vec![98, 83],
                    vec![-81, 100],
                    vec![-11, 46],
                    vec![-21, -38],
                    vec![-30, -41],
                    vec![86, 18],
                    vec![-68, 6],
                    vec![80, 75],
                    vec![-96, -44],
                    vec![-19, 66],
                    vec![21, 84],
                    vec![-56, -64],
                    vec![39, -15],
                    vec![0, 45],
                    vec![-81, -54],
                    vec![-66, -93],
                    vec![-4, 2],
                    vec![-42, -67],
                    vec![-15, -33],
                    vec![1, -32],
                    vec![-74, -24],
                    vec![7, 18],
                    vec![-62, 84],
                    vec![19, 61],
                    vec![39, 79],
                    vec![60, -98],
                    vec![-76, 45],
                    vec![58, -98],
                    vec![33, 26],
                    vec![-74, -95],
                    vec![22, 30],
                    vec![-68, -62],
                    vec![-59, 4],
                    vec![-62, 35],
                    vec![-78, 80],
                    vec![-82, 54],
                    vec![-42, 81],
                    vec![56, -15],
                    vec![32, -19],
                    vec![34, 93],
                    vec![57, -100],
                    vec![-1, -87],
                    vec![68, -26],
                    vec![18, 86],
                    vec![-55, -19],
                    vec![-68, -99],
                    vec![-9, 47],
                    vec![24, 94],
                    vec![92, 97],
                    vec![5, 67],
                    vec![97, -71],
                    vec![63, -57],
                    vec![-52, -14],
                    vec![-86, -78],
                    vec![-17, 92],
                    vec![-61, -83],
                    vec![-84, -10],
                    vec![20, 13],
                    vec![-68, -47],
                    vec![7, 28],
                    vec![66, 89],
                    vec![-41, -17],
                    vec![-14, -46],
                    vec![-72, -91],
                    vec![4, 52],
                    vec![-17, -59],
                    vec![-85, -46],
                    vec![-94, -23],
                    vec![-48, -3],
                    vec![-64, -37],
                    vec![2, 26],
                    vec![76, 88],
                    vec![-8, -46],
                    vec![-19, -68]
                ]
            ),
            5140
        );
    }
}
