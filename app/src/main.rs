use std::fs;
use wry::application::event::{Event, StartCause, WindowEvent};
use wry::application::event_loop::{ControlFlow, EventLoop};
use wry::application::window::WindowBuilder;
use wry::http::ResponseBuilder;
use wry::webview::{WebContext, WebViewBuilder};

fn main() -> anyhow::Result<()> {
    let mut current_dir = std::env::current_dir()?;
    current_dir.push("data");

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Hello World")
        .build(&event_loop)?;

    let mut web_context = WebContext::new(Some(current_dir));

    let webview = WebViewBuilder::new(window)?
        .with_web_context(&mut web_context)
        .with_url("wry://static/index.html")?
        .with_custom_protocol("wry".into(), move |request| {
            let path = request.uri().replace("wry://", "");
            let content = fs::read(fs::canonicalize(&path)?)?;

            let (data, meta) = if path.ends_with(".html") {
                (content, "text/html")
            } else if path.ends_with(".js") {
                (content, "text/javascript")
            } else {
                unimplemented!()
            };

            ResponseBuilder::new().mimetype(meta).body(data)
        })
        .with_devtools(true)
        .build()?;

    webview.open_devtools();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => println!("Wry application started!"),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
