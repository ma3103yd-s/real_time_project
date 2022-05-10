extern crate rand;
extern crate crossterm;
extern crate tui;

const MAX_AMP: f32 = 5.0;
const WIDTH: f64 = 200.0;
use rand::random;
use std::{io, thread::{self, }, time::{Duration, Instant}, sync::{Mutex, Condvar, mpsc::TryIter}, os::unix::prelude::MetadataExt};
use std::error::Error;
use std::io::Write;
use std::fs::File;
use std::f32::consts::PI;

use tui::{
    Frame,
    text::{Span, Spans},
    style::{Color,Style, Modifier},
    backend::{Backend, CrosstermBackend},
    widgets::{Widget, Block, Borders, Cell,
        Row, Table, TableState, Axis, List, ListItem, ListState, GraphType, Dataset, Chart, LineGauge, Gauge, canvas::{Rectangle, Line, Canvas, Map, MapResolution}, Tabs},
    layout::{Rect, Layout, Constraint, Direction},
    Terminal, symbols
};

use crossterm::{
    event::{self, poll, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode,
    MouseEvent, MouseEventKind, MouseButton}, execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
    LeaveAlternateScreen}, cursor::{SavePosition, EnableBlinking, MoveDown, MoveTo, Show},
};
use std::sync::{Arc, RwLock, mpsc};
use pid::{PID, PIDparam};
use regul::ReferenceGenerator;
use mode::{Mode};
use ref_mode::{RefMode};

pub struct BeamCanvas {
    ball_pos: (f64, f64),
    beam_angle: f32,
    angle_recv: mpsc::Receiver<f32>,
    pos_recv: mpsc::Receiver<f32>,
    length: f64,
    coords: [(f64, f64);4],
    shapes: (Vec<Line>, Rectangle)

}

impl BeamCanvas {
    pub fn new(angle_recv: mpsc::Receiver<f32>,
        pos_recv: mpsc::Receiver<f32>, length: f64) -> Self {
            let coords = [
                (-0.5*length, -0.5*WIDTH),
                (0.5*length, -0.5*WIDTH),
                (0.5*length, 0.5*WIDTH),
                (-0.5*length, 0.5*WIDTH),
                ];
            let shapes = Self::to_shapes(coords, (0.0, 0.5*WIDTH));
            Self {
                ball_pos: (0.0, WIDTH*0.5),
                beam_angle: 0.0,
                angle_recv,
                pos_recv,
                coords,
                length,
                shapes,
                
            }
        }

    fn to_shapes(coords: [(f64, f64);4],
    ball_pos: (f64, f64)) -> (Vec<Line>, Rectangle) {
        let line_1 = Line {
            x1: coords[0].0,
            y1: coords[0].1,
            x2: coords[1].0,
            y2: coords[1].1,
            color: Color::White,
        };
        let line_2 = Line {
            x1: coords[1].0,
            y1: coords[1].1,
            x2: coords[2].0,
            y2: coords[2].1,
            color: Color::White,

        };
        
        let line_3 = Line {
            x1: coords[2].0,
            y1: coords[2].1,
            x2: coords[3].0,
            y2: coords[3].1,
            color: Color::White,

        };

        let line_4 = Line {
            x1: coords[3].0,
            y1: coords[3].1,
            x2: coords[0].0,
            y2: coords[0].1,
            color: Color::White,

        };

        let rect= Rectangle {
            x: ball_pos.0 as f64,
            y: ball_pos.1 as f64,
            width: 50.0,
            height: 50.0,
            color: Color::Yellow,

        };

        (vec![line_1, line_2, line_3, line_4], rect)
    }

