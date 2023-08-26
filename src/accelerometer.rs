use iced::{
    widget::canvas::{path::lyon_path::geom::euclid::num::Ceil, Cache, Frame, Geometry},
    Element, Length, Size,
};
use plotters::prelude::ChartBuilder;
use plotters_iced::{plotters_backend::DrawingBackend, Chart, ChartWidget, Renderer};

use super::Message;

// functions to:
// - calculate resultant acceleration speed
// to be presented along the charts
const TIME_RANGE: u64 = 30000; // 30 seconds

pub struct Datapoint {
    pub timestamp: u64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct CurrentValue2DChart {
    cache: Cache,
    datapoints: Vec<Datapoint>,
}

impl Chart<Message> for CurrentValue2DChart {
    type State = ();

    #[inline]
    fn draw<R: Renderer, F: Fn(&mut Frame)>(
        &self,
        renderer: &R,
        bounds: Size,
        draw_fn: F,
    ) -> Geometry {
        renderer.draw_cache(&self.cache, bounds, draw_fn)
    }

    fn build_chart<DB: DrawingBackend>(&self, _state: &Self::State, mut builder: ChartBuilder<DB>) {
        println!("in build chart of accelerometer");
        use plotters::prelude::*;

        let x_range_end = self
            .datapoints
            .last()
            .and_then(|x| {
                Some(if x.timestamp >= 30 {
                    (x.timestamp / 30).ceil() as f64
                } else {
                    30.0
                })
            })
            .unwrap_or(30.0);
        let x_range_start = x_range_end - 30.0;

        let mut chart = builder
            .caption("Current value 2d chart", &BLACK)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(x_range_start..x_range_end, -5.0..5.0)
            .unwrap();

        chart.configure_mesh().draw().unwrap();

        chart
            .draw_series(LineSeries::new(
                self.datapoints
                    .iter()
                    .map(|x| (x.timestamp as f64 / 1000., x.x)),
                &RED,
            ))
            .unwrap();

        // chart
        //     .draw_series((0..8).map(|x| Circle::new((x, (x * 2)), 3, GREEN.filled())))
        //     .unwrap();
    }
}

impl CurrentValue2DChart {
    pub fn view(&self) -> Element<Message> {
        println!("in view of accelerometer");
        let chart = ChartWidget::new(self)
            .height(Length::FillPortion(3))
            .width(Length::FillPortion(3));

        chart.into()
    }
    pub fn push_datapoint(&mut self, timestamp: u64, x: f64, y: f64, z: f64) {
        // self.datapoints.iter().filter(|x| x.timestamp > timestamp - TIME_RANGE);
        self.datapoints
            .retain(|x| x.timestamp > timestamp.saturating_sub(TIME_RANGE));
        self.datapoints.push(Datapoint { timestamp, x, y, z });
        self.cache.clear()
    }
}

impl Default for CurrentValue2DChart {
    fn default() -> Self {
        Self {
            cache: Cache::new(),
            datapoints: vec![],
        }
    }
}

struct Speed2DChart {
    cache: Cache,
}

struct AggregatePosition2DChart {
    cache: Cache,
}

// TODO later
struct Aggregate3DChart {}
