
//***********/
// UNIT TEST 
//***********/
#[cfg(test)]
mod test_yaml_utils{
    use std::sync::Once;

    use bt_logger::{build_logger, LogLevel, LogTarget};
    use bt_yaml_utils::{convert_yaml_to_vec_string, get_bool, get_i32, get_u32, get_yaml, get_yaml_from_string};
    use once_cell::sync::Lazy;
    
    static INIT: Once = Once::new();
    fn ini_log() {
        INIT.call_once(|| {
            build_logger("BACHUETECH", "UNIT TEST RUST LLAMA", LogLevel::VERBOSE, LogTarget::STD_ERROR, None );     
        });
    }

    #[test]
    fn test_load_from_str_success(){
        ini_log();

        //let test_config = get_yaml("fake_variable", "test_files/test-config_file.yml") ;
        const YML_CONTENT: &str = include_str!("../test_files/test-config_file.yml");
        println!("YAML Content: {:?}",&YML_CONTENT);
        let test_config = get_yaml_from_string(YML_CONTENT);
        println!("YAML: {:?}",&test_config);

        assert_eq!(test_config.unwrap()["app_name"].as_str().unwrap(),"BACHUETECH AI");
    }

    #[test]
    fn test_load_from_str_bad_file(){
        ini_log();

        //let test_config = get_yaml("fake_variable", "test_files/test-config_file.yml") ;
        const YML_CONTENT: &str = include_str!("../test_files/bad_content.yml");
        println!("YAML Content: {:?}",&YML_CONTENT);
        let test_config = get_yaml_from_string(YML_CONTENT);
        println!("YAML: {:?}",&test_config);

        assert!(test_config.unwrap()["app_name"].is_badvalue());
    }    

    #[test]
    fn test_relative_location(){
        ini_log();

        let test_config = get_yaml("fake_variable", "test_files/test-config_file.yml") ;
        
        println!("YAML: {:?}",&test_config);
        assert_eq!(test_config.unwrap()["app_name"].as_str().unwrap(),"BACHUETECH AI");
    }

    #[test]
    fn test_env_variable(){
        ini_log();

        let file_loc_var = "file_loc";
        unsafe { std::env::set_var(file_loc_var, "test_files/test-config_file.yml") };

        let test_config = get_yaml(file_loc_var, "fake_location/test-config_file.yml") ;
        
        println!("YAML: {:?}",&test_config);
        assert_eq!(test_config.unwrap()["app_name"].as_str().unwrap(),"BACHUETECH AI");
    }

    #[test]
    fn test_negative(){
        ini_log();
        let test_config = get_yaml("fake_variable", "fake_location/test-config_file.yml") ;
        assert!(test_config.is_err());
    }

    #[test]
    fn test_relative_location_vector(){
        ini_log();

        let test_config = get_yaml("fake_variable", "test_files/test-config_file.yml") ;
        let y = test_config.unwrap();
        let v = convert_yaml_to_vec_string(&y["tools"]);

        println!("YAML: {:?}",&v);
        assert_eq!(v.get(0).unwrap(),"do_math_expressions");
    }

    #[test]
    fn test_negative_vector(){
        ini_log();
        let test_config = get_yaml("fake_variable", "test_files/test-config_file.yml") ;
        let y = test_config.unwrap();
        let v = convert_yaml_to_vec_string(&y);

        assert_eq!(v.len(),0);
    }

    #[test]
    fn test_get_u32(){
        ini_log();
        let test_config = get_yaml("fake_variable", "test_files/test-config_file.yml") ;
        let y = test_config.unwrap();
        //let v = convert_yaml_to_vec_string(&y["size"]);

        assert_eq!(get_u32(Some(&y["size"]),0),4000);
    }

    #[test]
    fn test_get_u32_neg(){
        ini_log();
        let test_config = get_yaml("fake_variable", "test_files/test-config_file.yml") ;
        let y = test_config.unwrap();
        //let v = convert_yaml_to_vec_string(&y["size"]);

        assert_eq!(get_u32(Some(&y["nsize"]),100),0);
    }

    #[test]
    fn test_get_bool(){
        ini_log();
        let test_config = get_yaml("fake_variable", "test_files/test-config_file.yml") ;
        let y = test_config.unwrap();
        //let v = convert_yaml_to_vec_string(&y["size"]);

        assert_eq!(get_bool(Some(&y["read_all"]),false),true);
    } 

    #[test]
    fn test_get_negnum_i32(){
        ini_log();
        let test_config = get_yaml("fake_variable", "test_files/test-config_file.yml") ;
        let y = test_config.unwrap();
        //let v = convert_yaml_to_vec_string(&y["size"]);
        assert_eq!(get_i32(Some(&y["nsize"]),0),-1);
    }     

    #[test]
    fn test_get_posnum_i32(){
        ini_log();
        let test_config = get_yaml("fake_variable", "test_files/test-config_file.yml") ;
        let y = test_config.unwrap();
        //let v = convert_yaml_to_vec_string(&y["size"]);
        assert_eq!(get_i32(Some(&y["size"]),0),4000);
    }              
}
