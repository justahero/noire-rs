use gl;
use gl::types::*;
use regex::Regex;

use std::fs::File;
use std::path::Path;
use std::cmp;
use std::ffi::CString;
use std::io::prelude::*;
use std::ptr;
use std::str;

#[derive(Debug)]
pub struct Shader {
    pub source: String,
    pub id: u32,
    pub shader_type: GLenum,
}

fn get_compile_error(shader: u32) -> String {
    let log_text: String;
    unsafe {
        let mut length: i32 = 0;
        gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut length);
        let mut buffer = Vec::with_capacity(length as usize);
        buffer.set_len((length as usize) - 1);

        gl::GetShaderInfoLog(
            shader,
            length,
            ptr::null_mut(),
            buffer.as_mut_ptr() as *mut GLchar,
        );

        log_text = str::from_utf8(&buffer)
            .ok()
            .expect("ShaderInfoLog not valid utf8")
            .to_string();
    }
    log_text
}

fn get_source_location(line: i32, source: &str) -> String {
    let shader_lines: Vec<&str> = source.split('\n').collect();
    let min = cmp::max(0, line - 3);
    let max = cmp::min(shader_lines.len() as i32, line + 2);

    let mut result: Vec<String> = vec![];
    for i in (min as usize)..(max as usize) {
        result.push(format!("{}: {}", i, &shader_lines.get(i).unwrap()));
    }
    result.join("\n").to_string()
}

fn get_errors(errors: &String, source: &str) -> Vec<String> {
    let mut result = Vec::new();
    let lines: Vec<&str> = errors.split('\n').collect();
    for line in lines {
        let regex = Regex::new(r"ERROR:\s([0-9]*):([0-9]*):\s(.*)").unwrap();

        if let Some(groups) = regex.captures(line) {
            let location = groups.get(2).unwrap().as_str();
            let error = groups.get(3).unwrap().as_str();
            let text = get_source_location(location.parse::<i32>().unwrap(), source);
            result.push(format!("{}: in:\n{}", error, text));
        }
    }
    result
}

fn compile_shader(source: &str, shader_type: GLenum) -> Result<u32, String> {
    let c_str = CString::new(source.as_bytes()).unwrap();

    let shader = unsafe { gl::CreateShader(shader_type) };
    unsafe {
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        let mut status: GLint = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        if status != (gl::TRUE as GLint) {
            let log_text = get_compile_error(shader);
            let error_msg = get_errors(&log_text, source).join("\n");
            return Err(error_msg);
        }
    }
    Ok(shader)
}

impl Shader {
    pub fn from_file(file_path: &String, shader_type: GLenum) -> Result<Self, String> {
        let path = Path::new(file_path);
        let display = path.display();

        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(_) => return Err(format!("Failed to open file {}", display)),
        };

        let mut source = String::new();
        let source = match file.read_to_string(&mut source) {
            Ok(_) => source,
            Err(_) => return Err(format!("Could not read content from file {}", display)),
        };

        Shader::create(&source, shader_type)
    }

    pub fn create(source: &str, shader_type: GLenum) -> Result<Self, String> {
        match compile_shader(source, shader_type) {
            Ok(id) => {
                Ok(Shader {
                    id,
                    source: String::from(source),
                    shader_type,
                })
            }
            Err(message) => Err(message),
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        assert!(self.id > 0);
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}
