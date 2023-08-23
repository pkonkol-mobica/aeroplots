use iced::{
    executor,
    time::Duration,
    widget::{
        button,
        canvas::{Cache, Frame, Geometry},
        row, text, Column, Container, Text,
    },
    Alignment, Application, Command, Element, Length, Settings, Size, Subscription, Theme,
};
use plotters::prelude::ChartBuilder;
// use plotters_backend::DrawingBackend;
use plotters_iced::{plotters_backend::DrawingBackend, Chart, ChartWidget, Renderer};

mod accelerometer;
mod datasource;
mod magnetometer;

fn main() {
    let _c = State::run(Settings::default());
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
const DATA2: [(i32, i32); 30] = [
    (1, 22),
    (0, 22),
    (1, 20),
    (2, 24),
    (4, 26),
    (6, 24),
    (5, 27),
    (6, 27),
    (7, 27),
    (8, 30),
    (10, 30),
    (10, 33),
    (12, 34),
    (13, 31),
    (15, 35),
    (14, 33),
    (17, 36),
    (16, 35),
    (17, 39),
    (19, 38),
    (21, 38),
    (22, 39),
    (23, 43),
    (24, 44),
    (24, 46),
    (26, 47),
    (27, 48),
    (26, 49),
    (28, 47),
    (28, 50),
];

struct State {
    value: i32,
    chart: MyChart,
    chart2: My3DChart,
    // accelerometer values, magnetometer values
    // update the values centrally and allow to present them in different manners
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
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
                chart: MyChart::default(),
                chart2: My3DChart::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("iced test")
    }

    fn view(&self) -> Element<Message> {
        let x = row![
            button("+").on_press(Message::Increment),
            text(self.value).size(50),
            button("-").on_press(Message::Decrement),
        ]
        .padding(20)
        .align_items(iced::Alignment::Center);

        let content = Column::new()
            .spacing(10)
            .align_items(Alignment::Center)
            .width(Length::Fill)
            .height(Length::Fill)
            .push(Text::new("iced plotters test"))
            .push(x)
            .push(self.chart.view())
            .push(self.chart2.view());

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(50)
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
                self.value += 1;
            }
            Message::Tick => {
                self.value += 1;
            }
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        const FPS: u64 = 1;
        iced::time::every(Duration::from_millis(1000 / FPS)).map(|_| Message::Tick)
        // iced::Subscription::none()
    }
}

///
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
        let _candlestick =
            CandleStick::new(2, 130.0600, 131.3700, 128.8300, 129.1500, &GREEN, &RED, 15);

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

        // let elem = EmptyElement::at((-5., -5., -5.)) + polygon;

        // let mut chart = builder
        //     .caption("y=x^2", ("sans-serif", 50).into_font())
        //     .margin(5)
        //     .x_label_area_size(30)
        //     .y_label_area_size(30)
        //     .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)
        //     .unwrap();

        // chart.configure_mesh().draw().unwrap();

        // chart
        //     .draw_series(LineSeries::new(
        //         (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
        //         &RED,
        //     ))
        //     .unwrap()
        //     .label("y = x^2")
        //     .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

        // chart
        //     .configure_series_labels()
        //     .background_style(&WHITE.mix(0.8))
        //     .border_style(&BLACK)
        //     .draw()
        //     .unwrap();
        //root.present();
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