    pub fn on_tick(&mut self) {
        let new_pos = self.pos_recv.
        try_iter().last().unwrap() as f64;
        let new_angle = self.angle_recv.try_iter()
        .last().unwrap();
        self.beam_angle = new_angle;
        self.to_beam();
        let dx = -WIDTH*(self.beam_angle/PI*0.5).sin() as f64 * 0.5;
        let dy = WIDTH*(self.beam_angle/PI*0.5).cos() as f64 * 0.5;
        let ball_pos = ((new_pos*(self.length/10.0))*(new_angle as f64/PI as f64*0.5).cos(),
        (new_pos*(self.length/10.0)) * (new_angle as f64/PI as f64*0.5).sin() + dy);
        self.ball_pos = ball_pos;
        self.shapes = Self::to_shapes(self.coords, self.ball_pos);

    }

    fn to_beam(&mut self) {
        let new_x = self.length*(self.beam_angle/PI*0.5).cos() as f64;
        let dx = WIDTH*(self.beam_angle/PI*0.5).sin() as f64 * 0.5;
        let dy = WIDTH*(self.beam_angle/PI*0.5).cos() as f64 * 0.5;
        let new_y = self.length*(self.beam_angle/PI*0.5).sin() as f64;
        self.coords[0] = (-0.5*new_x+dx, -0.5*new_y-dy);
        self.coords[1] = (0.5*new_x+dx, 0.5*new_y-dy);
        self.coords[2] = (0.5*new_x-dx, 0.5*new_y+dy);
        self.coords[3] = (-0.5*new_x-dx, -0.5*new_y+dy);
   
    }
}
#[derive(PartialEq, Eq)]
enum TabIndex {
    First,
    Second,
}

impl TabIndex {
    pub fn next(&mut self) {
        *self = match *self {
            TabIndex::First => TabIndex::Second,
            TabIndex::Second => TabIndex::First,
            
        }
    }
}

pub struct App<S: Iterator> {
    regul: Arc<RwLock<PID>>,
    ref_gen: Arc<RwLock<ReferenceGenerator>>,
    controller_data: ControllerData,
    params: ParameterData,
    mode: ModeData,
    ref_mode: RefModeData,
    input_mode: InputMode,
    selection: SelectionMode,
    signal: PlotSignal<S>,
    reference_area: Option<Rect>,
    canvas: BeamCanvas,
    index: TabIndex,
}

impl App<ControllerSignal> {
    pub fn new(regul: Arc<RwLock<PID>>, ref_gen: Arc<RwLock<ReferenceGenerator>>,
        mode: Arc<(Mutex<Mode>, Condvar)>, ref_mode: Arc<(Mutex<RefMode>, Condvar)>, 
        recv: mpsc::Receiver<f64>, tick_rate: usize, canvas: BeamCanvas) -> Self {
        let regul_lock = (*regul).read().unwrap();
        let ref_gen_lock = (*ref_gen).read().unwrap();
        let pid_params = (*regul_lock).get_params();
        let sampling_time = pid_params.H;
        let amp = (*ref_gen_lock).get_amp();
        let controller_data = ControllerData::new(sampling_time, amp);

        let sampling_time_ms = sampling_time*1000.0;
        let nbr_points = (tick_rate as f32/ sampling_time_ms) as usize;
        let controller_signal = ControllerSignal::new(recv, sampling_time,
                                                      nbr_points as f64*sampling_time as f64);
        //let points = vec![(0.0, 0.0);nbr_points];
        let points: Vec<(f64, f64)> = (0..nbr_points)
            .map(|x| (x as f64 * sampling_time as f64, 0.0)).collect();
        //let points: Vec<(f64, f64)> = Vec::new();
        let mut fs = File::create("output.txt").unwrap();
        fs.write(format!("{}", nbr_points).as_bytes());


        let right_window = tick_rate as f64 / 1000.0;
        drop(regul_lock);
        drop(ref_gen_lock);
        let signal = PlotSignal::new(
            controller_signal,
            points,
            tick_rate,
            sampling_time_ms as usize,
            [0.0, right_window],

        );

        let params = ParameterData::new(pid_params);
        let mode = ModeData::new(mode);
        let ref_mode = RefModeData::new(ref_mode);
        let input_mode = InputMode::Normal;
        let selection = SelectionMode::Parameters;
        Self {
            regul,
            ref_gen,
            controller_data,
            params,
            mode,
            ref_mode,
            input_mode,
            selection,
            signal,
            reference_area: None,
            canvas,
            index: TabIndex::First,
        }

    }
}

