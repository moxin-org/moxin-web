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

    TEXT_COLOR = #d93a14
    DEMO_COLOR_1 = #03f
    DEMO_COLOR_2 = #08f
    DEMO_COLOR_3 = #333
    LINK_LABEL_SIZE = 15
    ICON_WIDTH = 20.

    ZooTitle = <View> {
        width: Fill, height: Fit,
        flow: Down,
        align: { x: 0.0, y: 0.5},
        margin: <THEME_MSPACE_3> {},
        spacing: 10.,
        show_bg: false,
        title = <H2> { text: "Moxin Web" }
    }

    ZooHeader = <View> {
        width: Fill, height: Fit,
        flow: Down,
        spacing: (THEME_SPACE_1),
        margin: <THEME_MSPACE_H_3> {}
        divider = <Hr> { }
        title = <H3> { text: "Header" }
    }

    ZooGroup = <RoundedView> {
        height: Fit, width: Fill,
        flow: Right,
        align: { x: 0.0, y: 0.5},
        margin: 0.,
        show_bg: false;
        draw_bg: { color: (COLOR_CONTAINER) }
    }

    ZooDesc = <P> { text: "" }

    ZooBlock = <RoundedView> {
        width: 240., height: 50.
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
                    spacing: 0,
                    padding: 0,
                    align: {
                        x: 0.5,
                        y: 0.5
                    },

                    water_bg = <View> {
                        flow: Overlay,
                        width: Fill,
                        height: Fill
                        spacing: 0,
                        padding: 0,
                        align: {
                            x: 0.5,
                            y: 0.5
                        },
                        show_bg: true,
                        draw_bg: {
                            fn pixel(self) -> vec4{
                                return vec4(0.70,0.72,0.72,1)
                            }
                        }

                        <Image>{
                            width: Fill;
                            height: Fill;
                            source: dep("crate://self/resources/background.jpg")
                        }

                        <Image>{
                            width: Fill;
                            height: Fill;
                            source: dep("crate://self/resources/water_mask.png")
                            draw_bg: {

                                fn pixel(self) -> vec4{
                                    let col = sample2d(self.image, self.pos);
                                    //let s = sin(self.pos.y * 150.0 + sin(self.pos.x*100.)*10.0 ) *0.5 + 0.5;

                                    let q = 10.0;
                                    let r = 1000.0;

                                    let s2 = sin((self.pos.x * 100.0)/self.pos.y +self.time+ sin(self.time*0.3+self.pos.y*r + self.pos.x*0.2/self.pos.y)*q)*0.25 + 0.75;
                                     let g = vec3(s2*0.9,s2*0.92,s2*0.95);
                                     let a = col.x * 0.3;

                                    return vec4(g*a, a);
                                }
                            }
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

                    content = <ScrollXYView>{
                        flow: Down,
                        spacing:10,
                        align: {
                            x: 0.5,
                            y: 0.5
                        },

                        <View> {
                            width: Fit, height: Fit, flow: Down,
                            <View> {
                                show_bg: false, draw_bg: { color: (THEME_COLOR_BG_CONTAINER)}, width: 100, height: 100, flow: Down,
                                <Image> { source: dep("crate://self/resources/moxin-logo.png" ) }
                            }
                        }


                        <View> {
                            width: 340, height: Fit, flow: Down,
                            <H1> {
                                draw_text: {
                                    color: (TEXT_COLOR),
                                },
                                text: "Moxin App"
                            }
                        }

                        <ZooBlock> {
                            flow: Right,

                            <ButtonFlat> {
                                icon_walk: { width: (ICON_WIDTH) }
                                draw_icon: {
                                    color: (DEMO_COLOR_1),
                                    svg_file: dep("crate://self/resources/github-mark.svg"),
                                }
                            }
                            <LinkLabel> {
                                draw_text: {
                                    fn get_color(self) -> vec4 {
                                        return (DEMO_COLOR_1)
                                    }
                                    text_style: {
                                        font_size: (LINK_LABEL_SIZE)
                                    }
                                },
                                text: "Release Page",
                                width: Fit,
                                url: "https://github.com/moxin-org/moxin/releases/tag/v0.1.0-alpha",
                                open_in_place: false
                            }
                        }
                        <ZooBlock> {
                            flow: Right,
                            <ButtonFlat> {
                                icon_walk: { width: (ICON_WIDTH) }
                                draw_icon: {
                                    color: (DEMO_COLOR_2),
                                    svg_file: dep("crate://self/resources/apple-logo.svg"),
                                }
                            }
                            <LinkLabel> {
                                draw_text: {
                                    fn get_color(self) -> vec4 {
                                        return (DEMO_COLOR_2)
                                    }
                                    text_style: {
                                        font_size: (LINK_LABEL_SIZE)
                                    }
                                },
                                text: "Download macOS",
                                width: Fit,
                                url: "https://github.com/moxin-org/moxin/releases/download/v0.1.0-alpha/Moxin_0.1.0_aarch64.dmg",
                                open_in_place: false
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
                                text: "Download Debian",
                                width: Fit,
                                url: "https://github.com/moxin-org/moxin/releases/download/v0.1.0-alpha/moxin_0.1.0_amd64.deb",
                                open_in_place: false
                            }
                        }
                        <ZooBlock> {
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
                                text: "Download Linux",
                                width: Fit,
                                url: "https://github.com/moxin-org/moxin/releases/download/v0.1.0-alpha/moxin_0.1.0_x86_64.AppImage",
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
                                text: "Download Arch Linux",
                                width: Fit,
                                url: "https://github.com/moxin-org/moxin/releases/download/v0.1.0-alpha/Moxin_0.1.0_aarch64.dmg",
                                open_in_place: true
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
