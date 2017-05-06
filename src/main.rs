extern crate env_logger;
extern crate x11;

extern crate lanta;


use lanta::{Config, RustWindowManager};
use lanta::keys::{KeyCombo, KeyHandlers, ModKey};
use std::process::Command;
use std::rc::Rc;


fn main() {
    env_logger::init().unwrap();

    let keys = KeyHandlers::new(vec![(KeyCombo::new(vec![ModKey::Mod4], x11::keysym::XK_t),
                                      Rc::new(lanta::close_window)),
                                     (KeyCombo::new(vec![ModKey::Mod4], x11::keysym::XK_y),
                                      Rc::new(lanta::focus_next)),
                                     (KeyCombo::new(vec![ModKey::Mod4], x11::keysym::XK_u),
                                      Rc::new(lanta::focus_previous)),
                                     (KeyCombo::new(vec![ModKey::Mod4], x11::keysym::XK_i),
                                      Rc::new(lanta::shuffle_next)),
                                     (KeyCombo::new(vec![ModKey::Mod4], x11::keysym::XK_o),
                                      Rc::new(lanta::shuffle_previous)),
                                     (KeyCombo::new(vec![ModKey::Mod4], x11::keysym::XK_p),
                                      lanta::spawn_command(Command::new("xterm"))),
                                     (KeyCombo::new(vec![ModKey::Mod4], x11::keysym::XK_b),
                                      Rc::new(lanta::layout_next)),
                                     (KeyCombo::new(vec![ModKey::Mod4], x11::keysym::XK_n),
                                      lanta::switch_group("g1")),
                                     (KeyCombo::new(vec![ModKey::Mod4], x11::keysym::XK_m),
                                      lanta::switch_group("g2"))]);

    let config = Config { keys: keys };

    let mut wm = RustWindowManager::new(config).unwrap();
    wm.run_event_loop();
}
