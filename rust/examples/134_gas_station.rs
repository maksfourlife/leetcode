// https://leetcode.com/problems/gas-station/

fn can_complete_circuit(gas: &[i32], cost: &[i32]) -> i32 {
    let (idx, _, s1, s2) = gas.iter().zip(cost).enumerate().fold(
        (0, true, 0, 0),
        |(mut idx, mut flag, mut s1, mut s2), (i, (gas, cost))| {
            s1 += gas;
            s2 += cost;
            if s1 >= s2 {
                if flag {
                    idx = i as i32;
                    flag = false;
                }
            } else {
                flag = true;
            }
            dbg!((gas, cost, flag, idx, s1 >= s2));
            (idx, flag, s1, s2)
        },
    );
    if s1 >= s2 { idx } else { -1 }
}

fn main() {
    assert_eq!(can_complete_circuit(&[1, 2, 3, 4, 5], &[3, 4, 5, 1, 2]), 3);
    assert_eq!(can_complete_circuit(&[2, 3, 4], &[3, 4, 3]), -1);
    assert_eq!(can_complete_circuit(&[3, 1, 1], &[1, 2, 2]), 0);
    assert_eq!(can_complete_circuit(&[7, 1, 0, 11, 4], &[5, 9, 1, 2, 5]), 3);
}
