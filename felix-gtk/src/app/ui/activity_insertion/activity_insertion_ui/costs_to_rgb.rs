use felix_backend::data::{InsertionCost, Rgba};
use felix_backend::Time;

use std::collections::HashMap;

/// From a set of possible insertion beginnings with their associated costs,
/// returns the color associated to each time.
pub(super) fn costs_to_rgb(insertion_costs: &[InsertionCost]) -> HashMap<Time, Rgba> {
    let mut beginnings_and_associated_costs = HashMap::new();

    // If the insertion cost is empty, max_by_key and min_by_key can return None
    if !insertion_costs.is_empty() {
        let mut max_cost = insertion_costs
            .iter()
            .map(|insertion_cost| insertion_cost.cost)
            .max()
            .expect("Max of empty set");
        // Avoid division by zero
        if max_cost == 0 {
            for insertion_cost in insertion_costs {
                beginnings_and_associated_costs.insert(
                    insertion_cost.beginning,
                    Rgba {
                        red: 0.0,
                        green: 1.0,
                        blue: 0.0,
                        alpha: 1.0,
                    },
                );
            }
        } else {
            let min_cost = insertion_costs
                .iter()
                .map(|insertion_cost| insertion_cost.cost)
                .min()
                .expect("Min of empty set");

            // Normalize max cost
            max_cost -= min_cost;

            for insertion_cost in insertion_costs {
                // Turn each cost into a [0; 1] value
                let normalized_cost = (insertion_cost.cost - min_cost) as f64 / max_cost as f64;

                // 2.0 * for brighter colors (yellow in the middle instead of muddy brown)
                let color = Rgba {
                    red: (2.0 * normalized_cost).min(1.0),
                    green: (2.0 * (1.0 - normalized_cost)).min(1.0),
                    blue: 0.0,
                    alpha: 1.0,
                };

                beginnings_and_associated_costs.insert(insertion_cost.beginning, color);
            }
        }
    }
    beginnings_and_associated_costs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_costs_to_rgb() {
        let (time1, time2) = (Time::new(8, 0), Time::new(9, 0));
        let costs = &[
            InsertionCost {
                beginning: time1,
                cost: 5,
            },
            InsertionCost {
                beginning: time2,
                cost: 0,
            },
        ]
        .iter()
        .cloned()
        .collect::<Vec<_>>();

        let rgb = costs_to_rgb(&costs);

        let color1 = Rgba {
            red: 1.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
        };
        let color2 = Rgba {
            red: 0.0,
            green: 1.0,
            blue: 0.0,
            alpha: 1.0,
        };
        assert_eq!(rgb[&time1], color1);
        assert_eq!(rgb[&time2], color2);
    }
}
