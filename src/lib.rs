use std::io::Error;
use bt_file_utils::get_file;
use yaml_rust2::{Yaml, YamlLoader};
use bt_logger::log_warning;
   
    /// Loads a YAML configuration file.
    /// 
    /// This function retrieves the YAML content either from an environment variable or a specified file.
    /// It then parses the YAML content and returns the first element of the parsed YAML structure.
    /// 
    /// # Arguments
    /// - `env_variable`: The name of the environment variable that may contain the file path.
    /// - `or_file_name`: A fallback file name if the environment variable is not set.
    /// 
    /// # Returns
    /// - `Ok(Yaml)`: The parsed YAML data.
    /// - `Err(Error)`: If the file cannot be retrieved.
    pub fn get_yaml(env_variable: &str, or_file_name: &str) -> Result<Yaml, Error>{
        let config_yml_content: String; 
        match get_file(env_variable, or_file_name) {
            Ok(content) => config_yml_content = content,
            Err(e) => return Err(e),
        }

        let yml_config = YamlLoader::load_from_str(&config_yml_content).unwrap();
        Ok(yml_config[0].clone())
    }

    /// Converts a YAML sequence into a vector of strings.
    /// 
    /// This function checks if the provided YAML value is a list and extracts string values from it.
    /// 
    /// # Arguments
    /// - `yaml`: A reference to the YAML structure.
    /// 
    /// # Returns
    /// - `Vec<String>`: A vector containing extracted string values.
    /// 
    /// # Example
    /// ```
    /// use yaml_rust2::Yaml;
    /// use bt_yaml_utils::convert_yaml_to_vec_string;
    /// let yaml_data = Yaml::Array(vec![Yaml::String("item1".to_string()), Yaml::String("item2".to_string())]);
    /// let result = convert_yaml_to_vec_string(&yaml_data);
    /// assert_eq!(result, vec!["item1", "item2"]);
    /// ```
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

    /// Retrieves a boolean value from a YAML element.
    /// 
    /// If the YAML value does not exist or is invalid, it returns the provided default value.
    /// 
    /// # Arguments
    /// - `yaml_val`: An optional reference to the YAML element.
    /// - `default_val`: The default boolean value.
    /// 
    /// # Returns
    /// - `bool`: The extracted boolean value or the default.
    pub fn get_bool(yaml_val: Option<&Yaml>, default_val: bool) -> bool {
        yaml_val.map(|b| b.as_bool().unwrap_or(default_val)).unwrap_or(default_val)
    }

    /// Retrieves an unsigned integer (u32) from a YAML element.
    /// 
    /// If the YAML value is negative or exceeds `u32::MAX`, it adjusts accordingly.
    /// 
    /// # Arguments
    /// - `yaml_val`: An optional reference to the YAML element.
    /// - `default_val`: The default u32 value.
    /// 
    /// # Returns
    /// - `u32`: The extracted or adjusted value.
    pub fn get_u32(yaml_val: Option<&Yaml>, default_val: u32) -> u32{
        let r = yaml_val.map(|u| u.as_i64().unwrap_or(default_val.into())).unwrap_or(default_val.into());
        if r < 0 {
            return 0; 
        } else{
            if r > u32::MAX as i64 {
                u32::MAX
            }else {
                r as u32
            }
        }
    }

    /// Retrieves a signed integer (i32) from a YAML element.
    /// 
    /// If the YAML value exceeds `i32::MAX` or falls below `i32::MIN`, it adjusts accordingly.
    /// 
    /// # Arguments
    /// - `yaml_val`: An optional reference to the YAML element.
    /// - `default_val`: The default i32 value.
    /// 
    /// # Returns
    /// - `i32`: The extracted or adjusted value.
    pub fn get_i32(yaml_val: Option<&Yaml>, default_val: i32) -> i32{
        let r = yaml_val.map(|i| i.as_i64().unwrap_or(default_val.into())).unwrap_or(default_val.into());
        if r > i32::MAX as i64 {
            i32::MAX
        }else {
            if r < i32::MIN as i64 {
                i32::MIN
            }else{
                r as i32
            }
        }
    }  

    /// Retrieves a floating-point number (f64) from a YAML element.
    /// 
    /// If the YAML value does not exist or is invalid, it returns the provided default value.
    /// 
    /// # Arguments
    /// - `yaml_val`: An optional reference to the YAML element.
    /// - `default_val`: The default f64 value.
    /// 
    /// # Returns
    /// - `f64`: The extracted floating-point value.

    pub fn get_f64(yaml_val: Option<&Yaml>, default_val: f64) -> f64{
        yaml_val.map(|i| i.as_f64().unwrap_or(default_val.into())).unwrap_or(default_val.into())
    } 