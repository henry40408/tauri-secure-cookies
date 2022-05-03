use std::fs::{canonicalize, read};
use wry::application::event::{Event, StartCause, WindowEvent};
use wry::application::event_loop::{ControlFlow, EventLoop};
use wry::application::window::WindowBuilder;
use wry::http::ResponseBuilder;
use wry::webview::WebViewBuilder;

fn main() -> anyhow::Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Hello World")
        .build(&event_loop)?;

    let _webview = WebViewBuilder::new(window)?
        .with_custom_protocol("wry".into(), move |request| {
            // Remove url scheme
            let path = request.uri().replace("wry://", "");
            // Read the file content from file path
            let content = read(canonicalize(&path)?)?;

            // Return asset contents and mime types based on file extentions
            // If you don't want to do this manually, there are some crates for you.
            // Such as `infer` and `mime_guess`.
            let (data, meta) = if path.ends_with(".html") {
                (content, "text/html")
            } else if path.ends_with(".js") {
                (content, "text/javascript")
            } else if path.ends_with(".png") {
                (content, "image/png")
            } else {
                unimplemented!();
            };

            ResponseBuilder::new().mimetype(meta).body(data)
        })
        // tell the webview to load the custom protocol
        .with_url("https://localhost:3030")?
        .build()?;

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
