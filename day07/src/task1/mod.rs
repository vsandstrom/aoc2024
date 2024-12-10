pub fn mul(val: u64, nums: &[u64]) -> bool {
  if nums[0] > val { return false; }
  let x = nums[0] * nums[1];
  if nums.len() == 2 { return x == val }
  let y = [&[x], &nums[2..]].concat();
  mul(val, &y) || add(val, &y)
}

pub fn add(val: u64, nums: &[u64]) -> bool {
  if nums[0] > val { return false; }
  let x = nums[0] + nums[1];
  if nums.len() == 2 { return x == val }
  let y = [&[x], &nums[2..]].concat();
  mul(val, &y) || add(val, &y)
}