pub struct ControllerData {
    sample_time: f32,
    amp: f32,
    //inner: Arc<RwLock<PID>>,
    //outer: Arc<RwLock<PID>>,
}

pub struct ControllerSignal {
    data: mpsc::Receiver<f64>,
    sample_time: f32,
    time: f64,
}

impl ControllerSignal {

    pub fn new(data: mpsc::Receiver<f64>, sample_time: f32, time: f64) -> Self {
        Self {
            data,
            sample_time,
            time,
        }
    }

}

impl Iterator for ControllerSignal {
    type Item = (f64, f64);
    fn next(&mut self) -> Option<Self::Item> {
       let ts = self.sample_time;
       let t = self.time+ts as f64;
       self.time = t;
       self.data.try_recv().ok().map(|v| (t, v))
       //self.data.try_iter().map(|v| (t, v)).next()
    }
    
}

impl ControllerData {
    pub fn new(sample_time: f32, amp: f32) -> Self {
        Self {
            sample_time,
            amp,
        }
    }
    fn get_amp(&self) -> f32 {
        return self.amp;
    }
    fn set_amplitude(&mut self, val: f32) {
        self.amp = val;
    }

}

pub struct PlotSignal<S: Iterator> {
    data_source: S,
    points: Vec<S::Item>,
    tick_rate: usize,
    sample_rate: usize,
    window: [f64;2],
}

impl<S> PlotSignal<S> where S:Iterator {
    fn on_tick(&mut self) {

        let mut data_length = self.points.len();
        for _ in 0..data_length {
            self.points.remove(0);
        }
        self.points.extend(self.data_source.by_ref().take(data_length));
        //let mut fs = File::create("output.txt").unwrap();
        //fs.write(format!("{}", data_length).as_bytes());

        //println!("{:?}", self.points.len());
        /*if data_length == 0 {
            self.points.extend(self.data_source.by_ref());
            data_length = self.points.len();
        } else {
            for _ in 0.. data_length {
                self.points.remove(0);
            }
            self.points.extend(self.data_source.by_ref().take(data_length));
        }*/

        self.window[0] += data_length as f64 * (self.sample_rate as f64 / 1000.0);
        self.window[1] += data_length as f64 * (self.sample_rate as f64 / 1000.0);
//self.sample_rate as f64 / 1000.0;
    }

    fn new(data_source: S, points: Vec<S::Item>,
        tick_rate: usize, sample_rate: usize, window: [f64;2]) -> Self {
            Self {
                data_source,
                points,
                tick_rate,
                sample_rate,
                window,
            }
        }

}


struct VirtualControllerSignal {
    t: f64,
    sampling_rate: usize,
}

impl VirtualControllerSignal {
    fn new(sampling_rate: usize) -> Self {
        Self {
            t: 0.0,
            sampling_rate,
        }
    }
}

impl Iterator for VirtualControllerSignal {
    type Item = (f64, f64);
    fn next(&mut self) -> Option<Self::Item> {
        let x = self.t + self.sampling_rate as f64 / 1000.0;
        let y: f64 = random();
        self.t = x;
        Some((x, y))
    }

}
#[derive(PartialEq, Eq)]
enum SelectionMode {
    Mode,
    Parameters,
    Reference,

}

impl SelectionMode {

    pub fn next(&mut self) {   
        *self = match *self {
            SelectionMode::Mode => SelectionMode::Parameters,
            SelectionMode::Parameters => SelectionMode::Reference,
            SelectionMode::Reference => SelectionMode::Mode,
        }
    }

}


pub trait Selectable {

    fn selected(&self) -> Option<usize>;

    fn select(&mut self, index: Option<usize>) -> ();

    fn len(&self) -> usize;

