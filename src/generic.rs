use iced::{
    widget::canvas::{path::lyon_path::geom::euclid::num::Ceil, Cache, Frame, Geometry},
    Element, Length, Size,
};
use plotters::prelude::ChartBuilder;
use plotters_iced::{plotters_backend::DrawingBackend, Chart, ChartWidget, Renderer};

use super::Message;

const TIME_RANGE: u64 = 5000; // miliseconds

pub struct Datapoint {
    pub timestamp: u64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct CurrentValue2DChart {
    cache: Cache,
    datapoints: Vec<Datapoint>,
    title: String,
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
        use plotters::prelude::*;

        let x_range_end = self
            .datapoints
            .last()
            .and_then(|x| {
                Some(if x.timestamp >= TIME_RANGE {
                    x.timestamp as f64 / 1000.
                } else {
                    TIME_RANGE as f64 / 1000.
                })
            })
            .unwrap_or(TIME_RANGE as f64 / 1000.);
        let x_range_start = x_range_end - (TIME_RANGE as f64 / 1000.);

        let mut chart = builder
            .caption(&self.title, ("sasns-serif", 30, &BLACK))
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(x_range_start..x_range_end, -1.0..1.0)
            .unwrap();

        chart.configure_mesh().draw().unwrap();

        chart
            .draw_series(LineSeries::new(
                self.datapoints
                    .iter()
                    .map(|x| (x.timestamp as f64 / 1000., x.x)),
                &RED,
            ))
            .unwrap()
            .label("X")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
        chart
            .draw_series(LineSeries::new(
                self.datapoints
                    .iter()
                    .map(|x| (x.timestamp as f64 / 1000., x.y)),
                &GREEN,
            ))
            .unwrap()
            .label("Y")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));
        chart
            .draw_series(LineSeries::new(
                self.datapoints
                    .iter()
                    .map(|x| (x.timestamp as f64 / 1000., x.z)),
                &BLUE,
            ))
            .unwrap()
            .label("Z")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

        chart
            .configure_series_labels()
            .border_style(&BLACK)
            .background_style(&WHITE.mix(0.8))
            .position(SeriesLabelPosition::MiddleLeft)
            .draw()
            .unwrap();
    }
}

impl CurrentValue2DChart {
    pub fn view(&self) -> Element<Message> {
        let chart = ChartWidget::new(self)
            .height(Length::FillPortion(3))
            .width(Length::FillPortion(3));

        chart.into()
    }
    pub fn push_datapoint(&mut self, timestamp: u64, x: f64, y: f64, z: f64) {
        self.datapoints
            .retain(|x| x.timestamp > timestamp.saturating_sub(TIME_RANGE));
        self.datapoints.push(Datapoint { timestamp, x, y, z });
        self.cache.clear()
    }

    pub fn with_title(title: &str) -> Self {
        Self {
            cache: Cache::new(),
            datapoints: vec![],
            title: String::from(title),
        }
    }
}

impl Default for CurrentValue2DChart {
    fn default() -> Self {
        Self {
            cache: Cache::new(),
            datapoints: vec![],
            title: String::from("default current value chart"),
        }
    }
}
