use std::collections::HashMap;

pub fn tally(match_results: &str) -> String {
    let mut results = match_results
        .split('\n')
        .map(|line| line.split(';').collect::<Vec<_>>())
        .filter(|v| v.len() > 2)
        .map(|v| {
            let (t1, t2) = match v[2] {
                "win" => ((1, 0, 0), (0, 0, 1)),
                "draw" => ((0, 1, 0), (0, 1, 0)),
                "loss" => ((0, 0, 1), (1, 0, 0)),
                _ => panic!(),
            };
            vec![(v[0], t1), (v[1], t2)]
        })
        .flatten()
        .fold(HashMap::new(), |mut map, (n, (w, d, l))| {
            let e = map.entry(n).or_insert((n, 0, 0, 0, 0, 0));
            *e = (e.0, e.1 + 1, e.2 + w, e.3 + d, e.4 + l, e.5 + (w * 3 + d));
            map
        })
        .values()
        .map(|&(n, m, w, d, l, p)| {
            (
                (-p, n),
                format!(
                    "{:030} | {:2} | {:2} | {:2} | {:2} | {:2}",
                    n, m, w, d, l, p
                )
            )
        })
        .collect::<Vec<_>>();

        results.sort_by_key(|&(p, _)| p);

        let mut lines = vec![format!("{:030} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            "Team", "MP", "W", "D", "L", "P")];
        lines.extend(results.drain(..).map(|(_, l)| l));
        lines.join("\n")
}