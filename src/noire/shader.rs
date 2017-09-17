#![allow(dead_code)]

extern crate gl;
extern crate regex;

use self::gl::types::*;
use self::regex::Regex;

use std::ffi::CString;
use std::cmp;
use std::ptr;
use std::str;

pub enum ShaderType {
    VertexShader,
    PixelShader,
}

pub struct Shader {
    pub source: String,
    pub id: u32,
}

fn gl_shader_type(shader_type: ShaderType) -> u32 {
    match shader_type {
        ShaderType::VertexShader => gl::VERTEX_SHADER,
        ShaderType::PixelShader => gl::FRAGMENT_SHADER,
    }
}

fn log_compile_info(shader: u32) -> String {
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

fn get_source_location(line: i32, source: &String) -> String {
    let shader_lines: Vec<&str> = source.split('\n').collect();
    let min: usize = cmp::max(0, (line - 3) as usize);
    let max: usize = cmp::min(shader_lines.len(), (line + 2) as usize);

    let mut result: Vec<String> = vec![];
    for i in min..max {
        result.push(format!("{}: {}", i, &shader_lines.get(i).unwrap()));
    }
    result.join("\n").to_string()
}

fn get_errors(errors: &String, source: &String) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    let lines: Vec<&str> = errors.split('\n').collect();
    for line in lines {
        let regex = Regex::new(r"/ERROR:\s([0-9]*):([0-9]*):\s(.*)/").unwrap();
        let groups = regex.captures(line).unwrap();

        if groups.len() > 1 {
            let error_msg: &str = groups.get(2).map_or("", |m| m.as_str());
            result.push(get_source_location(
                error_msg.parse::<i32>().unwrap(),
                source,
            ));
        }
    }
    result
}

pub fn compile_shader(source: &str, shader_type: ShaderType) -> Result<u32, String> {
    let shader;
    unsafe {
        shader = gl::CreateShader(gl_shader_type(shader_type));
        let c_str = CString::new(source.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        let mut status = gl::FALSE as i32;

        // check if shader compiled correctly
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        if status != gl::TRUE as i32 {
            let log_text = log_compile_info(shader);
            let error_msg = get_errors(&log_text, &source.to_string()).join("\n");
            return Err(error_msg);
        }
    }
    Ok(shader)
}

impl Shader {
    pub fn create(source: &str, shader_type: ShaderType) -> Shader {
        let id = compile_shader(source, shader_type).unwrap();
        Shader {
            id: id,
            source: String::from(source),
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
