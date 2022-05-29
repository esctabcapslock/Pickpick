// #![windows_subsystem="windows"] 

use iced::{
    alignment, button, scrollable,  text_input, Button,  Color,
    Column, Container,  Element, Length,  Row, 
    Scrollable, Settings,  Space, Text, TextInput, canvas::{self, },//Program
    Application, executor, Command, time, Subscription, 
};
//slider,ContentFit,Slider, Radio,Sandbox,Checkbox,Toggler,Rectangle,

// use iced::Canvas;
use pickpick::{ServertimeWait};

use clock::Clock;


// use std::{thread, sync::{Mutex, Arc}, borrow::BorrowMut};

pub fn main() -> iced::Result {
    // println!("j");
    let mut setting = Settings::default();
    setting.window.always_on_top = true;
    setting.window.size = (300,300); //width and height
    setting.window.resizable = false;
    Tour::run(setting)
}

pub struct Tour {
    steps: Steps,
    scroll: scrollable::State,
    // back_button: button::State,
    // next_button: button::State,
    debug: bool,
    servertime:ServertimeWait,
    // servertime:Arc<Mutex<ServertimeWait>>,
}

// impl Tour {
//     fn setoffset(&self){
//         let k = self.servertime.get_offset_mean();
//     }
// }

impl Application for Tour {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = ();


    // fn cview(&mut self) -> Element<Message> {
    //     canvas::Canvas::new(&mut self.state)
    //         .width(Length::Fill)
    //         .height(Length::Fill)
    //         .into()
    // }

    fn new(_flags: Self::Flags) -> (Tour,Command<Message>) {
        (Tour {
            steps: Steps::new(),
            scroll: scrollable::State::new(),
            // back_button: button::State::new(),
            // next_button: button::State::new(),
            debug: false,
            servertime: ServertimeWait::new(),
            // servertime: Arc::new(Mutex::new(ServertimeWait::new())),
        },            
        Command::none())
    }

    fn title(&self) -> String {
        format!("{} - Pickpick", self.steps.title())
    }

    fn update(&mut self, event: Message) -> Command<Message>{
        match event {
            Message::BackPressed => {
                self.steps.go_back();
            }
            Message::NextPressed => {
                self.steps.advance();
            }
            Message::StepMessage(step_msg) => {
                match step_msg {
                    StepMessage::Calculate =>{
                        self.steps.advance();
                        // self.view();
                        // self.update(Message::NextPressed);

                        // let k = Arc::new(self.servertime);
                        // let join = thread::spawn(|| {

                        // let servertime = self.servertime.lock().unwrap();
                        // servertime.
                        // let k = Mutex::new(String::from("_"));
                        
                        // servertime
                        
                        let setserver_res = self.servertime.set_server();
                        // .set_server();
                        
                        match setserver_res {
                            Ok(_) => {
                                println!("[GUI] set_server ok");
                                let (offset, offsetrange) = self.servertime.get_offset_mean();
                                let host = self.servertime.get_host();
                                self.steps.update(StepMessage::CalculateEnded { offset, offsetrange, host }, &mut false);
                            }
                            Err(_) => {
                                self.update(Message::StepMessage(StepMessage::ParseError { msg: String::from("set_server error") }));
                            
                            }
                        }  
                    // });
                    },
                    StepMessage::AddressInputChanged(ref inputvalue) =>{
                        let value = inputvalue.clone();
                        println!("fff {}",value);
                        let k = &mut self.steps.steps[self.steps.current];

                        // let servertime = Arc::clone(&self.servertime).lock().unwrap();

                        match self.servertime.add_address(value) {
                            Ok(())=>{
                                println!("[GIT] no error");
                                match k {
                                    Step::SetupAddr { error, ..} =>{
                                       // error.map(|_| "Error".to_string());
                                        error.take();
                                   },
                                   _ => {}
                               }
                                
                            }
                            Err(msg) =>{
                                match k {
                                    Step::SetupAddr { error, ..} =>{
                                       error.replace(msg);//String::from("Invalid URL")
                                   },
                                   _ => {}
                               }
                            }
                        }
                    }
                    StepMessage::ParseError{ref msg} =>{
                        let k = &mut self.steps.steps[0];
                        match k {
                                Step::SetupAddr { error, ..} =>{
                                // error.map(|_| "Error".to_string());
                                error.replace(String::from(msg));
                            },
                            _ => {}
                        }
                       // 원점으로 초기화
                       self.steps.current = 0;
                    }
                    _ => {}
                }   
                self.steps.update(step_msg, &mut self.debug);
            },
            Message::Tick=>{
                if let Step::Clock { .. } = self.steps.steps[self.steps.current] {
                    //self.steps.update(StepMessage::TimerTic, &mut self.debug)
                    self.view();
                }
            }
        }
        return Command::none()
    }

    

