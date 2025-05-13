
//***********/
// UNIT TEST 
//***********/
#[cfg(test)]
mod test_yaml_utils{
    use bt_logger::{build_logger, LogLevel, LogTarget};
    use bt_yaml_utils::{convert_yaml_to_vec_string, get_bool, get_i32, get_u32, get_yaml};

    #[test]
    fn test_relative_location(){
        build_logger("BACHUETECH", "BT.YAML.UTILS", LogLevel::VERBOSE, LogTarget::STD_ERROR );

        let test_config = get_yaml("fake_variable", "test_files/test-config_file.yml") ;
        
        println!("YAML: {:?}",&test_config);
        assert_eq!(test_config.unwrap()["app_name"].as_str().unwrap(),"BACHUETECH AI");
    }

    #[test]
    fn test_env_variable(){
        build_logger("BACHUETECH", "BT.YAML.UTILS", LogLevel::VERBOSE, LogTarget::STD_ERROR );

        let file_loc_var = "file_loc";
        unsafe { std::env::set_var(file_loc_var, "test_files/test-config_file.yml") };

        let test_config = get_yaml(file_loc_var, "fake_location/test-config_file.yml") ;
        
        println!("YAML: {:?}",&test_config);
        assert_eq!(test_config.unwrap()["app_name"].as_str().unwrap(),"BACHUETECH AI");
    }

    #[test]
    fn test_negative(){
        build_logger("BACHUETECH", "BT.YAML.UTILS", LogLevel::VERBOSE, LogTarget::STD_ERROR );
        let test_config = get_yaml("fake_variable", "fake_location/test-config_file.yml") ;
        assert!(test_config.is_err());
    }

    #[test]
    fn test_relative_location_vector(){
        build_logger("BACHUETECH", "BT.YAML.UTILS", LogLevel::VERBOSE, LogTarget::STD_ERROR );

        let test_config = get_yaml("fake_variable", "test_files/test-config_file.yml") ;
        let y = test_config.unwrap();
        let v = convert_yaml_to_vec_string(&y["tools"]);

        println!("YAML: {:?}",&v);
        assert_eq!(v.get(0).unwrap(),"do_math_expressions");
    }

    #[test]
    fn test_negative_vector(){
        build_logger("BACHUETECH", "BT.YAML.UTILS", LogLevel::VERBOSE, LogTarget::STD_ERROR );
        let test_config = get_yaml("fake_variable", "test_files/test-config_file.yml") ;
        let y = test_config.unwrap();
        let v = convert_yaml_to_vec_string(&y);

        assert_eq!(v.len(),0);
    }

    #[test]
    fn test_get_u32(){
        build_logger("BACHUETECH", "BT.YAML.UTILS", LogLevel::VERBOSE, LogTarget::STD_ERROR );
        let test_config = get_yaml("fake_variable", "test_files/test-config_file.yml") ;
        let y = test_config.unwrap();
        //let v = convert_yaml_to_vec_string(&y["size"]);

        assert_eq!(get_u32(Some(&y["size"]),0),4000);
    }

    #[test]
    fn test_get_u32_neg(){
        build_logger("BACHUETECH", "BT.YAML.UTILS", LogLevel::VERBOSE, LogTarget::STD_ERROR );
        let test_config = get_yaml("fake_variable", "test_files/test-config_file.yml") ;
        let y = test_config.unwrap();
        //let v = convert_yaml_to_vec_string(&y["size"]);

        assert_eq!(get_u32(Some(&y["nsize"]),100),0);
    }

    #[test]
    fn test_get_bool(){
        build_logger("BACHUETECH", "BT.YAML.UTILS", LogLevel::VERBOSE, LogTarget::STD_ERROR );
        let test_config = get_yaml("fake_variable", "test_files/test-config_file.yml") ;
        let y = test_config.unwrap();
        //let v = convert_yaml_to_vec_string(&y["size"]);

        assert_eq!(get_bool(Some(&y["read_all"]),false),true);
    } 

    #[test]
    fn test_get_negnum_i32(){
        build_logger("BACHUETECH", "BT.YAML.UTILS", LogLevel::VERBOSE, LogTarget::STD_ERROR );
        let test_config = get_yaml("fake_variable", "test_files/test-config_file.yml") ;
        let y = test_config.unwrap();
        //let v = convert_yaml_to_vec_string(&y["size"]);
        assert_eq!(get_i32(Some(&y["nsize"]),0),-1);
    }     

    #[test]
    fn test_get_posnum_i32(){
        build_logger("BACHUETECH", "BT.YAML.UTILS", LogLevel::VERBOSE, LogTarget::STD_ERROR );
        let test_config = get_yaml("fake_variable", "test_files/test-config_file.yml") ;
        let y = test_config.unwrap();
        //let v = convert_yaml_to_vec_string(&y["size"]);
        assert_eq!(get_i32(Some(&y["size"]),0),4000);
    }              
}
