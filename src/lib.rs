//use std::io::Error;
use std::error::Error;
use bt_file_utils::get_file;
use yaml_rust2::{Yaml, YamlLoader};
use bt_logger::{get_error, log_warning};
   
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
    pub fn get_yaml(env_variable: &str, or_file_name: &str) -> Result<Yaml,  Box<dyn Error>>{
        let config_yml_content: String; 
        match get_file(env_variable, or_file_name) {
            Ok(content) => config_yml_content = content,
            Err(e) => return Err(get_error!("get_yaml","Error getting File. Error: {}",e).into()),
        }

        let yml_config = YamlLoader::load_from_str(&config_yml_content)?;
        Ok(yml_config[0].clone())
    }

    /// Loads a YAML configuration from str
    pub fn get_yaml_from_string(file_content: &str) -> Result<Yaml,  Box<dyn Error>>{
        let yml_config = YamlLoader::load_from_str(file_content)?;
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
    /// - `default_val`: The default i32 value , in case the YML value is not an integer.
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

    /// Retrieves an unsigned integer (usize) from a YAML element.
    /// 
    /// If the YAML value exceeds `usize::MAX` or falls below `0`, it adjusts accordingly.
    /// 
    /// # Arguments
    /// - `yaml_val`: An optional reference to the YAML element.
    /// - `default_val`: The default usize value, in case the YML value is not an integer.
    /// 
    /// # Returns
    /// - `usize`: The extracted or adjusted value.
    pub fn get_usize(yaml_val: Option<&Yaml>, default_val: usize) -> usize{
        let r = yaml_val.map(|i| i.as_i64().unwrap_or(default_val as i64)).unwrap_or(default_val as i64);
        if r > usize::MAX as i64 {
            usize::MAX
        }else {
            if r < 0 as i64 {
                0
            }else{
                r as usize
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

    /// Retrieves a floating-point number (f32) from a YAML element.
    /// 
    /// If the YAML value does not exist or is invalid, it returns the provided default value.
    /// 
    /// # Arguments
    /// - `yaml_val`: An optional reference to the YAML element.
    /// - `default_val`: The default f32 value.
    /// 
    /// # Returns
    /// - `f32`: The extracted floating-point value.
    pub fn get_f32(yaml_val: Option<&Yaml>, default_val: f32) -> f32{
        let r = yaml_val.map(|i| i.as_f64().unwrap_or(default_val as f64)).unwrap_or(default_val as f64);
        if r > f32::MAX as f64 {
            f32::MAX
        }else {
            if r < f32::MIN as f64 {
                f32::MIN
            }else{
                r as f32
            }
        }
    }      