    fn next(&mut self) {
        let i = match self.selected() {
            Some(i) => {
                if i >= self.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.selected() {
            Some(i) => {
                if i == 0 {
                    self.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.select(Some(i));
    }

    fn unselect(&mut self) {
        self.select(None);
    }

    

}
pub struct ParameterData {
    items: Vec<Vec<String>>,
    params: PIDparam,
    state: TableState,
}

impl Selectable for ParameterData {

    fn selected(&self) -> Option<usize> {
        self.state.selected()
    }

    fn select(&mut self, index: Option<usize>) {
        self.state.select(index);
    }

    fn len(&self) -> usize {
        self.items.len()
    }

}

impl ParameterData {
    fn new(params: PIDparam) -> Self {
        let items = vec![
        vec!["K".to_owned(), format!("{}", params.K)],
        vec!["Ti".to_owned(), format!("{}", params.Ti)],
        vec!["Td".to_owned(), format!("{}", params.Td)],
        vec!["Tr".to_owned(), format!("{}", params.Tr)],
        vec!["N".to_owned(), format!("{}", params.N)],
        vec!["Beta".to_owned(), format!("{}", params.Beta)],
        vec!["H".to_owned(), format!("{}", params.H)],
        vec!["Integrator on".to_owned(), match params.integrator_on {
            true => "true".to_owned(),
            false => "false".to_owned(),
        }],
        ];
        Self {
            items,
            params,
            state: TableState::default(),
        }
    }

    fn set_items(&mut self, items: Vec<Vec<String>>) {
        self.items = items;
        self.state = TableState::default();
    }


    fn items_to_params(items: &Vec<Vec<String>>) -> PIDparam {
        let length = items.len();
        let vals: Vec<f32> = items.iter().take(length-1).map(|s| s[1].as_str())
        .map(|ss| {
            let temp = ss.parse::<f32>().unwrap();
            temp
        }).collect();
        let integrator_on = match items[length-1][1].as_str() {
            "false" => false,
            "true" => true,
            _ => false,
        };
        PIDparam::new(
            vals[0],
            vals[1],
            vals[2],
            vals[3],
            vals[4],
            vals[5],
            vals[6],
            integrator_on,
        )
    }

    pub fn get_params(&self) -> PIDparam {
        self.params
    }

    pub fn set_param(&mut self) {
        self.params = Self::items_to_params(&self.items);
    }

    pub fn add_char(&mut self, param: char) {
        if let Some(index ) = self.state.selected() {
            (self.items[index])[1].push(param);
            //self.state = TableState::default();
        };
    }

    pub fn remove_char(&mut self) {
        if let Some(index ) = self.state.selected() {
            (self.items[index])[1].pop();
            //self.state = TableState::default();
        };

        
    }

}
#[derive(PartialEq, Eq)]
enum ControllerMode {
    BALL,
    BEAM,
    OFF,
}

struct ModeData {
    items: Vec<String>,
    state: ListState,
    mode: Arc<(Mutex<Mode>, Condvar)>,
    loc_mode: Mode,

}

struct RefModeData {
    items: Vec<String>,
    state: ListState,
    ref_mode: Arc<(Mutex<RefMode>, Condvar)>,
    loc_ref_mode: RefMode,
}

impl RefModeData {
    pub fn new(ref_mode: Arc<(Mutex<RefMode>, Condvar)>) -> Self {
        let items = vec!["MANUAL".to_owned(), "SQUARE".to_owned(), "OPTIMAL".to_owned()];
        let (ref_lock, _) = &*ref_mode;
        let loc_ref_mode = (*ref_lock.lock().unwrap()).clone();
        let mut temp = Self {
            items,
            state: ListState::default(),
            ref_mode,
            loc_ref_mode,
        };
        match loc_ref_mode {
            RefMode::MANUAL => temp.select(Some(0)),
            RefMode::SQUARE => temp.select(Some(1)),
            RefMode::OPTIMAL => temp.select(Some(2)), 
        }
        temp

    }
}

impl Selectable for RefModeData {
     fn select(&mut self, index: Option<usize>) {
        self.state.select(index);
    }
    fn selected(&self) -> Option<usize> {
        self.state.selected()
    }

    fn len(&self) -> usize {
        self.items.len()
    }
   
}

impl ModeData {
    pub fn new(mode: Arc<(Mutex<Mode>, Condvar)>) -> Self {
        let items = vec!["OFF".to_owned(), "BALL".to_owned(), "BEAM".to_owned()];
        let (lock, _) = &*mode;
        let loc_mode = *lock.lock().unwrap();
        Self {
            items,
            state: ListState::default(),
            mode,
            loc_mode,
        }
    }


}

impl Selectable for ModeData {
    fn select(&mut self, index: Option<usize>) {
        self.state.select(index);
    }
    fn selected(&self) -> Option<usize> {
        self.state.selected()
    }

    fn len(&self) -> usize {
        self.items.len()
    }

}

fn draw_ref_mode_list<B: Backend>(f: &mut Frame<B>, ref_mode: &mut RefModeData, area: Rect) {
    let ref_list_items: Vec<ListItem> = ref_mode.items.iter().map(|s| {
        ListItem::new(s.as_ref())
    }).collect();
    let list = List::new(ref_list_items).block(Block::default().borders(Borders::ALL)
        .title("Reference Mode"))
        .highlight_symbol("*").highlight_style(Style::default().add_modifier(Modifier::SLOW_BLINK));
    f.render_stateful_widget(list, area, &mut ref_mode.state);
   
}

fn draw_reference_bar<B, S>(f: &mut Frame<B>, app: &mut App<S>, area: Rect)
where B: Backend, S: Iterator
{
    //let amp = &*app.regul.read().unwrap().ref_gen.get_amp();
    let amp = app.controller_data.get_amp()/MAX_AMP;
    let label = format!("{}", amp);
    let gauge = Gauge::default().block(Block::default().title("Amplitude")
    .borders(Borders::ALL)).gauge_style(Style::default().fg(Color::Cyan))
    .ratio(amp as f64)
    .label(label);
    if app.reference_area == None {
        app.reference_area = Some(area.clone());
    }

    f.render_widget(gauge, area);



}

fn draw_chart<B: Backend, S: Iterator<Item = (f64, f64)>>(
    f: &mut Frame<B>, signal: &mut PlotSignal<S>, area: Rect) {
    let dataset = vec![Dataset::default()
        .name("Control signal")
        .marker(symbols::Marker::Braille)
        .style(Style::default().fg(Color::Cyan))
        .graph_type(GraphType::Line)
        .data(&signal.points)];
    

    let x_labels = vec![
    Span::styled(
        format!("{:.1}", signal.window[0]),
        Style::default().add_modifier(Modifier::BOLD),
    ),
    Span::raw(format!("{:.1}", (signal.window[0] + signal.window[1]) / 2.0)),
    Span::styled(
        format!("{:.1}", signal.window[1]),
        Style::default().add_modifier(Modifier::BOLD),
    ),
    ];

    let chart = Chart::new(dataset)
        .block(
            Block::default()
                .title(Span::styled(
                    "Control signal",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ))
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .title("X Axis")
                .style(Style::default().fg(Color::Gray))
                .labels(x_labels)
                .bounds(signal.window)
        )
        .y_axis(
            Axis::default()
                .title("Y Axis")
                .style(Style::default().fg(Color::Gray))
                .bounds([-5.0, 5.0])
                .labels(vec![
                    Span::styled("-5.0", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw("0.0"),
                    Span::styled("5.0", Style::default().add_modifier(Modifier::BOLD)),
                ]),
        );
    f.render_widget(chart, area);

}

fn draw_mode_list<B>(f: &mut Frame<B>, mode: &mut ModeData, area: Rect)
where B: Backend
{
    let items: Vec<ListItem> = mode.items.iter().enumerate().map(|(i, s)| {
            let cur_mode = list_index_to_mode(i);
            let style = if cur_mode == mode.loc_mode {
                Style::default().fg(Color::Green)
            } else {Style::default()};
        
        ListItem::new(s.as_ref()).style(style)
    }).collect();
    
    let list = List::new(items).block(Block::default().borders(Borders::ALL)
        .title("MODE"))
        .highlight_symbol(">>");
    f.render_stateful_widget(list, area, &mut mode.state);

}


fn draw_parameter_table<B>(f: &mut Frame<B>, params: &mut ParameterData, control_data: &mut ControllerData, area: Rect)
where
    B: Backend
{
    /*let items = vec![
        ListItem::new("K"),
        ListItem::new("Ti"),
        ListItem::new("Td"),
        ListItem::new("Beta"),
    ];*/
    let items: Vec<Row> = params.items.iter().map(|i| {
        Row::new(i.to_owned())
    }).collect();
    let t = Table::new(items)
    .widths(&[Constraint::Length(20), Constraint::Length(5)])
    //.style(Style::default().fg(Color::White))
    .block(Block::default().borders(Borders::ALL).title("Parameters"))
    .highlight_style(Style::default().add_modifier(Modifier::BOLD))
    .highlight_symbol(">>");
    f.render_stateful_widget(t, area, &mut params.state);
    // You can set the style of the entire Table.
}
#[derive(PartialEq, Eq)]
enum InputMode {
    Normal,
    Editing,
    Quit,
}




fn get_parameter_string(params: &mut ParameterData) -> String {
    let mut s = String::new();
    loop {
        if let Ok(Event::Key(event)) = read() {
            match event.code {
                KeyCode::Char(ch) => {
                    s.push(ch);

                },
                KeyCode::Enter => {
                    return s;
                },
                KeyCode::Backspace => {s.pop();},
                _ => (),
            }

        }
    }


}


fn draw_first_tab<B, S>(f: &mut Frame<B>, app: &mut App<S>, area: Rect)
    where B: Backend, S: Iterator<Item = (f64, f64)>
{

     let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Min(0)].as_ref())
        .split(area);
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(5), Constraint::Min(0)].as_ref())
        .split(main_chunks[0]);
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(5), Constraint::Min(0)].as_ref())
        .split(main_chunks[1]);
    let ref_list_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(20), Constraint::Min(0)].as_ref())
        .split(right_chunks[0]);
    draw_parameter_table(f, &mut app.params, &mut app.controller_data, left_chunks[1]);
    draw_mode_list(f, &mut app.mode, left_chunks[0]);
    draw_chart(f, &mut app.signal, right_chunks[1]);
    draw_ref_mode_list(f, &mut app.ref_mode, ref_list_chunks[0]);
    draw_reference_bar(f, app, ref_list_chunks[1]);
    match app.input_mode {
        InputMode::Normal => {},
        InputMode::Editing => {
            if let Some(ind) = app.params.selected() {
                f.set_cursor(
                    left_chunks[1].left() + 20 + app.params.items[ind][1].len() as u16 + 4,
                    left_chunks[1].top() + 1 + ind as u16,
                )
            }

        },
        InputMode::Quit => {},
        
    }
}

