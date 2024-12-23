advent_of_code::solution!(13);

#[derive(Debug, Clone, Copy)]
struct Button{
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy)]
struct ClawMachine {
    a: Button,
    b: Button,
    prize: (i64, i64),
}

fn parse(input: &str) -> Vec<ClawMachine> {
    // always take 4 lines together
    let mut lines = input.lines();
    let mut machines = Vec::new();
    while let Some(a_line) = lines.next() {
        let b_line = lines.next().unwrap();
        let price_line = lines.next().unwrap();
        lines.next();

        // "Button A: X+83, Y+59" to x: 83, y: 59
        let a = Button {
            x: a_line.split(", ").nth(0).unwrap().split("+").nth(1).unwrap().parse().unwrap(),
            y: a_line.split(", ").nth(1).unwrap().split("+").nth(1).unwrap().parse().unwrap(),
        };
        let b = Button {
            x: b_line.split(", ").nth(0).unwrap().split("+").nth(1).unwrap().parse().unwrap(),
            y: b_line.split(", ").nth(1).unwrap().split("+").nth(1).unwrap().parse().unwrap(),
        };
        // Prize: X=4485, Y=8127 to (4485, 8127)
        let prize = (
            price_line.split(", ").nth(0).unwrap().split("=").nth(1).unwrap().parse().unwrap(),
            price_line.split(", ").nth(1).unwrap().split("=").nth(1).unwrap().parse().unwrap(),
        );
        machines.push(ClawMachine { a, b, prize });
    }
    machines
}

fn calculate_steps(machine: &ClawMachine) -> f64 {
    let a_ratio = machine.prize.0 as f64 / machine.a.x as f64;
    let b_ratio = (machine.prize.1 as f64 * machine.b.x as f64) / (machine.b.y as f64 * machine.a.x as f64);
    let step_ratio = (machine.b.x as f64 * machine.a.y as f64) / (machine.a.x as f64 * machine.b.y as f64);
    (a_ratio - b_ratio) / (1.0 - step_ratio)
}

fn find_solution(machine: &ClawMachine) -> Option<u64>{
    let num_a_steps: f64 = calculate_steps(machine);
    if (num_a_steps - num_a_steps.round()).abs() > 0.001 {
        return None;
    } else {
        let num_b_steps = (machine.prize.0 as f64 - machine.a.x as f64 * num_a_steps) / machine.b.x as f64;
        if (num_b_steps - num_b_steps.round()) > 0.001 {
            return None;
        } else {
            return Some((3*num_a_steps.round() as i64 + num_b_steps.round() as i64) as u64);
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let claw_machines = parse(input);
    Some(claw_machines.iter().filter_map(|machine| find_solution(machine)).sum())
}

fn add_10000000000000(mut machine: ClawMachine) -> ClawMachine {
    machine.prize.0 += 10000000000000;
    machine.prize.1 += 10000000000000;
    machine
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut claw_machines = parse(input);
    claw_machines = claw_machines.iter().map(|machine| add_10000000000000(*machine)).collect();
    Some(claw_machines.iter().filter_map(|machine| {
        find_solution(machine)
}).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}