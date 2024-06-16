// We use this in the doc strings.
#[allow(unused_imports)]
use super::PathChart;

/// Render configuration specific to [`PathChart`]s.
///
/// ### Example
/// ```
/// # use flat::PathChartConfig;
/// let pathchart_config = PathChartConfig {
///     ..PathChartConfig::default()
/// };
/// ```
#[derive(Debug, Default)]
pub struct PathChartConfig {}