    fn view(&mut self) -> Element<Message> {
        let Tour {
            steps,
            scroll,
            // back_button,
            // next_button,
            ..
        } = self;

        let mut controls = Row::new();

        // 뒤쪽 버튼
        // if steps.has_previous() {
        //     controls = controls.push(
        //         button(back_button, "Back")
        //             .on_press(Message::BackPressed)
        //             .style(style::Button::Secondary),
        //     );
        // }

        controls = controls.push(Space::with_width(Length::Fill));

        // 다음 버튼
        // if steps.can_continue() {
        //     controls = controls.push(
        //         button(next_button, "Next")
        //             .on_press(Message::NextPressed)
        //             .style(style::Button::Primary),
        //     );
        // }

        let content: Element<_> = Column::new()
            .max_width(540)
            .spacing(20)
            .padding(20)
            .push(steps.view(self.debug).map(Message::StepMessage))
            .push(controls)
            .into();

        let content = if self.debug {
            content.explain(Color::BLACK)
        } else {
            content
        };

        let scrollable = Scrollable::new(scroll)
            .push(Container::new(content).width(Length::Fill).center_x());

        Container::new(scrollable)
            .height(Length::Fill)
            .center_y()
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(std::time::Duration::from_millis(50))
            .map(|_instant| Message::Tick)
    }

}

#[derive(Debug, Clone)]
pub enum Message {
    BackPressed,
    NextPressed,
    StepMessage(StepMessage),
    Tick,
    // Calculate,
}

struct Steps {
    steps: Vec<Step>,
    current: usize,
}

impl Steps {
    fn new() -> Steps {
        // let k = 
        Steps {
            steps: vec![
                Step::SetupAddr{
                    addr_value: String::new(),
                    state: text_input::State::new(),
                    error:None,
                    state_btn_calculate: button::State::new(),
                },
                // Step::Loading{
                //     value:String::from("1235"),
                // },
                // Step::Welcome,
                Step::Clock{
                    state: Clock::new(3.3),//{radius: 50.0},
                    host:String::from("this computer"),
                    offset:0,
                    offsetrange:50.,
                },
                // canvas: canvas::Canvas::new(Clock { radius: 50.0 }),
                // Setp::iteminfo
                // Step::Slider {
                //     state: slider::State::new(),
                //     value: 50,
                // },
                // Step::RowsAndColumns {
                //     layout: Layout::Row,
                //     spacing_slider: slider::State::new(),
                //     spacing: 20,
                // },
                // Step::Text {
                //     size_slider: slider::State::new(),
                //     size: 30,
                //     color_sliders: [slider::State::new(); 3],
                //     color: Color::BLACK,
                // },
                // Step::Radio { selection: None },
                // Step::Toggler {
                //     can_continue: false,
                // },
                // // Step::Image {
                // //     height: 200,
                // //     current_fit: ContentFit::Contain,
                // //     slider: slider::State::new(),
                // // },
                // Step::Scrollable,
                // Step::TextInput {
                //     value: String::new(),
                //     is_secure: false,
                //     state: text_input::State::new(),
                // },
                // Step::Debugger,
                // Step::End,
            ],
            current: 0,
        }
    }

    fn update(&mut self, msg: StepMessage, debug: &mut bool) {
        self.steps[self.current].update(msg, debug);
    }

    fn view(&mut self, debug: bool) -> Element<StepMessage> {
        self.steps[self.current].view(debug)
    }

    fn advance(&mut self) {
        if self.can_continue() {
            self.current += 1;
        }
    }

    fn go_back(&mut self) {
        if self.has_previous() {
            self.current -= 1;
        }
    }

    fn has_previous(&self) -> bool {
        self.current > 0
    }

    fn can_continue(&self) -> bool {
        self.current + 1 < self.steps.len()
            && self.steps[self.current].can_continue()
    }

    fn title(&self) -> &str {
        self.steps[self.current].title()
    }
}