fn draw_second_tab<B, S>(f: &mut Frame<B>, app: &mut App<S>, area: Rect)
where B: Backend, S: Iterator
{   
    let c = Canvas::default()
    .block(Block::default().title("Beam").borders(Borders::ALL))
    .x_bounds([-2000.0, 2000.0])
    .y_bounds([-2000.0, 2000.0])
    .marker(symbols::Marker::Braille)
    .paint(|ctx| {
        /*ctx.draw(&Map {
            resolution: MapResolution::High,
            color: Color::Black,
        });*/
        //ctx.layer();
        for line in app.canvas.shapes.0.iter() {
            ctx.draw(line);
        }
        ctx.draw(&app.canvas.shapes.1);

    });
    f.render_widget(c, area);

}

fn ui<B, S>(f: &mut Frame<B>, app: &mut App<S>)
where B: Backend,
      S: Iterator<Item = (f64, f64)>
{

    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());
    let titles = ["Controller", "Ball and beam"].iter()
        .map(|s| Spans::from(Span::styled(*s, Style::default().fg(Color::Blue))))
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("UI"))
        .highlight_style(Style::default().fg(Color::Yellow));
    match app.index {
        TabIndex::First => {
            let tabs = tabs.select(0);
            f.render_widget(tabs, chunks[0]);
            draw_first_tab(f, app, chunks[1]);
        },
        TabIndex::Second =>  {
            let tabs = tabs.select(1);
            f.render_widget(tabs, chunks[0]);
            draw_second_tab(f, app, chunks[1]);
           
        },
    }


   /*  let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Min(0)].as_ref())
        .split(f.size());
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(5), Constraint::Min(0)].as_ref())
        .split(main_chunks[0]);
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(5), Constraint::Min(0)].as_ref())
        .split(main_chunks[1]);
    let ref_list_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(20), Constraint::Min(0)].as_ref())
        .split(right_chunks[0]);
    draw_parameter_table(f, &mut app.params, &mut app.controller_data, left_chunks[1]);
    draw_mode_list(f, &mut app.mode, left_chunks[0]);
    draw_chart(f, &mut app.signal, right_chunks[1]);
    draw_ref_mode_list(f, &mut app.ref_mode, ref_list_chunks[0]);
    draw_reference_bar(f, app, ref_list_chunks[1]);
    match app.input_mode {
        InputMode::Normal => {},
        InputMode::Editing => {
            if let Some(ind) = app.params.selected() {
                f.set_cursor(
                    left_chunks[1].left() + 20 + app.params.items[ind][1].len() as u16 + 4,
                    left_chunks[1].top() + 1 + ind as u16,
                )
            }

        },
        InputMode::Quit => {},
        
    }*/
    
}

