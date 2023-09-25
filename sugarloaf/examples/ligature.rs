extern crate tokio;
use sugarloaf::{
    core::{Sugar, SugarStyle},
    layout::SugarloafLayout,
    Sugarloaf,
};
use winit::platform::run_ondemand::EventLoopExtRunOnDemand;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

#[tokio::main]
async fn main() {
    let mut event_loop = EventLoop::new().unwrap();
    let width = 400.0;
    let height = 200.0;

    let window = WindowBuilder::new()
        .with_title("Font ligature example")
        .with_inner_size(LogicalSize::new(width, height))
        .with_resizable(true)
        .build(&event_loop)
        .unwrap();

    let scale_factor = window.scale_factor();
    let font_size = 30.;

    let sugarloaf_layout = SugarloafLayout::new(
        width as f32,
        height as f32,
        (10.0, 10.0, 0.0),
        scale_factor as f32,
        font_size,
        1.0,
        (2, 1),
    );

    let mut sugarloaf = Sugarloaf::new(
        &window,
        wgpu::PowerPreference::HighPerformance,
        sugarloaf::font::fonts::SugarloafFonts::default(),
        sugarloaf_layout,
        None,
    )
    .await
    .expect("Sugarloaf instance should be created");

    let _ = event_loop.run_ondemand(move |event, _, control_flow| {
        control_flow.set_wait();

        let spaced1 = vec![
            Sugar {
                content: 'a',
                foreground_color: [1.0, 1.0, 1.0, 1.0],
                background_color: [0.0, 0.0, 0.0, 1.0],
                style: None,
                decoration: None,
            },
            Sugar {
                content: 'b',
                foreground_color: [0.0, 0.0, 0.0, 1.0],
                background_color: [1.0, 1.0, 1.0, 1.0],
                style: None,
                decoration: None,
            },
            Sugar {
                content: 'a',
                foreground_color: [1.0, 1.0, 1.0, 1.0],
                background_color: [0.0, 0.0, 0.0, 1.0],
                style: None,
                decoration: None,
            },
            Sugar {
                content: 'b',
                foreground_color: [0.0, 0.0, 0.0, 1.0],
                background_color: [1.0, 1.0, 1.0, 1.0],
                style: None,
                decoration: None,
            },
            Sugar {
                content: 'a',
                foreground_color: [1.0, 1.0, 1.0, 1.0],
                background_color: [0.0, 0.0, 0.0, 1.0],
                style: None,
                decoration: None,
            },
            Sugar {
                content: 'b',
                foreground_color: [0.0, 0.0, 0.0, 1.0],
                background_color: [1.0, 1.0, 1.0, 1.0],
                style: None,
                decoration: None,
            },
            Sugar {
                content: 'a',
                foreground_color: [1.0, 1.0, 1.0, 1.0],
                background_color: [0.0, 0.0, 0.0, 1.0],
                style: None,
                decoration: None,
            },
            Sugar {
                content: 'b',
                foreground_color: [0.0, 0.0, 0.0, 1.0],
                background_color: [1.0, 1.0, 1.0, 1.0],
                style: None,
                decoration: None,
            },
            Sugar {
                content: 'a',
                foreground_color: [1.0, 1.0, 1.0, 1.0],
                background_color: [0.0, 0.0, 0.0, 1.0],
                style: None,
                decoration: None,
            },
            Sugar {
                content: 'b',
                foreground_color: [0.0, 0.0, 0.0, 1.0],
                background_color: [1.0, 1.0, 1.0, 1.0],
                style: None,
                decoration: None,
            },
        ];

        let spaced2 = vec![
            Sugar {
                content: 'c',
                foreground_color: [1.0, 1.0, 1.0, 1.0],
                background_color: [0.0, 0.0, 0.0, 1.0],
                style: None,
                decoration: None,
            },
            Sugar {
                content: 'd',
                foreground_color: [0.0, 0.0, 0.0, 1.0],
                background_color: [1.0, 1.0, 1.0, 1.0],
                style: None,
                decoration: None,
            },
        ];

        let sugar = vec![
            Sugar {
                content: '-',
                foreground_color: [1.0, 1.0, 1.0, 1.0],
                background_color: [0.0, 0.0, 0.0, 1.0],
                style: None,
                decoration: None,
            },
            Sugar {
                content: '>',
                foreground_color: [0.0, 0.0, 0.0, 1.0],
                background_color: [1.0, 1.0, 1.0, 1.0],
                style: Some(SugarStyle {
                    is_italic: false,
                    is_bold_italic: false,
                    is_bold: true,
                }),
                decoration: None,
            },
            Sugar {
                content: '!',
                foreground_color: [1.0, 1.0, 1.0, 1.0],
                background_color: [0.0, 0.0, 0.0, 1.0],
                style: None,
                decoration: None,
            },
            Sugar {
                content: '=',
                foreground_color: [0.0, 0.0, 0.0, 1.0],
                background_color: [1.0, 1.0, 1.0, 1.0],
                style: None,
                decoration: None,
            },
        ];

        let loaf = vec![
            Sugar {
                content: '㏑',
                foreground_color: [0.0, 0.0, 0.0, 1.0],
                background_color: [0.0, 1.0, 1.0, 1.0],
                style: None,
                decoration: None,
            },
            // Font Symbol (apple symbols font)
            Sugar {
                content: '⫹',
                foreground_color: [1.0, 1.0, 1.0, 1.0],
                background_color: [0.0, 0.0, 0.0, 1.0],
                style: None,
                decoration: None,
            },
            // Font Regular (firamono)
            Sugar {
                content: 'λ',
                foreground_color: [0.0, 0.0, 0.0, 1.0],
                background_color: [0.0, 1.0, 1.0, 1.0],
                style: None,
                decoration: None,
            },
            // Font Emojis
            Sugar {
                content: '🥇',
                foreground_color: [1.0, 1.0, 1.0, 1.0],
                background_color: [0.0, 0.0, 0.0, 1.0],
                style: None,
                decoration: None,
            },
            Sugar {
                content: '👷',
                foreground_color: [0.0, 0.0, 0.0, 1.0],
                background_color: [0.0, 0.0, 1.0, 1.0],
                style: None,
                decoration: None,
            },
        ];

        match event {
            Event::Resumed => {
                sugarloaf.set_background_color(wgpu::Color::RED);
                // sugarloaf.calculate_bounds();
                window.request_redraw();
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => control_flow.set_exit(),
                WindowEvent::ScaleFactorChanged {
                    // mut inner_size_writer,
                    scale_factor,
                    ..
                } => {
                    let scale_factor_f32 = scale_factor as f32;
                    let new_inner_size = window.inner_size();
                    sugarloaf
                        .rescale(scale_factor_f32)
                        .resize(new_inner_size.width, new_inner_size.height);
                    // .calculate_bounds();
                    window.request_redraw();
                }
                winit::event::WindowEvent::Resized(new_size) => {
                    sugarloaf.resize(new_size.width, new_size.height);
                    // .calculate_bounds();
                    window.request_redraw();
                }
                _ => (),
            },
            Event::RedrawRequested { .. } => {
                sugarloaf.stack(spaced1);
                sugarloaf.stack(spaced2);
                sugarloaf.stack(sugar);
                sugarloaf.stack(loaf);
                sugarloaf.render();
            }
            _ => {
                *control_flow = winit::event_loop::ControlFlow::Wait;
            }
        }
    });
}
