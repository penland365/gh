pub mod orgs;
pub mod pullreqs;
pub mod users;

#[cfg(windows)] pub const NEW_LINE: &'static str      = "\r\n";
#[cfg(not(windows))] pub const NEW_LINE: &'static str = "\n";
