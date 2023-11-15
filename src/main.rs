use std::{cmp::max, io::stdin};

pub fn solve_knapsack_problem(input_weights: &[u32], input_cost: &[u32], capacity: u32) -> Vec<usize> {
    let n = input_weights.len();
    let mut a: Vec<Vec<u32>> = vec![vec![0; (capacity + 1) as usize]; n + 1];
    let capacity = capacity as usize;
    for i in 0..=n {
        for j in 0..=capacity {
            // m[i, j] compiled according to the following rule:
            if i == 0 || j == 0 {
                a[i][j] = 0;
            } else if input_weights[i - 1] <= j as u32 {
                // If `i` is in the knapsack
                // Then m[i, j] is equal to the maximum value of the knapsack,
                // where the weight `j` is reduced by the weight of the `i-th` item and the set of admissible items plus the value `k`
                a[i][j] = max(input_cost[i - 1] + a[i - 1][j - (input_weights[i - 1] as usize)], a[i - 1][j]);
            } else {
                // If the item `i` did not get into the knapsack
                // Then m[i, j] is equal to the maximum cost of a knapsack with the same capacity and a set of admissible items
                a[i][j] = a[i - 1][j]
            }
        }
    }
    
    knapsack_items(input_weights, &a, n, capacity as u32)

}

fn knapsack_items(weights: &[u32], m: &[Vec<u32>], i: usize, j: u32) -> Vec<usize> {
    if i == 0 {
        return vec![];
    }
    if m[i][j as usize] > m[i - 1][j as usize] {
        let mut knap: Vec<usize> = knapsack_items(weights, m, i - 1, j - weights[i - 1]);
        knap.push(i);
        knap
    } else {
        knapsack_items(weights, m, i - 1, j)
    }
}


fn main() -> Result<(), String> {
    let mut s = String::new();
    println!("Программа решения задачи о рюкзаке.");
    println!("Введите строку с весами предметов, разделённые пробелами");
    stdin().read_line(&mut s).unwrap();
    let weights: Vec<_> = s.split_whitespace().map(|x|x.trim().parse::<u32>()).collect();
    if let Some(incorrect_ind) = weights.iter().enumerate().find(|(ind, x)| x.is_err()).map(|(ind, _)| ind) {
        println!("{}-тое значение не является числом!", incorrect_ind + 1);
        return Err(format!("{}-тое значение не является числом!", incorrect_ind + 1));
    };
    let weights: Vec<u32> = weights.iter().map(|x|x.as_ref().unwrap()).copied().collect();
    println!("Введите строку со стоимостью предметов, разделённые пробелами");
    
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    let costs: Vec<_> = s.split_whitespace().map(|x|x.trim().parse::<u32>()).collect();
    if let Some(incorrect_ind) = costs.iter().enumerate().find(|(ind, x)| x.is_err()).map(|(ind, _)| ind) {
        println!("{}-тое значение не является числом!", incorrect_ind + 1);
        return Err(format!("{}-тое значение не является числом!", incorrect_ind + 1));
    };
    let costs: Vec<u32> = costs.iter().map(|x|x.as_ref().unwrap()).copied().collect();
    if weights.len() != costs.len() {
        println!("Количество весов и количество стоимостей не совпадает!");
        return Err("Количество весов и количество стоимостей не совпадает!".to_string());
    }
    println!("Введите строку с максимальным суммарным весом предметов в рюкзаке");

    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();

    let r = s.trim().parse::<u32>();
    if r.is_err() {
        println!("Введёное значение не является числом!");
        return Err("Введёное значение не является числом!".to_string());
    }
    let capacity = r.unwrap();

    let kp = solve_knapsack_problem(&weights, &costs, capacity);
    println!("Номера предметов для размещения(индексация с 1):");
    for i in &kp {
        print!("{} ", i + 1);
    }
    println!();
    println!("Суммарный вес: {}", kp.iter().map(|x|weights[*x]).sum::<u32>());
    println!("Суммарная стоимость: {}", kp.iter().map(|x|costs[*x]).sum::<u32>());
    

    Ok(())
}

#[cfg(test)]
mod tests {
    // Took test datasets from https://people.sc.fsu.edu/~jburkardt/datasets/bin_packing/bin_packing.html
    use super::*;

    #[test]
    fn test_p02() {
        assert_eq!(
            (vec![2, 3, 4]),
            solve_knapsack_problem(&[12, 7, 11, 8, 9], &[24, 13, 23, 15, 16], 26)
        );
    }

    #[test]
    fn test_p04() {
        assert_eq!(
            (vec![1, 2, 5]),
            solve_knapsack_problem(
                &[56, 59, 80, 64, 75, 17],
                &[50, 50, 64, 46, 50, 5],
                190
            )
        );
    }

    #[test]
    fn test_p01() {
        assert_eq!(
            (vec![1, 2, 3, 4, 6]),
            solve_knapsack_problem(
                &[23, 31, 29, 44, 53, 38, 63, 85, 89, 82],
                &[92, 57, 49, 68, 60, 43, 67, 84, 87, 72],
                165
            )
        );
    }

    #[test]
    fn test_p06() {
        assert_eq!(
            (vec![2, 4, 7]),
            solve_knapsack_problem(
                
                &[41, 50, 49, 59, 55, 57, 60],
                &[442, 525, 511, 593, 546, 564, 617],
                170,
            )
        );
    }

    #[test]
    fn test_p07() {
        assert_eq!(
            (vec![1, 3, 5, 7, 8, 9, 14, 15]),
            solve_knapsack_problem(
                &[70, 73, 77, 80, 82, 87, 90, 94, 98, 106, 110, 113, 115, 118, 120],
                &[135, 139, 149, 150, 156, 163, 173, 184, 192, 201, 210, 214, 221, 229, 240],
                750
            )
        );
    }
}