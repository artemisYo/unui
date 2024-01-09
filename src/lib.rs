#[cfg(test)]
mod tests {
    #[test]
    fn simple_window() {
        const IMG_HEIGHT: u16 = 150;
        const IMG_WIDTH: u16 = 150;
        
        use xcb::{x::{self, Drawable, Window, Gcontext}, Connection};
        // create connection
        let (connection, screen_num) = Connection::connect(None).unwrap();
        // get env paramateres
        let setup = connection.get_setup();
        // get the preferred screen to show on
        let screen = setup.roots().nth(screen_num as usize).unwrap();
        // generate some handles used for requests
        let window: Window = connection.generate_id();
        let drawable_win = Drawable::Window(window);
        let graphics: Gcontext = connection.generate_id();

        // create buffer to be displayed
        let mut img = std::iter::repeat(0u32)
            .take((IMG_HEIGHT * IMG_WIDTH) as usize)
            .flat_map(|i| i.to_be_bytes().into_iter())
            .collect::<Vec<_>>();

        // create window and check for errors
        // then also show the window
        connection.check_request(
            connection.send_request_checked(&x::CreateWindow {
                depth: screen.root_depth(),
                wid: window,
                parent: screen.root(),
                x: 0, y: 0,
                width: IMG_WIDTH,
                height: IMG_HEIGHT,
                border_width: 0,
                class: x::WindowClass::InputOutput,
                visual: screen.root_visual(),
                value_list: &[
                    x::Cw::BackPixel(screen.white_pixel()),
                    x::Cw::EventMask(x::EventMask::EXPOSURE | x::EventMask::KEY_PRESS)
                ],
            })
        ).unwrap();
        connection.check_request(
            connection.send_request_checked(&x::MapWindow { window })
        ).unwrap();
        // initialize graphic context
        connection.check_request(
            connection.send_request_checked(&x::CreateGc {
                cid: graphics,
                drawable: drawable_win,
                value_list: &[],
            })
        ).unwrap();

        // event loop
        let mut current_row = 0;
        let mut mask = 0xffu8;
        loop {
            match connection.wait_for_event().unwrap() {
                // if window changed (size, visibility, etc)
                // then show the buffer
                xcb::Event::X(x::Event::Expose(_)) =>
                    connection.check_request(
                        connection.send_request_checked(&x::PutImage {
                            // this format is the typical all colors at once format
                            // though it uses BGR(A) instead for some reason
                            // there is also XYPixmap, which has all channels separate
                            format: x::ImageFormat::ZPixmap,
                            drawable: drawable_win,
                            gc: graphics,
                            width: IMG_WIDTH, height: IMG_HEIGHT,
                            dst_x: 0, dst_y: 0,
                            left_pad: 0,
                            depth: screen.root_depth(),
                            data: &img,
                        })
                    ).unwrap(),
                // if spacebar is pressed modify and redraw buffer
                xcb::Event::X(x::Event::KeyPress(key)) => {
                    if key.detail() == 65 {
                        if current_row >= IMG_HEIGHT as usize {
                            mask = !mask;
                            current_row = 0;
                        }
                        let offset = 4 * current_row * IMG_WIDTH as usize;
                        for i in 0..IMG_WIDTH as usize {
                            img[i * 4 + offset] = 0xfdu8 & mask;
                            img[i * 4 + offset + 1] = 0x87u8 & mask;
                            img[i * 4 + offset + 2] = 0x72u8 & mask;
                            img[i * 4 + offset + 3] = 0xffu8;
                        }
                        current_row += 1;
                    } else {
                        println!("{:?}",
                            xcb::Event::X(x::Event::KeyPress(key)));
                    }
                    connection.check_request(
                        connection.send_request_checked(&x::PutImage {
                            format: x::ImageFormat::ZPixmap,
                            drawable: drawable_win,
                            gc: graphics,
                            width: IMG_WIDTH, height: IMG_HEIGHT,
                            dst_x: 0, dst_y: 0,
                            left_pad: 0,
                            depth: screen.root_depth(),
                            data: &img,
                        })
                    ).unwrap()
                }
                // print any unhandled events
                o => println!("{:?}", o),
            }
        }
    }
    #[test]
    fn shm_window() {        
        let (connection, _) = xcb::Connection::connect(None).unwrap();
        let shm_ver = connection.wait_for_reply(
            connection.send_request(&xcb::shm::QueryVersion {})
        ).unwrap();
        dbg!(shm_ver); // prints no support for shared pixmaps on my system
    }
}
