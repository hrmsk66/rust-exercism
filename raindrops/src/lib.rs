const FACTORS: [(u32, &'static str); 3] = [(3, "Pling"), (5, "Plang"), (7, "Plong")];

pub fn raindros(n: u32) -> String {
    FACTORS.iter()
        .filter(|&(factor_value, _)| n % factor_value == 0)
        .fold(None, |sound, &(_, factor_sound)| {
            Some(sound.map_or_else(|| factor_sound.to_string(), |sound| sound +factor_sound))
        }).unwrap_or_else(|| n.to_string())
}