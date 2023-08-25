use iced::{
    widget::canvas::{Cache, Frame, Geometry},
    Element, Length, Size,
};
use plotters::prelude::ChartBuilder;
use plotters_iced::{plotters_backend::DrawingBackend, Chart, ChartWidget, Renderer};

use super::Message;

// functions to:
// - calculate resultant acceleration speed
// to be presented along the charts

pub struct CurrentValue2DChart {
    cache: Cache,
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

        let mut chart = builder
            .caption("Current value 2d chart", &BLACK)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(0..30, 0..10)
            .unwrap();

        chart.configure_mesh().draw().unwrap();

        chart
            .draw_series((0..8).map(|x| Circle::new((x, (x * 2)), 3, GREEN.filled())))
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
}

impl Default for CurrentValue2DChart {
    fn default() -> Self {
        Self {
            cache: Cache::new(),
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
