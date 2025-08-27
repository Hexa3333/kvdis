use kvdis::commmand;

fn main() {
    commmand::parse_line_to_command("SET key value").unwrap();
}
