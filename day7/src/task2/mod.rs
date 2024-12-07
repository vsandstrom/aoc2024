pub fn mul(val: u64, nums: &[u64]) -> bool {
  if nums[0] > val { return false; }
  let x = nums[0] * nums[1];
  if nums.len() == 2 { return x == val }
  let y = [&[x], &nums[2..]].concat();
  mul(val, &y) || add(val, &y) || cat(val, &y)
}

pub fn add(val: u64, nums: &[u64]) -> bool {
  if nums[0] > val { return false; }
  let x = nums[0] + nums[1];
  if nums.len() == 2 { return x == val }
  let y = [&[x], &nums[2..]].concat();
  mul(val, &y) || add(val, &y) || cat(val, &y)
}

pub fn cat(val: u64, nums: &[u64]) -> bool {
  if nums[0] > val { return false; }
  let x = con(nums[0], nums[1]);
  if nums.len() == 2 { return x == val }
  let y = [&[x], &nums[2..]].concat();
  mul(val, &y) || add(val, &y) || cat(val, &y)
}


fn con(a:u64, b:u64) -> u64 {
  a * 10u64.pow(b.ilog10() + 1) + b
}
