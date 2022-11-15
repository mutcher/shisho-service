use super::ShishoCommand;

pub struct GetBookCommand;
pub struct GetAllBooksCommand;

impl ShishoCommand for GetBookCommand {
    fn perform(args: &[String], stream: &mut std::net::TcpStream) {
        todo!()
    }
}

impl ShishoCommand for GetAllBooksCommand {
    fn perform(args: &[String], stream: &mut std::net::TcpStream) {
        todo!()
    }
}