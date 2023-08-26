use std::{pin::pin, vec};

use iced::{
    executor,
    futures::{SinkExt, TryFutureExt},
    time::Duration,
    widget::{
        button,
        canvas::{Cache, Frame, Geometry},
        column, row, text, Column, Container, Scrollable, Text,
    },
    window::Action,
    Alignment, Application, Command, Element, Length, Settings, Size, Subscription, Theme,
};
use plotters::prelude::ChartBuilder;
// use plotters_backend::DrawingBackend;
use plotters_iced::{plotters_backend::DrawingBackend, Chart, ChartWidget, Renderer};
use tokio::time::Instant;
use tokio_stream::{Stream, StreamExt};

use datasource::{stream_file, Data};
use generic::CurrentValue2DChart;

mod accelerometer;
mod datasource;
mod generic;
mod magnetometer;

const TEST_INPUT: &str = "test-input.csv";

fn main() {
    let _c = State::run(Settings::default());
}

struct State {
    value: i32,
    input_values: Vec<Data>,
    chart: MyChart,
    chart2: My3DChart,
    // accelerometer values\
    acc_current_chart: CurrentValue2DChart,
    mag_current_chart: CurrentValue2DChart,
    // magnetometer values
    // update the values centrally and allow to present them in different manners
}

#[derive(Debug, Clone)]
pub enum Message {
    ReceivedNewData(Data),
    Increment,
    Decrement,
    Tick,
}

impl Application for State {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            State {
                value: 1,
                input_values: vec![],
                chart: MyChart::default(),
                chart2: My3DChart::default(),
                acc_current_chart: CurrentValue2DChart::with_title("Accelerometer current value"),
                mag_current_chart: CurrentValue2DChart::with_title("Magnetometer current value"),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("aeroplot")
    }

    fn view(&self) -> Element<Message> {
        let data_str = format!("{}", self.input_values.last().unwrap_or(&Data::default()));
        let buttons = row![
            button("+").on_press(Message::Increment),
            text(self.value).size(20),
            button("-").on_press(Message::Decrement),
        ];
        let x = column![
            buttons,
            text(data_str).size(25),
            text(format!("input data len: {}", self.input_values.len())).size(25),
        ]
        .padding(20)
        .align_items(iced::Alignment::Center);

        let acc_charts = row![self.acc_current_chart.view()].height(600);
        let mag_charts = row![self.mag_current_chart.view()].height(600);
        let test_charts = row![self.chart.view(), self.chart2.view()].height(600);
        let chart_container = column![acc_charts, mag_charts, test_charts].padding(20);
        println!("in view of State");

        let content = Column::new()
            .spacing(10)
            .align_items(Alignment::Center)
            .width(Length::FillPortion(1))
            .height(Length::FillPortion(1))
            .push(x)
            .push(chart_container);

        let scrollable = Scrollable::new(content);

        Container::new(scrollable)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .center_x()
            .center_y()
            .into()
    }

    fn update(&mut self, msg: Self::Message) -> Command<Self::Message> {
        match msg {
            Message::Decrement => {
                self.value -= 1;
            }
            Message::Increment => {
                self.value += 10;
            }
            Message::Tick => {
                self.value += 1;
            }
            Message::ReceivedNewData(d) => {
                println!("received data {d:?}");
                self.acc_current_chart
                    .push_datapoint(d.timestamp, d.acc.x, d.acc.y, d.acc.z);
                self.mag_current_chart
                    .push_datapoint(d.timestamp, d.mag.x, d.mag.y, d.mag.z);
                self.input_values.push(d);
                // self.acc_current_chart.update(state, event, bounds, cursor)

                // TODO parse the new data, propagate it, what more?
            }
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        struct Connect;

        iced::subscription::channel(std::any::TypeId::of::<Connect>(), 100, |mut x| async move {
            let mut input_stream = stream_file(&TEST_INPUT).await;
            let mut interval = tokio::time::interval(Duration::from_millis(1000));
            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        x.send(Message::Tick).await.unwrap();
                    },
                    Some(data) = input_stream.next() => {
                        x.send(Message::ReceivedNewData(data)).await.unwrap();
                    },
                };
            }
        })
    }
}

///
/// TEST
/// MY CHART
/// 2D
///
struct MyChart {
    cache: Cache,
}

impl Chart<Message> for MyChart {
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
            // .set_all_label_area_size(40)
            .x_label_area_size(40)
            .y_label_area_size(40)
            // .set_all_label_area_size(40)
            .build_cartesian_2d(0..10, 0..50)
            .unwrap();

        chart.configure_mesh().draw().unwrap();

        chart
            .draw_series((0..8).map(|x| Circle::new((x, (x * 2)), 3, GREEN.filled())))
            .unwrap();

