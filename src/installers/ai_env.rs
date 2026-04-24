use crate::core::cmd;
use super::apt;
use anyhow::Result;
use crate::core::models::InstallStatus;
use crate::core::version;

pub fn install_ai_media() -> Result<InstallStatus> {
    let required_cmds = ["ffmpeg", "convert", "pdftotext", "tesseract"];
    let all_exist = required_cmds.iter().all(|&cmd| cmd::command_exists(cmd));
    
    if all_exist {
        let ver = version::get_generic_version("ffmpeg");
        println!("{} AI Media Tools are already installed ({})", InstallStatus::AlreadyExists(String::new()).icon(), ver);
        return Ok(InstallStatus::AlreadyExists(ver));
    }
    
    println!("⏳ Installing AI Media & Docs Tools (ffmpeg, imagemagick, poppler-utils, tesseract-ocr)...");
    
    apt::update()?;
    apt::install(&[
        "ffmpeg", 
        "imagemagick", 
        "poppler-utils", 
        "tesseract-ocr",
        "tesseract-ocr-eng",
        "tesseract-ocr-chi-sim"
    ])?;
    
    let ver = version::get_generic_version("ffmpeg");
    println!("{} AI Media Tools installed successfully ({})", InstallStatus::Installed(String::new()).icon(), ver);
    Ok(InstallStatus::Installed(ver))
}

pub fn install_web_auto() -> Result<InstallStatus> {
    let required_cmds = ["xvfb-run"];
    let all_exist = required_cmds.iter().all(|&cmd| cmd::command_exists(cmd));
    
    if all_exist {
        let ver = version::get_generic_version("xvfb-run");
        println!("{} Web Automation Dependencies are already installed ({})", InstallStatus::AlreadyExists(String::new()).icon(), ver);
        return Ok(InstallStatus::AlreadyExists(ver));
    }
    
    println!("⏳ Installing Web Automation Dependencies (Puppeteer/Playwright deps)...");
    
    apt::update()?;
    apt::install(&[
        "xvfb", 
        "libnss3", 
        "libgbm-dev", 
        "libasound2", 
        "libatk1.0-0",
        "libatk-bridge2.0-0",
        "libcups2",
        "libdrm2",
        "libxkbcommon0",
        "libxcomposite1",
        "libxdamage1",
        "libxfixes3",
        "libxrandr2",
        "libgbm1",
        "libpango-1.0-0",
        "libcairo2"
    ])?;
    
    let ver = version::get_generic_version("xvfb-run");
    println!("{} Web Automation Dependencies installed successfully ({})", InstallStatus::Installed(String::new()).icon(), ver);
    Ok(InstallStatus::Installed(ver))
}
