use speedtest_rs::speedtest;

pub fn list_servers() -> Result<(), String>  {
    println!("Listing available servers...");

    let config = speedtest::get_configuration()
        .map_err(|e| format!("Failed to get config: {:?}", e))?;

    let servers = speedtest::get_server_list_with_config(&config)
        .map_err(|e| format!("Failed to get server list: {:?}", e))?;

    let sorted_servers = servers.servers_sorted_by_distance(&config);

    println!("\nTop 10 Closest Servers:");
    println!("{:<10} {:<25} {:<15} {:<10}", "ID", "Sponsor", "Name", "Dist (km)");
    println!("{}", "-".repeat(65));

    for server in sorted_servers.iter().take(10) {
        println!(
            "{:<10} {:<25} {:<15} {:.2}",
            server.id,
            ellipsize(&server.sponsor, 24), 
            ellipsize(&server.name, 14),
            server.distance.unwrap_or(0.0)
        );
    }

    Ok(())
}

fn ellipsize(text: &str, max_len: usize) -> String {
    if text.len() > max_len {
        format!("{}...", &text[..max_len-3])
    } else {
        text.to_string()
    }
}

pub fn do_test(server_id: Option<String>, do_down: bool, do_up: bool) -> Result<(), String> {
    if let Some(id) = server_id {
        println!("Testing with server ID: {}", id);
    } else {
        println!("Testing with default server...");
    }

    if do_down {
        println!("Performing download speed test...");
    }
    if do_up {
        println!("Performing upload speed test...");
    }
    Ok(())
}