fn scroll_selectable<T: Selectable>(key: KeyCode, list: &mut T) {
    if key==KeyCode::Up {
        list.previous();
    } else if key==KeyCode::Down {
        list.next();
    }

}

fn list_index_to_mode(index: usize) -> Mode {

    match index {
        0 => Mode::OFF,
        1 => Mode::BALL,
        2 => Mode::BEAM,
        _ => Mode::OFF,
    }


}

fn map_to(interval_1: &[u16;2], interval_2: &[f64;2], point: u16) -> Option<f64> {
    if point > interval_1[1] || point< interval_1[0] {return None;}
    let length_1 = (interval_1[1] - interval_1[0]);
    let length_2 = (interval_2[1]-interval_2[0]);
    let ratio = (point - interval_1[0]) as f64/length_1 as f64;
    return Some(ratio*length_2 + interval_2[0]);
} 

fn on_mouse<S: Iterator>(app: &mut App<S>, mouse: MouseEvent) {
    //app.controller_data.set_amplitude(0.5);

        if let MouseEventKind::Drag(button) = mouse.kind {
            if button == MouseButton::Left {
                let mouse_x = mouse.column;
                let mouse_y = mouse.row;
                let area = app.reference_area.
                unwrap_or(Rect::new(0, 0, 0, 0));
                let area_x = area.x;
                let area_y = area.y;
                let gauge_left = area_x;
                let gauge_right = area_x + area.width-1;
                let gauge_upper = area_y;
                let gauge_lower = area_y+area.height;
                if let Some(p) = map_to(&[gauge_left, gauge_right], &[0.0, MAX_AMP as f64], mouse_x) {
                    if mouse_y > gauge_upper && mouse_y < gauge_lower {
                        //let p = mouse_x as f32 / gauge_right as f32;
                        app.controller_data.set_amplitude(p  as f32);
                        
                    } else {
                        //app.controller_data.set_amplitude(0.5);
                    }
                } else {
                    //app.controller_data.set_amplitude(0.5);
                }

                
            }
        } else if let MouseEventKind::Up(button) = mouse.kind {
            if button == MouseButton::Left {
                let lock = &mut *app.ref_gen.write().unwrap();
                lock.set_amp(app.controller_data.get_amp());
            }
        }
        


}


