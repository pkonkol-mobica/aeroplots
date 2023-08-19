use iced::{
    alignment::{Horizontal, Vertical},
    executor, font,
    futures::io::CopyBufAbortable,
    time::Duration,
    widget::{
        button,
        canvas::{path::lyon_path::commands::PathCommandsBuilder, Cache, Frame, Geometry},
        column, row, text, Column, Container, Row, Scrollable, Space, Text,
    },
    Alignment, Application, Command, Element, Font, Length, Settings, Size, Subscription, Theme,
};
use plotters::prelude::ChartBuilder;
// use plotters_backend::DrawingBackend;
use plotters_iced::{plotters_backend::DrawingBackend, Chart, ChartWidget, Renderer};

fn main() {
    let c = State::run(Settings::default());
}

struct State {
    value: i32,
    chart: MyChart,
    chart2: My3DChart,
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
///
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

    fn build_chart<DB: DrawingBackend>(&self, state: &Self::State, mut builder: ChartBuilder<DB>) {
        use plotters::prelude::*;
        let mut chart = builder.build_cartesian_2d(0..10, 0..10).unwrap();

        chart.configure_mesh().draw().unwrap();

        chart
            .draw_series((0..8).map(|x| Circle::new((x, (x * 2)), 3, GREEN.filled())))
            .unwrap();
        let candlestick =
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
            .draw_series(
                [(2, 3)].map(|(x, y)| {
                    EmptyElement::at((x, y)) 
                    + Circle::new((0, 0), 10, BLUE)
                    + TriangleMarker::new(((4,5)), 5, RED)
                }),
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

    fn build_chart<DB: DrawingBackend>(&self, state: &Self::State, mut builder: ChartBuilder<DB>) {
        use plotters::prelude::*;
        let mut chart = builder.caption("test 3D", ("sans-serif", 50).into_font())
        .margin(10)
        .x_label_area_size(5)
        .y_label_area_size(25)
        .build_cartesian_3d(-10.0..10.0, -10.0..10.0, -10.0..10.0)
        .unwrap();

        chart.configure_axes().draw().unwrap();

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
