use makepad_widgets::*;

live_design!{
  import makepad_widgets::base::*;
  import makepad_widgets::theme_desktop_dark::*;
  import makepad_draw::shader::std::*;

  import moxin_web::my_widget::MyWidget;
  import moxin_web::particles::ParticleSystem;
  import moxin_web::birds::BirdSystem;

  COLOR_CONTAINER = (THEME_COLOR_D_1)
  COLOR_ACCENT = (THEME_COLOR_MAKEPAD)

  TEXT_COLOR = #333
  DEMO_COLOR_1 = #03f
  DEMO_COLOR_2 = #08f
  DEMO_COLOR_3 = #444
  LINK_LABEL_SIZE = 15
  ICON_WIDTH = 30.

  ZooBlock = <RoundedView> {
    width: 140., height: 50.
    margin: 0.,
    spacing: 0.,

    show_bg: true;
    draw_bg: {
      fn get_color(self) -> vec4 {
        return mix(self.color, self.color*0.5, self.pos.y);
      }
      radius: (THEME_CONTAINER_CORNER_RADIUS)
    }
  }

  App = {{App}} {
    ui: <Root>{
      main_window = <Window>{
        show_bg: true
        width: Fill,
        height: Fill

        draw_bg: {
          fn pixel(self) -> vec4 {
            // test
            return mix(#8, #5, self.pos.y);
          }
        }

        body = <View>
        {
          flow: Overlay,
          width: Fill,
          height: Fill
          // align: {
          //   x: 0.5,
          //   y: 0.5
          // },

          water_bg = <View> {
            flow: Overlay,
            width: Fill,
            height: Fill
            // spacing: 0,
            // padding: 0,
            // align: {
            //   x: 0.5,
            //   y: 0.5
            // },
            show_bg: true,
            draw_bg: {
              fn pixel(self) -> vec4{
                return vec4(0.70,0.72,0.72,1)
              }
            }

            <Image>{
              width: Fill;
              height: Fill;
              source: dep("crate://self/resources/moxin-bg.jpg")
            }

            <ParticleSystem> {
              width: Fill,
              height: Fill,
              maxparticles: 50,
              spawnrate: 10,
              drop_width: 3,
              drop_height: 60,
              particletexture:{
                 source: dep("crate://self/resources/drop.png")
              }
            }

            <BirdSystem> {
              width: Fill,
              height: Fill,
              max_birds: 5,
              spawnrate: 2,
              bird_width: 20,
              bird_height: 20,
              birdtexture:{
                 source: dep("crate://self/resources/bird_combined.png")
              }
            }
          }

          content = <View>{
            flow: Down,
            align: {
              x: 0.7,
              y: 0.1,
            },

            <View> {
              // width: Fit,
              height: Fit,
              flow: Down,
              align: {
                x: 0.96,
                y: 0.5,
              },
              <View> {
                show_bg: false,
                width: Fit, height: Fit,
                flow: Down,
                <Image> {
                  source: dep("crate://self/resources/moxin-logo.png" )
                }
              }
            }

            <View> {
                width: 510, height: 320, flow: Down,
                align: {
                  x: 0.5,
                  y: 0.7,
                },
                <H2> {
                    draw_text: {
                        color: (TEXT_COLOR),
                    },
                    text: "An AI platform in pure Rust"
                }
            }

            <View> {
                // width: Fit,
               height: Fit,
               flow: Right,
                align: {
                  x: 0.7,
                  y: 0.2,
                },

                <ZooBlock> {
                    width: 140,
                    height: Fit,
                    flow: Right,
                    <ButtonFlat> {
                        icon_walk: { width: (ICON_WIDTH-5) }
                        draw_icon: {
                          color: (DEMO_COLOR_3),
                          svg_file: dep("crate://self/resources/windows-logo.svg"),
                        },
                    }
                    <LinkLabel> {
                        draw_text: {
                        fn get_color(self) -> vec4 {
                            return (DEMO_COLOR_3)
                        }
                        text_style: {
                            font_size: (LINK_LABEL_SIZE)
                        }
                        },
                        text: "Windows",
                        width: Fit,
                        url: "https://github.com/moxin-org/moxin/releases/download/v0.1.0-beta/moxin_0.1.0_x64-setup.exe",
                        open_in_place: false
                    }
                }

                <ZooBlock> {
                    width: 115,
                    height: Fit,
                    flow: Right,
                    <ButtonFlat> {
                        icon_walk: { width: (ICON_WIDTH-5) }
                        draw_icon: {
                        color: (DEMO_COLOR_3),
                        svg_file: dep("crate://self/resources/apple-logo.svg"),
                        }
                    }
                    <LinkLabel> {
                        draw_text: {
                        fn get_color(self) -> vec4 {
                            return (DEMO_COLOR_3)
                        }
                        text_style: {
                            font_size: (LINK_LABEL_SIZE)
                        }
                        },
                        text: "macOS",
                        width: Fit,
                        url: "https://github.com/moxin-org/moxin/releases/download/v0.1.0-beta/Moxin_0.1.0_aarch64.dmg",
                        open_in_place: false
                    }
                }

                <ZooBlock> {
                    width: 110,
                    flow: Right,
                    <ButtonFlat> {
                        icon_walk: { width: (ICON_WIDTH) }
                        draw_icon: {
                        color: (DEMO_COLOR_3),
                        svg_file: dep("crate://self/resources/linux-logo.svg"),
                        }
                    }
                    <LinkLabel> {
                        draw_text: {
                        fn get_color(self) -> vec4 {
                            return (DEMO_COLOR_3)
                        }
                        text_style: {
                            font_size: (LINK_LABEL_SIZE)
                        }
                        },
                        text: "Linux",
                        width: 100,
                        url: "https://github.com/moxin-org/moxin/releases/download/v0.1.0-beta/moxin_0.1.0_x86_64.AppImage",
                        open_in_place: false
                    }
                }

                <ZooBlock> {
                    flow: Right,
                    <ButtonFlat> {
                        icon_walk: { width: (ICON_WIDTH) }
                        draw_icon: {
                        color: (DEMO_COLOR_3),
                        svg_file: dep("crate://self/resources/arch-logo.svg"),
                        }
                    }
                    <LinkLabel> {
                        draw_text: {
                        fn get_color(self) -> vec4 {
                            return (DEMO_COLOR_3)
                        }
                        text_style: {
                            font_size: (LINK_LABEL_SIZE)
                        }
                        },
                        text: "archlinux",
                        width: Fit,
                        url: "https://github.com/moxin-org/moxin/releases/download/v0.1.0-beta/moxin_0.1.0_x86_64.tar.gz",
                        open_in_place: true
                    }
                }

                <ZooBlock> {
                    flow: Right,
                    <ButtonFlat> {
                        icon_walk: { width: (ICON_WIDTH) }
                        draw_icon: {
                          color: (DEMO_COLOR_3),
                          svg_file: dep("crate://self/resources/debian-logo.svg"),
                        }
                    }
                    <LinkLabel> {
                        draw_text: {
                        fn get_color(self) -> vec4 {
                            return (DEMO_COLOR_3)
                        }
                        text_style: {
                            font_size: (LINK_LABEL_SIZE)
                        }
                        },
                        text: "debian",
                        width: Fit,
                        url: "https://github.com/moxin-org/moxin/releases/download/v0.1.0-beta/moxin_0.1.0_amd64.deb",
                        open_in_place: false
                    }
                }

              }

              <View> {
                height: 100,
                flow: Right,
                align: {
                  x: 0.63,
                  y: 0.3,
                },

                <ZooBlock> {
                    width: 130,
                    height: Fit,
                    flow: Right,
                    <ButtonFlat> {
                        icon_walk: { width: (ICON_WIDTH) }
                        draw_icon: {
                          color: (DEMO_COLOR_3),
                          svg_file: dep("crate://self/resources/github-logo.svg"),
                        }
                    }
                    <LinkLabel> {
                        draw_text: {
                        fn get_color(self) -> vec4 {
                            return (DEMO_COLOR_3)
                        }
                        text_style: {
                            font_size: (LINK_LABEL_SIZE)
                        }
                        },
                        text: "GitHub",
                        width: Fit,
                        url: "https://github.com/moxin-org/moxin/releases/tag/v0.1.0-beta",
                        open_in_place: false
                    }
                }

                <ZooBlock> {
                    width: 140,
                    height: Fit,
                    flow: Right,
                    <ButtonFlat> {
                        icon_walk: { width: (ICON_WIDTH) }
                        draw_icon: {
                        color: (DEMO_COLOR_3),
                        svg_file: dep("crate://self/resources/discord-logo.svg"),
                        }
                    }
                    <LinkLabel> {
                        draw_text: {
                        fn get_color(self) -> vec4 {
                            return (DEMO_COLOR_3)
                        }
                        text_style: {
                            font_size: (LINK_LABEL_SIZE)
                        }
                        },
                        text: "Discord",
                        width: Fit,
                        url: "https://discord.gg/x6Ghmusu",
                        open_in_place: false
                    }
                }

              }
          }
        }
      }
    }
  }
}

app_main!(App);

#[derive(Live, LiveHook)]
pub struct App {
  #[live] ui: WidgetRef,
 }

impl LiveRegister for App {
  fn live_register(cx: &mut Cx) {
    crate::makepad_widgets::live_design(cx);
    crate::my_widget::live_design(cx);
    crate::particles::live_design(cx);
    crate::birds::live_design(cx);
  }
}


impl AppMain for App {
  fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
    // self.match_event(cx, event);
    self.ui.handle_event(cx, event, &mut Scope::empty());
  }
}
