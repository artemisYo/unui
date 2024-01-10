mod pixel;
mod buffer;
mod util;

#[cfg(test)]
mod tests {
    use super::{pixel::*, buffer::*};
    #[test]
    fn simple_window() {        
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
        let mut img = PixelBuffer::with_size(150, 150);

        // create window and check for errors
        // then also show the window
        connection.check_request(
            connection.send_request_checked(&x::CreateWindow {
                depth: screen.root_depth(),
                wid: window,
                parent: screen.root(),
                x: 0, y: 0,
                width: 150,
                height: 150,
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
                xcb::Event::X(x::Event::Expose(e)) => {
                    // adjust the size of the image to fit the entire buffer
                    img = img.resize(e.width() as usize, e.height() as usize);
                    connection.check_request(
                        connection.send_request_checked(&x::PutImage {
                            // this format is the typical all colors at once format
                            // though it uses BGR(A) instead for some reason
                            // there is also XYPixmap, which has all channels separate
                            format: x::ImageFormat::ZPixmap,
                            drawable: drawable_win,
                            gc: graphics,
                            width: img.width() as u16,
                            height: img.height() as u16,
                            dst_x: 0, dst_y: 0,
                            left_pad: 0,
                            depth: screen.root_depth(),
                            data: img.as_bgrx_slice(),
                        })
                    ).unwrap();
                }
                // if spacebar is pressed modify and redraw buffer
                xcb::Event::X(x::Event::KeyPress(key)) => {
                    if key.detail() == 65 {
                        if current_row >= img.height() {
                            mask = !mask;
                            current_row = 0;
                        }
                        let row = &mut img[current_row];
                        for p in row.iter_mut() {
                            p[Red] = 0x72 & mask;
                            p[Green] = 0x87 & mask;
                            p[Blue] = 0xfd & mask;
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
                            width: img.width() as u16,
                            height: img.height() as u16,
                            dst_x: 0, dst_y: 0,
                            left_pad: 0,
                            depth: screen.root_depth(),
                            data: img.as_bgrx_slice(),
                        })
                    ).unwrap()
                }
                // print any unhandled events
                o => println!("{:?}", o),
            }
        }
    }
}