enum Step {
    SetupAddr{
        addr_value: String,
        state: text_input::State,
        error: Option<String>,
        state_btn_calculate: button::State
    },
    // Loading{
    //     value: String,
    // },
    Clock{
        state: Clock,
        host:String,
        offset:i64,
        offsetrange:f32,
        // canvas: canvas::Canvas<(),Clock>
    },
    // Slider {
    //     state: slider::State,
    //     value: u8,
    // },
    // RowsAndColumns {
    //     layout: Layout,
    //     spacing_slider: slider::State,
    //     spacing: u16,
    // },
    // Text {
    //     size_slider: slider::State,
    //     size: u16,
    //     color_sliders: [slider::State; 3],
    //     color: Color,
    // },
    // Radio {
    //     selection: Option<Language>,
    // },
    // Toggler {
    //     can_continue: bool,
    // },
    // Image {
    //     height: u16,
    //     slider: slider::State,
    //     current_fit: ContentFit,
    // },
    // Scrollable,
    // TextInput {
    //     value: String,
    //     is_secure: bool,
    //     state: text_input::State,
    // },
    // Debugger,
    // End,
}

#[derive(Debug, Clone)]
pub enum StepMessage {
    // SliderChanged(u8),
    // LayoutChanged(Layout),
    // SpacingChanged(u16),
    // TextSizeChanged(u16),
    // TextColorChanged(Color),
    // LanguageSelected(Language),
    // ImageHeightChanged(u16),
    // ImageFitSelected(ContentFit),
    // InputChanged(String),
    // ToggleSecureInput(bool),
    // DebugToggled(bool),
    // TogglerChanged(bool),
    AddressInputChanged(String),
    Calculate,
    TimerTic,
    CalculateEnded{
        offset:i64,
        offsetrange:f32,
        host:String,
    },
    ParseError{
        msg:String,
    },

}

