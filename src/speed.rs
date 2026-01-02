pub fn list_servers() -> Result<(), String>  {
    println!("Listing available servers...");
    Ok(())
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