use beryllium::{
  events::Event,
  init::InitFlags,
  video::{CreateWinArgs, GlProfile, GlSwapInterval},
  Sdl,
};
use gles31::{glClear, glClearColor, load_gl_functions, GL_COLOR_BUFFER_BIT};

fn main() {
  // Initializes SDL2
  let sdl = Sdl::init(InitFlags::EVERYTHING);

  // This part asks for an ES 3.1 context "just for fun", because that's what
  // works best between Windows and also Raspberry Pi. Mac doesn't support ES
  // contexts, but this is just a demo so for Mac we'll skip any configuration
  // at all and just get some "don't care" GL context.
  #[cfg(not(target_os = "macos"))]
  {
    sdl.set_gl_profile(GlProfile::ES).unwrap();
    sdl.set_gl_context_major_version(3).unwrap();
    sdl.set_gl_context_minor_version(1).unwrap();
  }

  // Makes the window with a GL Context.
  let win = sdl
    .create_gl_window(CreateWinArgs {
      title: "Example GL Window",
      resizable: true,
      ..Default::default()
    })
    .unwrap();
  println!("GL window size: {:?}", win.get_window_size());
  println!("GL drawable size: {:?}", win.get_drawable_size());
  println!(
    "GL_KHR_debug supported: {}",
    win.supports_extension("GL_KHR_debug")
  );
  win.set_swap_interval(GlSwapInterval::Vsync).ok();

  if let Err(err_list) =
    unsafe { load_gl_functions(&|name| win.get_proc_address(name)) }
  {
    for err in err_list {
      println!("Function didn't load: {err}");
    }
  } else {
    println!("All functions loaded.");
  }

  let (mut win_width, mut win_height) = win.get_window_size();
  let (mut ptr_x, mut ptr_y) = (win_width, win_height);

  // program "main loop".
  'the_loop: loop {
    // Process events from this frame.
    #[allow(clippy::never_loop)]
    while let Some((event, _timestamp)) = sdl.poll_events() {
      match event {
        Event::Quit => break 'the_loop,
        Event::MouseMotion { x_win, y_win, .. } => {
          ptr_x = x_win;
          ptr_y = y_win;
        }
        Event::WindowResized { width, height, .. } => {
          win_width = width;
          win_height = height;
        }
        _ => (),
      }
    }

    unsafe {
      glClearColor(
        ptr_x as f32 / win_width as f32,
        ptr_y as f32 / win_height as f32,
        0.0,
        1.0,
      )
    };

    unsafe { glClear(GL_COLOR_BUFFER_BIT) };

    win.swap_window();
  }

  // All the cleanup is handled by the various drop impls.
}
