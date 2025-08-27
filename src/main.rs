use kvdis::query;

fn main() {
    query::parse_line_to_command("SET key value").unwrap();
}
