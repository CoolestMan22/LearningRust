// Crates
use iced::theme::Theme;
use iced::widget::{Text};
use iced::{Element, Sandbox, Settings};

// Main
fn main() -> iced::Result {
    RustUI::run(Settings::default())
}

// RustUI Struct
struct RustUI {
    theme: Theme, // Dark or light theme
    page: Page,   // What page we are on
    login_field: LoginField,
}

// LoginField Struct
struct LoginField {
    email: String,
    password: String,
}

// Page Enum
#[derive(Debug, Clone, PartialEq, Eq)]
enum Page {
    Login,
    Register,
}

// Message Enum
#[derive(Debug, Clone)]
enum Message {
    ToggleTheme,                       // Changes theme
    LoginSubmit,                       // Prints email and password
    Router(String),                    // Changes page
    LoginFieldChanged(String, String), // Changes email and password
}

// Sandbox for RustUI
impl Sandbox for RustUI {
    type Message = Message;

    // App Constructor
    fn new() -> Self {
        Self {
            theme: Theme::Dark, // Default theme (Dark)
            page: Page::Login,  // Default page (1)
            login_field: LoginField {
                email: String::new(),    // Default email ("")
                password: String::new(), // Default password ("")
            },
        }
    }

    // App Title
    fn title(&self) -> String {
        String::from("RustUI - Iced")
    }

    // App Theme Func
    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    // App Update Func
    fn update(&mut self, message: Message) {
        match message {
            Message::ToggleTheme => {
                // Set the theme to dark if it's light, and vice versa
                self.theme = match self.theme {
                    Theme::Dark => Theme::Light,
                    Theme::Light => Theme::Dark,
                    _ => todo!()
                };
             
                println!("{}", self.theme);

            }

            Message::LoginFieldChanged(email, password) => {
                // self.login_field.email = email;
                // self.login_field.password = password;
            }

            Message::LoginSubmit => {
                // println!("Email: {}, Password: {}", self.login_field.email, self.login_field.password);
            }

            Message::Router(page) => {
                // match page.as_str() {
                //     "login" => self.page = Page::Login,
                //     "register" => self.page = Page::Register,
                //     _ => {}
                // }
            }
        }
    }

    // App View Func
    fn view(&self) -> Element<Message> {
        // Make a simple button in the middle
        Text::new("LIFELINE IS A FEMBOY").into()
    }
}
