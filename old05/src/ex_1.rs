use regex::Regex;

#[derive(Debug)]
struct Mapping {
    src: usize,
    dst: usize,
    length: usize,
}

#[derive(Debug)]
struct Step {
    mappings: Vec<Mapping>,
}

impl Step {
    fn maps_to(&self, from: usize) -> usize {
        let mapping = self
            .mappings
            .iter()
            .find(|mapping| mapping.maps_to(from).is_some());
        let res = match mapping {
            Some(mapping) => mapping.maps_to(from).unwrap(),
            None => from,
        };

        println!("{from} maps to {res}");
        return res;
    }
}
impl Mapping {
    fn maps_to(&self, from: usize) -> Option<usize> {
        if from >= self.src && from <= self.src + self.length {
            Some(self.dst + from - self.src)
        } else {
            None
        }
    }
}

pub fn solve(input: &str) -> usize {
    let re = Regex::new(r"([0-9\s]+)").unwrap();
    let lines: Vec<&str> = input.split("\n\n").collect();
    let seeds: Vec<usize> = re.captures(lines[0]).unwrap()[1]
        .split_whitespace()
        .map(|it| it.parse::<usize>().unwrap())
        .collect();

    let steps: Vec<Step> = lines[1..]
        .iter()
        .map(|it| {
            let mappings = it.lines().collect::<Vec<&str>>()[1..]
                .iter()
                .map(|line| {
                    let split: Vec<usize> = line
                        .split_whitespace()
                        .map(|it| it.parse::<usize>().unwrap())
                        .collect();
                    return Mapping {
                        src: split[1],
                        dst: split[0],
                        length: split[2],
                    };
                })
                .collect();
            return Step { mappings };
        })
        .collect();

    let res: Vec<usize> = seeds
        .iter()
        .copied()
        .map(|seed| steps.iter().fold(seed, |a, b| b.maps_to(a)))
        .collect();

    dbg!(&res);

    return res.iter().copied().min().unwrap();
}