        let mut pie = Pie::new(
            &(50, 50),
            &10.0,
            &[50.0, 25.25, 20.0, 5.5],
            &[RED, BLUE, GREEN, WHITE],
            &["Red", "Blue", "Green", "White"],
        );
        pie.start_angle(-90.0); // retract to a right angle, so it starts aligned to a vertical Y axis.

        chart
            .draw_series([(2, 3)].map(|(x, y)| {
                EmptyElement::at((x, y))
                    + Circle::new((0, 0), 10, BLUE)
                    + TriangleMarker::new((4, 5), 5, RED)
                // + pie
            }))
            .unwrap();

        chart
            .draw_series(LineSeries::new((0..10).map(|x| (x, 10 - x)), &BLACK))
            .unwrap();

        chart
            .draw_series(
                DATA1
                    .iter()
                    .map(|point| TriangleMarker::new(*point, 5, &BLUE)),
            )
            .unwrap();

        let data = [25, 37, 15, 32, 45, 33, 32, 10, 29, 0, 21];

        chart
            .draw_series(
                AreaSeries::new(
                    (0..).zip(data.iter().map(|x| *x)), // The data iter
                    0,                                  // Baseline
                    &RED.mix(0.2),                      // Make the series opac
                )
                .border_style(&RED), // Make a brighter border
            )
            .unwrap();

        let data = [25, 37, 15, 32, 45, 33, 32, 10, 0, 21, 5];

        chart
            .draw_series((0..).zip(data.iter()).map(|(y, x)| {
                let mut bar = Rectangle::new([(0, y), (*x, y + 1)], GREEN.filled());
                bar.set_margin(5, 5, 0, 0);
                bar
            }))
            .unwrap();

        chart
            .draw_series((0..).zip(data.iter()).map(|(x, y)| {
                let mut bar = Rectangle::new([(x, 0), (x + 1, *y)], RED.filled());
                bar.set_margin(0, 0, 10, 10);
                bar
            }))
            .unwrap();

        chart
            .draw_series(
                Histogram::vertical(&chart)
                    .margin(10)
                    .data((0..100).map(|x| (x, x % 3))),
            )
            .unwrap();
    }
}

impl MyChart {
    fn view(&self) -> Element<Message> {
        let chart = ChartWidget::new(self)
            .height(Length::Fill)
            .width(Length::Fill);

        chart.into()
    }
}

impl Default for MyChart {
    fn default() -> Self {
        Self {
            cache: Cache::new(),
        }
    }
}

///
/// MY3DCHART
///

struct My3DChart {
    cache: Cache,
}

impl Chart<Message> for My3DChart {
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
            .caption("test 3D", ("sans-serif", 50).into_font())
            .margin(10)
            .set_label_area_size(LabelAreaPosition::Left, -50)
            .build_cartesian_3d(-10.0..10.0, -10.0..10.0, -10.0..10.0)
            .unwrap();

        chart.configure_axes().tick_size(1).draw().unwrap();

        let cubiod = Cubiod::new([(0., 0., 0.), (3., 2., 1.)], BLUE.mix(0.2), BLUE);
        chart.draw_series(std::iter::once(cubiod)).unwrap();

        let polygon = Polygon::new([(1., 2., 3.), (3., 4., 5.), (20., 20., 20.)], BLACK);
        chart.draw_series(std::iter::once(polygon)).unwrap();

        chart
            .draw_series(
                SurfaceSeries::xoz(
                    (-30..30).map(|v| v as f64 / 10.0),
                    (-30..30).map(|v| v as f64 / 10.0),
                    |x: f64, z: f64| (0.7 * (x * x + z * z)).cos(),
                )
                .style(&BLUE.mix(0.5)),
            )
            .unwrap();
    }
}

impl My3DChart {
    fn view(&self) -> Element<Message> {
        let chart = ChartWidget::new(self)
            .height(Length::Fill)
            .width(Length::Fill);

        chart.into()
    }
}

impl Default for My3DChart {
    fn default() -> Self {
        Self {
            cache: Cache::new(),
        }
    }
}

const DATA1: [(i32, i32); 30] = [
    (-3, 1),
    (-2, 3),
    (4, 2),
    (3, 0),
    (6, -5),
    (3, 11),
    (6, 0),
    (2, 14),
    (3, 9),
    (14, 7),
    (8, 11),
    (10, 16),
    (7, 15),
    (13, 8),
    (17, 14),
    (13, 17),
    (19, 11),
    (18, 8),
    (15, 8),
    (23, 23),
    (15, 20),
    (22, 23),
    (22, 21),
    (21, 30),
    (19, 28),
    (22, 23),
    (30, 23),
    (26, 35),
    (33, 19),
    (26, 19),
];
