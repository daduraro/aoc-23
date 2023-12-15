advent_of_code::solution!(15);

fn compute_hash(text: &str) -> u32 {
    assert!(text.is_ascii());
    text.as_bytes().iter().fold(0u32, |mut acc, c| {
        acc += (*c) as u32;
        acc *= 17;
        acc %= 256;
        acc
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.trim_end().split(',').map(compute_hash).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let boxes = input.trim_end().split(',').fold(vec![Vec::new(); 256], |mut boxes, seq|{
        if seq.ends_with('-') {
            let label = &seq[0..seq.len()-1];
            let b = &mut boxes[compute_hash(label) as usize];
            if let Some(idx) = b.iter().position(|(lbl, _)| *lbl == label) {
                b.remove(idx);
            }
        }
        else {
            let (label, focal) = seq.split_once('=').unwrap();
            let focal: u32 = focal.parse().unwrap();
            let b = &mut boxes[compute_hash(label) as usize];
            if let Some(idx) = b.iter().position(|(lbl, _)| *lbl == label) {
                b[idx] = (label, focal);
            }
            else {
                b.push((label, focal));
            }
        }
        boxes
    });
    let result = boxes.iter().enumerate().map(|(box_num, b)|{
        b.iter().enumerate().map(|(slot_num, (_, focal))|{
            (*focal) * (box_num + 1) as u32 * (slot_num + 1) as u32
        }).sum::<u32>()
    }).sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
