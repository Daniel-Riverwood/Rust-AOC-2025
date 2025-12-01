
advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut start = 50;
    let mut sum: u64 = 0;

    for c in input.lines()  {
      let rot = c.chars().nth(0).unwrap();
      let val: i32 = c.split(rot).nth(1).unwrap().parse().unwrap();

      match rot
      {
        'L' => {
            start -= val;
        }
        'R' => {
            start += val;
        }
        _ => {}
      }

      if start % 100 == 0
      {
        sum += 1;
      }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut start = 50;
    let mut sum: u64 = 0;

    for c in input.lines()  {
      let rot = c.chars().nth(0).unwrap();
      let val: i32 = c.split(rot).nth(1).unwrap().parse().unwrap();

      match rot
      {
        'L' => {
            for _i in 0..val
            {
                start -= 1;
                if start == 0
                {
                    sum += 1;
                }
                if start < 0
                {
                    start = 99;
                }
            }
        }
        'R' => {
            for _i in 0..val
            {
                start += 1;
                if start == 0
                {
                    sum += 1;
                }
                if start > 99
                {
                    start = 0;
                    sum += 1;
                }
            }
        }
        _ => {}
      }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.is_some(), true);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.is_some(), true);
    }
}
