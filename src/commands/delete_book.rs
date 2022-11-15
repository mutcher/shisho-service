use super::ShishoCommand;

pub struct DeleteBookCommand;

impl ShishoCommand for DeleteBookCommand {
    fn perform(args: &[String], stream: &mut std::net::TcpStream) {
        todo!()
    }
}
