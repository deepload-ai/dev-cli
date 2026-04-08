use crate::core::models::ToolInfo;
use console::style;

pub fn print_summary(tools: &[ToolInfo]) {
    println!("\n{}", style("=================================================================================").cyan());
    println!("  {}", style("DevEnv CLI - Component Summary").bold().cyan());
    println!("{}", style("=================================================================================").cyan());
    println!(" {:<20} | {:<25} | {}", "Component", "Status", "Version");
    println!("{}", style("----------------------+---------------------------+------------------------------").dim());
    
    for tool in tools {
        // We use string manipulation to ensure the icons align correctly even with emoji widths
        let status_text = format!("{} {}", tool.status.icon(), tool.status.text());
        
        println!(" {:<20} | {:<25} | {}", 
            style(&tool.name).bold(), 
            status_text, 
            tool.status.version()
        );
    }
    println!("{}", style("=================================================================================").cyan());
}
