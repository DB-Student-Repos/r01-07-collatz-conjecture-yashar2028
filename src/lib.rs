pub fn collatz(n: u64) -> Option<u64> {
    if n < 1 {
        return None;
    }
    let mut lim: u64 = n;
    let mut steps: u64 = 0;
    while lim != 1 {
        if lim % 2 == 0 {
             lim /= 2;
        } else {
              lim = lim.checked_mul(3)?.checked_add(1)?; 
        }
    steps += 1;
    }
    Some(steps)
}
