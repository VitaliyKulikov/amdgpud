use crate::hw_mon::HwMon;
use crate::{hw_mon, Card, ROOT_DIR};

/// linear mapping from the xrange to the yrange
pub fn linear_map(x: f64, x1: f64, x2: f64, y1: f64, y2: f64) -> f64 {
    let m = (y2 - y1) / (x2 - x1);
    m * (x - x1) + y1
}

/// Read all available graphic cards from direct rendering manager
pub fn read_cards() -> std::io::Result<Vec<Card>> {
    Ok(std::fs::read_dir(ROOT_DIR)?
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.file_name().as_os_str().to_str().map(String::from))
        .filter_map(|file_name| file_name.parse::<Card>().ok())
        .collect())
}

/// Wrap cards in HW Mon manipulator and
/// filter cards so only amd and listed in config cards are accessible
pub fn hw_mons(filter: bool) -> std::io::Result<Vec<HwMon>> {
    Ok(read_cards()?
        .into_iter()
        .map(|card| {
            log::info!("opening hw mon for {:?}", card);
            hw_mon::open_hw_mon(card)
        })
        .flatten()
        .filter(|hw_mon| {
            !filter || {
                log::info!("is vendor ok? {}", hw_mon.is_amd());
                hw_mon.is_amd()
            }
        })
        .filter(|hw_mon| {
            !filter || {
                log::info!("is hwmon name ok? {}", hw_mon.name_is_amd());
                hw_mon.name_is_amd()
            }
        })
        .collect())
}
