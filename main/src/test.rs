
#[cfg(test)]
mod test {
    use std::env;
    use std::path::Path;
    use crate::{read_save, Save};

    #[test]
    fn basic_test() {
        assert!(true)
    }

    #[test]
    fn test_name() {
        let path = env::current_dir().unwrap();
        println!("The current dir: {}", path.display());
        println!("testing the savegames name");
        let s: Save = read_save(Path::new("saves/yellow_surfing_test.sav")).unwrap();
        println!("{}", s.trainer_name);
        assert!(true);
    }

}