impl<'a> Step {
    fn  update(&mut self, msg: StepMessage, _debug: &mut bool) {
        // let v = canvas::Canvas::new(program);
        match msg {
            // StepMessage::DebugToggled(value) => {
            //     if let Step::Debugger = self {
            //         *debug = value;
            //     }
            // }
            // StepMessage::LanguageSelected(language) => {
            //     if let Step::Radio { selection } = self {
            //         *selection = Some(language);
            //     }
            // }
            // StepMessage::SliderChanged(new_value) => {
            //     if let Step::Slider { value, .. } = self {
            //         *value = new_value;
            //     }
            // }
            // StepMessage::TextSizeChanged(new_size) => {
            //     if let Step::Text { size, .. } = self {
            //         *size = new_size;
            //     }
            // }
            // StepMessage::TextColorChanged(new_color) => {
            //     if let Step::Text { color, .. } = self {
            //         *color = new_color;
            //     }
            // }
            // StepMessage::LayoutChanged(new_layout) => {
            //     if let Step::RowsAndColumns { layout, .. } = self {
            //         *layout = new_layout;
            //     }
            // }
            // StepMessage::SpacingChanged(new_spacing) => {
            //     if let Step::RowsAndColumns { spacing, .. } = self {
            //         *spacing = new_spacing;
            //     }
            // }
            // StepMessage::ImageHeightChanged(new_height) => {
            //     if let Step::Image { height, .. } = self {
            //         *height = new_height;
            //     }
            // }
            // StepMessage::ImageFitSelected(fit) => {
            //     if let Step::Image { current_fit, .. } = self {
            //         *current_fit = fit;
            //     }
            // }
            // StepMessage::InputChanged(new_value) => {
            //     if let Step::TextInput { value, .. } = self {
            //         *value = new_value;
            //     }
            // }
            StepMessage::AddressInputChanged(new_value) => {
                if let Step::SetupAddr { addr_value, .. } = self {
                    *addr_value = new_value;
                    
                }
            }
            // StepMessage::ToggleSecureInput(toggle) => {
            //     if let Step::TextInput { is_secure, .. } = self {
            //         *is_secure = toggle;
            //     }
            // }
            // StepMessage::TogglerChanged(value) => {
            //     // self ;
            //     if let Step::Toggler { can_continue, .. } = self {
            //         *can_continue = value;
            //     }
            // }
            StepMessage::Calculate=>{
                if let Step::SetupAddr { .. } = self {
                    // *addr_value = new_value;
                }
            },
            StepMessage::TimerTic=>{
                if let Step::Clock { .. } = self{
                    //\
                    // self.update(msg, debug)
                    // self.steps[self.current].update(msg, debug);

                    // state.update(event, bounds, cursor);
                    // state.draw(Rectangle::default(), canvas::Cursor::Unavailable);
                }
            },
            StepMessage::CalculateEnded { offset, offsetrange, host } => {
                let (ch_offset, ch_offsetrange, ch_host) = (offset, offsetrange, host);
                if let Step::Clock { state, offset, offsetrange, host } = self{
                    state.offset = ch_offset;
                    state.loading = false;
                    *offset = ch_offset;
                    *offsetrange = ch_offsetrange;
                    *host = ch_host;
                }
            },
            StepMessage::ParseError{..} => {

            }
        };
    }

    fn title(&self) -> &str {
        match self {
            Step::SetupAddr { .. } => "Setup",
            // Step::Loading { .. } => "Loading...",
            Step::Clock { host, .. } => {
                &host
            },
            // Step::Welcome => "Welcome",
            // Step::Radio { .. } => "Radio button",
            // Step::Toggler { .. } => "Toggler",
            // Step::Slider { .. } => "Slider",
            // Step::Text { .. } => "Text",
            // // Step::Image { .. } => "Image",
            // Step::RowsAndColumns { .. } => "Rows and columns",
            // Step::Scrollable => "Scrollable",
            // Step::TextInput { .. } => "Text input",
            // Step::Debugger => "Debugger",
            // Step::End => "End",
        }
    }

    fn can_continue(&self) -> bool {
        match self {
            Step::SetupAddr { .. } => true,
            // Step::Loading { .. } => true,
            Step::Clock {..} => false,
            // Step::Welcome => true,
            // Step::Radio { selection } => *selection == Some(Language::Rust),
            // Step::Toggler { can_continue } => *can_continue,
            // Step::Slider { .. } => true,
            // Step::Text { .. } => true,
            // // Step::Image { .. } => true,
            // Step::RowsAndColumns { .. } => true,
            // Step::Scrollable => true,
            // Step::TextInput { value, .. } => !value.is_empty(),
            // Step::Debugger => true,
            // Step::End => false,
        }
    }

    fn view(&mut self, _debug: bool) -> Element<StepMessage> {
        match self {
            Step::SetupAddr {state, addr_value, error, state_btn_calculate} => Self::setupaddr(state, addr_value, &mut *error, state_btn_calculate),
            // Step::Loading {value} => Self::loading(value),
            Step::Clock { state, offset, offsetrange, host }  => Self::clock(state, *offset, *offsetrange, host),
            // Step::Clock { state }  => Self::clock(state),
            // Step::Welcome => Self::welcome(),
            // Step::Radio { selection } => Self::radio(*selection),
            // Step::Toggler { can_continue } => Self::toggler(*can_continue),
            // Step::Slider { state, value } => Self::slider(state, *value),
            // Step::Text {
            //     size_slider,
            //     size,
            //     color_sliders,
            //     color,
            // } => Self::text(size_slider, *size, color_sliders, *color),
            // // Step::Image {
            // //     height,
            // //     slider,
            // //     current_fit,
            // // } => Self::image(*height, slider, *current_fit),
            // Step::RowsAndColumns {
            //     layout,
            //     spacing_slider,
            //     spacing,
            // } => Self::rows_and_columns(*layout, spacing_slider, *spacing),
            // Step::Scrollable => Self::scrollable(),
            // Step::TextInput {
            //     value,
            //     is_secure,
            //     state,
            // } => Self::text_input(value, *is_secure, state),
            // Step::Debugger => Self::debugger(debug),
            // Step::End => Self::end(),
        }
        .into()
    }

    fn container(title: &str) -> Column<'a, StepMessage> {
        Column::new().spacing(20).push(Text::new(title).size(50))
    }

    fn setupaddr(state: &'a mut text_input::State, value: &str, error:&'a mut Option<String>, state_btn_calculate: &'a mut button::State) -> Column<'a, StepMessage> {
        let text_input = TextInput::new(
            state,
            "ex) github.com",
            value,
            StepMessage::AddressInputChanged,
        )
        .padding(10)
        .size(30);

        let button = button(state_btn_calculate, "Calculate")
        .on_press(StepMessage::Calculate)
        .style(style::Button::Primary);

        // let content: Element<Message> = Column::new();

        let container = Self::container("")
            .push(Text::new(
                "Enter the address of the site where you want to time",
            ))
            .push(
                text_input
            );
            // .push(Text::new(
            //     "A text input produces a message every time it changes. It is \
            //      very easy to keep track of its contents:",
            // ))
            match error {
                None => {
                    container.push(
                        button
                        
                    )
                    // container
                },
                Some(str) => {
                    container.push(
                        Text::new(str.to_string())
                            .width(Length::Fill)
                            .horizontal_alignment(alignment::Horizontal::Center),
                    )
                }
            }
    }

    // fn loading(value: &str)-> Column<'a, StepMessage> {
    //     Self::container("").push(
    //         Text::new(value.to_string())
    //                         .width(Length::Fill)
    //                         .horizontal_alignment(alignment::Horizontal::Center),
    //     )
        
    // }

    // fn clock(state:Clock) -> Column<'a, StepMessage> {
    fn clock(state:&'a mut Clock, offset:i64, offsetrange:f32, host:&'a mut String) -> Column<'a, StepMessage> {
    // fn clock(state:&'a mut Clock) -> Element<StepMessage>  {
        let con = Self::container("");

        let k:canvas::Canvas<StepMessage, Clock> = canvas::Canvas::new(*state);
        // k
        
        
        let kk:Element<StepMessage> = k.into();
        // into();

        // let k = 

        // con
        con
        .push(kk)
        .push(Text::new(format!("{} is {}±{}ms slower then this computer",host,offset, offsetrange)))
    }


    // fn _clock(state:&'a mut Clock) -> Element<StepMessage> {
    //     // fn clock(state:&'a mut Clock) -> Element<StepMessage>  {
    //         // let con = Self::container("");
    //         let k:canvas::Canvas<StepMessage, Clock> = canvas::Canvas::new(Clock { radius: state.radius });
            
    //         let kk = k.into();
    //         kk
    //     }

    // fn welcome() -> Column<'a, StepMessage> {
    //     Self::container("Welcome!")
    //         .push(Text::new(
    //             "This is a simple tour meant to showcase a bunch of widgets \
    //              that can be easily implemented on top of Iced.",
    //         ))
    //         .push(Text::new(
    //             "Iced is a cross-platform GUI library for Rust focused on \
    //              simplicity and type-safety. It is heavily inspired by Elm.",
    //         ))
    //         .push(Text::new(
    //             "It was originally born as part of Coffee, an opinionated \
    //              2D game engine for Rust.",
    //         ))
    //         .push(Text::new(
    //             "On native platforms, Iced provides by default a renderer \
    //              built on top of wgpu, a graphics library supporting Vulkan, \
    //              Metal, DX11, and DX12.",
    //         ))
    //         .push(Text::new(
    //             "Additionally, this tour can also run on WebAssembly thanks \
    //              to dodrio, an experimental VDOM library for Rust.",
    //         ))
    //         .push(Text::new(
    //             "You will need to interact with the UI in order to reach the \
    //              end!",
    //         ))
    // }

    // fn slider(
    //     state: &'a mut slider::State,
    //     value: u8,
    // ) -> Column<'a, StepMessage> {
    //     Self::container("Slider")
    //         .push(Text::new(
    //             "A slider allows you to smoothly select a value from a range \
    //              of values.",
    //         ))
    //         .push(Text::new(
    //             "The following slider lets you choose an integer from \
    //              0 to 100:",
    //         ))
    //         .push(Slider::new(
    //             state,
    //             0..=100,
    //             value,
    //             StepMessage::SliderChanged,
    //         ))
    //         .push(
    //             Text::new(value.to_string())
    //                 .width(Length::Fill)
    //                 .horizontal_alignment(alignment::Horizontal::Center),
    //         )
    // }

    // fn rows_and_columns(
    //     layout: Layout,
    //     spacing_slider: &'a mut slider::State,
    //     spacing: u16,
    // ) -> Column<'a, StepMessage> {
    //     let row_radio = Radio::new(
    //         Layout::Row,
    //         "Row",
    //         Some(layout),
    //         StepMessage::LayoutChanged,
    //     );

    //     let column_radio = Radio::new(
    //         Layout::Column,
    //         "Column",
    //         Some(layout),
    //         StepMessage::LayoutChanged,
    //     );

    //     let layout_section: Element<_> = match layout {
    //         Layout::Row => Row::new()
    //             .spacing(spacing)
    //             .push(row_radio)
    //             .push(column_radio)
    //             .into(),
    //         Layout::Column => Column::new()
    //             .spacing(spacing)
    //             .push(row_radio)
    //             .push(column_radio)
    //             .into(),
    //     };

    //     let spacing_section = Column::new()
    //         .spacing(10)
    //         .push(Slider::new(
    //             spacing_slider,
    //             0..=80,
    //             spacing,
    //             StepMessage::SpacingChanged,
    //         ))
    //         .push(
    //             Text::new(format!("{} px", spacing))
    //                 .width(Length::Fill)
    //                 .horizontal_alignment(alignment::Horizontal::Center),
    //         );

    //     Self::container("Rows and columns")
    //         .spacing(spacing)
    //         .push(Text::new(
    //             "Iced uses a layout model based on flexbox to position UI \
    //              elements.",
    //         ))
    //         .push(Text::new(
    //             "Rows and columns can be used to distribute content \
    //              horizontally or vertically, respectively.",
    //         ))
    //         .push(layout_section)
    //         .push(Text::new(
    //             "You can also easily change the spacing between elements:",
    //         ))
    //         .push(spacing_section)
    // }

    // fn text(
    //     size_slider: &'a mut slider::State,
    //     size: u16,
    //     color_sliders: &'a mut [slider::State; 3],
    //     color: Color,
    // ) -> Column<'a, StepMessage> {
    //     let size_section = Column::new()
    //         .padding(20)
    //         .spacing(20)
    //         .push(Text::new("You can change its size:"))
    //         .push(Text::new(format!("This text is {} pixels", size)).size(size))
    //         .push(Slider::new(
    //             size_slider,
    //             10..=70,
    //             size,
    //             StepMessage::TextSizeChanged,
    //         ));

    //     let [red, green, blue] = color_sliders;

    //     let color_sliders = Row::new()
    //         .spacing(10)
    //         .push(color_slider(red, color.r, move |r| Color { r, ..color }))
    //         .push(color_slider(green, color.g, move |g| Color { g, ..color }))
    //         .push(color_slider(blue, color.b, move |b| Color { b, ..color }));

    //     let color_section = Column::new()
    //         .padding(20)
    //         .spacing(20)
    //         .push(Text::new("And its color:"))
    //         .push(Text::new(format!("{:?}", color)).color(color))
    //         .push(color_sliders);

    //     Self::container("Text")
    //         .push(Text::new(
    //             "Text is probably the most essential widget for your UI. \
    //              It will try to adapt to the dimensions of its container.",
    //         ))
    //         .push(size_section)
    //         .push(color_section)
    // }

    // fn radio(selection: Option<Language>) -> Column<'a, StepMessage> {
    //     let question = Column::new()
    //         .padding(20)
    //         .spacing(10)
    //         .push(Text::new("Iced is written in...").size(24))
    //         .push(Language::all().iter().cloned().fold(
    //             Column::new().padding(10).spacing(20),
    //             |choices, language| {
    //                 choices.push(Radio::new(
    //                     language,
    //                     language,
    //                     selection,
    //                     StepMessage::LanguageSelected,
    //                 ))
    //             },
    //         ));

    //     Self::container("Radio button")
    //         .push(Text::new(
    //             "A radio button is normally used to represent a choice... \
    //              Surprise test!",
    //         ))
    //         .push(question)
    //         .push(Text::new(
    //             "Iced works very well with iterators! The list above is \
    //              basically created by folding a column over the different \
    //              choices, creating a radio button for each one of them!",
    //         ))
    // }

    // fn toggler(can_continue: bool) -> Column<'a, StepMessage> {
    //     Self::container("Toggler")
    //         .push(Text::new(
    //             "A toggler is mostly used to enable or disable something.",
    //         ))
    //         .push(
    //             Container::new(Toggler::new(
    //                 can_continue,
    //                 String::from("Toggle me to continue..."),
    //                 StepMessage::TogglerChanged,
    //             ))
    //             .padding([0, 40]),
    //         )
    // }

    // // fn image(
    // //     height: u16,
    // //     slider: &'a mut slider::State,
    // //     current_fit: ContentFit,
    // // ) -> Column<'a, StepMessage> {
    // //     const FIT_MODES: [(ContentFit, &str); 3] = [
    // //         (ContentFit::Contain, "Contain"),
    // //         (ContentFit::Cover, "Cover"),
    // //         (ContentFit::Fill, "Fill"),
    // //     ];

    // //     let mode_selector = FIT_MODES.iter().fold(
    // //         Column::new().padding(10).spacing(20),
    // //         |choices, (mode, name)| {
    // //             choices.push(Radio::new(
    // //                 *mode,
    // //                 *name,
    // //                 Some(current_fit),
    // //                 StepMessage::ImageFitSelected,
    // //             ))
    // //         },
    // //     );

    // //     Self::container("Image")
    // //         .push(Text::new("Pictures of things in all shapes and sizes!"))
    // //         .push(ferris(height, current_fit))
    // //         .push(Slider::new(
    // //             slider,
    // //             50..=500,
    // //             height,
    // //             StepMessage::ImageHeightChanged,
    // //         ))
    // //         .push(
    // //             Text::new(format!("Height: {} px", height))
    // //                 .width(Length::Fill)
    // //                 .horizontal_alignment(alignment::Horizontal::Center),
    // //         )
    // //         .push(Text::new("Pick a content fit strategy:"))
    // //         .push(mode_selector)
    // // }

    // fn scrollable() -> Column<'a, StepMessage> {
    //     Self::container("Scrollable")
    //         .push(Text::new(
    //             "Iced supports scrollable content. Try it out! Find the \
    //              button further below.",
    //         ))
    //         .push(
    //             Text::new(
    //                 "Tip: You can use the scrollbar to scroll down faster!",
    //             )
    //             .size(16),
    //         )
    //         .push(Column::new().height(Length::Units(4096)))
    //         .push(
    //             Text::new("You are halfway there!")
    //                 .width(Length::Fill)
    //                 .size(30)
    //                 .horizontal_alignment(alignment::Horizontal::Center),
    //         )
    //         .push(Column::new().height(Length::Units(4096)))
    //         // .push(ferris(200, ContentFit::Contain))
    //         .push(
    //             Text::new("You made it!")
    //                 .width(Length::Fill)
    //                 .size(50)
    //                 .horizontal_alignment(alignment::Horizontal::Center),
    //         )
    // }

    // fn text_input(
    //     value: &str,
    //     is_secure: bool,
    //     state: &'a mut text_input::State,
    // ) -> Column<'a, StepMessage> {
    //     let text_input = TextInput::new(
    //         state,
    //         "Type something to continue...",
    //         value,
    //         StepMessage::InputChanged,
    //     )
    //     .padding(10)
    //     .size(30);
    //     Self::container("Text input")
    //         .push(Text::new(
    //             "Use a text input to ask for different kinds of information.",
    //         ))
    //         .push(if is_secure {
    //             text_input.password()
    //         } else {
    //             text_input
    //         })
    //         .push(Checkbox::new(
    //             is_secure,
    //             "Enable password mode",
    //             StepMessage::ToggleSecureInput,
    //         ))
    //         .push(Text::new(
    //             "A text input produces a message every time it changes. It is \
    //              very easy to keep track of its contents:",
    //         ))
    //         .push(
    //             Text::new(if value.is_empty() {
    //                 "You have not typed anything yet..."
    //             } else {
    //                 value
    //             })
    //             .width(Length::Fill)
    //             .horizontal_alignment(alignment::Horizontal::Center),
    //         )
    // }

    // fn debugger(debug: bool) -> Column<'a, StepMessage> {
    //     Self::container("Debugger")
    //         .push(Text::new(
    //             "You can ask Iced to visually explain the layouting of the \
    //              different elements comprising your UI!",
    //         ))
    //         .push(Text::new(
    //             "Give it a shot! Check the following checkbox to be able to \
    //              see element boundaries.",
    //         ))
    //         .push(if cfg!(target_arch = "wasm32") {
    //             Element::new(
    //                 Text::new("Not available on web yet!")
    //                     .color([0.7, 0.7, 0.7])
    //                     .horizontal_alignment(alignment::Horizontal::Center),
    //             )
    //         } else {
    //             Element::new(Checkbox::new(
    //                 debug,
    //                 "Explain layout",
    //                 StepMessage::DebugToggled,
    //             ))
    //         })
    //         .push(Text::new("Feel free to go back and take a look."))
    // }

    // fn end() -> Column<'a, StepMessage> {
    //     Self::container("You reached the end!")
    //         .push(Text::new(
    //             "This tour will be updated as more features are added.",
    //         ))
    //         .push(Text::new("Make sure to keep an eye on it!"))
    // }
}