fn on_key<S: Iterator>(app: &mut App<S>, key: KeyCode) {
    match app.input_mode {
        InputMode::Normal => {
            if let KeyCode::Char(ch) = key {
                if ch =='q' {
                    app.input_mode = InputMode::Quit;
                    return;
                } else if ch == 't' {
                    app.selection.next();
                }
            }
            if key == KeyCode::Tab {
                app.index.next();
                return;
            }
            
            match app.selection {
                SelectionMode::Mode => {
                    app.params.unselect();
                    scroll_selectable(key, &mut app.mode);
                   if key == KeyCode::Enter {
                        if let Some(index) = app.mode.selected() {
                            let (mode_lock, cvar) = &(*app.mode.mode);
                            let mode = list_index_to_mode(index);
                            *(mode_lock.lock().unwrap()) = mode;
                            app.mode.loc_mode = mode;
                            cvar.notify_all();
                        }
                   }
                },  
                SelectionMode::Parameters => {
                    app.mode.unselect();
                    if let KeyCode::Char(ch) = key {
                        if (ch == 'i') {app.input_mode = InputMode::Editing};
                    } else {scroll_selectable(key, &mut app.params)}
                    
                },
                SelectionMode::Reference => {
                    scroll_selectable(key, &mut app.ref_mode);
                    if let Some(index) = app.ref_mode.selected() {
                        let ref_mode = match index {
                            0 => RefMode::MANUAL,
                            1 => RefMode::SQUARE,
                            2 => RefMode::OPTIMAL,
                            _ => RefMode::MANUAL,
                        };
                        let (rmode, _)= &*app.ref_mode.ref_mode;
                        *(rmode.lock().unwrap()) = ref_mode;
                       // app.ref_mode.loc_ref_mode = ref_mode;
                    }

                },
            }

        },
        InputMode::Editing => {
            match key {
                KeyCode::Char(ch) => app.params.add_char(ch),
                KeyCode::Enter => {
                    app.input_mode = InputMode::Normal;
                    app.params.set_param();
                    &(*app.regul.write().unwrap()).set_parameters(app.params.get_params());

                },
                KeyCode::Backspace => app.params.remove_char(),
                KeyCode::Delete => app.params.remove_char(),
                _ => (),          
                
            }
        },
        InputMode::Quit => (),
    }
}

