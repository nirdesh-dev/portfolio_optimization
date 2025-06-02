use std::collections::HashMap;

pub fn filter_price_map(
    symbols: &[&str],
    price_map: Vec<HashMap<String, Vec<f32>>>,
) -> Vec<HashMap<String, Vec<f32>>> {
    let symbols_set: std::collections::HashSet<&str> = symbols.iter().copied().collect();

    price_map
        .into_iter()
        .map(|hm| {
            hm.into_iter()
                .filter(|(k, _)| symbols_set.contains(k.as_str()))
                .collect::<HashMap<_, _>>()
        })
        .filter(|hm| !hm.is_empty()) // <-- Only keep non-empty HashMaps
        .collect()
}
