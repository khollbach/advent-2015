fn main() {
    let mut matrix = vec![vec![0u64; 10_000]; 10_000];

    let mut next = 20_151_125_u64;
    let mut get_next = || {
        let ret = next;
        next *= 252_533;
        next %= 33_554_393;
        ret
    };

    for mut r in 0isize..10_000 {
        let mut c = 0;
        while r >= 0 {
            matrix[r as usize][c] = get_next();
            r -= 1;
            c += 1;
        }
    }

    // 0-based indexing, please.
    let row = 2981 - 1;
    let col = 3075 - 1;

    let ans = matrix[row][col];
    dbg!(ans);
}
