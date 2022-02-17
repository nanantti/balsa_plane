mod polygon;

use polygon::Polygon;

#[cfg(test)]
mod tests {
    use super::get_command;
    use super::get_parameters;
    use super::read_script;

    #[test]
    fn point_init_string() {
        assert_eq!(read_script("POLY 0,0 4,0 2,1").area(), 2.0);
    }

    #[test]
    fn get_commmand_from_script() {
        assert_eq!(get_command("POLY 0,0 4,0 2,1"), "POLY");
    }

    #[test]
    fn get_parameters_from_script() {
        assert_eq!(get_parameters("POLY 0,0 4,0 2,1"), "0,0 4,0 2,1");
    }
}

fn get_command(script: &str) -> &str {
    let decomposed_script: Vec<&str> = script.split(' ').collect();
    decomposed_script[0]
}

fn get_parameters(script: &str) -> &str {
    script.trim_start_matches(get_command(script)).trim_start()
}

fn read_script(script: &str) -> Polygon {
    if get_command(script) == "POLY" {
        Polygon::init(get_parameters(script))
    } else {
        panic!("Invalid command ");
    }
}