fn on_input<S: Iterator>(app: &mut App<S>) -> io::Result<()> {

    match read()? {
        Event::Mouse(event) => on_mouse(app, event),//app.controller_data.set_amplitude(0.5),
        Event::Key(event) => {
            if app.index == TabIndex::First {
                on_key(app, event.code);
            } else {
                if event.code == KeyCode::Char('q') {
                    app.input_mode = InputMode::Quit;
                } else if event.code == KeyCode::Tab {
                    app.index.next();
                }
            }
        },
        _ => (),
    }
    Ok(())
}

fn run_app<B: Backend, S: Iterator<Item = (f64, f64)>> (terminal: &mut Terminal<B>, mut app: App<S>) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {

        terminal.draw(|f| {
            ui(f, &mut app);

        })?;
        let tick_rate = Duration::from_millis(app.signal.tick_rate as u64);
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            on_input(&mut app)?;
            if app.input_mode == InputMode::Quit {
                return Ok(());
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.signal.on_tick();
            app.canvas.on_tick();
            last_tick = Instant::now();
        }
        

    }

}

pub fn run<S: Iterator<Item = (f64, f64)>>(mut app: App<S>) -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    run_app(&mut terminal, app)?;
        
        /*if poll(Duration::from_millis(500))? {
            match read()? {
                Event::Key(event) => {
                    match event.code {
                        KeyCode::Down => param_data.next(),
                        KeyCode::Up => param_data.previous(),
                        KeyCode::Char('i') => {
                            let s = get_parameter_string(&mut param_data);
                            param_data.set_param(&s);
                        }
                        KeyCode::Char('q') => {break;},
                        _ => (),
                    }
                },
                _ => (),

            }
        }*/


    



    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;
    Ok(())
}



/*fn main() {
    let t = thread::spawn(|| {
        //run().unwrap();
    });
    t.join();
}*/
