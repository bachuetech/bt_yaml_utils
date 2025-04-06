use std::io::Error;
use bt_file_utils::get_file;
use yaml_rust2::{Yaml, YamlLoader};
use bt_logger::log_warning;
   
    pub fn get_yaml(env_variable: &str, or_file_name: &str) -> Result<Yaml, Error>{
        let config_yml_content: String; 
        match get_file(env_variable, or_file_name) {
            Ok(content) => config_yml_content = content,
            Err(e) => return Err(e),
        }

        let yml_config = YamlLoader::load_from_str(&config_yml_content).unwrap();
        Ok(yml_config[0].clone())
    }

    pub fn convert_yaml_to_vec_string(yaml: &Yaml) -> Vec<String> {
        // Ensure the YAML is a sequence (list)
        if let Yaml::Array(array) = yaml {
            // Convert each item in the array to a String
            array.iter().filter_map(|item| {
                // Ensure the item is a string and then convert it
                if let Yaml::String(s) = item {
                    Some(s.clone())  // clone the string into the Vec
                } else {
                    None
                }
            }).collect()
        } else {
            log_warning!("convert_yaml_to_vec_string","YAML is not a sequence (LIST). Returning empty Vector");
            Vec::new()  // Return an empty Vec if not a sequence
        }
    }