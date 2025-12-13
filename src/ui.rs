use colored::Colorize;
use comfy_table::{Attribute, Cell, CellAlignment, Color, Table, presets::UTF8_BORDERS_ONLY};
use indicatif::{ProgressBar, ProgressStyle};

pub fn print_banner() {
    println!();
    let banner = r#"
    ╔═══════════════════════════════════════════════════════════════╗
    ║                  🔄  TOKEN PRICE AGGREGATOR  🔄               ║
    ║                  Multi-DEX Price Discovery                    ║
    ╚═══════════════════════════════════════════════════════════════╝
    "#;
    println!("{}", banner.cyan().bold());
    println!();
}

pub fn print_section_header(title: &str) {
    println!("{}", format!("\n▶ {}", title).bold().cyan());
    println!("{}", "─".repeat(60).cyan());
}

pub fn print_success(message: &str) {
    println!("{} {}", "✓".green().bold(), message);
}

pub fn print_error(message: &str) {
    println!("{} {}", "✗".red().bold(), message.red());
}

pub fn print_info(message: &str) {
    println!("{} {}", "ℹ".blue().bold(), message);
}

pub fn create_progress_bar(total: u64, message: &str) -> ProgressBar {
    let pb = ProgressBar::new(total);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.cyan} {msg} [{bar:40.cyan/blue}] {pos}/{len}")
            .expect("Failed to set progress bar style")
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    pb.set_message(message.to_string());
    pb
}

pub fn print_price_table(results: Vec<(&str, f64)>) {
    let mut table = Table::new();
    table.load_preset(UTF8_BORDERS_ONLY);
    table.set_header(vec![
        Cell::new("Token").add_attribute(Attribute::Bold),
        Cell::new("Price (USD)").add_attribute(Attribute::Bold),
        Cell::new("Status").add_attribute(Attribute::Bold),
    ]);

    for (token, price) in results {
        table.add_row(vec![
            Cell::new(token),
            Cell::new(format!("${:.6}", price))
                .set_alignment(CellAlignment::Right)
                .fg(Color::Green),
            Cell::new("✓ OK").fg(Color::Green),
        ]);
    }

    println!();
    println!("{}", table);
    println!();
}

pub fn print_token_result(name: &str, price: f64, pool_count: usize) {
    let status = if price > 0.0 {
        "✓".green()
    } else {
        "✗".red()
    };
    println!(
        "  {} {} {:<15} ${:<12.6} {} {} {}",
        status,
        "│".cyan(),
        name.bold(),
        price,
        "│".cyan(),
        format!("({} pools)", pool_count).dimmed(),
        "│".cyan()
    );
}

pub fn print_footer(duration: std::time::Duration) {
    println!();
    println!("{}", "─".repeat(60).cyan());
    println!(
        "{} {}",
        "Completed in".cyan(),
        format!("{:.2}s", duration.as_secs_f64()).yellow().bold()
    );
    println!();
}