// fn ferris<'a>(
//     height: u16,
//     content_fit: ContentFit,
// ) -> Container<'a, StepMessage> {
//     Container::new(
//         // This should go away once we unify resource loading on native
//         // platforms
//         if cfg!(target_arch = "wasm32") {
//             // Image::new("tour/images/ferris.png")
//         } else {
//             // Image::new(format!(
//             //     "{}/images/ferris.png",
//             //     env!("CARGO_MANIFEST_DIR"),
//             // ))
//         }
//         .height(Length::Units(height))
//         .content_fit(content_fit),
//     )
//     .width(Length::Fill)
//     .center_x()
// }

fn button<'a, Message: Clone>(
    state: &'a mut button::State,
    label: &str,
) -> Button<'a, Message> {
    Button::new(
        state,
        Text::new(label).horizontal_alignment(alignment::Horizontal::Center),
    )
    .padding(12)
    .width(Length::Units(100))
}

// fn color_slider(
//     state: &mut slider::State,
//     component: f32,
//     update: impl Fn(f32) -> Color + 'static,
// ) -> Slider<f64, StepMessage> {
//     Slider::new(state, 0.0..=1.0, f64::from(component), move |c| {
//         StepMessage::TextColorChanged(update(c as f32))
//     })
//     .step(0.01)
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum Language {
//     Rust,
//     Elm,
//     Ruby,
//     Haskell,
//     C,
//     Other,
// }

// impl Language {
//     fn all() -> [Language; 6] {
//         [
//             Language::C,
//             Language::Elm,
//             Language::Ruby,
//             Language::Haskell,
//             Language::Rust,
//             Language::Other,
//         ]
//     }
// }

// impl From<Language> for String {
//     fn from(language: Language) -> String {
//         String::from(match language {
//             Language::Rust => "Rust",
//             Language::Elm => "Elm",
//             Language::Ruby => "Ruby",
//             Language::Haskell => "Haskell",
//             Language::C => "C",
//             Language::Other => "Other",
//         })
//     }
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layout {
    Row,
    Column,
}

mod style {
    use iced::button;
    use iced::{Background, Color, Vector};

    pub enum Button {
        Primary,
        Secondary,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(match self {
                    Button::Primary => Color::from_rgb(0.11, 0.42, 0.87),
                    Button::Secondary => Color::from_rgb(0.5, 0.5, 0.5),
                })),
                border_radius: 12.0,
                shadow_offset: Vector::new(1.0, 1.0),
                text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
                ..button::Style::default()
            }
        }

        fn hovered(&self) -> button::Style {
            button::Style {
                text_color: Color::WHITE,
                shadow_offset: Vector::new(1.0, 2.0),
                ..self.active()
            }
        }
    }
}




