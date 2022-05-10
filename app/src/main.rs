use wry::application::event::{Event, StartCause, WindowEvent};
use wry::application::event_loop::{ControlFlow, EventLoop};
use wry::application::window::WindowBuilder;
use wry::webview::{WebContext, WebViewBuilder};

fn main() -> anyhow::Result<()> {
    let mut current_dir = std::env::current_dir()?;
    current_dir.push("data");
    dbg!(&current_dir);

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Hello World")
        .build(&event_loop)?;

    let mut webcontext = WebContext::new(Some(current_dir));

    let webview = WebViewBuilder::new(window)?
        .with_url("https://localhost:3000")?
        .with_devtools(true)
        .with_web_context(&mut webcontext)
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