mod clock{

    use iced::{
        canvas::{self, Text},//Cache, Canvas, Cursor, Frame, Geometry, Path, 
        Point, // mouse, Color, Element, Length, Rectangle, Size, Vector, Font,
    };

    // use super::StepMessage;
    use chrono::{self, Duration};

    // pub enum Message {
    //     Populate,
    // }

    #[derive(Debug, Clone, Copy)]
    // #[derive(Debug)]
    pub struct Clock {
        pub radius: f32,
        pub loading: bool,
        pub offset:i64,
        // tiem_cache:canvas::Cache,
    }

    impl Clock {
        // pub fn view<'a>(&'a mut self) -> Element<'a, Message> {
        //     let c = canvas::Canvas::new(self)
        //         .width(Length::Fill)
        //         .height(Length::Fill)
        //         .into();
        //     c
        // }
        pub fn new(radius:f32) -> Clock{
            Clock{
                radius,
                loading:false,
                offset:0,
                // tiem_cache:canvas::Cache::default(),
            }
        }

    }

    // Then, we implement the `Program` trait
    impl<StepMessage> canvas::Program<StepMessage> for Clock {
        fn draw(&self, bounds: iced::Rectangle, _cursor: canvas::Cursor) -> Vec<canvas::Geometry>{
            // We prepare a new `Frame`

            let mut frame = canvas::Frame::new(bounds.size());

            match self.loading {
                true => {
                    let mut t = Text::default();
                    t.content = String::from("loading...");
                    frame.fill_text(t);
                },
                false => {
                    
                    let now = chrono::offset::Local::now();
                    match now.checked_add_signed(Duration::milliseconds(self.offset)) {
                        Some(svtime) => {
                            let mut date = Text::default();
                            let mut time = Text::default();
                            let datestring = svtime.format("%Y-%m-%d").to_string();
                            let timestring = svtime.format("%H:%M:%S%.3f").to_string();
                            date.content = datestring;
                            time.size = 51.;
                            time.content = timestring;
                            time.position = Point{x:0.,y:18.};
                            frame.fill_text(date);
                            frame.fill_text(time);
                            
                        },
                        None =>{
                            let mut t = Text::default();
                            t.content = String::from("error");
                            frame.fill_text(t);
                        }
                    }
                    
                }
            }

            

            

            // We create a `Path` representing a simple circle
            // let circle = Path::circle(frame.center(), self.radius);
            // let t = Text{
            //     content:"A",
            //     position:Point { x: 10, y: 10 },
            //     color: Color::BLACK,
            //     size: 3.3,
            //     font: iced::Font::Default(),
            //     horizontal_alignment: Horizontal,
            //     vertical_alignment: Vertical,
            // }
            // chrono::offset::
            


            // And fill it with some color
            // frame.fill(&circle, Color::BLACK);

            // Finally, we produce the geometry
            vec![frame.into_geometry()]
        }
    }
}